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

pub mod inmemory;
pub mod redis;

use crate::{
    config::caching::{Config, Strategy},
    redis::Client,
};
use eyre::Result;
use serde::{de::DeserializeOwned, Serialize};
use std::{borrow::Cow, fmt::Display, ops::Deref, time::Duration};

/// Default max object size (15mb)
pub const DEFAULT_MAX_OBJECT_SIZE: u64 = 15 * 1024 * 1024; // 15mb

/// Default [`Duration`] for cached objects.
pub const DEFAULT_TTL_LIFESPAN: Duration = Duration::from_secs(15 * 60); // 15 minutes

/// Cache key for caching organizations to underpressure the PostgreSQL database.
pub const ORGANIZATIONS: CacheKey = CacheKey(Cow::Borrowed("organizations"));

/// Cache key for caching repositories to underpressure the PostgreSQL database.
pub const REPOSITORIES: CacheKey = CacheKey(Cow::Borrowed("repositories"));

/// Cache key for caching ratelimits from HTTP requests
pub const RATELIMITS: CacheKey = CacheKey(Cow::Borrowed("ratelimits"));

/// Cache key for caching users to underpressure the PostgreSQL database.
pub const USERS: CacheKey = CacheKey(Cow::Borrowed("users"));

/// Represents a key for a cache object that is stored from Redis or in-memory.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CacheKey(Cow<'static, str>);
impl CacheKey {
    /// Creates a new [`CacheKey`] instance.
    pub fn new<I: Into<Cow<'static, str>>>(namespace: I) -> CacheKey {
        CacheKey(namespace.into())
    }

    /// Conjoins this existing cache key and returns a new one.
    pub fn join<I: Into<String>>(self, key: I) -> CacheKey {
        CacheKey(format!("{}:{}", self, key.into()).into())
    }

    /// Converts this [`CacheKey`] into a Redis-related key that can be queried easily.
    pub fn as_redis_key(self) -> String {
        format!("charted:cache:{}", self)
    }
}

impl Display for CacheKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl Deref for CacheKey {
    type Target = Cow<'static, str>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Represents a worker that handles caching routines (like database objects) that
/// can be configured to expire within a certain timeframe.
///
/// Objects that can be cached are anything that implement [`Serialize`](serde::ser::Serialize) for
/// putting objects in cache, and [`Deserialize`](serde::de::Deserialize) for retrieving cache
/// objects if they are found.
#[async_trait]
pub trait CacheWorker<Target: Serialize + DeserializeOwned>: Send + Sync {
    /// Gets a cached object from this worker or `None` if there is no
    /// cached object available.
    async fn get(&mut self, key: CacheKey) -> Result<Option<Target>>;

    /// Reserve a cache object within a given [`CacheKey`], returns a error
    /// if the cache key was already inserted into the cache.
    async fn put(&mut self, key: CacheKey, obj: Target) -> Result<()>
    where
        Target: 'async_trait;

    /// Attempts to delete a cache key from the cache if it exists.
    async fn delete(&mut self, key: CacheKey) -> Result<()>;

    /// Checks if the cache key exists in the cache or not.
    async fn exists(&mut self, key: &CacheKey) -> Result<bool>;
}

#[async_trait]
impl<Target: Serialize + DeserializeOwned + Send + Sync> CacheWorker<Target> for Box<dyn CacheWorker<Target>> {
    async fn get(&mut self, key: CacheKey) -> Result<Option<Target>> {
        (**self).get(key).await
    }

    async fn put(&mut self, key: CacheKey, obj: Target) -> Result<()> {
        (**self).put(key, obj).await
    }

    async fn delete(&mut self, key: CacheKey) -> Result<()> {
        (**self).delete(key).await
    }

    async fn exists(&mut self, key: &CacheKey) -> Result<bool> {
        (**self).exists(key).await
    }
}

#[cfg(test)]
fn __assert_object_safe() {
    struct SomeCacheWorker;
    #[async_trait]
    impl CacheWorker<()> for SomeCacheWorker {
        async fn get(&mut self, _key: CacheKey) -> Result<Option<()>> {
            todo!()
        }

        async fn put(&mut self, _key: CacheKey, _obj: ()) -> Result<()> {
            Ok(())
        }

        async fn delete(&mut self, _key: CacheKey) -> Result<()> {
            Ok(())
        }

        async fn exists(&mut self, _key: &CacheKey) -> Result<bool> {
            Ok(false)
        }
    }

    let _: &dyn CacheWorker<()> = &SomeCacheWorker;
}

pub fn choose_strategy<Target: Serialize + DeserializeOwned + Send + Sync + 'static>(
    config: &Config,
    redis: &Client,
) -> Box<dyn CacheWorker<Target>> {
    match config.strategy {
        Strategy::InMemory => Box::new(inmemory::InMemoryCache::new(config.clone())),
        Strategy::Redis => Box::new(redis::RedisCache::new(redis.clone(), config.clone())),
    }
}
