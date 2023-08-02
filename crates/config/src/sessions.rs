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

use crate::{make_config, var, FromEnv};

make_config! {
    SessionConfig {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub integrations: Vec<integrations::Config> {
            default: vec![];
            env_value: vec![];
        };

        #[serde(default, with = "serde_yaml::with::singleton_map")]
        pub backend: SessionBackend {
            default: SessionBackend::default();
            env_value: SessionBackend::from_env();
        };
    }
}

/// Represents the session backend to delegate towards when creating a session
/// provider.
///
/// * [`Passwordless`][SessionBackend::Passwordless] is essentially the same as the
/// [local session backend][SessionBackend::Local], but will send the user an email to authenticate.
/// This will require you to run charted's gRPC-based email microservice.
///
/// * [`Local`][SessionBackend::Local] will use the local session provider, which will use passwords
/// from the `password` database field in the `users` database table.
///
/// * [`LDAP`][SessionBackend::LDAP] will connect and authenticate users from an established LDAP server.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum SessionBackend {
    LDAP(ldap::Config),
    Passwordless,

    #[default]
    Local,
}

impl FromEnv<SessionBackend> for SessionBackend {
    fn from_env() -> SessionBackend {
        match var!("CHARTED_SESSIONS_BACKEND", is_optional: true) {
            Some(backend) => match backend.as_str() {
                "local" => SessionBackend::Local,
                "ldap" => SessionBackend::LDAP(ldap::Config::from_env()),
                _ => panic!("expected 'local' or 'ldap', received {}", backend),
            },
            None => SessionBackend::default(),
        }
    }
}

pub mod ldap {
    use crate::{make_config, var};
    use charted_common::TRUTHY_REGEX;

    make_config! {
        Config {
            //pub conn_timeout: Duration {};

            #[serde(default)]
            pub no_tls_verify: bool {
                default: false;
                env_value: var!("CHARTED_SESSIONS_LDAP_NO_VERIFY_TLS", {
                    or_else: false;
                    mapper: |val| TRUTHY_REGEX.is_match(val.as_str());
                });
            };

            #[serde(default)]
            pub starttls: bool {
                default: false;
                env_value: var!("CHARTED_SESSIONS_LDAP_STARTTLS", {
                    or_else: false;
                    mapper: |val| TRUTHY_REGEX.is_match(val.as_str());
                });
            };
        }
    }
}

/*
pub fn new() -> LdapConnSettings
Create an instance of the structure with default settings.

source
pub fn set_conn_timeout(self, timeout: Duration) -> Self
Set the connection timeout. If a connetion to the server can’t be established before the timeout expires, an error will be returned to the user. Defaults to None, meaning an infinite timeout.

source
pub fn set_no_tls_verify(self, no_tls_verify: bool) -> Self
If true, try to establish a TLS connection without certificate verification. Defaults to false.
*/

pub mod integrations {
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(tag = "type")]
    pub enum Config {
        GitHub(github::Config),
    }

    pub mod github {
        use crate::{make_config, var, SecureSetting, SecureSettingError};

        make_config! {
            Config {
                #[serde(default)]
                client_secret: String {
                    default: "".into();
                    env_value: var!("CHARTED_SESSIONS_INTEGRATIONS_GITHUB_CLIENT_SECRET", or_else: "".into());
                };

                #[serde(default = "default_server_url")]
                pub server_url: String {
                    default: default_server_url();
                    env_value: var!("CHARTED_SESSIONS_INTEGRATIONS_GITHUB_SERVER", or_else: default_server_url());
                };

                #[serde(default)]
                pub client_id: String {
                    default: "".into();
                    env_value: var!("CHARTED_SESSIONS_INTEGRATIONS_GITHUB_CLIENT_ID", or_else: "".into());
                };
            }
        }

        impl Config {
            pub fn client_secret(&self) -> Result<String, SecureSettingError> {
                let secure_setting = SecureSetting::new("sessions.integrations.github.client_secret".into());
                secure_setting.load(self.client_secret.clone())
            }
        }

        fn default_server_url() -> String {
            String::from("https://github.com")
        }
    }
}
