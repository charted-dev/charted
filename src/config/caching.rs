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

use crate::common::serde::Duration;
use eyre::Report;
use noelware_config::{env, merge::Merge, TryFromEnv};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, env::VarError, str::FromStr};
use ubyte::ByteUnit;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Strategy {
    /// Uses the internal in-memory cache worker. All objects that are put into the cache
    /// are destroyed after the server closes. This is not recommended for production
    /// environments.
    #[default]
    InMemory,

    /// Uses the Redis client that was previously configured to cache objects in. The objects
    /// are cached as long they exist in the database, a job worker is created to clean up old
    /// objects every ~15 minutes to not bloat up Redis.
    Redis,
}

impl Merge for Strategy {
    fn merge(&mut self, other: Self) {
        match (self.clone(), other) {
            (Self::InMemory, Self::InMemory) => {}
            (Self::Redis, Self::Redis) => {}
            (Self::InMemory, Self::Redis) => {
                *self = Strategy::Redis;
            }

            (Self::Redis, Self::InMemory) => {
                *self = Strategy::InMemory;
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Max size of a object that can be stored in the cache. By default, it is 1 megabyte of the
    /// object that has been serialized from JSON.
    #[serde(default = "__one_megabyte")]
    pub max_object_size: ByteUnit,

    /// what caching strategy should be used
    #[serde(default)]
    pub strategy: Strategy,

    /// Time-to-live for all objects to be discarded in-memory. This will default to 15 minutes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl: Option<Duration>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            max_object_size: __one_megabyte(),
            strategy: Strategy::default(),
            ttl: None,
        }
    }
}

impl TryFromEnv for Config {
    type Output = Config;
    type Error = Report;

    fn try_from_env() -> Result<Self::Output, Self::Error> {
        Ok(Config {
            max_object_size: crate::common::env("CHARTED_CACHE_MAX_OBJECT_SIZE", __one_megabyte(), |err| {
                Cow::Owned(err.to_string())
            })?,

            ttl: env!("CHARTED_CACHE_TTL", |val| Duration::from_str(&val).ok(); or None),
            strategy: match env!("CHARTED_CACHE_STRATEGY") {
                Ok(res) => match res.as_str() {
                    "inmemory" | "in-memory" => Strategy::InMemory,
                    "redis" => Strategy::Redis,
                    res => {
                        return Err(eyre!(
                            "unknown value [{res}], wanted [inmemory/in-memory, redis] instead"
                        ))
                    }
                },
                Err(VarError::NotPresent) => Strategy::default(),
                Err(_) => return Err(eyre!("received invalid UTF-8 content")),
            },
        })
    }
}

impl Merge for Config {
    fn merge(&mut self, other: Self) {
        self.strategy.merge(other.strategy);
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

const fn __one_megabyte() -> ByteUnit {
    ByteUnit::Megabyte(1)
}

#[cfg(test)]
mod tests {
    use super::{Config, Strategy};
    use noelware_config::{expand_with, merge::Merge, TryFromEnv};
    use ubyte::ToByteUnit;

    #[test]
    fn test_env_config() {
        expand_with("CHARTED_CACHING_STRATEGY", "inmemory", || {
            let config = Config::try_from_env();
            assert!(config.is_ok());
        });
    }

    #[test]
    fn test_merge_config() {
        expand_with("CHARTED_CACHING_STRATEGY", "inmemory", || {
            let config = Config::try_from_env();
            assert!(config.is_ok());

            let mut config = config.unwrap();
            config.merge(Config {
                strategy: Strategy::Redis,
                ..Default::default()
            });

            assert_eq!(config.strategy, Strategy::Redis);

            config.merge(Config {
                max_object_size: 512.kibibytes(),
                ..Default::default()
            });

            assert_eq!(config.max_object_size, 512.kibibytes());
        });
    }
}
