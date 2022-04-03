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

package search

// Options represents the options that are supposed to be supported in Engine.Search.
type Options struct {
	// ~ + ~ these options are only supported in ES/Tsubasa engines + ~ + \\
	// ~ + ~     this will silently fail in Meilisearch engines!    + ~ + \\

	Fuzzy      bool
	WithClause string

	// ~ + ~ these options are only supported in Meilisearch + ~ + \\
	// ~ + ~   this will silenty fail in ES/Tsubasa engines  +  ~ + \\

	Offset int64
	Limit  int64
}

type OptionFunc func(o *Options)

func WithFuzzy(fuzzy bool) OptionFunc {
	return func(o *Options) {
		o.Fuzzy = fuzzy
	}
}

func WithOffset(offset int64) OptionFunc {
	return func(o *Options) {
		o.Offset = offset
	}
}

func WithLimit(limit int64) OptionFunc {
	return func(o *Options) {
		o.Limit = limit
	}
}

func WithClase(clause string) OptionFunc {
	return func(o *Options) {
		o.WithClause = clause
	}
}
