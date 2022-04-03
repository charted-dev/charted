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

package internal

import (
	"bytes"
	"fmt"
	"os"
	"regexp"
	"strings"

	"github.com/sirupsen/logrus"
)

var format = "Jan 02, 2006 - 15:04:05 MST"
var packageName = "noelware.org/charted/server/"

// in release, the entry caller file is set to the github
// action runner, so...
var commonDirectories = []string{
	"/home/runner/work/charted",
	"/home/runner/work/charted-server",
}

// Formatter is the logrus.Formatter to use when outputtings logs
// into the terminal.
type Formatter struct {
	// If colours should be disabled when outputting logs.
	DisableColors bool
}

// NewFormatter creates a new Formatter struct pointer.
func NewFormatter() *Formatter {
	disableColors := false

	// Check if the `CHARTED_DISABLE_COLOURS` environment variable is present
	if env, ok := os.LookupEnv("CHARTED_DISABLE_COLORS"); ok {
		regex, err := regexp.Compile("^(no|false)$")
		if err == nil && regex.MatchString(env) {
			disableColors = true
		}
	}

	return &Formatter{disableColors}
}

func (f *Formatter) Format(entry *logrus.Entry) ([]byte, error) {
	// TODO: make a function to fill in this stuff
	fields := make(logrus.Fields)
	for k, v := range entry.Data {
		fields[k] = v
	}

	level := f.getLevel(entry.Level)
	b := &bytes.Buffer{}

	// Add the time to the writer
	if f.DisableColors {
		fmt.Fprintf(b, "[%s] ", entry.Time.Format(format))
	} else {
		fmt.Fprintf(b, "\x1b[38;2;134;134;134m[%s] \x1b[0m", entry.Time.Format(format))
	}

	// Output the level name
	levelName := strings.ToUpper(entry.Level.String())
	levelId := levelName[:4] //nolint

	if f.DisableColors {
		fmt.Fprintf(b, "[%s] ", levelId)
	} else {
		fmt.Fprintf(b, "%s [%s] \x1b[0m", level, levelId)
	}

	// Output the log message fields if any
	if len(fields) != 0 {
		for k, v := range fields {
			fmt.Fprintf(b, "[%s->%v] ", k, v)
		}
	}

	if entry.HasCaller() {
		var pkg string
		if strings.HasPrefix(entry.Caller.Function, packageName) {
			pkg = strings.TrimPrefix(entry.Caller.Function, packageName)
		} else {
			pkg = entry.Caller.Function
		}

		// Sometimes, goroutines can be execute logs, so we need to remove
		// `.func({int})`.
		if strings.Contains(entry.Caller.Function, ".func") {
			regex, _ := regexp.Compile(`\.(func\d+)`)
			pkg = regex.ReplaceAllString(entry.Caller.Function, "")
		}

		// To make more logs readable, we need to remove the current directory
		cwd, _ := os.Getwd()
		file := strings.TrimPrefix(strings.Replace(entry.Caller.File, cwd, "", -1), "/")

		for _, wd := range commonDirectories {
			file = strings.TrimPrefix(strings.Replace(file, wd, "", -1), "/")
		}

		if f.DisableColors {
			fmt.Fprintf(b, "[%s (%s:%d)] ", pkg, file, entry.Caller.Line)
		} else {
			fmt.Fprintf(b, "\x1b[38;2;134;134;134m[%s (%s:%d)]\x1b[0m ", pkg, file, entry.Caller.Line)
		}
	}

	b.WriteString(":: ")
	b.WriteString(strings.TrimSpace(entry.Message))
	b.WriteByte('\n')

	return b.Bytes(), nil
}

func (f *Formatter) getLevel(level logrus.Level) string {
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
