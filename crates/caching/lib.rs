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

use charted_common::BoxedFuture;
use serde::{de::DeserializeOwned, Serialize};
use std::{borrow::Cow, fmt::Display, time::Duration};

/// Maximum size for serialized objects (15mb)
pub const MAXIUMUM_MAX_OBJECT_SIZE: usize = 15 * 1024 * 1024;

/// Maximum life span for objects to be cached (15 minutes)
pub const MAXIMUM_TIME_TO_LIVE_SPAN: Duration = Duration::from_secs(15 * 60);

/// [`CacheKey`] for caching organization objects.
pub const ORGANIZATION: CacheKey = CacheKey(Cow::Borrowed("organization"));

/// [`CacheKey`] for caching organization member objects.
pub const ORGANIZATION_MEMBER: CacheKey = CacheKey(Cow::Borrowed("organization:member"));

/// [`CacheKey`] for caching repository member objects.
pub const REPOSITORY_MEMBER: CacheKey = CacheKey(Cow::Borrowed("repository:members"));

/// [`CacheKey`] for caching repository objects.
pub const REPOSITORY: CacheKey = CacheKey(Cow::Borrowed("repositories"));

/// [`CacheKey`] for caching user objects.
pub const USER: CacheKey = CacheKey(Cow::Borrowed("users"));

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CacheKey(Cow<'static, str>);
impl CacheKey {
    /// Consumes `self` and returns a new [`CacheKey`] that joins as `self:key`.
    pub fn join<I: Into<String>>(self, key: I) -> CacheKey {
        CacheKey(Cow::Owned(format!("{}:{}", self, key.into())))
    }

    /// Converts this [`CacheKey`] to a Redis-related key with the `charted:cache` namespace.
    pub fn as_redis_key(&self) -> String {
        format!("charted:cache:{}", self)
    }
}

impl Display for CacheKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Worker trait that handles caching routines for objects.
///
/// Objects that can be cached are anything that implement [`Serialize`](serde::ser::Serialize) for
/// putting objects in cache, and [`Deserialize`](serde::de::Deserialize) for retrieving cache
/// objects if they are found.
pub trait CacheWorker<Target: Serialize + DeserializeOwned + Send>: Send + Sync {
    /// Get a cached object from this worker. Returns `Ok(None)` if the object with
    /// the cache key is not avaliable.
    fn get(&mut self, key: CacheKey) -> BoxedFuture<eyre::Result<Option<Target>>>;

    /// Reserve a cache object within a given [`CacheKey`], returns `Ok(())`
    /// if the cache key was already inserted into the cache OR if it was reserved
    /// successfully.
    fn put<'a>(&'a mut self, key: CacheKey, obj: Target) -> BoxedFuture<'a, eyre::Result<()>>
    where
        Target: 'a;

    /// Attempts to delete a cache key from the cache if it exists.
    fn delete(&mut self, key: CacheKey) -> BoxedFuture<eyre::Result<()>>;

    /// Checks if the cache key exists in the cache or not.
    fn exists(&mut self, key: CacheKey) -> BoxedFuture<eyre::Result<bool>>;
}

impl<Target: Serialize + DeserializeOwned + Send> CacheWorker<Target> for Box<dyn CacheWorker<Target>> {
    fn get(&mut self, key: CacheKey) -> BoxedFuture<eyre::Result<Option<Target>>> {
        (**self).get(key)
    }

    fn put<'a>(&'a mut self, key: CacheKey, obj: Target) -> BoxedFuture<'a, eyre::Result<()>>
    where
        Target: 'a,
    {
        (**self).put(key, obj)
    }

    fn delete(&mut self, key: CacheKey) -> BoxedFuture<eyre::Result<()>> {
        (**self).delete(key)
    }

    fn exists(&mut self, key: CacheKey) -> BoxedFuture<eyre::Result<bool>> {
        (**self).exists(key)
    }
}
