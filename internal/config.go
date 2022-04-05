// 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Go.
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

package internal

import (
	"errors"
	"io/ioutil"
	"os"

	"github.com/pelletier/go-toml/v2"
	"github.com/sirupsen/logrus"
	"noelware.org/charted/server/internal/email"
	"noelware.org/charted/server/internal/search/elastic"
	"noelware.org/charted/server/internal/search/meilisearch"
	"noelware.org/charted/server/internal/search/tsubasa"
	"noelware.org/charted/server/internal/storage/filesystem"
	"noelware.org/charted/server/internal/storage/s3"
	"noelware.org/charted/server/util"
)

var (
	// ErrNotInt is an error if a non-integer was provided.
	ErrNotInt = errors.New("provided value was not a valid integer")
)

// Config represents the base configuration for charted-server.
type Config struct {
	// RunPendingMigrations returns a bool if migrations should be run
	// on server-start, if any.
	RunPendingMigrations bool `toml:"run_pending_migrations,omitempty"`

	// SecretKeyBase returns a string of the JWT key to use to validate
	// sessions.
	SecretKeyBase string `toml:"secret_key_base,omitempty"`

	// Registrations returns a bool if user creation can be enabled without
	// administrators creating the account.
	Registrations bool `toml:"registrations,omitempty"`

	// Telemetry enables Noelware's anonymous telemetry services for analytical
	// information and to further improve the experience of Noelware products.
	Telemetry bool `toml:"telemetry,omitempty"`

	// Analytics enables the server to send analytical data to have a visualization dashboard on the server. This is
	// different between Telemetry and Analytics, since Analytics don't store data of the server.
	Analytics bool `toml:"analytics,omitempty"`

	// Metrics enables Prometheus metrics on the server.
	Metrics bool `toml:"metrics,omitempty"`

	// Sets the current username to use for non-JWT requests, example
	// would be a request to `127.0.0.1/` since that is non-authenticated.
	Username *string `toml:"username,omitempty"`

	// Sets the current password to use for non-JWT request, example
	// would be a request to `127.0.0.1/` since that is non-authenticated.
	Password *string `toml:"password,omitempty"`

	// SentryDSN returns a string (or `nil`) if Sentry should be enabled
	// on the server.
	SentryDSN *string `toml:"sentry_dsn,omitempty"`

	// Port returns the port to use when connecting to the service via HTTP.
	Port *int `toml:"port"`

	// Host returns the host to use when running. You can use `127.0.0.1` to only stick on the local network,
	// by default, it'll be open under `0.0.0.0`
	Host *string `toml:"host"`

	// Email enables the Email service to send out emails on invitations, user verification,
	// and more.
	Email *email.Config `toml:"email"`

	// Search returns a SearchConfig to configure the search endpoint.
	Search *SearchConfig `toml:"search,omitempty"`

	// Storage returns a StorageConfig object to configure the storage trailer.
	Storage StorageConfig `toml:"storage,omitempty"`

	// Database returns a PostgresConfig object to configure the database.
	Database PostgresConfig `toml:"database,omitempty"`

	// Redis returns a RedisConfig object to configure Redis.
	Redis RedisConfig `toml:"redis,omitempty"`
}

// SearchConfig represents the 'search' table to configure search.
type SearchConfig struct {
	// Elastic is the configuration object to use an Elasticsearch cluster
	// to query information.
	Elastic *elastic.Config `toml:"elastic,omitempty"`

	// Tsubasa is the configuration object to use Tsubasa with the query
	// language to search information. :)
	Tsubasa *tsubasa.Config `toml:"tsubasa,omitempty"`

	// Meili is the configuration object to use Meilisearch to query
	// information. :P
	Meili *meilisearch.Config `toml:"meilisearch,omitempty"`
}

// StorageConfig represents the 'storage' table to configure storage.
type StorageConfig struct {
	// Filesystem is the configuration to use the local disk that
	// charted-server is running on to store information.
	Filesystem *filesystem.Config `toml:"filesystem,omitempty"`

	// S3 is the configuration object to use Amazon S3 or a S3-compliant
	// server to store information.
	S3 *s3.Config `toml:"s3,omitempty"`

	// Fs is just an alias for Filesystem; you cannot use both of these.
	Fs *filesystem.Config `toml:"fs,omitempty"`
}

// PostgresConfig represents the PostgreSQL database to use when storing
// metadata on certain features.
type PostgresConfig struct {
	// EnableTLS sets the SSL mode of the connection to `verify-full`. Otherwise,
	// SSL is disabled.
	EnableTls bool `toml:"tls_enable,omitempty"` //nolint

	// Timeout for establishing new connections, default is 30s
	DialTimeout int32 `toml:"dial_timeout,omitempty"`

	// Timeout for socket reads, defaults to 15s
	ReadTimeout int32 `toml:"read_timeout,omitempty"`

	// Timeout for socket writes, defaults to 25s
	WriteTimeout int32 `toml:"write_timeout,omitempty"`

	// Username represents the username to authenticate with the Postgres database.
	Username string `toml:"username,omitempty"`

	// Password represents the password to authenticate with the Postgres database.
	Password string `toml:"password,omitempty"`

	// Schema represents the schema to use when the database is created.
	Schema *string `toml:"schema,omitempty"`

	// Port is the connection port to use, by default, it will use 5432
	Port int32 `toml:"port,omitempty"`

	// Host is the connection host to use, by default, it will use localhost
	Host string `toml:"host,omitempty"`

	// Db represents the database name, by default, it will be `charted_server`.
	Db string `toml:"db,omitempty"`
}

// RedisConfig represents the configuration for using Redis as a cache.
// Sentinel and Standalone are supported!
type RedisConfig struct {
	// Sets a list of sentinel servers to use when using Redis Sentinel
	// instead of Redis Standalone. The `master` key is required if this
	// is defined. This returns a List of `host:port` strings. If you're
	// using an environment variable to set this, split it with `,` so it can be registered properly!
	Sentinels []string `toml:"sentinels,omitempty"`

	// If `requirepass` is set on your Redis server config, this property will authenticate
	// charted-server once the connection is being dealt with.
	Password *string `toml:"password,omitempty"`

	// Returns the master name for connecting to any Redis sentinel servers.
	MasterName *string `toml:"master,omitempty"`

	// Returns the database index to use.
	DbIndex int `toml:"index,omitempty"`

	// Returns the host for connecting to Redis.
	Host string `toml:"host,omitempty"`

	// Returns the port to use when connecting to Redis.
	Port int `toml:"port,omitempty"`
}

func NewConfig(path string) *Config {
	logrus.Debug("Loading configuration...")

	success := false
	if path == "" {
		logrus.Debug("Checking if `CHARTED_CONFIG_PATH` exists...")
		if p, ok := os.LookupEnv("CHARTED_CONFIG_PATH"); ok {
			logrus.Debugf("CHARTED_CONFIG_PATH exists as a environment variable under path '%s', now loading...", p)

			// Check if it exists
			if _, err := os.Stat(p); err != nil {
				if os.IsNotExist(err) {
					logrus.Fatalf("CHARTED_CONFIG_PATH '%s' was not found in the path it was specified in.", p)
				} else {
					logrus.Fatalf("Unable to stat path '%s' because: %s", p, err)
				}
			} else {
				path = p
				success = true

				logrus.Debugf("Found configuration path under %s.", p)
			}
		}

		if !success {
			logrus.Debug("Couldn't find it under `CHARTED_CONFIG_PATH` environment variable, checking /app/noelware/charted/server/config.toml...")

			if _, err := os.Stat("/app/noelware/charted/server/config.toml"); err == nil {
				path = "/app/noelware/charted/server/config.toml"
				success = true

				logrus.Debugf("Found configuration path under /app/noelware/charted/server/config.toml!")
			}
		}

		if !success {
			logrus.Debug("Unable to locate it under `CHARTED_CONFIG_PATH` environment variable or under path '/app/noelware/charted/server/config.toml', checking root directory...")

			if _, err := os.Stat("./config.toml"); !success && err == nil {
				path = "./config.toml"
				success = true

				logrus.Debugf("Found configuration path under the root directory!")
			}
		}
	} else {
		success = true
	}

	if !success {
		logrus.Fatal("Unable to locate configuration, exiting...")
	}

	logrus.Debugf("Found configuration in path '%s', now constructing...", path)
	contents, err := ioutil.ReadFile(path)
	if err != nil {
		logrus.Fatalf("Unable to read configuration file under path '%s' because: %s", path, err)
	}

	var config *Config
	if err := toml.Unmarshal(contents, &config); err != nil {
		logrus.Fatalf("Unable to unmarshal TOML in config path '%s' because: %s", path, err)
	}

	// Check if the secret key base is nothing
	if config.SecretKeyBase == "" {
		logrus.Warn("Missing `secret_key_base` configuration path! This will affect user sessions!")

		hash := util.GenerateHash(32)
		if hash == "" {
			logrus.Fatal("Unable to generate hash for JWT secret key, please re-run charted-server!")
		}

		config.SecretKeyBase = hash
		data, err := toml.Marshal(&config)
		if err != nil {
			logrus.Warn("Unable to update configuration path due to config being corrupted.")
		} else {
			err = ioutil.WriteFile(path, data, 0o770) //nolint
			if err != nil {
				logrus.Warnf("Unable to update configuration under path '%s' because: %s (is it not writable?)", path, err)
			}
		}

		logrus.Warn("I generated a new JWT secret key hash for you! Warning that this will break user sessions.")
	}

	return config
}
