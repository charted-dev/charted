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

//go:build !windows && !nacl && !plan9
// +build !windows,!nacl,!plan9

package syslog

import (
	"log/syslog"

	"github.com/sirupsen/logrus"
	lSyslog "github.com/sirupsen/logrus/hooks/syslog"
)

// EnableSyslog enables logrus to write out data to syslog.
func EnableSyslog(debug bool) error {
	var level syslog.Priority
	if debug {
		level = syslog.LOG_DEBUG
	} else {
		level = syslog.LOG_INFO
	}

	hook, err := lSyslog.NewSyslogHook("udp", "localhost:514", level, "")
	if err == nil {
		logrus.AddHook(hook)
		return nil
	}

	return nil
}
