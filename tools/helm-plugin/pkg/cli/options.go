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

package cli

import "charts.noelware.org/tools/helm-plugin/pkg/verbose"

// GlobalOptions represents the global options from the CLI.
type GlobalOptions struct {
	// ServerUrl represents the instance URL. By default, it will return "https://charts.noelware.org"
	// if this was not present.
	ServerUrl string

	// Enables verbose mode, which will log all API requests.
	Verbose bool

	log *verbose.VerboseLogger
}

// Log returns the VerboseLogger if the --verbose flag was enabled.
func (o *GlobalOptions) Log() *verbose.VerboseLogger {
	if o.Verbose {
		// If we already have a cached one, just return it.
		if o.log != nil {
			return o.log
		}

		o.log = verbose.NewVerboseLogger()
		return o.log
	}

	return nil
}

// Print prints out the message into the standard output if --verbose flag is enabled.
func (o *GlobalOptions) Print(message ...any) {
	log := o.Log()
	if log != nil {
		log.Print(message...)
	}
}

// Printf prints out the message with a format and optional arguments into the standard output if --verbose flag is enabled.
func (o *GlobalOptions) Printf(format string, args ...any) {
	log := o.Log()
	if log != nil {
		log.Printf(format, args...)
	}
}

// Fatal prints out the message into the standard output if --verbose flag is enabled.
func (o *GlobalOptions) Fatal(message ...any) {
	log := o.Log()
	if log != nil {
		log.Fatal(message...)
	}
}
