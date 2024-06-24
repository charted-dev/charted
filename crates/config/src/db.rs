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
use azalia::TRUTHY_REGEX;
use charted_common::serde::Duration;
use eyre::eyre;
use noelware_config::{env, merge::Merge, TryFromEnv};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    collections::HashMap,
    env::{self, VarError},
    fmt::Display,
    path::PathBuf,
    str::FromStr,
};

/// Represents the `?sslmode` configuration parameter that can be used in [`Config`].
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SSLMode {
    /// Only try a non-SSL connection.
    Disable,

    /// First try a non-SSL connection; if that fails, try an SSL connection.
    Allow,

    /// First try an SSL connection; if that fails, try a non-SSL connection.
    #[default]
    Prefer,

    /// Only try an SSL connection. If a root CA file is present, verify the connection
    /// in the same way as if `VerifyCa` was specified.
    Require,

    /// Only try an SSL connection, and verify that the server certificate is issued by a
    /// trusted certificate authority (CA).
    VerifyCa,

    /// Only try an SSL connection; verify that the server certificate is issued by a trusted
    /// CA and that the requested server host name matches that in the certificate.
    VerifyFull,
}

impl From<SSLMode> for sqlx::postgres::PgSslMode {
    fn from(me: SSLMode) -> Self {
        match me {
            SSLMode::Disable => sqlx::postgres::PgSslMode::Disable,
            SSLMode::Allow => sqlx::postgres::PgSslMode::Allow,
            SSLMode::Prefer => sqlx::postgres::PgSslMode::Prefer,
            SSLMode::Require => sqlx::postgres::PgSslMode::Require,
            SSLMode::VerifyCa => sqlx::postgres::PgSslMode::VerifyCa,
            SSLMode::VerifyFull => sqlx::postgres::PgSslMode::VerifyFull,
        }
    }
}

impl Merge for SSLMode {
    fn merge(&mut self, other: Self) {
        match (*self, other) {
            // don't even attempt to merge the same objects
            (SSLMode::Disable, SSLMode::Disable) => {}
            (SSLMode::Allow, SSLMode::Allow) => {}
            (SSLMode::Prefer, SSLMode::Prefer) => {}
            (SSLMode::Require, SSLMode::Require) => {}
            (SSLMode::VerifyCa, SSLMode::VerifyCa) => {}
            (SSLMode::VerifyFull, SSLMode::VerifyFull) => {}

            // overwrite it if they're different
            (_, other) => {
                *self = other;
            }
        }
    }
}

impl TryFromEnv for SSLMode {
    type Output = SSLMode;
    type Error = eyre::Report;

    fn try_from_env() -> Result<Self::Output, Self::Error> {
        Ok(match env!("CHARTED_DATABASE_SSLMODE") {
            Ok(mode) => match &*mode.as_str().to_ascii_lowercase() {
                "disable" => SSLMode::Disable,
                "allow" => SSLMode::Allow,
                "require" => SSLMode::Require,
                "verify-ca" => SSLMode::VerifyCa,
                "verify-full" => SSLMode::VerifyFull,
                s => return Err(eyre!("unknown mode {s} for `sslmode`")),
            },

            Err(env::VarError::NotPresent) => Default::default(),
            Err(_) => {
                return Err(eyre!(
                    "unable to represent environment variable `$CHARTED_DATABASE_SSLMODE` as a valid unicode string"
                ))
            }
        })
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Merge)]
pub struct SSL {
    /// Sets the file containing the SSL certificate authority (CA) certificate(s). If the file
    /// exists, the server's certificate will be verified to be signed by one of these authorities.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root_cert: Option<PathBuf>,

    /// Sets the file containing the client's SSL certificate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_cert: Option<PathBuf>,

    /// Sets the file containing the client's certificate key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_key: Option<PathBuf>,
}

impl TryFromEnv for SSL {
    type Output = Option<SSL>;
    type Error = eyre::Report;

    fn try_from_env() -> Result<Self::Output, Self::Error> {
        match env!("CHARTED_DATABASE_SSL_ENABLE") {
            Ok(res) if TRUTHY_REGEX.is_match(&res) => Ok(Some(SSL {
                client_cert: env!("CHARTED_DATABASE_SSL_CLIENT_CERT", |val| PathBuf::from(val)).ok(),
                client_key: env!("CHARTED_DATABASE_SSL_CLIENT_KEY", |val| PathBuf::from(val)).ok(),
                root_cert: env!("CHARTED_DATABASE_SSL_ROOT_CERT", |val| PathBuf::from(val)).ok(),
            })),

            Ok(_) => Ok(None),
            Err(env::VarError::NotPresent) => Ok(None),
            Err(_) => Err(eyre!(
                "unable to represent `$CHARTED_DATABASE_SSL_ENABLE` as a valid unicode string"
            )),
        }
    }
}

/// Represents the configuration details for configuring charted-server's
/// database connections. charted-server uses [SQLx](https://github.com/launchbadge/sqlx) as
/// the database library.
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

    /// Sets a maximum idle duration for individual connections. Any connection that remains in the idle
    /// queue longer than this will be closed, for usage-based database server billing, this is a cost saver!
    #[serde(default)]
    pub idle_timeout: Option<Duration>,

    /// Configures each PostgreSQL connection URI string with a set of options.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub options: HashMap<String, String>,

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

    /// SSL mode to use.
    #[serde(default)]
    pub sslmode: SSLMode,

    /// Database host to connect to.
    #[serde(default = "__host")]
    pub host: String,

    /// Database port to connect to.
    #[serde(default = "__port")]
    pub port: u16,

    /// SSL configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssl: Option<SSL>,
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("postgres://")?;

        match (self.username.as_ref(), self.password.as_ref()) {
            (Some(user), Some(pass)) => write!(f, "{user}:{pass}@")?,
            (Some(user), None) => write!(f, "{user}@")?,
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
            idle_timeout: None,
            password: None,
            username: None,
            database: __database(),
            caching: super::caching::Config::default(),
            sslmode: SSLMode::default(),
            options: HashMap::default(),
            schema: None,
            host: __host(),
            port: __port(),
            ssl: None,
        }
    }
}

impl TryFromEnv for Config {
    type Output = Config;
    type Error = eyre::Report;

    fn try_from_env() -> Result<Self::Output, Self::Error> {
        Ok(Config {
            run_migrations: env!("CHARTED_DATABASE_RUN_MIGRATIONS", |val| TRUTHY_REGEX.is_match(&val)).unwrap_or(false),
            idle_timeout: env!("CHARTED_DATABASE_IDLE_TIMEOUT", |val| Duration::from_str(&val).ok(); or None),
            database: env!("CHARTED_DATABASE_HOST").unwrap_or(__database()),
            username: env!("CHARTED_DATABASE_USERNAME", optional),
            password: env!("CHARTED_DATABASE_PASSWORD", optional),
            options: charted_common::env_map("CHARTED_DATABASE_OPTIONS")?,
            sslmode: SSLMode::try_from_env()?,
            schema: env!("CHARTED_DATABASE_SCHEMA", optional),
            host: env!("CHARTED_DATABASE_HOST").unwrap_or(__host()),
            port: charted_common::env("CHARTED_DATABASE_PORT", __port(), |err| Cow::Owned(err.to_string()))?,
            ssl: SSL::try_from_env()?,

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
