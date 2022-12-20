// 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
// Copyright 2022-2023 Noelware <team@noelware.org>
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

	"charts.noelware.org/tools/helm-plugin/pkg/cli"
	"charts.noelware.org/tools/helm-plugin/pkg/constants"
	"github.com/spf13/cobra"
)

var (
	globalOptions cli.GlobalOptions

	longUsage = `Helm plugin to help you push your Helm charts into charted-server!~

# Examples
$ helm charted push noel/hazel . # Push the Helm chart in the root directory on noel/hazel
$ helm charted search noel       # Search all the Helm charts owned by @noel on https://charts.noelware.org
$ helm charted login             # Login into charted-server on the official instance (https://charts.noelware.org)
`

	rootCmd = &cobra.Command{
		Use:          "helm charted",
		Short:        "Helm plugin to help you push your Helm charts into charted-server!~",
		Long:         longUsage,
		SilenceUsage: true,
		Version:      fmt.Sprintf("%s+%s (%s)", constants.Version, constants.Version, constants.BuildDate),
	}
)

func init() {
	globalOptions = cli.GlobalOptions{}
	rootCmd.Flags().StringVarP(&globalOptions.ServerUrl, "server-url", "s", "https://charts.noelware.org", "server instance url")
	rootCmd.Flags().BoolVarP(&globalOptions.Verbose, "verbose", "V", false, "verbose mode (logs api requests and other misc. info)")
}
