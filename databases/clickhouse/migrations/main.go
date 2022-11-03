// 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
// Copyright 2022 Noelware <team@noelware.org>
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
	rootCmd = &cobra.Command{
		Use:           "clickhouse-migrations",
		Short:         "Manages and runs the migrations for charted-server's ClickHouse database.",
		RunE:          execute,
		SilenceErrors: true,
		SilenceUsage:  true,
	}

	log = slog.Make(sloghuman.Sink(os.Stdout))

	tableName *string
	host      *string
	port      *int
	timeout   *string
	username  *string
	password  *string
	database  *string
)

func init() {
	tableName = rootCmd.Flags().StringP("table", "t", "migrations", "The migrations table name")
	host = rootCmd.Flags().String("host", "127.0.0.1", "host to connect to")
	port = rootCmd.Flags().IntP("port", "p", 9000, "port (tcp interface) to connect to")
	timeout = rootCmd.Flags().String("timeout", "15s", "timeout from connecting to server")
	username = rootCmd.Flags().StringP("username", "u", "", "username for authentication when connecting")
	password = rootCmd.Flags().String("password", "", "password for authentication when connecting")
	database = rootCmd.Flags().StringP("database", "d", "charted", "database name")
}

func main() {
	if err := rootCmd.Execute(); err != nil {
		log.Fatal(context.TODO(), "failed to run command line runner", slog.F("err", err))
		os.Exit(1)
	}
}

func connectionUrl() (*string, error) {
	b := &strings.Builder{}
	b.WriteString("clickhouse://")

	if username != nil && *username != "" {
		if password == nil || *password == "" {
			return nil, errors.New("missing 'password' flag, which is required when using --username flag")
		}

		b.WriteString(fmt.Sprintf("%s:%s@", string(*username), string(*password)))
	}

	b.WriteString(fmt.Sprintf("%s:%d", string(*host), int(*port)))

	if database != nil {
		b.WriteRune('/')
		b.WriteString(*database)
	}

	url := b.String()
	return &url, nil
}

func execute(_ *cobra.Command, _ []string) error {
	url, err := connectionUrl()
	if err != nil {
		return err
	}

	t, err := time.ParseDuration(*timeout)
	if err != nil {
		return err
	}

	log.Info(context.TODO(), "preparing clickhouse connection with url", slog.F("url", url))
	opts := &ch.Options{
		Addr:        []string{fmt.Sprintf("%s:%d", *host, *port)},
		DialTimeout: t,
		Settings: ch.Settings{
			"allow_experimental_object_type": true,
		},
		Debugf: func(format string, v ...interface{}) {
			log.Debug(context.TODO(), fmt.Sprintf(format, v...))
		},
	}

	auth := ch.Auth{Database: *database}

	// safe because `username` and `password` are validated in connectionUrl()
	if username != nil && password != nil {
		if *username != "" {
			auth.Username = *username
		}

		if *password != "" {
			auth.Password = *password
		}
	}

	opts.Auth = auth

	// TODO(@auguwu): support SSL !

	db := ch.OpenDB(opts)
	if err := db.Ping(); err != nil {
		return err
	}

	config := &clickhouse.Config{
		DatabaseName:          *database,
		MigrationsTable:       *tableName,
		MultiStatementEnabled: true,
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
