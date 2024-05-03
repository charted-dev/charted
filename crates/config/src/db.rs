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
use crate::caching::Strategy;
use charted_common::{serde::Duration, TRUTHY_REGEX};
use eyre::eyre;
use noelware_config::{env, merge::Merge, TryFromEnv};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, env::VarError, fmt::Display, str::FromStr};

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

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("postgres://")?;

        match (self.username.as_ref(), self.password.as_ref()) {
            (Some(user), Some(pass)) => write!(f, "{user}:{pass}@")?,
            (Some(user), None) => write!(f, "{user}:@")?,
            (None, Some(pass)) => write!(f, "postgres:{pass}@")?,
            _ => {}
        }

        write!(f, "{}:{}/{}", self.host, self.port, self.database)
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
            schema: env!("CHARTED_DATABASE_SCHEMA", optional),
            host: env!("CHARTED_DATABASE_HOST").unwrap_or(__host()),
            port: charted_common::env("CHARTED_DATABASE_PORT", __port(), |err| Cow::Owned(err.to_string()))?,
            max_connections: charted_common::env("CHARTED_DATABASE_MAX_CONNECTIONS", __max_connections(), |err| {
                Cow::Owned(err.to_string())
            })?,

            caching: caching::Config {
                max_object_size: charted_common::env(
                    "CHARTED_DATABASE_CACHE_MAX_OBJECT_SIZE",
                    caching::__one_megabyte(),
                    |err| Cow::Owned(err.to_string()),
                )?,

                ttl: env!("CHARTED_DATABASE_CACHE_TTL", |val| Duration::from_str(&val).ok(); or None),
                strategy: match env!("CHARTED_DATABASE_CACHE_STRATEGY") {
                    Ok(res) => match res.as_str() {
                        "inmemory" | "in-memory" => Strategy::InMemory,
                        "redis" => Strategy::Redis,
                        res => {
                            return Err(eyre!(
                                "unknown value [{res}], wanted [inmemory/in-memory, redis] instead"
                            ))
                        }
                    },

                    Err(VarError::NotPresent) => Strategy::default(),
                    Err(_) => return Err(eyre!("received invalid UTF-8 content")),
                },
            },
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

#[cfg(test)]
mod tests {
    use crate::caching;

    use super::{Config, Strategy};
    use noelware_config::{expand_with, merge::Merge, TryFromEnv};
    use ubyte::ToByteUnit;

    #[test]
    fn test_env_config() {
        expand_with("CHARTED_DATABASE_CACHE_STRATEGY", "inmemory", || {
            let config = Config::try_from_env();
            assert!(config.is_ok());
        });
    }

    #[test]
    fn test_merge_config() {
        expand_with("CHARTED_DATABASE_CACHE_STRATEGY", "inmemory", || {
            let config = Config::try_from_env();
            assert!(config.is_ok());

            let mut config = config.unwrap();
            config.merge(Config {
                caching: caching::Config {
                    strategy: Strategy::Redis,
                    ..Default::default()
                },
                ..Default::default()
            });

            assert_eq!(config.caching.strategy, Strategy::Redis);

            config.merge(Config {
                caching: caching::Config {
                    max_object_size: 512.kibibytes(),
                    ..Default::default()
                },

                ..Default::default()
            });

            assert_eq!(config.caching.max_object_size, 512.kibibytes());
        });
    }
}
