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

use self::{logging::LoggingConfig, server::ServerConfig, smtp::SmtpConfig};
use charted_common::panic_message;
use charted_config::{make_config, var, FromEnv, SecureSetting, SecureSettingError};
use eyre::{eyre, Result};
use merge_struct::merge;
use once_cell::sync::OnceCell;
use std::{
    fs::File,
    panic::catch_unwind,
    path::{Path, PathBuf},
};

fn default_templates_dir() -> PathBuf {
    PathBuf::from("./templates")
}

make_config! {
    Config {
        /// DSN string for using Sentry to catch errors when they are captured.
        #[serde(skip_serializing_if = "Option::is_none")]
        sentry_dsn: Option<String> {
            default: None;
            env_value: var!("EMAILS_SENTRY_DSN", is_optional: true);
        };

        /// [`PathBuf`] to the directory to look for templates.
        #[serde(default = "default_templates_dir")]
        pub templates: PathBuf {
            default: default_templates_dir();
            env_value: var!("EMAILS_TEMPLATE_DIR", to: PathBuf, or_else: default_templates_dir());
        };

        #[serde(default)]
        pub logging: LoggingConfig {
            default: LoggingConfig::default();
            env_value: LoggingConfig::from_env();
        };

        #[serde(default)]
        pub server: ServerConfig {
            default: ServerConfig::default();
            env_value: ServerConfig::from_env();
        };

        #[serde(default)]
        pub smtp: SmtpConfig {
            default: SmtpConfig::default();
            env_value: SmtpConfig::from_env();
        };
    }
}

static CONFIG: OnceCell<Config> = OnceCell::new();

impl Config {
    fn find_default_location() -> Option<PathBuf> {
        let mut config_dir = Path::new("./config").to_path_buf();
        if config_dir.is_dir() {
            config_dir.push("emails.yaml");
            if config_dir.exists() && config_dir.is_file() {
                return Some(config_dir.clone());
            }
        }

        match std::env::var("EMAILS_CONFIG_FILE") {
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

    /// Returns a reference of the already-initialized configuration.
    ///
    /// ## Panics
    /// This method might panic if `Config::load` wasn't called.
    pub fn get() -> Config {
        CONFIG.clone().into_inner().unwrap()
    }

    pub fn sentry_dsn(&self) -> Result<Option<String>, SecureSettingError> {
        let secure_setting = SecureSetting::new("sentry_dsn");
        match self.sentry_dsn.clone() {
            Some(dsn) => secure_setting.load_optional(dsn),
            None => Ok(None),
        }
    }

    pub fn load<P: AsRef<Path>>(path: Option<P>) -> Result<()> {
        if CONFIG.get().is_some() {
            warn!("configuration was already previously loaded!");
            return Ok(());
        }

        let env = catch_unwind(Config::from_env)
            .map_err(|e| eyre!("env variable transformation failed: {}", panic_message(e)))?;

        let path = match path {
            Some(path) => path.as_ref().to_path_buf(),
            None => match Config::find_default_location() {
                Some(path) => path,
                None => {
                    CONFIG.set(env).unwrap();
                    return Ok(());
                }
            },
        };

        let config = serde_yaml::from_reader::<_, Config>(File::open(path)?)?;
        CONFIG.set(merge(&env, &config.clone())?).unwrap();

        Ok(())
    }
}

mod server {
    use charted_config::{make_config, var};

    make_config! {
        ServerConfig {
            /// Server port to bind to. This will bind to `32121` by default.
            #[serde(default = "port")]
            pub port: u16 {
                default: port();
                env_value: var!("EMAILS_SERVER_PORT", to: u16, or_else: port());
            };

            /// Host string to bind to. By default, the service will bind on `0.0.0.0`.
            #[serde(default = "host")]
            pub host: String {
                default: host();
                env_value: var!("EMAILS_SERVER_HOST", or_else: host());
            };
        }
    }

    fn port() -> u16 {
        32121
    }

    fn host() -> String {
        "0.0.0.0".into()
    }
}

mod logging {
    use charted_common::TRUTHY_REGEX;
    use charted_config::{make_config, serde_tracing, var, LogstashOutput};
    use tracing::Level;

    make_config! {
        LoggingConfig {
            /// Log level to output log messages in.
            #[serde(with = "serde_tracing")]
            pub level: Level {
                default: Level::INFO;
                env_value: ::std::env::var("EMAILS_LOG_LEVEL").map(|val| match val.as_str() {
                    "trace" => Level::TRACE,
                    "debug" => Level::DEBUG,
                    "info" => Level::INFO,
                    "warn" => Level::WARN,
                    _ => Level::INFO,
                }).unwrap_or(Level::INFO);
            };

            /// The output type to use for outputting events that were received via the JSON visitor into.
            ///
            /// * [`LogstashOutput::TCP`] is for the [TCP input plugin](https://www.elastic.co/guide/en/logstash/current/plugins-inputs-tcp.html)
            ///   that will output everything in a single TCP socket connection.
            /// * [`LogstashOutput::UDP`] is for the [UDP input plugin](https://www.elastic.co/guide/en/logstash/current/plugins-inputs-udp.html)
            ///   that will output everything in a UDP socket connection.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub logstash_output: Option<LogstashOutput> {
                default: None;
                env_value: var!("EMAILS_LOGSTASH_OUTPUT", to: LogstashOutput, is_optional: true);
            };

            /// Connection URI to use when connecting to the configured Logstash TCP or UDP stream.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub logstash_connect_uri: Option<String> {
                default: None;
                env_value: var!("EMAILS_LOGSTASH_CONNECTION_URI", is_optional: true);
            };

            /// Whether the logger should only output JSON messages or not.
            #[serde(rename = "json", default)]
            pub json_logging: bool {
                default: false;
                env_value: var!("EMAILS_LOG_JSON", {
                    or_else: false;
                    mapper: |val| TRUTHY_REGEX.is_match(val.as_str());
                });
            };
        }
    }
}

mod smtp {
    use charted_common::TRUTHY_REGEX;
    use charted_config::{make_config, var, SecureSetting, SecureSettingError};
    use eyre::Result;

    make_config! {
        SmtpConfig {
            /// Username for authenticating to the SMTP server.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub username: Option<String> {
                default: None;
                env_value: var!("EMAILS_SMTP_USERNAME", is_optional: true);
            };

            /// Password for authenticating to the SMTP server.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            password: Option<String> {
                default: None;
                env_value: var!("EMAILS_SMTP_PASSWORD", is_optional: true);
            };

            /// Address to use when forwarding emails.
            pub from: String {
                default: "".into();
                env_value: var!("EMAILS_SMTP_FROM_ADDRESS").unwrap_or_else(|_| panic!("missing required environment variable: `EMAILS_SMTP_FROM_ADDRESS`"));
            };

            #[serde(default = "host")]
            pub host: String {
                default: host();
                env_value: var!("EMAILS_SMTP_HOST", or_else: host());
            };

            #[serde(default = "port")]
            pub port: u16 {
                default: port();
                env_value: var!("EMAILS_SMTP_PORT", to: u16, or_else: port());
            };

            #[serde(default)]
            pub tls: bool {
                default: false;
                env_value: var!("EMAILS_SMTP_STARTTLS", {
                    or_else: false;
                    mapper: |val| TRUTHY_REGEX.is_match(val.as_str());
                });
            };

            #[serde(default)]
            pub ssl: bool {
                default: false;
                env_value: var!("EMAILS_SMTP_SSL", {
                    or_else: false;
                    mapper: |val| TRUTHY_REGEX.is_match(val.as_str());
                });
            };
        }
    }

    impl SmtpConfig {
        pub fn password(&self) -> Result<Option<String>, SecureSettingError> {
            let secure_setting = SecureSetting::new("smtp.password");
            match self.password.clone() {
                Some(passwd) => secure_setting.load_optional(passwd),
                None => Ok(None),
            }
        }
    }

    fn port() -> u16 {
        587
    }

    fn host() -> String {
        "localhost".into()
    }
}
