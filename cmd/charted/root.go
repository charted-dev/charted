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
	"net"
	"os"
	"strings"
	"time"

	logrustash "github.com/bshuster-repo/logrus-logstash-hook"
	"github.com/sirupsen/logrus"
	"github.com/spf13/cobra"
	"noelware.org/charted/server/internal"
	noelSyslog "noelware.org/charted/server/internal/syslog"
	"noelware.org/charted/server/server"
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
	logstashTcpUri string //nolint
	logstashUdpUri string //nolint
	syslog         = false
	enableLogstash = false
	useJsonLogs    = false //nolint
	verbose        = false
)

func init() {
	rootCmd.Flags().StringVarP(&configPath, "config.path", "c", "", "Returns the configuration to load from.")
	rootCmd.Flags().BoolVarP(&verbose, "verbose", "v", false, "Enables debug mode, you will receive more logs!")
	rootCmd.Flags().StringVarP(&logstashUdpUri, "logstash.udp.uri", "u", "?", "Returns the UDP URI to connect to Logstash if running ")
	rootCmd.Flags().StringVarP(&logstashTcpUri, "logstash.tcp.uri", "t", "?", "Returns the TCP URL to connect to Logstash.")
	rootCmd.Flags().BoolVarP(&enableLogstash, "logstash.enable", "l", false, "Enables the Logstash hook to connect the ELK stack with charted-server.")
	rootCmd.Flags().BoolVarP(&useJsonLogs, "json", "j", false, "Uses the JSON formatter instead of the default, pretty formatter for logs.")
	rootCmd.Flags().BoolVarP(&syslog, "syslog", "s", false, "Enables logrus to output logs to syslog. (UNIX only, no-op on Windows)")

	rootCmd.AddCommand(newValidateCommand(), newGenerateCommand(), newVersionCommand(), newPingCommand())
}

// Execute is the main function that executes the CLI handler.
func Execute() int {
	if err := rootCmd.Execute(); err != nil {
		return 1
	}

	return 0
}

func runServer(_ *cobra.Command, _ []string) error {
	if _, ok := os.LookupEnv("PRISMA_CLIENT_GO_LOG"); !ok {
		_ = os.Setenv("PRISMA_CLIENT_GO_LOG", "debug")
	}

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

	err := noelSyslog.EnableSyslog(verbose)
	if err != nil {
		logrus.Errorf("Unable to enable syslogs: %s; skipping", err)
	}

	if internal.Docker() {
		logrus.Warn("It is recommended to create a volume if you're using the Filesystem storage trailer for persistence.")
	}

	if internal.Root() {
		logrus.Warn("It is recommended not to run charted-server under root/Administrator!")
	}

	uuid := internal.GetInstanceUUID()
	logrus.Debugf("Using instance UUID '%s'! If analytics are enabled, you can visit https://analytics.noelware.org/charted/instance/%s to have a in-depth analysis on the server running.", uuid, uuid)

	// TODO: support UDP connections
	if enableLogstash {
		logrus.Debug("Enabling Logstash support for charted-server...")
		success := false

		// Check if the user provided both
		if logstashTcpUri != "?" && logstashUdpUri != "?" {
			logrus.Fatalf("Using both `--logstash.tcp.uri` and `--logstash.udp.uri` is not supported.")
		}

		// Check if we need to use TCP
		if logstashTcpUri != "?" {
			if strings.HasPrefix(logstashTcpUri, "tcp://") {
				logrus.Warnf("You need to remove the `tcp://` prefix since it'll be automatically used when dialing! But, I'll do it for you.")
				logstashTcpUri = strings.TrimPrefix(logstashTcpUri, "tcp://")
			}

			logrus.Debugf("Now connecting to tcp://%s...", logstashTcpUri)

			conn, err := net.Dial("tcp", logstashTcpUri)
			if err != nil {
				logrus.Fatalf("Unable to dial TCP connection to Logstash ('%s'): %s", logstashTcpUri, err)
			}

			logrus.Debugf("Connected to tcp://%s! Enabling Logstash formatter...", logstashTcpUri)
			hook := logrustash.New(conn, logrustash.DefaultFormatter(logrus.Fields{
				"vendor":     "Noelware",
				"version":    internal.Version,
				"commit_sha": internal.CommitSHA,
				"app":        "charted-server",
				"conn_type":  "tcp",
			}))

			logrus.AddHook(hook)
			success = true
		}

		// No? What if we need to use UDP?
		if !success && logstashUdpUri != "?" {
			if strings.HasPrefix(logstashUdpUri, "udp://") {
				logrus.Warnf("You need to remove the `udp://` prefix since it'll be automatically used when dialing! But, I'll do it for you.")
				logstashUdpUri = strings.TrimPrefix(logstashUdpUri, "udp://")
			}

			logrus.Debugf("Now connecting to udp://%s...", logstashUdpUri)

			conn, err := net.Dial("udp", logstashUdpUri)
			if err != nil {
				logrus.Fatalf("Unable to dial UDP connection to Logstash ('%s'): %s", logstashUdpUri, err)
			}

			logrus.Debugf("Connected to udp://%s! Enabling Logstash formatter...", logstashTcpUri)
			hook := logrustash.New(conn, logrustash.DefaultFormatter(logrus.Fields{
				"vendor":     "Noelware",
				"version":    internal.Version,
				"commit_sha": internal.CommitSHA,
				"app":        "charted-server",
				"conn_type":  "udp",
			}))

			logrus.AddHook(hook)
			success = true
		}

		if !success {
			logrus.Fatalf("Unable to dial TCP/UDP connection for Logstash (did you not provide `--logstash.[udp|tcp].uri=...`?)")
		}
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
