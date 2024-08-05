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

use azalia::config::merge::Merge;
use serde::{Deserialize, Serialize};
use ubyte::ByteUnit;

/// What caching backend that should be used.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Strategy {
    /// Uses the in-memory caching backend.
    #[default]
    InMemory,

    /// Disables caching all-together. Useful for evaluation purposes.
    Disable,

    /// For high-avaliablity, Redis is recommended for caching objects.
    Redis,
}

impl Merge for Strategy {
    fn merge(&mut self, other: Self) {
        match (*self, other) {
            // if they're the same, then just don't merge in general
            (Self::InMemory, Self::InMemory) => {}
            (Self::Disable, Self::Disable) => {}
            (Self::Redis, Self::Redis) => {}

            (_, other) => {
                *self = other;
            }
        }
    }
}

/// Allows to cache external objects for fast reads rather than hitting the database
/// multiple times for the same exact object that hasn't been modified.
///
/// At most, all objects live for ~15 minutes before being invalidated. This allows
/// the server to optimize for high throughput without database impact in case of
/// a denial-of-service attack.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    /// Strategy on how to cache objects. By default, the in-memory cache backend
    /// is enabled.
    #[serde(default)]
    pub strategy: Strategy,

    /// Size in bytes of how big objects should be. If this exceeds the amount, then
    /// the server will reject caching it.
    #[serde(default = "__default_max_object_size")]
    pub max_object_size: ByteUnit,
}

impl Merge for Config {
    fn merge(&mut self, other: Self) {
        self.strategy.merge(other.strategy);
        self.max_object_size = ByteUnit::from({
            let mut me = self.max_object_size.as_u64();
            me.merge(other.max_object_size.as_u64());

            me
        });
    }
}

const fn __default_max_object_size() -> ByteUnit {
    ByteUnit::Megabyte(1)
}
