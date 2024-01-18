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

use eyre::Report;
use noelware_config::{env, merge::Merge, FromEnv, TryFromEnv};
use serde::{Deserialize, Serialize};

/// Represents a union-sum type that can represent what cache worker we should use
/// for a object. We support using the provided Redis configuration as the cache or
/// in-memory where it lives as long the program lives and destroys when the program
/// closes.
///
/// - Mergable: false
///
/// ## Environment Variables
/// - `CHARTED_CACHE_STRATEGY` (`"inmemory"` | `"redis"`): the cache strategy to use.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Config {
    /// Uses the internal in-memory cache worker. All objects that are put into the cache
    /// are destroyed after the server closes. This is not recommended for production
    /// environments.
    InMemory(inmemory::Config),

    /// Uses the Redis client that was previously configured to cache objects in. The objects
    /// are cached as long they exist in the database, a job worker is created to clean up old
    /// objects every ~15 minutes to not bloat up Redis.
    Redis(redis::Config),
}

impl Merge for Config {
    fn merge(&mut self, other: Self) {
        match (self, other) {
            (Self::InMemory(inmem), Self::InMemory(inmem2)) => inmem.merge(inmem2),
            (Self::Redis(redis), Self::Redis(redis2)) => redis.merge(redis2),
            _ => panic!("cannot replace given `self` -> `other` (this is a bug in your configuration, not to us)"),
        }
    }
}

impl Default for Config {
    fn default() -> Config {
        Config::InMemory(inmemory::Config::default())
    }
}

impl TryFromEnv for Config {
    type Output = Config;
    type Error = Report;

    fn try_from_env() -> Result<Self::Output, Self::Error> {
        match env!("CHARTED_CACHE_STRATEGY") {
            Ok(res) => match res.as_str() {
                "inmemory" | "in-memory" => Ok(Config::InMemory(inmemory::Config::from_env())),
                "redis" => Ok(Config::Redis(redis::Config::from_env())),
                out if out.is_empty() => Err(eyre!(
                    "expected a single value [inmemory/in-memory, redis]; received nothing"
                )),

                out => Err(eyre!(
                    "expected a valid value [inmemory/in-memory, redis]; received {out} instead"
                )),
            },

            // if it was not found or if it was invalid utf-8, then just
            // use the default than to panic.
            Err(_) => Ok(Default::default()),
        }
    }
}

pub mod inmemory {
    use crate::common::serde::Duration;
    use noelware_config::{env, merge::Merge, FromEnv};
    use serde::{Deserialize, Serialize};
    use ubyte::ByteUnit;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Config {
        /// Max size of a object that can be stored in the cache. By default, it is 1 megabyte of the
        /// object that has been serialized from JSON.
        #[serde(default = "__one_megabyte")]
        pub max_object_size: ByteUnit,

        /// Time-to-live for all objects to be discarded in-memory. This will default to 15 minutes.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ttl: Option<Duration>,
    }

    impl Merge for Config {
        fn merge(&mut self, other: Self) {
            self.ttl.merge(other.ttl);

            // don't do anything if they are the same
            if self.max_object_size == other.max_object_size {
                return;
            }

            // do it if they're not the same
            if self.max_object_size != other.max_object_size {
                self.max_object_size = other.max_object_size;
            }
        }
    }

    impl Default for Config {
        fn default() -> Config {
            Config {
                max_object_size: __one_megabyte(),
                ttl: Some(std::time::Duration::from_secs(15 * 60).into()),
            }
        }
    }

    impl FromEnv for Config {
        type Output = Config;

        fn from_env() -> Self::Output {
            Config {
                max_object_size: env!("CHARTED_MAX_OBJECT_CACHE_SIZE", to: ByteUnit, or_else: __one_megabyte()),
                ttl: Some(
                    env!("CHARTED_CACHE_INMEMORY_TTL", to: Duration, or_else: std::time::Duration::from_secs(15 * 60).into()),
                ),
            }
        }
    }

    const fn __one_megabyte() -> ByteUnit {
        ByteUnit::Megabyte(1)
    }
}

pub mod redis {
    use crate::common::serde::Duration;
    use noelware_config::{env, merge::Merge, FromEnv};
    use serde::{Deserialize, Serialize};
    use ubyte::ByteUnit;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Config {
        /// Max size of a object that can be stored in the cache. By default, it is 1 megabyte of the
        /// object that has been serialized from JSON.
        #[serde(default = "__one_megabyte")]
        pub max_object_size: ByteUnit,

        /// Time-to-live for all objects to be discarded in-memory. This will default to 15 minutes.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ttl: Option<Duration>,
    }

    impl Merge for Config {
        fn merge(&mut self, other: Self) {
            self.ttl.merge(other.ttl);

            // don't do anything if they are the same
            if self.max_object_size == other.max_object_size {
                return;
            }

            // do it if they're not the same
            if self.max_object_size != other.max_object_size {
                self.max_object_size = other.max_object_size;
            }
        }
    }

    impl Default for Config {
        fn default() -> Config {
            Config {
                max_object_size: __one_megabyte(),
                ttl: Some(std::time::Duration::from_secs(15 * 60).into()),
            }
        }
    }

    impl FromEnv for Config {
        type Output = Config;

        fn from_env() -> Self::Output {
            Config {
                max_object_size: env!("CHARTED_MAX_OBJECT_CACHE_SIZE", to: ByteUnit, or_else: __one_megabyte()),
                ttl: Some(
                    env!("CHARTED_CACHE_INMEMORY_TTL", to: Duration, or_else: std::time::Duration::from_secs(15 * 60).into()),
                ),
            }
        }
    }

    const fn __one_megabyte() -> ByteUnit {
        ByteUnit::Megabyte(1)
    }
}
