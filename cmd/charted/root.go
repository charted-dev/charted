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
	"github.com/bshuster-repo/logrus-logstash-hook"
	"github.com/sirupsen/logrus"
	"github.com/spf13/cobra"
	"net"
	"noelware.org/charted/server/internal"
	"noelware.org/charted/server/server"
	"time"
)

var (
	rootCmd = &cobra.Command{
		Use:   "charted-server [COMMAND] [ARGS...]",
		Short: "Command utility to manage charted-server",
		RunE:  runServer,
		Long: `charted-server is a Helm Chart registry made in Go that is created by Noelware
to provide a reliable, open source, and free way to have utilities around the
Helm ecosystem.

This command line exists to boot the server if no arguments are in-use, generate configuration details,
SSL certificates, validation of both SSL + config, and ping the server if needed.
	`,
	}

	configPath     string
	logstashTcpUri string
	enableLogstash = false
	useJsonLogs    = false
	verbose        = false
)

func init() {
	rootCmd.Flags().StringVarP(&configPath, "config.path", "c", "", "Returns the configuration to load from.")
	rootCmd.Flags().BoolVarP(&verbose, "verbose", "v", false, "Enables debug mode, you will receive more logs!")
	rootCmd.Flags().StringVarP(&logstashTcpUri, "logstash.tcp.uri", "t", "?", "Returns the TCP URL to connect to Logstash.")
	rootCmd.Flags().BoolVarP(&enableLogstash, "logstash.enable", "l", false, "Enables the Logstash hook to connect the ELK stack with charted-server.")
	rootCmd.Flags().BoolVarP(&useJsonLogs, "json", "j", false, "Uses the JSON formatter instead of the default, pretty formatter for logs.")

	rootCmd.AddCommand(newValidateCommand(), newGenerateCommand(), newVersionCommand(), newPingCommand())
}

// Execute is the main function that executes the CLI handler.
func Execute() int {
	if err := rootCmd.Execute(); err != nil {
		return 1
	} else {
		return 0
	}
}

func runServer(_ *cobra.Command, _ []string) error {
	// Define the logger stuff
	logrus.SetReportCaller(true)

	if useJsonLogs {
		logrus.SetFormatter(&logrus.JSONFormatter{})
	} else {
		logrus.SetFormatter(internal.NewFormatter())
	}

	if verbose {
		logrus.SetLevel(logrus.DebugLevel)
	} else {
		logrus.SetLevel(logrus.InfoLevel)
	}

	if internal.Docker() {
		logrus.Warn("It is recommended to create a volume if you're using the Filesystem storage trailer for persistence.")
	}

	if internal.Root() {
		logrus.Warn("It is recommended not to run charted-server under root/Administrator!")
	}

	// TODO: support UDP connections
	if enableLogstash {
		conn, err := net.Dial("tcp", logstashTcpUri)
		if err != nil {
			logrus.Fatalf("Unable to dial TCP connection to Logstash ('%s'): %s", logstashTcpUri, err)
		}

		hook := logrustash.New(conn, logrustash.DefaultFormatter(logrus.Fields{
			"vendor":     "Noelware",
			"version":    internal.Version,
			"commit_sha": internal.CommitSHA,
			"app":        "charted-server",
		}))

		logrus.AddHook(hook)
	}

	buildDate, _ := time.Parse(time.RFC3339, internal.BuildDate)
	logrus.Infof("Running v%s (commit: %s, build date: %s) of charted-server!",
		internal.Version,
		internal.CommitSHA,
		buildDate.Format(time.RFC1123))

	config := internal.NewConfig(configPath)
	internal.NewContainer(config)

	return server.Start()
}
