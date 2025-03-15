// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2025 Noelware, LLC. <team@noelware.org>
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

use crate::util;
use azalia::config::{TryFromEnv, merge::Merge};
use charted_core::serde::Duration;
use serde::{Deserialize, Serialize};

pub const MAX_CONNECTIONS: &str = "CHARTED_DATABASE_MAX_CONNECTIONS";
pub const RUN_PENDING_MIGRATIONS: &str = "CHARTED_DATABASE_RUN_PENDING_MIGRATIONS";
pub const DATABASE: &str = "CHARTED_DATABASE_NAME";
pub const ACQUIRE_TIMEOUT: &str = "CHARTED_DATABASE_ACQUIRE_TIMEOUT";
pub const CONNECT_TIMEOUT: &str = "CHARTED_DATABASE_CONNECT_TIMEOUT";
pub const IDLE_TIMEOUT: &str = "CHARTED_DATABASE_IDLE_TIMEOUT";
pub const URL: &str = "CHARTED_DATABASE_URL";

/// Common configuration shared within each database.
#[derive(Debug, Clone, Serialize, Deserialize, Merge)]
#[serde(deny_unknown_fields)]
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

    /// Maximum amount of time to spend waiting when acquiring a new connection.
    #[serde(default = "__acquire_timeout")]
    #[merge(strategy = crate::util::merge_duration)]
    pub acquire_timeout: Duration,

    /// Maximum amount of time to spend for connecting to the database.
    #[serde(default = "__connect_timeout")]
    #[merge(strategy = crate::util::merge_duration)]
    pub connect_timeout: Duration,

    /// Maximum amount of time to idle until it is relinquished and can be re-used.
    #[serde(default = "__idle_timeout")]
    #[merge(strategy = crate::util::merge_duration)]
    pub idle_timeout: Duration,

    /// The database name.
    #[serde(default = "__database")]
    pub database: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            max_connections: __max_connections(),
            acquire_timeout: __acquire_timeout(),
            connect_timeout: __connect_timeout(),
            run_migrations: false,
            idle_timeout: __idle_timeout(),
            database: __database(),
        }
    }
}

impl TryFromEnv for Config {
    type Error = eyre::Report;
    type Output = Config;

    fn try_from_env() -> Result<Self::Output, Self::Error> {
        Ok(Config {
            max_connections: util::env_from_str(MAX_CONNECTIONS, __max_connections())?,
            acquire_timeout: util::env_from_str(ACQUIRE_TIMEOUT, __acquire_timeout())?,
            connect_timeout: util::env_from_str(CONNECT_TIMEOUT, __connect_timeout())?,
            run_migrations: util::bool_env(RUN_PENDING_MIGRATIONS)?,
            idle_timeout: util::env_from_str(IDLE_TIMEOUT, __idle_timeout())?,
            database: util::env_from_str(DATABASE, __database())?,
        })
    }
}

const fn __max_connections() -> u32 {
    10
}

const fn __acquire_timeout() -> Duration {
    Duration::from_secs(30)
}

const fn __connect_timeout() -> Duration {
    Duration::from_secs(15)
}

const fn __idle_timeout() -> Duration {
    Duration::from_secs(120)
}

#[inline]
fn __database() -> String {
    String::from("charted")
}
