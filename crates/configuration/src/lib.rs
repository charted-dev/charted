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

pub mod database;
pub mod features;
pub mod logging;
pub mod metrics;
pub mod server;
pub mod sessions;
pub mod storage;
pub mod tracing;
pub(crate) mod util;

use azalia::config::{FromEnv, TryFromEnv, env, merge::Merge};
use sentry_types::Dsn;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};
use url::Url;

/// The root configuration for the API server.
///
/// **charted-server** uses a TOML-based configuration format for easy accessiblity.
///
/// **charted-server** also supports environment variables that can be overwritten when
/// the configuration is being loaded. The priority is **Environment Variables >
/// Configuration File**.
#[derive(Debug, Clone, Serialize, Deserialize, Merge)]
pub struct Config {
    /// A secret key for generating JWT tokens for session-based authentication.
    ///
    /// It is recommended to set this as the `CHARTED_JWT_SECRET_KEY` environment
    /// variable and **charted-server** will load it into this property.
    ///
    /// If this is ever messed with, sessions that are on-going will be permanently
    /// corrupted.
    #[serde(default)]
    #[merge(strategy = azalia::config::merge::strategy::strings::overwrite_empty)]
    pub jwt_secret_key: String,

    /// Whether if this instance accepts user registrations.
    #[serde(default = "crate::util::truthy")]
    pub registrations: bool,

    /// whether if this instance should like a single user registry.
    ///
    /// If this is the case, most features are disabled like organizations,
    /// repository/organization members, user creation, etc.
    ///
    /// You can use either the `charted admin user new <myname> <myemail> <mypassword>`
    /// if you're going to use the local session backend or use the static backend with
    /// `user = "password"`.
    #[serde(default)]
    pub single_user: bool,

    /// whether if this instance should act like a single organization registry.
    ///
    /// If so, most features are disabled like user creation, repository members, etc.
    #[serde(default)]
    pub single_org: bool,

    /// opt into reporting errors to a [Sentry] server.
    ///
    /// [Sentry]: https://sentry.io
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sentry_dsn: Option<Dsn>,

    /// URI that will redirect all API requests and Helm chart downloads towards.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_url: Option<Url>,

    #[serde(default)]
    pub database: database::Config,

    #[serde(default)]
    pub logging: logging::Config,

    #[serde(default)]
    pub server: server::Config,

    #[serde(default)]
    pub storage: storage::Config,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracing: Option<tracing::Config>,

    #[serde(default)]
    pub sessions: sessions::Config,
}

impl Config {
    pub fn find_default_location() -> eyre::Result<Option<PathBuf>> {
        const VALID_FILE_NAMES: &[&str; 2] = &["charted.toml", "config.toml"];

        let config_dir = PathBuf::from("./config");
        if config_dir.is_dir() && config_dir.try_exists()? {
            let charted_toml = config_dir.join(VALID_FILE_NAMES[0]);
            if charted_toml.try_exists()? && charted_toml.is_file() {
                return Ok(Some(charted_toml));
            }
        }

        for file in VALID_FILE_NAMES {
            let path = PathBuf::from(format!("./{file}"));
            if path.try_exists()? && path.is_file() {
                return Ok(Some(path));
            }
        }

        Ok(None)
    }

    pub fn load<P: AsRef<Path>>(path: Option<P>) -> eyre::Result<Self> {
        let Some(path) = path.as_ref() else {
            let mut config = Config::try_from_env()?;
            update_config(&mut config)?;

            return Ok(config);
        };

        let path = path.as_ref();
        if !path.try_exists()? {
            eprintln!(
                "[charted :: WARN] file [{}] doesn't exist; using system environment variables instead",
                path.display()
            );

            let mut config = Config::try_from_env()?;
            update_config(&mut config)?;

            return Ok(config);
        }

        let mut config = Config::try_from_env()?;
        let contents = fs::read_to_string(path)?;

        let file: Config = toml::from_str(&contents)?;
        config.merge(file);

        update_config(&mut config)?;

        Ok(config)
    }
}

pub const JWT_SECRET_KEY: &str = "CHARTED_JWT_SECRET_KEY";
pub const REGISTRATIONS: &str = "CHARTED_ENABLE_REGISTRATIONS";
pub const SINGLE_USER: &str = "CHARTED_SINGLE_USER";
pub const SINGLE_ORG: &str = "CHARTED_SINGLE_ORGANIZATION";
pub const SENTRY_DSN: &str = "CHARTED_SENTRY_DSN";
pub const BASE_URL: &str = "CHARTED_BASE_URL";

impl TryFromEnv for Config {
    type Error = eyre::Report;
    type Output = Self;

    fn try_from_env() -> Result<Self::Output, Self::Error> {
        Ok(Self {
            jwt_secret_key: util::env_from_result_lazy(env!(JWT_SECRET_KEY), || Ok(String::default()))?,
            registrations: util::bool_env(REGISTRATIONS)?,
            single_user: util::bool_env(SINGLE_USER)?,
            single_org: util::bool_env(SINGLE_ORG)?,
            sentry_dsn: util::env_optional_from_str(SENTRY_DSN, None)?,
            base_url: util::env_optional_from_str(BASE_URL, None)?,
            database: database::Config::try_from_env()?,
            sessions: sessions::Config::try_from_env()?,
            logging: logging::Config::from_env(),
            storage: storage::Config::try_from_env()?,
            server: server::Config::try_from_env()?,

            tracing: match util::bool_env(tracing::ENABLED) {
                Ok(true) => Some(tracing::Config::try_from_env()?),
                Ok(false) => None,
                Err(e) => return Err(e),
            },
        })
    }
}

fn update_config(config: &mut Config) -> eyre::Result<()> {
    if config.jwt_secret_key.is_empty() {
        let key = charted_core::rand_string(16);
        eprintln!(
            r#"[charted :: WARN] You are missing a JWT secret key either from the `${env}` environment variable or from the `jwt_secret_key` configuration property in your `config.toml`

I generated one for you here: `{secret}`

!! DO NOT LOSE IT !!"#,
            env = JWT_SECRET_KEY,
            secret = key
        );

        config.jwt_secret_key = key;
    }

    if config.base_url.is_none() {
        let scheme = match config.server.ssl {
            Some(_) => "https",
            None => "http",
        };

        let url = Url::parse(&format!("{scheme}://{}", config.server.to_socket_addr()))?;
        eprintln!("[charted :: WARN] `base_url` was not configured properly! All URLs will be mapped to {url}");

        config.base_url = Some(url);
    }

    Ok(())
}
