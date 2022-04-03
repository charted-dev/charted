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

package charted

import (
	"bytes"
	"fmt"
	"io/ioutil"
	"os"
	"text/template"
	"time"

	"github.com/spf13/cobra"
)

var defaultConfig = `# The default configuration for {{.ServiceName}}
# This was generated at {{.CurrentDate}}!

# Returns the PostgreSQL database for the main database of {{.ServiceName}}.
[database]
username = "postgres"
password = "postgres"
schema = "public"
port = 5432
host = "localhost"
db = "charted"

# Returns the Redis configuration for {{.ServiceName}}.
[redis]
port = 6379
host = "localhost"

# Enables the filesystem storage trailer to store repositories.
[storage.fs]
directory = "./data"
`

func newGenerateCommand() *cobra.Command {
	return &cobra.Command{
		Use:           "generate [DIRECTORY]",
		Short:         "Generates a configuration file in the working directory or a specified directory.",
		SilenceErrors: true,
		SilenceUsage:  true,
		RunE: func(cmd *cobra.Command, args []string) error {
			cwd, err := os.Getwd()

			if len(args) == 1 {
				cwd = args[0]
			} else {
				if err != nil {
					fmt.Printf("Unable to get the working directory: %s\n", err)
					return err
				}
			}

			tmpl := template.New("charted-server-config")
			b := &bytes.Buffer{}

			tmpl, err = tmpl.Parse(defaultConfig)
			if err != nil {
				fmt.Printf("Unable to run tmpl.Parse(string): %s\n", err)
			}

			if err := tmpl.Execute(b, struct {
				ServiceName string
				CurrentDate string
			}{
				ServiceName: "charted-server",
				CurrentDate: time.Now().Format("Jan 02, 2006 - 15:04:05 MST"),
			}); err != nil {
				fmt.Printf("Unable to run tmpl.Execute(*io.Reader, interface{}): %s\n", err)
				return err
			}

			if err := ioutil.WriteFile(fmt.Sprintf("%s/config.toml", cwd), b.Bytes(), 0o666); err != nil { //nolint
				fmt.Printf("Unable to write file '%s': %s\n", fmt.Sprintf("%s/config.toml", cwd), err)
				return err
			}

			fmt.Printf("Wrote default configuration in '%s'!\n", fmt.Sprintf("%s/config.toml", cwd))
			return nil
		},
	}
}
