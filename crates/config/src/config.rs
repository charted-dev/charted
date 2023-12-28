// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

#![allow(unused_doc_comments)]

use crate::{
    cdn::CdnConfig, make_config, search::SearchConfig, sessions::SessionConfig, var, DatabaseConfig, FromEnv,
    LoggingConfig, MetricsConfig, RedisConfig, SecureSetting, SecureSettingError, ServerConfig, StorageConfig,
};
use charted_common::{panic_message, rand_string};
use eyre::{eyre, Result};
use merge_struct::merge;
use once_cell::sync::OnceCell;
use std::{
    fs::File,
    panic::catch_unwind,
    path::{Path, PathBuf},
};
use tracing::warn;

make_config! {
    /// Represents the main configuration object that is used
    /// within the CLI arguments, environment variables, or with
    /// a YAML file.
    ///
    /// ## Example
    /// ```no_run
    /// # use charted_config::{FromEnv, Config};
    /// #
    /// let config = Config::from_env();
    /// // loads the config from the system environment variables
    ///
    /// let config2 = Config::load(Some("./file.yaml"));
    /// // loads the config from ./file.yaml
    ///
    /// let config3 = Config::load::<std::path::PathBuf>(None);
    /// // loads from ./config/charted.yaml or ./config.yml
    /// ```
    Config {
        /// Valid gRPC endpoint to connect to when using the [emails microservice](https://charts.noelware.org/docs/services/emails/latest).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub emails_grpc_endpoint: Option<String> {
            default: None;
            env_value: var!("CHARTED_EMAILS_GRPC_ENDPOINT", is_optional: true);
        };

        /// Secret key to use for encoding JWT values for sessions.
        #[serde(default)]
        jwt_secret_key: String {
            default: rand_string(32);
            env_value: var!("CHARTED_JWT_SECRET_KEY", use_default: true);
        };

        /// A valid [DSN](https://docs.sentry.io/product/sentry-basics/dsn-explainer/) that is used
        /// to allow charted-server to output errors to [Sentry](https://sentry.io).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sentry_dsn: Option<String> {
            default: None;
            env_value: var!("CHARTED_SENTRY_DSN", is_optional: true);
        };

        /// Whether if registrations should be enabled on the server or not.
        #[serde(default = "truthy")]
        pub registrations: bool {
            default: true;
            env_value: var!("CHARTED_ENABLE_REGISTRATIONS", to: bool, or_else: true);
        };

        /// Database configuration details.
        #[serde(default)]
        pub database: DatabaseConfig {
            default: DatabaseConfig::default();
            env_value: DatabaseConfig::from_env();
        };

        /// Logging configuration details.
        #[serde(default)]
        pub logging: LoggingConfig {
            default: LoggingConfig::default();
            env_value: LoggingConfig::from_env();
        };

        #[serde(default)]
        pub sessions: SessionConfig {
            default: SessionConfig::default();
            env_value: SessionConfig::from_env();
        };

        #[serde(default, with = "serde_yaml::with::singleton_map")]
        pub storage: StorageConfig {
            default: StorageConfig::default();
            env_value: StorageConfig::from_env();
        };

        #[serde(default)]
        pub metrics: MetricsConfig {
            default: MetricsConfig::default();
            env_value: MetricsConfig::from_env();
        };

        #[serde(default)]
        pub server: ServerConfig {
            default: ServerConfig::default();
            env_value: ServerConfig::from_env();
        };

        #[serde(default)]
        pub search: Option<SearchConfig> {
            default: None;
            env_value: SearchConfig::from_env();
        };

        #[serde(default)]
        pub redis: RedisConfig {
            default: RedisConfig::default();
            env_value: RedisConfig::from_env();
        };

        #[serde(default)]
        pub cdn: CdnConfig {
            default: CdnConfig::default();
            env_value: CdnConfig::from_env();
        };
    }
}

fn truthy() -> bool {
    true
}

static CONFIG: OnceCell<Config> = OnceCell::new();
impl Config {
    fn find_default_conf_location() -> Option<PathBuf> {
        let mut config_dir = Path::new("./config").to_path_buf();
        if config_dir.is_dir() {
            config_dir.push("charted.yaml");
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
                let last_resort = Path::new("./config.yml");
                if last_resort.exists() && last_resort.is_file() {
                    return Some(last_resort.to_path_buf());
                }

                None
            }
        }
    }

    /// Sets a temporary [`Config`] that allows you to use the `get() -> Config`
    /// command without it panic.
    pub fn temporary() {
        if CONFIG.get().is_none() {
            CONFIG.set(Config::default()).unwrap();
        }
    }

    /// Returns a reference of the already-initialized configuration.
    ///
    /// ## Panics
    /// This method might panic if `Config::load/load_from` wasn't called.
    pub fn get() -> Config {
        CONFIG.clone().into_inner().unwrap()
    }

    /// Loads up the configuration object from a path, or from the default
    /// locations if not found.
    pub fn load<P: AsRef<Path>>(path: Option<P>) -> Result<()> {
        if CONFIG.get().is_some() {
            warn!("configuration was already previously loaded, not doing anything.");
            return Ok(());
        }

        // Since from any FromEnv impl. can panic, let's capture any panics
        // and return back to the caller.
        let env = catch_unwind(Config::from_env)
            .map_err(|e| eyre!(format!("Panic'd during transformation: {}", panic_message(e))))?;

        let path = match path {
            Some(path) => path.as_ref().to_path_buf(),
            None => match Config::find_default_conf_location() {
                Some(p) => p,
                None => {
                    CONFIG.set(env).unwrap();
                    return Ok(());
                }
            },
        };

        let mut serialized = serde_yaml::from_reader::<_, Config>(File::open(path)?)?;
        if serialized.jwt_secret_key.as_str() == "" {
            let key = rand_string(64);
            eprintln!("[charted::preinit ~ WARN] JWT secret key was not found, so I generated one for you: {key}");
            eprintln!("[charted::preinit ~ WARN] Set this from the `CHARTED_JWT_SECRET_KEY` environment variable, or in `jwt_secret_key` in the configuration.");

            serialized.jwt_secret_key = key;
        }

        CONFIG.set(merge(&env, &serialized.clone())?).unwrap();
        Ok(())
    }
}

macro_rules! config_ext_trait {
    (
        $($(#[$meta:meta])* $key:ident: $ty:ty;)*
    ) => {
        $(
            $(#[$meta])*
            fn $key(&self) -> Result<$ty, $crate::SecureSettingError>;
        )*
    };
}

/// Extensions that can be used with the [`Config`] struct.
pub trait ConfigExt: private::Sealed {
    config_ext_trait! {
        /// A valid [DSN](https://docs.sentry.io/product/sentry-basics/dsn-explainer/) that is used
        /// to allow charted-server to output errors to [Sentry](https://sentry.io).
        sentry_dsn: Option<String>;

        /// Secret key to use for encoding JWT values for sessions.
        jwt_secret_key: String;
    }
}

impl ConfigExt for Config {
    fn sentry_dsn(&self) -> Result<Option<String>, SecureSettingError> {
        let secure_setting = SecureSetting::new("sentry_dsn");
        match self.sentry_dsn.clone() {
            Some(res) => secure_setting.load_optional(res),
            None => Ok(None),
        }
    }

    fn jwt_secret_key(&self) -> Result<String, SecureSettingError> {
        let secure_setting = SecureSetting::new("jwt_secret_key");
        secure_setting.load(self.jwt_secret_key.clone())
    }
}

mod private {
    pub trait Sealed {}
    impl Sealed for crate::Config {}
}
