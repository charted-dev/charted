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

import "github.com/spf13/cobra"

var IndexCommand = &cobra.Command{
	Use:   "index [--config=./path/to/config.yml]",
	Short: "Runs a quick index towards one or multiple tables in the database.",
	RunE: func(cmd *cobra.Command, args []string) error {
		return nil
	},
}

var indexflags = opts{}

func init() {
	IndexCommand.Flags().StringVarP(&indexflags.config, "config", "c", "", "configuration file to run the indexer")
}
