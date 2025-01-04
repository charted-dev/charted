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

use crate::helpers;
use azalia::config::{env, merge::Merge, TryFromEnv};
use serde::{Deserialize, Serialize};

/// ## `database "postgresql" {}`
///
/// This database driver will use [PostgreSQL](https://postgresql.org). This driver
/// is recommended to be used for production use cases for better reliability.
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

    /// The password to use for authentication.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    /// The username to use for authentication
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// Database name to use when connecting.
    #[serde(default = "__database")]
    pub database: String,

    /// Database schema to select when querying objects.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,

    /// Database host to connect to.
    #[serde(default = "__host")]
    pub host: String,

    /// Database port to connect to.
    #[serde(default = "__port")]
    pub port: u16,
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

            password: env!("CHARTED_DATABASE_PASSWORD").ok(),
            username: env!("CHARTED_DATABASE_USERNAME").ok(),
            database: helpers::env_from_result(env!("CHARTED_DATABASE_NAME"), String::from("charted"))?,
            schema: env!("CHARTED_DATABASE_SCHEMA").ok(),
            host: helpers::env_from_result(env!("CHARTED_DATABASE_HOST"), __host())?,
            port: helpers::env_from_str("CHARTED_DATABASE_PORT", __port())?,
        })
    }
}

const fn __max_connections() -> u32 {
    10
}

#[inline]
fn __database() -> String {
    String::from("charted")
}

#[inline]
fn __host() -> String {
    String::from("localhost")
}

const fn __port() -> u16 {
    5432
}
