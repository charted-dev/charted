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
	"encoding/json"
	"fmt"
	"os"
	"runtime"
	"strings"
	"text/template"
	"time"

	"github.com/spf13/cobra"
	"noelware.org/charted/server/internal"
)

type versionHandler struct {
	Version    string `json:"version"`
	CommitSHA  string `json:"commit_sha"`
	BuildDate  string `json:"build_date"`
	GoVersion  string `json:"go_version"`
	GoCompiler string `json:"go_compiler"`
	GoOS       string `json:"go_os"`
	GoArch     string `json:"go_arch"`
}

func newVersionCommand() *cobra.Command {
	var (
		tmpl    string
		useJson bool //nolint
	)

	cmd := &cobra.Command{
		Use:          "version [OPTIONS]",
		Short:        "Returns the current version of charted-server.",
		SilenceUsage: true,
		RunE: func(_ *cobra.Command, args []string) error {
			buildDate, _ := time.Parse(time.RFC3339, internal.BuildDate)

			// Check if we need to print it as JSON
			if useJson {
				version := &versionHandler{
					Version:    internal.Version,
					CommitSHA:  internal.CommitSHA,
					BuildDate:  buildDate.Format(time.RFC1123),
					GoVersion:  strings.TrimPrefix(runtime.Version(), "go"),
					GoCompiler: runtime.Compiler,
					GoArch:     runtime.GOARCH,
					GoOS:       runtime.GOOS,
				}

				if err := json.NewEncoder(os.Stdout).Encode(&version); err != nil {
					return err
				}

				return nil
			}

			// Check if we need to use a Go template to execute it.
			if tmpl != "" {
				version := &versionHandler{
					Version:    internal.Version,
					CommitSHA:  internal.CommitSHA,
					BuildDate:  buildDate.Format(time.RFC1123),
					GoVersion:  strings.TrimPrefix(runtime.Version(), "go"),
					GoCompiler: runtime.Compiler,
					GoArch:     runtime.GOARCH,
					GoOS:       runtime.GOOS,
				}

				t := template.New("charted-server version")
				t, err := t.Parse(tmpl)
				if err != nil {
					return err
				}

				if err := t.Execute(os.Stdout, &version); err != nil {
					return err
				}

				return nil
			}

			fmt.Printf("charted-server v%s (commit: %s, build date: %s) on %s/%s (go v%s)",
				internal.Version,
				internal.CommitSHA,
				buildDate.Format(time.RFC1123),
				runtime.GOOS,
				runtime.GOARCH,
				strings.TrimPrefix(runtime.Version(), "go"))

			return nil
		},
	}

	cmd.Flags().BoolVarP(&useJson, "json", "j", false, "Returns the version as JSON")
	cmd.Flags().StringVarP(&tmpl, "template", "t", "", "Returns the version as a Go template")

	return cmd
}
