// 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
// Copyright 2022 Noelware <team@noelware.org>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package main

import (
	"context"
	"database/sql"
	"fmt"
	"os"
	"strings"

	_ "github.com/ClickHouse/clickhouse-go"
	"github.com/golang-migrate/migrate/v4"
	"github.com/golang-migrate/migrate/v4/database/clickhouse"
	_ "github.com/golang-migrate/migrate/v4/source/file"
	"github.com/joho/godotenv"
	"github.com/sirupsen/logrus"
)

type logrusMigrateLogger struct {}

func (*logrusMigrateLogger) Printf(format string, v ...any) {
	logrus.Infof(format, v...)
}

func (*logrusMigrateLogger) Verbose() bool {
	return true
}

func init() {
	if _, err := os.Stat("./.env"); !os.IsNotExist(err) {
		if err := godotenv.Load("./.env"); err != nil {
			panic(err)
		}
	}

	keys := []string{
		"CLICKHOUSE_HOST",
		"CLICKHOUSE_PORT",
		"CLICKHOUSE_DATABASE",
	}

	for _, key := range keys {
		if _, exists := os.LookupEnv(key); !exists {
			fmt.Printf("[preinit] Missing environment variable [%s]\n", key)
			os.Exit(1)
		}
	}
}

func main() {
	logrus.Info("Running migrations...")
	dsn := getDatabaseUrl()
	logrus.Infof("Using ClickHouse DSN [%s]", dsn)

	db, err := sql.Open("clickhouse", dsn)
	if err != nil {
		logrus.Fatalf("Unable to open to ClickHouse via database/sql due to [%s]!", err)
	}

	if err := db.PingContext(context.Background()); err != nil {
		logrus.Fatalf("Unable to connect to ClickHouse: [%s]", err)
	}

	instance, err := clickhouse.WithInstance(db, &clickhouse.Config{
		MigrationsTable:       "migrations",
		MultiStatementEnabled: true,
	})

	if err != nil {
		logrus.Fatalf("Unable to create migration engine with DSN [%s] due to [%s]", dsn, err)
	}

	m, err := migrate.NewWithDatabaseInstance("file://./migrations", "clickhouse", instance)
	m.Log = &logrusMigrateLogger{}

	if err != nil {
		logrus.Fatalf("Unable to create migration engine with DSN [%s] due to [%s]", dsn, err)
	}

	logrus.Infof("Created migration engine with DSN [%s], running migrations...", dsn)
	if err := m.Up(); err != nil {
		logrus.Fatalf("Unable to run migrations due to [%s]", err)
	}

	logrus.Info("Successfully ran migrations~ ^_^")
	srcErr, dbErr := m.Close()
	if srcErr != nil {
		logrus.Errorf("Unable to close migration engine [%s]", srcErr)
	}

	if dbErr != nil {
		logrus.Errorf("Unable to close ClickHouse instance [%s]", dbErr)
	}
}

func getDatabaseUrl() string {
	builder := strings.Builder{}
	builder.WriteString("clickhouse://")

	var username *string
	var password *string

	// Check if we have `CLICKHOUSE_USERNAME` and `CLICKHOUSE_PASSWORD`
	if usr, exists := os.LookupEnv("CLICKHOUSE_USERNAME"); exists {
		username = &usr
	}

	if pwd, exists := os.LookupEnv("CLICKHOUSE_PASSWORD"); exists {
		password = &pwd
	}

	if username != nil && password != nil {
		builder.WriteString(fmt.Sprintf("%v:%v@", string(*username), string(*password)))
	}

	host := os.Getenv("CLICKHOUSE_HOST")
	port := os.Getenv("CLICKHOUSE_PORT")
	database := os.Getenv("CLICKHOUSE_DATABASE")
	builder.WriteString(fmt.Sprintf("%s:%s/%s", host, port, database))

	return builder.String()
}
