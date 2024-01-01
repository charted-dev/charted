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

use crate::{make_config, var, FromEnv};
use charted_common::serde::duration::Duration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CachingConfig {
    InMemory(InMemoryCacheConfig),
    Redis(RedisCacheConfig),
}

impl Default for CachingConfig {
    fn default() -> CachingConfig {
        CachingConfig::InMemory(InMemoryCacheConfig::default())
    }
}

impl FromEnv for CachingConfig {
    type Output = CachingConfig;

    fn from_env() -> Self::Output {
        match var!("CHARTED_CACHING_STRATEGY", or_else: "".into()).as_str() {
            "inmemory" | "in-memory" | "" => CachingConfig::InMemory(InMemoryCacheConfig::from_env()),
            "redis" => CachingConfig::Redis(RedisCacheConfig::from_env()),
            strategy => panic!("unknown value for `CHARTED_CACHING_STRATEGY`: {strategy}"),
        }
    }
}

make_config! {
    RedisCacheConfig {
        /// TTL for objects to be discarded in-memory, by default, this will always
        /// be 15 minutes.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub time_to_live: Option<Duration> {
            default: None;
            env_value: var!("CHARTED_CACHING_REDIS_TTL", to: Duration, is_optional: true);
        };
    }
}

make_config! {
    InMemoryCacheConfig {
        /// TTL for objects to be discarded in-memory, by default, this will always
        /// be 15 minutes.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub time_to_live: Option<Duration> {
            default: None;
            env_value: var!("CHARTED_CACHING_REDIS_TTL", to: Duration, is_optional: true);
        };

        // TODO(@auguwu): add `max_object_size` with custom unit (de)serializer
    }
}
