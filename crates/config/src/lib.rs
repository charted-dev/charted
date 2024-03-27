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

pub mod caching;
pub mod cdn;
pub mod db;
pub mod logging;
pub mod metrics;
pub mod redis;
pub mod search;
pub mod server;
pub mod sessions;
pub mod storage;

use charted_common::{rand_string, TRUTHY_REGEX};
use eyre::{eyre, Report};
use noelware_config::{env, merge::Merge, FromEnv, TryFromEnv};
use serde::{Deserialize, Serialize};
use std::{
    env::VarError,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Serialize, Deserialize, Merge)]
pub struct Config {
    /// whether or not if users can be registered on this instance
    #[serde(default = "__truthy")]
    #[merge(strategy = noelware_config::merge::strategy::bool::only_if_falsy)]
    pub registrations: bool,

    /// Secret key for encoding JWT tokens. This must be set once and never touched again.
    #[serde(default)]
    #[merge(skip)] // don't even attempt to merge jwt secret keys
    pub jwt_secret_key: String,

    /// Sentry [DSN](https://sentry.io) to configure to emit all errors to a Sentry service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sentry_dsn: Option<String>,

    /// Logging configuration to configure the API server's logging capabilities.
    #[serde(default)]
    pub logging: logging::Config,

    /// whether or not if the API server should act like a single user, where *most* features
    /// are disabled and only one user is allowed to roam.
    ///
    /// all publically available features like Audit Logging can be enabled but repository and
    /// organization members are disabled. most endpoints will be also disabled.
    #[serde(default)]
    #[merge(strategy = noelware_config::merge::strategy::bool::only_if_falsy)]
    pub single_user: bool,

    /// Configures how to connect to the PostgreSQL database.
    #[serde(default)]
    pub database: db::Config,

    /// metrics pipeline configuration
    #[serde(default)]
    pub metrics: metrics::Config,

    /// whether or not if the API server should act like a single organization, where most features
    /// are disabled like repository/organization members and audit logging.
    #[serde(default)]
    #[merge(strategy = noelware_config::merge::strategy::bool::only_if_falsy)]
    pub single_org: bool,

    /// Configures the storage for holding external media and chart indexes.
    #[serde(default)]
    #[merge(skip)]
    pub storage: storage::Config,

    /// Configures the session backend for authentication.
    #[serde(default)]
    pub sessions: sessions::Config,

    #[serde(default)]
    pub server: server::Config,

    /// Configures the Redis server for cache-related stuff.
    #[serde(default)]
    pub redis: redis::Config,

    /// Configures the CDN feature.
    #[serde(default)]
    pub cdn: cdn::Config,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            jwt_secret_key: String::default(),
            registrations: __truthy(),
            single_user: false,
            single_org: false,
            sentry_dsn: None,
            sessions: sessions::Config::default(),
            database: db::Config::default(),
            logging: logging::Config::default(),
            metrics: metrics::Config::default(),
            storage: storage::Config::default(),
            server: server::Config::default(),
            redis: redis::Config::default(),
            cdn: cdn::Config::default(),
        }
    }
}

impl TryFromEnv for Config {
    type Output = Config;
    type Error = Report;

    fn try_from_env() -> Result<Self::Output, Self::Error> {
        Ok(Config {
            registrations: env!("CHARTED_ENABLE_REGISTRATIONS", |val| TRUTHY_REGEX.is_match(&val); or true),
            jwt_secret_key: match env!("CHARTED_JWT_SECRET_KEY") {
                Ok(val) => val,
                Err(VarError::NotPresent) => __generated_secret_key(),
                Err(VarError::NotUnicode(_)) => {
                    return Err(eyre!("failed to parse `CHARTED_JWT_SECRET_KEY` as valid unicode"))
                }
            },

            single_user: env!("CHARTED_SINGLE_USER", |val| TRUTHY_REGEX.is_match(&val); or false),
            sentry_dsn: env!("CHARTED_SENTRY_DSN", optional),
            single_org: env!("CHARTED_SINGLE_ORG", |val| TRUTHY_REGEX.is_match(&val); or false),

            database: db::Config::try_from_env()?,
            sessions: sessions::Config::try_from_env()?,
            metrics: metrics::Config::from_env(),
            logging: logging::Config::from_env(),
            storage: storage::Config::try_from_env()?,
            server: server::Config::try_from_env()?,
            redis: redis::Config::try_from_env()?,
            cdn: cdn::Config::from_env(),
        })
    }
}

impl Config {
    /// Returns a default configuration path if it can find any.
    pub fn find_default_conf_location() -> Option<PathBuf> {
        let mut config_dir = Path::new("./config").to_path_buf();
        if config_dir.is_dir() {
            config_dir.push("charted.toml");
            if config_dir.exists() && config_dir.is_file() {
                return Some(config_dir.clone());
            }
        }

        match std::env::var("CHARTED_CONFIG_FILE") {
            Ok(path) => {
                let path = Path::new(&path);
                if path.exists() && path.is_file() {
                    return Some(path.to_path_buf());
                }

                None
            }

            Err(_) => {
                let last_resort = Path::new("./config.toml");
                if last_resort.exists() && last_resort.is_file() {
                    return Some(last_resort.to_path_buf());
                }

                None
            }
        }
    }

    /// Creates a new [`Config`] instance from a given path.
    pub fn new<P: AsRef<Path>>(path: Option<P>) -> eyre::Result<Config> {
        // priority: config file > env variables
        let Some(path) = path.as_ref() else {
            return Config::try_from_env();
        };

        let path = path.as_ref();
        if !path.try_exists()? {
            eprintln!(
                "[charted WARN] file {} doesn't exist, using system env variables",
                path.display()
            );

            return Config::try_from_env();
        }

        let mut cfg = Config::try_from_env()?;
        let mut contents = String::new();

        {
            let mut file = File::open(path)?;
            file.read_to_string(&mut contents)?;
        }

        let file: Config = toml::from_str(&contents)?;
        cfg.merge(file);

        if cfg.jwt_secret_key.is_empty() {
            let key = __generated_secret_key();
            eprintln!("[charted WARN] Missing a secret key for encoding JWT tokens, but I have generated one for you: {key} \
                Set this in the `CHARTED_JWT_SECRET_KEY` environment variable when loading the API server or in the `jwt_secret_key` in your `config.yml` file. \
                If any other key replaces this, then all JWT tokens will no longer be able to be verified, so it is recommended to keep this safe somewhere");

            cfg.jwt_secret_key = key;
        }

        Ok(cfg)
    }
}

fn __generated_secret_key() -> String {
    rand_string(16)
}

const fn __truthy() -> bool {
    true
}
