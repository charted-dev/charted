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

use charted_common::serde::Duration;
use noelware_config::merge::Merge;
use serde::{Deserialize, Serialize};
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

pub(crate) const fn __one_megabyte() -> ByteUnit {
    ByteUnit::Megabyte(1)
}
