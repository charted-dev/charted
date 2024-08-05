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

use std::path::PathBuf;

use azalia::config::merge::Merge;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Merge, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "__max_connections")]
    pub max_connections: u32,

    #[serde(default)]
    #[merge(strategy = azalia::config::merge::strategy::bool::only_if_falsy)]
    pub run_migrations: bool,

    /// Caching strategy for caching database objects.
    #[serde(default)]
    pub caching: crate::caching::Config,

    #[serde(default = "__db_path")]
    pub db_path: PathBuf,
}

const fn __max_connections() -> u32 {
    10
}

#[inline]
fn __db_path() -> PathBuf {
    PathBuf::from("./data/charted.db")
}
