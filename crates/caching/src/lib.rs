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

mod inmemory;
mod redis;

use std::fmt::Display;

use async_trait::async_trait;
use eyre::Result;
use serde::{de::DeserializeOwned, ser::Serialize};

/// [`CacheKey`] namespace for fetching repositories.
pub const REPOSITORIES: &str = "repositories";

/// [`CacheKey`] namespace for fetching users.
pub const USERS: &str = "users";

/// Represents a key that can be used to fetch from the cache.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CacheKey {
    namespace: &'static str,
    key: String,
}

impl CacheKey {
    /// Creates a new [`CacheKey`] instance.
    pub fn new<K: Into<String>>(namespace: &'static str, key: K) -> CacheKey {
        CacheKey {
            namespace,
            key: key.into(),
        }
    }

    /// Creates a [`CacheKey`] instance for a user with their ID
    pub fn user(id: i64) -> CacheKey {
        CacheKey::new(USERS, id.to_string())
    }

    /// Creates a [`CacheKey`] instance for a repository with their ID
    pub fn repository(id: i64) -> CacheKey {
        CacheKey::new(REPOSITORIES, id.to_string())
    }

    /// Takes ownership of this [`CacheKey`] and returns a Redis key representation
    /// of this [`CacheKey`].
    pub fn as_redis_key(&self) -> String {
        format!("charted:cache:{}:{}", self.namespace, self.key)
    }
}

impl Display for CacheKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.as_redis_key())
    }
}

/// Represents a worker that handles caching routines (like database objects) that
/// can be configured to expire within a certain timeframe.
///
/// Objects that can be cached are anything that implement [`Serialize`](serde::ser::Serialize) for
/// putting objects in cache, and [`Deserialize`](serde::de::Deserialize) for retrieving cache
/// objects if they are found.
#[async_trait]
pub trait CacheWorker {
    /// The name of the cache worker. This is used primarily in metrics.
    const NAME: &'static str;

    /// Gets a cached object from this worker or `None` if there is no
    /// cached object available.
    async fn get<O: DeserializeOwned + Send>(&mut self, key: CacheKey) -> Result<Option<O>>;

    /// Reserve a cache object within a given [`CacheKey`], returns a error
    /// if the cache key was already inserted into the cache.
    async fn put<O: Serialize + Send + Sync>(&mut self, key: CacheKey, obj: O) -> Result<()>;
}
