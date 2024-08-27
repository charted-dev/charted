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

use crate::helpers;
use azalia::config::{env, merge::Merge, TryFromEnv};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// ## `database "sqlite" {}`
///
/// This database driver uses the almighty, holy [SQLite](https://sqlite.org). This is mainly used
/// for development, evaluation purposes, or if PostgreSQL is too heavy for your use-cases.
#[derive(Debug, Clone, Merge, Serialize, Deserialize)]
pub struct Config {
    /// Maximum amount of connections that the database pool can hold.
    #[serde(default = "__max_connections")]
    pub max_connections: u32,

    /// whether if migrations should be ran on startup. By default, this is `false`.
    ///
    /// You can use the `charted migrations run` command to run all migrations.
    #[serde(default)]
    #[merge(strategy = azalia::config::merge::strategy::bool::only_if_falsy)]
    pub run_migrations: bool,

    /// Path to the SQLite database. By default, this will be in `./data/charted.db`.
    ///
    /// The [official Docker image](https://cr.noelware.cloud/~/charted/server) will overwrite this path to `/var/lib/noelware/charted/data/charted.db`.
    #[serde(default = "__db_path")]
    pub db_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            max_connections: __max_connections(),
            run_migrations: false,
            db_path: __db_path(),
        }
    }
}

impl TryFromEnv for Config {
    type Output = Config;
    type Error = eyre::Report;

    fn try_from_env() -> Result<Self::Output, Self::Error> {
        Ok(Config {
            max_connections: helpers::env_from_str("CHARTED_DATABASE_MAX_CONNECTIONS", __max_connections())?,
            run_migrations: helpers::env_from_result(
                env!("CHARTED_DATABASE_RUN_MIGRATIONS").map(|x| azalia::TRUTHY_REGEX.is_match(&x)),
                false,
            )?,

            db_path: env!("CHARTED_DATABASE_PATH", as PathBuf)?,
        })
    }
}

const fn __max_connections() -> u32 {
    10
}

#[inline]
fn __db_path() -> PathBuf {
    PathBuf::from("./data/charted.db")
}
