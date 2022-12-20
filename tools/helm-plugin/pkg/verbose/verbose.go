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

package verbose

import (
	"log"
	"os"
)

// VerboseLogger is the logger when verbose mode is enabled.
type VerboseLogger struct {
	logger *log.Logger
}

func NewVerboseLogger() *VerboseLogger {
	return &VerboseLogger{
		logger: log.New(os.Stdout, "charted", log.Ldate|log.Lmsgprefix),
	}
}

func (l *VerboseLogger) Print(message ...any) {
	l.logger.Println(message...)
}

func (l *VerboseLogger) Printf(format string, args ...any) {
	l.logger.Fatalf(format, args...)
}

func (l *VerboseLogger) Fatal(message ...any) {
	l.logger.Fatal(message...)
}
