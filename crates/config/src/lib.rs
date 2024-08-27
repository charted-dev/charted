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

use std::{
    env::VarError,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use azalia::{
    config::{env, merge::Merge, FromEnv, TryFromEnv},
    TRUTHY_REGEX,
};
use rand::distributions::{Alphanumeric, DistString};
use sentry_types::Dsn;
use serde::{Deserialize, Serialize};

pub(crate) mod helpers;

pub mod database;
pub mod logging;
pub mod metrics;
pub mod server;
pub mod sessions;
pub mod storage;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Merge)]
pub struct Config {
    /// whether or not if users can be registered on this instance
    #[serde(default = "__truthy")]
    #[merge(strategy = azalia::config::merge::strategy::bool::only_if_falsy)]
    pub registrations: bool,

    /// Secret key for encoding JWT tokens. This must be set once and never touched again.
    #[serde(default)]
    #[merge(skip)] // don't even attempt to merge jwt secret keys
    pub jwt_secret_key: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentry_dsn: Option<Dsn>,

    /// whether or not if the API server should act like a single organization, where most features
    /// are disabled like repository/organization members and audit logging.
    #[serde(default)]
    #[merge(strategy = azalia::config::merge::strategy::bool::only_if_falsy)]
    pub single_org: bool,

    /// whether or not if the API server should act like a single user, where *most* features
    /// are disabled and only one user is allowed to roam.
    ///
    /// all publically available features like Audit Logging can be enabled but repository and
    /// organization members are disabled. most endpoints will be also disabled.
    #[serde(default)]
    #[merge(strategy = azalia::config::merge::strategy::bool::only_if_falsy)]
    pub single_user: bool,

    #[serde(default)]
    pub database: database::Config,

    #[serde(default)]
    pub storage: storage::Config,

    #[serde(default)]
    pub logging: logging::Config,

    #[serde(default)]
    pub server: server::Config,

    #[serde(default)]
    pub sessions: sessions::Config,
}

impl TryFromEnv for Config {
    type Output = Config;
    type Error = eyre::Report;

    fn try_from_env() -> Result<Self::Output, Self::Error> {
        Ok(Config {
            jwt_secret_key: helpers::env_from_result(env!("CHARTED_JWT_SECRET_KEY"), __generated_secret_key())?,
            registrations: env!("CHARTED_ENABLE_REGISTRATIONS", |val| TRUTHY_REGEX.is_match(&val); or true),
            single_user: env!("CHARTED_SINGLE_USER", |val| TRUTHY_REGEX.is_match(&val); or false),
            single_org: env!("CHARTED_SINGLE_ORG", |val| TRUTHY_REGEX.is_match(&val); or false),
            sentry_dsn: helpers::env_optional_from_str("CHARTED_SENTRY_DSN", None)?,

            database: database::Config::try_from_env()?,
            sessions: sessions::Config::try_from_env()?,
            logging: logging::Config::from_env(),
            storage: storage::Config::try_from_env()?,
            server: server::Config::try_from_env()?,
        })
    }
}

impl Config {
    pub fn get_default_conf_location_if_any() -> eyre::Result<Option<PathBuf>> {
        let config_dir = PathBuf::from("./config");
        if config_dir.is_dir() && config_dir.try_exists()? {
            let hcl = config_dir.join("charted.hcl");
            if hcl.is_file() && hcl.try_exists()? {
                return Ok(Some(hcl));
            }
        }

        match azalia::config::env!("CHARTED_CONFIG_FILE").map(PathBuf::from) {
            Ok(path) if path.try_exists()? && path.is_file() => Ok(Some(path)),
            Ok(_) | Err(VarError::NotPresent) => {
                let last_resort = PathBuf::from("./config.hcl");
                if last_resort.is_file() && last_resort.is_file() {
                    return Ok(Some(last_resort));
                }

                Ok(None)
            }

            Err(e) => Err(eyre::eyre!(e)),
        }
    }

    pub fn new<P: AsRef<Path>>(path: Option<P>) -> eyre::Result<Config> {
        // priority: config file > env variables
        let Some(path) = path.as_ref() else {
            return Config::try_from_env();
        };

        let path = path.as_ref();
        if !path.try_exists()? {
            eprintln!(
                "[charted :: WARN] file '{}' doesn't exist; using system environment variable instead",
                path.display()
            );
            return Config::try_from_env();
        }

        let mut config = Config::try_from_env()?;
        let mut contents = String::new();

        {
            let mut file = File::open(path)?;
            file.read_to_string(&mut contents)?;
        }

        let file: Config = hcl::from_str(&contents)?;
        config.merge(file);

        if config.jwt_secret_key.is_empty() {
            let key = __generated_secret_key();
            eprintln!("[charted WARN] Missing a secret key for encoding JWT tokens, but I have generated one for you: {key} \
                Set this in the `CHARTED_JWT_SECRET_KEY` environment variable when loading the API server or in the `jwt_secret_key` in your `config.hcl` file. \
                If any other key replaces this, then all JWT tokens will no longer be able to be verified, so it is recommended to keep this safe somewhere");

            config.jwt_secret_key = key;
        }

        Ok(config)
    }
}

fn __generated_secret_key() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 16)
}

const fn __truthy() -> bool {
    true
}
