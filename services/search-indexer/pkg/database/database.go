// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package database

import (
	_ "github.com/lib/pq"
	"github.com/sirupsen/logrus"

	"context"
	"database/sql"
	"fmt"
	"time"

	"charts.noelware.org/search-indexer/pkg/config"
)

var (
	// Tables is a list of Postgres tables that we need to listen and notify for.
	Tables []string = []string{"users", "repositories", "organizations", "repository_releases"}

	// CreateNotifierFunctionScript is a SQL script that runs to install the trigger.
	CreateNotifierFunctionScript = `CREATE OR REPLACE FUNCTION indexer_notify_event() RETURNS TRIGGER AS $$
    DECLARE
        data json;
        notification json;
    BEGIN
        if (TG_OP = 'DELETE') THEN
            data = row_to_json(OLD);
        ELSE
            data = row_to_json(NEW);
        END IF;

        notification = json_build_object(
            'table', TG_TABLE_NAME,
            'action', TG_OP,
            'data', data
        );

        PERFORM pg_notify('charted.indexing'::text, notification::text);
        RETURN NULL;
    END
$$ LANGUAGE plpgsql;
`

	InstallTriggerScript = `CREATE TRIGGER %s_indexing_event_trigger
AFTER INSERT OR UPDATE OR DELETE ON %s
FOR EACH ROW EXECUTE PROCEDURE indexer_notify_event();
`

	// ListenChannel is the channel to listen to when receving events.
	ListenChannel = "charted.indexing"
)

// Database represents the database connection to receive NOTIFY events from.
type Database struct {
	db *sql.DB
}

// New creates a new database connection and keeps the lifetime of the
// connection until Dispose() is called.
func New(config *config.Config) (*Database, error) {
	logrus.Debug("connecting to postgres")

	conn, err := sql.Open("postgres", config.DatabaseURL)
	if err != nil {
		logrus.Error("unable to open database: ", err)
		return nil, err
	}

	ctx, cancel := context.WithTimeout(context.TODO(), 100*time.Millisecond)
	defer cancel()

	if err := conn.PingContext(ctx); err != nil {
		logrus.Error("unable to connect to database in 100ms: ", err)
		return nil, fmt.Errorf("unable to connect to database in 100ms: %v", err)
	}

	logrus.Info("connected to postgres successfully!")
	return &Database{db: conn}, nil
}

// Dispose will dispose the database connection's lifetime and call Dispose on all
// of the listeners
func (d *Database) Dispose() error {
	err := d.db.Close()
	if err != nil {
		logrus.Error("unable to dispose db connection:", err)
		return err
	}

	return nil
}

// InstallTriggers will install the necessary triggers for the listeners
// to work
func (d *Database) InstallTriggers() error {
	start := time.Now()
	ctx, cancel := context.WithCancel(context.TODO())
	tx, err := d.db.BeginTx(ctx, &sql.TxOptions{
		ReadOnly:  false,
		Isolation: sql.LevelRepeatableRead,
	})

	if err != nil {
		cancel()

		logrus.Error(fmt.Sprintf("unable to create db transaction [%s]: %v", time.Since(start).String(), err))
		return err
	}

	if _, err = tx.Exec(CreateNotifierFunctionScript); err != nil {
		cancel()

		logrus.Error(fmt.Sprintf("unable to create trigger function [%s]: %v", time.Since(start).String(), err))
		return fmt.Errorf("unable to create trigger function: %v", err)
	}

	for _, table := range Tables {
		if _, err := tx.Exec(fmt.Sprintf("DROP TRIGGER IF EXISTS %s_indexing_event_trigger ON %s;", table, table)); err != nil {
			cancel()

			logrus.Error(fmt.Sprintf("unable to drop trigger (if it existed) for table %s [%s]: %v", table, time.Since(start).String(), err))
			return err
		}

		if _, err := tx.Exec(fmt.Sprintf(InstallTriggerScript, table, table)); err != nil {
			cancel()

			logrus.Error(fmt.Sprintf("unable to install trigger script for table %s [%s]: %v", table, time.Since(start).String(), err))
			return fmt.Errorf("unable to install trigger script for table %s: %v", table, err)
		}

		logrus.Info("installed or updated trigger on table [", table, "] successfully [", time.Since(start).String(), "]")
	}

	return tx.Commit()
}
