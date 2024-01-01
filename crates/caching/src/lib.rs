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

mod inmemory;
mod redis;

pub use inmemory::*;
pub use redis::*;

use async_trait::async_trait;
use eyre::Result;
use serde::{de::DeserializeOwned, ser::Serialize};
use std::{fmt::Display, time::Duration};

/// [`CacheKey`] namespace for fetching repositories.
pub const REPOSITORIES: &str = "repositories";

/// [`CacheKey`] namespace for server ratelimits.
pub const RATELIMITS: &str = "ratelimits";

/// [`CacheKey`] namespace for fetching users.
pub const USERS: &str = "users";

/// Default max object size (15mb)
pub const DEFAULT_MAX_OBJECT_SIZE: u64 = 15 * 1024 * 1024; // 15mb

/// Default [`Duration`] for cached objects.
pub const DEFAULT_TTL_LIFESPAN: Duration = Duration::from_secs(15 * 60); // 15 minutes

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

    /// Creates a [`CacheKey`] instance for a ratelimit object
    pub fn ratelimit(id: i64) -> CacheKey {
        CacheKey::new(RATELIMITS, id.to_string())
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
pub trait CacheWorker: Send + Sync {
    /// The name of the cache worker. This is used primarily in metrics.
    const NAME: &'static str;

    /// Gets a cached object from this worker or `None` if there is no
    /// cached object available.
    async fn get<O: DeserializeOwned + Send>(&mut self, key: CacheKey) -> Result<Option<O>>;

    /// Reserve a cache object within a given [`CacheKey`], returns a error
    /// if the cache key was already inserted into the cache.
    async fn put<O: Serialize + Send + Sync>(&mut self, key: CacheKey, obj: O) -> Result<()>;

    /// Attempts to delete a cache key from the cache if it exists.
    async fn delete(&mut self, key: CacheKey) -> Result<()>;
}

/// Represents a dynamic cache worker, where it can be one or the other.
#[derive(Debug, Clone)]
pub enum DynamicCacheWorker {
    InMemory(inmemory::InMemoryCacheWorker),
    Redis(redis::RedisCacheWorker),
}

#[async_trait]
impl CacheWorker for DynamicCacheWorker {
    const NAME: &'static str = "dynamic";

    async fn get<O: DeserializeOwned + Send>(&mut self, key: CacheKey) -> Result<Option<O>> {
        match self {
            Self::InMemory(inmem) => inmem.get(key).await,
            Self::Redis(redis) => redis.get(key).await,
        }
    }

    async fn put<O: Serialize + Send + Sync>(&mut self, key: CacheKey, obj: O) -> Result<()> {
        match self {
            Self::InMemory(inmem) => inmem.put(key, obj).await,
            Self::Redis(redis) => redis.put(key, obj).await,
        }
    }

    async fn delete(&mut self, key: CacheKey) -> Result<()> {
        match self {
            Self::InMemory(inmem) => inmem.delete(key).await,
            Self::Redis(redis) => redis.delete(key).await,
        }
    }
}
