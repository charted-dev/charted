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
	"errors"
	"fmt"
	"os"
	"strings"
	"time"

	ch "github.com/ClickHouse/clickhouse-go/v2"
	"github.com/golang-migrate/migrate/v4"
	"github.com/golang-migrate/migrate/v4/database/clickhouse"
	_ "github.com/golang-migrate/migrate/v4/source/file"
	"github.com/sirupsen/logrus"
	"github.com/spf13/cobra"
)

var (
	rootCmd = &cobra.Command{
		Use:           "ch:migrations [ARGS...]",
		Short:         "Runs the ClickHouse migrations for charted-server.",
		RunE:          execute,
		SilenceUsage:  true,
		SilenceErrors: true,
	}

	clickhouseConfig *ClickHouseConfig
)

type ClickHouseConfig struct {
	Username *string
	Password *string
	Database *string
	Cluster  *string
	Host     string
	Port     int
}

func (config *ClickHouseConfig) Uri() string {
	str := &strings.Builder{}
	str.WriteString("clickhouse://")

	if config.Username != nil && config.Password != nil {
		if *config.Username != "" && *config.Password != "" {
			str.WriteString(fmt.Sprintf("%v:%v@", string(*config.Username), string(*config.Password)))
		}
	}

	str.WriteString(config.Host)
	str.WriteString(":")
	str.WriteString(fmt.Sprint(config.Port))

	dbName := "charted"
	if config.Database != nil {
		dbName = *config.Database
	}

	str.WriteRune('/')
	str.WriteString(dbName)

	return str.String()
}

func init() {
	host := rootCmd.Flags().String("host", "127.0.0.1", "The host to connect to your ClickHouse server")
	port := rootCmd.Flags().Int("port", 9000, "The port to connect to your ClickHouse server")
	username := rootCmd.Flags().StringP("username", "u", "", "The username to connect to your ClickHouse server if authentication is enabled [env: CLICKHOUSE_AUTH_USERNAME]")
	password := rootCmd.Flags().StringP("password", "p", "", "The password to connect to your ClickHouse server if authentication is enabled [env: CLICKHOUSE_AUTH_PASSWORD]")
	database := rootCmd.Flags().StringP("db", "d", "charted", "The database name")
	cluster := rootCmd.Flags().StringP("cluster", "c", "", "The cluster name to create the tables if ClickHouse is distributed.")

	clickhouseConfig = &ClickHouseConfig{
		Host: *host,
		Port: *port,
	}

	if username != nil {
		clickhouseConfig.Username = username
	}

	if password != nil {
		clickhouseConfig.Password = password
	}

	if database != nil {
		clickhouseConfig.Database = database
	}

	if cluster != nil {
		clickhouseConfig.Cluster = cluster
	}
}

func execute(cmd *cobra.Command, args []string) error {
	opts := &ch.Options{
		Addr:        []string{fmt.Sprintf("%s:%d", clickhouseConfig.Host, clickhouseConfig.Port)},
		DialTimeout: 15 * time.Second,
		Settings: ch.Settings{
			"allow_experimental_object_type": true,
		},
	}

	auth := &ch.Auth{Database: "charted"}
	if clickhouseConfig.Username != nil && clickhouseConfig.Password != nil {
		if *clickhouseConfig.Username != "" {
			auth.Username = *clickhouseConfig.Username
		}

		if *clickhouseConfig.Password != "" {
			auth.Password = *clickhouseConfig.Password
		}
	}

	opts.Auth = *auth

	// TODO(noel): support SSL
	//opts.TLS = &tls.Config{InsecureSkipVerify: true}

	db := ch.OpenDB(opts)
	if err := db.Ping(); err != nil {
		logrus.Errorf("Unable to open ClickHouse connection: %s", err)
		return err
	}

	migrationConfig := &clickhouse.Config{
		MigrationsTable:       "migrations",
		MultiStatementEnabled: true,
	}

	if clickhouseConfig.Database != nil {
		migrationConfig.DatabaseName = *clickhouseConfig.Database
	}

	if clickhouseConfig.Cluster != nil {
		migrationConfig.ClusterName = *clickhouseConfig.Cluster
	}

	ch, err := clickhouse.WithInstance(db, migrationConfig)
	if err != nil {
		logrus.Errorf("Unable to create ClickHouse migration engine due to: %s", err)
		return err
	}

	migrator, err := migrate.NewWithDatabaseInstance("file://./migrations", "clickhouse", ch)
	if err != nil {
		logrus.Errorf("Unable to create migration engine due to: %s", err)
		return err
	}

	logrus.Infof("Created migration engine! Running...")

	if err := migrator.Up(); err != nil {
		if errors.Is(err, migrate.ErrNoChange) {
			logrus.Info("I already did all the migrations and they were applied~ ^-^")
			return nil
		}

		logrus.Errorf("Unable to run pending migrations due to: %s", err)
		return err
	}

	logrus.Info("Successfully ran all migrations! ^~^")
	src, dbErr := migrator.Close()
	if src != nil {
		logrus.Errorf("Unable to close migration engine: %s", src)
		return src
	}

	if dbErr != nil {
		logrus.Errorf("Unable to close database: %s", dbErr)
		return dbErr
	}

	return nil
}

func main() {
	if err := rootCmd.Execute(); err != nil {
		os.Exit(1)
	}

	os.Exit(1)
}
