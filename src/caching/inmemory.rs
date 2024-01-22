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

use super::{CacheKey, CacheWorker};
use crate::caching::DEFAULT_TTL_LIFESPAN;
use moka::future::Cache;
use serde::{de::DeserializeOwned, Serialize};
use std::ops::Deref;

/// Represents a [`CacheWorker`] that works on the `Target` itself.
#[derive(Clone)]
pub struct InMemoryCache(Cache<CacheKey, String>);

impl InMemoryCache {
    pub fn new(config: crate::config::caching::inmemory::Config) -> InMemoryCache {
        trace!(cache.worker = "inmemory", "configuring in-memory cache worker...");

        InMemoryCache(
            Cache::builder()
                .max_capacity(config.max_object_size.as_u64())
                .time_to_live(*config.ttl.unwrap_or(DEFAULT_TTL_LIFESPAN.into()))
                .eviction_listener(|key, _, cause| {
                    trace!(cache.key = ?key, ?cause, evicted = cause.was_evicted(), "cache key was evicted");
                })
                .build(),
        )
    }
}

impl Deref for InMemoryCache {
    type Target = Cache<CacheKey, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
impl<Target: Serialize + DeserializeOwned + Send + Sync + 'static> CacheWorker<Target> for InMemoryCache {
    #[instrument(name = "charted.caching.inmemory.get", skip(self))]
    async fn get(&mut self, key: CacheKey) -> eyre::Result<Option<Target>> {
        match self.0.get(&key).await {
            Some(res) => {
                trace!(cache.worker = "inmemory", cache.key = %key, "cache hit success");
                Ok(serde_json::from_str(&res).map(|obj| Some(obj))?)
            }

            None => Ok(None),
        }
    }

    #[instrument(name = "charted.caching.inmemory.put", skip(self, obj))]
    async fn put(&mut self, key: CacheKey, obj: Target) -> eyre::Result<()>
    where
        Target: 'async_trait,
    {
        if self.0.contains_key(&key) {
            return Ok(());
        }

        let serialized = serde_json::to_string(&obj)?;
        self.0.insert(key.clone(), serialized).await;
        trace!(cache.worker = "inmemory", cache.key = %key, "cache inserted successfully");

        Ok(())
    }

    async fn delete(&mut self, key: CacheKey) -> eyre::Result<()> {
        // continue if no cache key exists
        if !self.0.contains_key(&key) {
            return Ok(());
        }

        self.0.remove(&key).await;
        trace!(cache.worker = "inmemory", cache.key = %key, "cache deleted");

        Ok(())
    }
}
