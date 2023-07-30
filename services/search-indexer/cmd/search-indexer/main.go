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

package main

import (
	"fmt"
	"os"

	"charts.noelware.org/search-indexer/pkg/buildinfo"
	"charts.noelware.org/search-indexer/pkg/cli"
	"charts.noelware.org/search-indexer/pkg/logging"
	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Use:     "search-indexer [CMD] [...ARGS]",
	Short:   "🐻‍❄️🔎 Search indexer for charted-server",
	Version: fmt.Sprintf("v%s+%s (https://github.com/charted-dev/search-indexer)", buildinfo.Version, buildinfo.GitCommit),
	Long: `🐻‍❄️🔎 Command-line for interacting and running the search indexer for charted-server, written in Go!

## Examples
$ search-indexer index  # Query all objects and indexes each object in parallel
$ search-indexer run    # Runs the search indexer
`,
}

func init() {
	rootCmd.AddCommand(cli.InstallCommand)
	rootCmd.AddCommand(cli.IndexCommand)
	rootCmd.AddCommand(cli.RunCommand)
}

func main() {
	exitCode := 0
	if err := rootCmd.Execute(); err != nil {
		if !logging.Initialized {
			fmt.Printf("%v\n", err)
		}

		exitCode = 1
	}

	os.Exit(exitCode)
}
