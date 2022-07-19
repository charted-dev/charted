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
	"errors"
	"os"
	"time"

	"github.com/gocql/gocql"
	"github.com/golang-migrate/migrate/v4"
	"github.com/golang-migrate/migrate/v4/database/cassandra"
	_ "github.com/golang-migrate/migrate/v4/source/file"
	"github.com/sirupsen/logrus"
	"github.com/spf13/cobra"
)

var (
	rootCmd = &cobra.Command{
		Use:           "migrations [ARGS...]",
		Short:         "Runs the Cassandra migrations for charted-server",
		RunE:          execute,
		SilenceUsage:  true,
		SilenceErrors: true,
	}

	migrationsTable *string
	hosts           *[]string
	port            *int
	protocol        *int
	timeout         *string
	username        *string
	password        *string
	keyspace        *string
	//disableHostLookup *bool
)

func init() {
	migrationsTable = rootCmd.Flags().String("table", "migrations", "The migrations table to use")
	hosts = rootCmd.Flags().StringSlice("hosts", []string{"127.0.0.1"}, "The cluster hosts to connect to")
	port = rootCmd.Flags().Int("port", 9042, "The port to connect to your Cassandra cluster")
	username = rootCmd.Flags().StringP("username", "u", "", "The username to connect to your Cassandra cluster if authentication is enabled. [env: CASSANDRA_USERNAME]")
	password = rootCmd.Flags().StringP("password", "p", "", "The password to connect to your Cassandra cluster if authentication is enabled. [env: CASSANDRA_PASSWORD]")
	protocol = rootCmd.Flags().Int("protocol", 0, "Cassandra protocol to use.")
	timeout = rootCmd.Flags().StringP("timeout", "t", "1m", "The dial timeout to use when connecting.")
	keyspace = rootCmd.Flags().StringP("keyspace", "k", "charted", "The keyspace to run migrations on.")
	//disableHostLookup = rootCmd.Flags().BoolP("disable-lookup", "l", false, "If the migration engine should disable host lookups.")
}

func execute(_ *cobra.Command, _ []string) error {
	cluster := gocql.NewCluster(*hosts...)
	cluster.Port = *port
	cluster.Consistency = gocql.All
	cluster.Keyspace = *keyspace

	if protocol != nil {
		cluster.ProtoVersion = *protocol
	}

	if timeout != nil {
		t, err := time.ParseDuration(*timeout)
		if err != nil {
			logrus.Errorf("Unable to parse duration %s: %s", *timeout, err)
			return err
		}

		cluster.Timeout = t
	}

	if username != nil {
		if password == nil {
			logrus.Fatalf("Username was provided but no password?")
		}

		cluster.Authenticator = gocql.PasswordAuthenticator{
			Username: *username,
			Password: *password,
		}
	}

	logrus.Info("Connecting to cluster...")
	session, err := cluster.CreateSession()
	if err != nil {
		logrus.Errorf("Unable to connect to cluster: %s", err)
		return err
	}

	logrus.Info("Connected to cluster!")
	config := &cassandra.Config{
		MigrationsTable:       *migrationsTable,
		KeyspaceName:          *keyspace,
		MultiStatementEnabled: true,
	}

	ca, err := cassandra.WithInstance(session, config)
	if err != nil {
		logrus.Errorf("Unable to create Cassandra migration engine: %s", err)
		return err
	}

	migrator, err := migrate.NewWithDatabaseInstance("file://./migrations", "cassandra", ca)
	if err != nil {
		logrus.Errorf("Unable to create migration engine: %v", err)
		return err
	}

	logrus.Info("Created migration engine!")
	if err := migrator.Up(); err != nil {
		if errors.Is(err, migrate.ErrNoChange) {
			logrus.Info("All the migrations were applied already~ ^-^")
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

	os.Exit(0)
}
