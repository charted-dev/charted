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

use crate::{make_config, var, SecureSetting, SecureSettingError};
use charted_common::TRUTHY_REGEX;

make_config! {
    /// Configuration to create a Redis connection pool
    RedisConfig {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        master_name: Option<String> {
            default: None;
            env_value: var!("CHARTED_REDIS_SENTINEL_MASTER_NAME", is_optional: true);
        };

        #[serde(default, skip_serializing_if = "Option::is_none")]
        password: Option<String> {
            default: None;
            env_value: var!("CHARTED_REDIS_PASSWORDS", is_optional: true);
        };

        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub hosts: Vec<String> {
            default: default_redis_hosts();
            env_value: var!("CHARTED_REDIS_HOSTS", {
                or_else: default_redis_hosts();
                mapper: |val| val.split(',').map(|f| f.to_string()).collect::<Vec<_>>();
            });
        };

        #[serde(default = "default_tls")]
        pub tls: bool {
            default: default_tls();
            env_value: var!("CHARTED_REDIS_TLS_ENABLE", {
                or_else: default_tls();
                mapper: |val| TRUTHY_REGEX.is_match(val.as_str());
            });
        };

        #[serde(default = "default_db")]
        pub db: u8 {
            default: default_db();
            env_value: var!("CHARTED_REDIS_DB", to: u8, or_else: default_db());
        };
    }
}

impl RedisConfig {
    pub fn password(&self) -> Result<Option<String>, SecureSettingError> {
        match self.password.clone() {
            Some(password) => SecureSetting::new("redis.password").load_optional(password.as_str()),
            None => Ok(None),
        }
    }

    pub fn master_name(&self) -> Result<Option<String>, SecureSettingError> {
        match self.password.clone() {
            Some(password) => SecureSetting::new("redis.master_name").load_optional(password.as_str()),
            None => Ok(None),
        }
    }
}

fn default_redis_hosts() -> Vec<String> {
    vec!["redis://localhost:6379".into()]
}

fn default_tls() -> bool {
    false
}

fn default_db() -> u8 {
    2
}
