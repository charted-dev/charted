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

package buildinfo

import "time"

var (
	// Version is the current version of the search indexer. This is replaced with `master`
	// if this was not built by Bazel.
	Version = "master"

	// GitCommit is the current commit hash from the Git repository. This is replaced with
	// `d1cebae` if this was not built by Bazel.
	GitCommit = "d1cebae"

	// BuiltAt is a RFC3339-formatted timestamp of when the search indexer was last built.
	// You should use the `BuildDate()` method from this package to get an accurate
	// representation.
	BuiltAt = ""
)

// BuildDate returns a time.Time instance of when the search indexer was last built,
// this will return nil if this was not built by Bazel.
func BuildDate() *time.Time {
	if BuiltAt == "" {
		return nil
	}

	t, _ := time.Parse(time.RFC3339, BuiltAt)
	return &t
}
