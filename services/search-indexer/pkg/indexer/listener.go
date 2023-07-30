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

package indexer

import (
	"charts.noelware.org/search-indexer/pkg/database"
	"github.com/sirupsen/logrus"
)

type LogListener struct{}

func NewLogListener() database.Listener {
	return LogListener{}
}

func (LogListener) Connected() {
	logrus.WithField("reporter", "indexer.logListener.Connected").Info("connected successfully!")
}

func (LogListener) Disposed() {
	logrus.WithField("reporter", "indexer.logListener.Disposed").Warn("indexer has been disposed")
}

func (LogListener) Create(table string, payload any) {
	logrus.WithField("reporter", "indexer.logListener.Create").Debug("+ ", table, ": @<unknown> (<id>)")
}

func (LogListener) Update(table string, payload any) {
	logrus.WithField("reporter", "indexer.logListener.Update").Debug("* ", table, ": @<unknown> (<id>)")
}

func (LogListener) Delete(table string, payload any) {
	logrus.WithField("reporter", "indexer.logListener.Delete").Debug("- ", table, ": @<unknown> (<id>)")
}
