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

use super::caching;
use charted_common::TRUTHY_REGEX;
use eyre::Context;
use noelware_config::{env, merge::Merge, TryFromEnv};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Represents the configuration details for configuring charted-server's
/// database connections. charted-server uses [SQLx](https://github.com/launchbadge/sqlx) as
/// the database module used, so you can only configure the maximum amount of connections.
#[derive(Debug, Clone, Serialize, Deserialize, Merge)]
pub struct Config {
    /// Set the maxmium number of connections that the database connection
    /// pool should maintain.
    #[serde(default = "__max_connections")]
    pub max_connections: u32,

    /// Runs all migrations when the API server starts. This is disabled by default, if you want to run migrations,
    /// use the `charted migrations run` command.
    #[serde(default)]
    #[merge(strategy = noelware_config::merge::strategy::bool::only_if_falsy)]
    pub run_migrations: bool,

    /// Caching strategy for caching database objects.
    #[serde(default)]
    pub caching: super::caching::Config,

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

impl ToString for Config {
    fn to_string(&self) -> String {
        use std::fmt::Write;

        let mut buf = String::from("postgres://");
        match (self.username.as_ref(), self.password.as_ref()) {
            (Some(user), Some(password)) => write!(buf, "{user}:{password}@").unwrap(),
            (Some(user), None) => write!(buf, "{user}:@").unwrap(),
            (None, Some(pass)) => write!(buf, "postgres:{pass}@").unwrap(),
            _ => {}
        }

        write!(buf, "{}:{}/{}", self.host, self.port, self.database).unwrap();
        buf
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            max_connections: __max_connections(),
            run_migrations: false,
            password: None,
            username: None,
            database: __database(),
            caching: super::caching::Config::default(),
            schema: None,
            host: __host(),
            port: __port(),
        }
    }
}

impl TryFromEnv for Config {
    type Output = Config;
    type Error = eyre::Report;

    fn try_from_env() -> Result<Self::Output, Self::Error> {
        Ok(Config {
            run_migrations: env!("CHARTED_DATABASE_RUN_MIGRATIONS", |val| TRUTHY_REGEX.is_match(&val)).unwrap_or(false),
            database: env!("CHARTED_DATABASE_HOST").unwrap_or(__database()),
            username: env!("CHARTED_DATABASE_USERNAME", optional),
            password: env!("CHARTED_DATABASE_PASSWORD", optional),
            caching: caching::Config::try_from_env().context("failed to parse caching configuration")?,
            schema: env!("CHARTED_DATABASE_SCHEMA", optional),
            host: env!("CHARTED_DATABASE_HOST").unwrap_or(__host()),
            port: charted_common::env("CHARTED_DATABASE_PORT", __port(), |err| Cow::Owned(err.to_string()))?,
            max_connections: charted_common::env("CHARTED_DATABASE_MAX_CONNECTIONS", __max_connections(), |err| {
                Cow::Owned(err.to_string())
            })?,
        })
    }
}

const fn __max_connections() -> u32 {
    10
}

fn __host() -> String {
    String::from("localhost")
}

fn __database() -> String {
    String::from("charted")
}

const fn __port() -> u16 {
    5432
}
