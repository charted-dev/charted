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

use crate::{var, FromEnv};
use serde::{Deserialize, Serialize};

/// Represents the configuration for configuring a Elasticsearch cluster
/// or a Meilisearch server to do full-text search capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchConfig {
    Elasticsearch(elasticsearch::Config),
    Meilisearch(meilisearch::Config),
}

impl FromEnv<Option<SearchConfig>> for SearchConfig {
    fn from_env() -> Option<SearchConfig> {
        match var!("CHARTED_SEARCH_BACKEND", is_optional: true) {
            Some(backend) => match backend.as_str() {
                "es" | "elasticsearch" => Some(SearchConfig::Elasticsearch(elasticsearch::Config::from_env())),
                "meili" | "meilisearch" => Some(SearchConfig::Meilisearch(meilisearch::Config::from_env())),
                _ => None,
            },
            None => None,
        }
    }
}

pub mod elasticsearch {
    use crate::{make_config, var, FromEnv};
    use base64::Engine;
    use charted_common::TRUTHY_REGEX;
    use serde::{Deserialize, Serialize};

    /// Represents a union struct of the possible available authentication
    /// types for Elasticsearch.
    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    pub enum AuthType {
        ApiKey(String),
        BasicAuth {
            username: String,
            password: String,
        },

        Cloud {
            id: String,
            auth: String,
        },

        #[default]
        None,
    }

    impl FromEnv<Option<AuthType>> for AuthType {
        fn from_env() -> Option<AuthType> {
            if let Some(key) = var!("CHARTED_ELASTICSEARCH_AUTH_API_KEY", is_optional: true) {
                return Some(AuthType::ApiKey(key));
            }

            match (
                var!("CHARTED_ELASTICSEARCH_AUTH_USERNAME", is_optional: true),
                var!("CHARTED_ELASTICSEARCH_AUTH_PASSWORD", is_optional: true),
            ) {
                (Some(username), Some(password)) => return Some(AuthType::BasicAuth { username, password }),
                (None, Some(_)) => panic!(
                    "`search.elasticsearch.password` was provided, but `search.elasticsearch.username` was missing"
                ),
                (Some(_), None) => panic!(
                    "`search.elasticsearch.username` was provided, but `search.elasticsearch.password` was missing"
                ),
                _ => {} // escape
            }

            if let Some(cloud_auth) = var!("CHARTED_ELASTICSEARCH_AUTH_CLOUD_ID", is_optional: true) {
                if cloud_auth.contains(':') {
                    let auth_details = cloud_auth.split(':').collect::<Vec<_>>();
                    match (auth_details.first(), auth_details.get(1)) {
                        (Some(id), Some(creds)) => {
                            return Some(AuthType::Cloud {
                                id: id.to_string(),
                                auth: creds.to_string(),
                            })
                        }
                        (Some(_), None) => panic!("cloud id credentials is malformed: missing creds in '<id>:<creds>'"),
                        (None, Some(_)) => {
                            panic!("cloud id credentials is malformed: missing deployment id in '<id>:<creds>'")
                        }
                        _ => {}
                    }
                }

                let auth_details =
                    String::from_utf8_lossy(&base64::engine::general_purpose::STANDARD.decode(cloud_auth).expect("`CHARTED_ELASTICSEARCH_AUTH_CLOUD_ID` env var to be a base64 string of '<deployment id>:<creds>"))
                        .to_string();

                if !auth_details.contains(':') {
                    panic!("cloud id is malformed: payload didn't contain ':' (expected: '<deployment id>:<creds>");
                }

                let auth_details = auth_details.split(':').collect::<Vec<_>>();
                match (auth_details.first(), auth_details.get(1)) {
                    (Some(id), Some(creds)) => {
                        return Some(AuthType::Cloud {
                            id: id.to_string(),
                            auth: creds.to_string(),
                        })
                    }
                    (Some(_), None) => panic!("cloud id credentials is malformed: missing creds in '<id>:<creds>'"),
                    (None, Some(_)) => {
                        panic!("cloud id credentials is malformed: missing deployment id in '<id>:<creds>'")
                    }
                    _ => {}
                }
            }

            None
        }
    }

    make_config! {
        /// Configuration to configure an Elasticsearch cluster with one
        /// or more more nodes.
        Config {
            /// List of Elasticsearch nodes to use for full-text search.
            #[serde(default = "default_es_nodes")]
            pub nodes: Vec<String> {
                default: default_es_nodes();
                env_value: var!("CHARTED_ELASTICSEARCH_NODES", {
                    or_else: default_es_nodes();
                    mapper: |val| val.split(',').map(|f| f.to_string()).collect::<Vec<_>>();
                });
            };

            /// Global authentication type for all the Elasticsearch nodes.
            #[serde(default, skip_serializing_if = "Option::is_none", with = "serde_yaml::with::singleton_map")]
            pub auth: Option<AuthType> {
                default: None;
                env_value: AuthType::from_env();
            };

            /// Whether if SSL is enabled for all Elasticsearch nodes or not.
            #[serde(default)]
            pub ssl: bool {
                default: false;
                env_value: var!("CHARTED_ELASTICSEARCH_SSL", {
                    or_else: false;
                    mapper: |val| TRUTHY_REGEX.is_match(val.as_str());
                });
            };
        }
    }

    fn default_es_nodes() -> Vec<String> {
        vec!["http://localhost:9200".into()]
    }
}

pub mod meilisearch {
    use crate::{make_config, var, SecureSetting, SecureSettingError};

    make_config! {
        /// Configuration to configure a Meilisearch server to do
        /// full-text search.
        Config {
            #[serde(default = "default_meilisearch_url")]
            pub server_url: String {
                default: default_meilisearch_url();
                env_value: var!("CHARTED_SEARCH_MEILISEARCH_SERVER_URL", or_else: default_meilisearch_url());
            };

            #[serde(default, skip_serializing_if = "Option::is_none")]
            master_key: Option<String> {
                default: None;
                env_value: var!("CHARTED_SEARCH_MEILISEARCH_MASTER_KEY", is_optional: true);
            };
        }
    }

    impl Config {
        pub fn master_key(&self) -> Result<Option<String>, SecureSettingError> {
            let secure_setting = SecureSetting::new("search.meilisearch.master_key");
            match self.master_key.clone() {
                Some(key) => secure_setting.load_optional(key.as_str()),
                None => Ok(None),
            }
        }
    }

    fn default_meilisearch_url() -> String {
        "http://localhost:7700".into()
    }
}
