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

package config

import (
	"errors"
	"fmt"
	"os"
	"strings"

	"charts.noelware.org/search-indexer/pkg/logging"
	"gopkg.in/yaml.v3"
)

var (
	ErrEmptyDatabaseUrl = errors.New("`db_url` is empty, please supply a db url")
	badlyFormattedVars  = []string{"LS_COLORS=", "DBUS_SESSION_"}
)

// Config is the main structure of a config.yml file
type Config struct {
	// DatabaseURL is the db url to use when connecting to a PostgreSQL
	// database.
	DatabaseURL string `yaml:"db_url"`

	// Logging refers to the logging configuration
	Logging *logging.Config `yaml:"logging"`
}

func Load(file *string) (*Config, error) {
	if file == nil {
		return FromEnvironmentVariables(os.Environ())
	}

	if len(*file) == 0 {
		return FromEnvironmentVariables(os.Environ())
	}

	contents, err := os.ReadFile(*file)
	if err != nil {
		return nil, fmt.Errorf("unable to read file '%s': %v", *file, err)
	}

	var config Config
	if err := yaml.Unmarshal(contents, &config); err != nil {
		return nil, fmt.Errorf("unable to parse yaml contents in file '%s': %v", *file, err)
	}

	if len(config.DatabaseURL) == 0 {
		return nil, ErrEmptyDatabaseUrl
	}

	return &config, nil
}

// exposed for testing

func FromEnvironmentVariables(vars []string) (*Config, error) {
	config := &Config{}
	for _, v := range vars {
		key, value, err := parseEnvVar(v)
		if err != nil {
			return nil, fmt.Errorf("unable to parse environment variable: %v", err)
		}

		if len(key) == 0 {
			continue
		}

		if !strings.HasPrefix(key, "INDEXER_") {
			continue
		}

		switch key {
		case "INDEXER_DATABASE_URL":
			if len(value) == 0 {
				return nil, ErrEmptyDatabaseUrl
			}

			config.DatabaseURL = value

		case "INDEXER_LOG_LEVEL":
			if len(value) == 0 {
				return nil, logging.ErrEmptyLogLevel
			}

			if config.Logging == nil {
				config.Logging = &logging.Config{}
			}
		}
	}

	if len(config.DatabaseURL) == 0 {
		return nil, ErrEmptyDatabaseUrl
	}

	return config, nil
}

func parseEnvVar(input string) (string, string, error) {
	items := strings.SplitN(input, "=", 2)
	if len(items) != 2 {
		return "", "", fmt.Errorf("unable to parse input '%s': expected length of 2, received %d", input, len(items))
	}

	return items[0], items[1], nil
}
