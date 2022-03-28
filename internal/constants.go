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

var (
	// Version returns the current version of charted-server.
	// This is populated using the `-X` flag when using `go build`.
	//
	// This is automatically populated in `make build`, which is the
	// recommended way to build charted-server from the repository.
	Version = "master"

	// CommitSHA returns the current Git commit hash from
	// the charted-server repository. This is populated
	// using the `-X` flag when using `go build`.
	//
	// This is automatically populated in `make build`, which is the
	// recommended way to build charted-server from the repository.
	CommitSHA = "abcdefgh"

	// BuildDate returns the current build date in ISO8601 format.
	// This is populated using the `-X` flag when using `go build`.
	//
	// This is automatically populated in `make build`, which is the
	// recommended way to build charted-server from the repository.
	BuildDate = "???"
)
