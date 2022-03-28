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
	"fmt"
	"io/ioutil"
	"os"

	"github.com/pelletier/go-toml/v2"
	"github.com/spf13/cobra"
)

func newValidateCommand() *cobra.Command {
	return &cobra.Command{
		Use:          "validate [PATH]",
		Short:        "Validates the configuration format to see if it's valid.",
		Args:         cobra.MinimumNArgs(1),
		SilenceUsage: true,
		RunE: func(cmd *cobra.Command, args []string) error {
			path := args[0]

			// Check if it exists
			if _, err := os.Stat(path); err != nil {
				if os.IsNotExist(err) {
					fmt.Printf("Couldn't find configuration path in '%s'.\n", path)
					return err
				} else {
					fmt.Printf("Unable to stat configuration path in '%s': %s", path, err)
					return err
				}
			}

			// Check if we can parse it
			contents, err := ioutil.ReadFile(path)
			if err != nil {
				fmt.Printf("Couldn't read configuration in path '%s' because '%s'.\n", path, err)
				return err
			}

			var d interface{}
			if err := toml.Unmarshal(contents, &d); err != nil {
				fmt.Printf("Couldn't unmarshal config contents in '%s' because '%s'.\n", path, err)
				return err
			} else {
				fmt.Println("Everything seems alright!")
				return nil
			}
		},
	}
}
