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

package logging

import (
	"bytes"
	"fmt"
	"os"
	"regexp"
	"strings"

	"github.com/sirupsen/logrus"
)

var Initialized = false

// Formatter is a logrus.Formatter to use when outputting logs from the terminal.
type Formatter struct {
	// Whether if color output is disabled or not. This is controlled from the INDEXER_DISABLE_COLORS
	// or `logging.colors: false` configuration option.
	DisableColors bool
}

func NewFormatter(disable bool) *Formatter {
	shouldDisable := disable
	if env, ok := os.LookupEnv("INDEXER_DISABLE_COLORS"); ok {
		regex, _ := regexp.Compile("^(yes|true|si*|enable|e|1)$")
		if env != "" && !regex.MatchString(env) {
			shouldDisable = true
		}
	}

	Initialized = true
	return &Formatter{DisableColors: shouldDisable}
}

func (f *Formatter) Format(entry *logrus.Entry) ([]byte, error) {
	fields := make(logrus.Fields)
	for k, v := range entry.Data {
		fields[k] = v
	}

	level := f.level(entry.Level)
	b := &bytes.Buffer{}

	if f.DisableColors {
		fmt.Fprintf(b, "% 10s", strings.ToUpper(entry.Level.String()))
	} else {
		fmt.Fprintf(b, "%s%-10s\x1b[0m ", level, strings.ToUpper(entry.Level.String()))
	}

	b.WriteString(strings.TrimSpace(entry.Message))
	for k, v := range fields {
		if f.DisableColors {
			fmt.Fprintf(b, " [%s=%v]", k, v)
		} else {
			fmt.Fprintf(b, " \x1b[38;2;134;134;134m%s=%v\x1b[0m", k, v)
		}
	}

	b.WriteByte('\n')

	return b.Bytes(), nil
}

func (f *Formatter) level(level logrus.Level) string {
	if f.DisableColors {
		return ""
	}

	switch level {
	case logrus.DebugLevel, logrus.TraceLevel:
		// #A3B68A
		return "\x1b[1m\x1b[38;2;163;182;138m"

	case logrus.ErrorLevel, logrus.FatalLevel:
		// #994B68
		return "\x1b[1m\x1b[38;2;153;75;104m"

	case logrus.WarnLevel:
		// #F3F386
		return "\x1b[1m\x1b[38;2;243;243;134m"

	case logrus.InfoLevel:
		// #B29DF3
		return "\x1b[1m\x1b[38;2;178;157;243m"

	default:
		// #2f2f2f
		return "\x1b[1m\x1b[38;2;47;47;47m"
	}
}
