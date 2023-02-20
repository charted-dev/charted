// 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

package main

import (
	"context"
	"errors"
	"fmt"
	"os"
	"runtime"
	"strings"
	"time"

	"cdr.dev/slog"
	"cdr.dev/slog/sloggers/sloghuman"
	ch "github.com/ClickHouse/clickhouse-go/v2"
	"github.com/golang-migrate/migrate/v4"
	"github.com/golang-migrate/migrate/v4/database/clickhouse"
	_ "github.com/golang-migrate/migrate/v4/source/file"
	"github.com/spf13/cobra"
)

var (
	version = "master"
	log     = slog.Make(sloghuman.Sink(os.Stdout))

	options = CliOptions{}
	rootCmd = &cobra.Command{
		Use:          "ch-migrations",
		Short:        "Manages and runs the migrations for ClickHouse",
		RunE:         execute,
		SilenceUsage: true,
		Version:      fmt.Sprintf("ch-migrations %s on %s/%s", version, runtime.GOOS, runtime.GOARCH),
	}
)

type CliOptions struct {
	migrationsTableName string
	username            string
	password            string
	database            string
	timeout             time.Duration
	hosts               []string
}

func init() {
	rootCmd.Flags().StringVarP(&options.migrationsTableName, "migrations-table", "t", "migrations", "Table name for holding migration metadata")
	rootCmd.Flags().StringSliceVar(&options.hosts, "hosts", []string{"localhost:9000"}, "List of ClickHouse nodes to connect to")
	rootCmd.Flags().DurationVar(&options.timeout, "timeout", time.Second*15, "timeout from connecting to ClickHouse nodes")
	rootCmd.Flags().StringVarP(&options.username, "username", "u", "", "username when connecting to the ClickHouse nodes")
	rootCmd.Flags().StringVarP(&options.password, "password", "p", "", "password when connecting to the ClickHouse nodes")
	rootCmd.Flags().StringVarP(&options.database, "database", "d", "charted", "database to connect to")
}

func main() {
	if err := rootCmd.Execute(); err != nil {
		log.Fatal(context.TODO(), "failed to run migrations", slog.F("err", err))
		os.Exit(1)
	}
}

func connectionUri() (*string, error) {
	b := &strings.Builder{}
	b.WriteString("clickhouse://")

	for i, host := range options.hosts {
		if options.username != "" {
			if options.password == "" {
				return nil, errors.New("missing 'password' flag, which is required when using --username flag")
			}

			b.WriteString(fmt.Sprintf("%s:%s@", options.username, options.password))
		}

		b.WriteString(host)

		if (i + 1) != len(options.hosts) {
			b.WriteRune(',')
		}
	}

	if options.database != "" {
		b.WriteRune('/')
		b.WriteString(options.database)
	}

	url := b.String()
	return &url, nil
}

func execute(_ *cobra.Command, _ []string) error {
	url, err := connectionUri()
	if err != nil {
		return err
	}

	log.Info(context.TODO(), "preparing clickhouse connection", slog.F("url", url))
	opts := &ch.Options{
		Addr:        options.hosts,
		DialTimeout: options.timeout,
		Settings: ch.Settings{
			"allow_experimental_object_type": true,
		},
		Debugf: func(format string, v ...interface{}) {
			log.Debug(context.TODO(), fmt.Sprintf(format, v...))
		},
	}

	auth := ch.Auth{Database: options.database}
	if options.username != "" && options.password != "" {
		auth.Username = options.username
		auth.Password = options.password
	}

	opts.Auth = auth
	db := ch.OpenDB(opts)

	log.Info(context.TODO(), "attempting to ping database...", slog.F("url", url))
	if err := db.Ping(); err != nil {
		return err
	}

	config := &clickhouse.Config{
		MultiStatementEnabled: true,
		MigrationsTable:       options.migrationsTableName,
		DatabaseName:          options.database,
	}

	ch, err := clickhouse.WithInstance(db, config)
	if err != nil {
		return err
	}

	migrator, err := migrate.NewWithDatabaseInstance("file://./migrations", "clickhouse", ch)
	if err != nil {
		return err
	}

	log.Info(context.TODO(), "created migration engine! running pending migrations...")
	if err = migrator.Up(); err != nil {
		if errors.Is(err, migrate.ErrNoChange) {
			log.Info(context.TODO(), "nothing to do!")
			return nil
		}

		return err
	}

	log.Info(context.TODO(), "finished all migrations~! ^-^")
	srcErr, dbErr := migrator.Close()
	if srcErr != nil {
		return srcErr
	}

	if dbErr != nil {
		return dbErr
	}

	return nil
}
