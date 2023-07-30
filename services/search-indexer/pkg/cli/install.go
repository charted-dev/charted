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

package cli

import (
	"charts.noelware.org/search-indexer/pkg/config"
	"charts.noelware.org/search-indexer/pkg/database"
	"charts.noelware.org/search-indexer/pkg/logging"
	"github.com/sirupsen/logrus"
	"github.com/spf13/cobra"
)

var InstallCommand = &cobra.Command{
	Use:           "install [--config=./path/to/config.yml]",
	Short:         "Installs the PostgreSQL triggers into the database",
	SilenceUsage:  true,
	SilenceErrors: true,
	RunE: func(cmd *cobra.Command, args []string) error {
		config, err := config.Load(&installopts.config)
		if err != nil {
			return err
		}

		logrus.SetFormatter(logging.NewFormatter(false))
		logrus.SetLevel(logrus.DebugLevel)

		db, err := database.New(config)
		if err != nil {
			return err
		}

		if err := db.InstallTriggers(); err != nil {
			return err
		}

		return db.Dispose()
	},
}

var installopts = opts{}

func init() {
	InstallCommand.Flags().StringVarP(&installopts.config, "config", "c", "", "configuration file to run the indexer")
}
