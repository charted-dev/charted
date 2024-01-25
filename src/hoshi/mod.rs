// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2024 Noelware, LLC. <team@noelware.org>
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

/// Represents the instance to allow Hoshi to interact with the API service. This is useful
/// for editing Hoshi-specific settings if the web UI is bundled in. We decided a small SQLite
/// database can work for specific settings and to determine if we're in a "first install" state, i.e,
/// if the SQLite database doesn't exist.
#[derive(Clone)]
pub struct Hoshi {
    /// Whether or not if we are in the "first install" state, if both `charted` and hoshi dbs
    /// don't exist.
    pub first_install: bool,
}
