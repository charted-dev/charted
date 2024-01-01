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

use crate::{CacheKey, CacheWorker, DEFAULT_MAX_OBJECT_SIZE, DEFAULT_TTL_LIFESPAN};
use async_trait::async_trait;
use charted_config::caching::InMemoryCacheConfig;
use eyre::{Context, Result};
use moka::future::Cache;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;
use tracing::{info, instrument, trace};

#[derive(Clone)]
pub struct InMemoryCacheWorker {
    pool: Cache<CacheKey, String>,
}

impl Debug for InMemoryCacheWorker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InMemoryCacheWorker")
            .field("pool", &format!("{} entries", self.pool.entry_count()))
            .finish()
    }
}

impl InMemoryCacheWorker {
    /// Creates a new [`InMemoryCacheWorker`] instance.
    pub fn new(config: InMemoryCacheConfig) -> InMemoryCacheWorker {
        let ttl = config
            .time_to_live
            .unwrap_or(charted_common::serde::duration::Duration(DEFAULT_TTL_LIFESPAN));

        info!(
            cache.worker = "inmemory",
            config.ttl = tracing::field::display(ttl),
            "using configured ttl"
        );

        let max_object_size = DEFAULT_MAX_OBJECT_SIZE; // we haven't implemented it yet
        info!(
            cache.worker = "inmemory",
            config.max_object_size = max_object_size,
            "configured max object size"
        );

        InMemoryCacheWorker {
            pool: Cache::builder()
                .weigher(|_, value: &String| -> u32 { value.len().try_into().unwrap_or(u32::MAX) })
                .max_capacity(max_object_size)
                .time_to_live(*ttl) // ~15 minute ttl
                .eviction_listener(|key, _, cause| {
                    trace!(cache.key = %key, ?cause, was_evicted = cause.was_evicted(), "cached key was evicted");
                })
                .build(),
        }
    }
}

#[async_trait]
impl CacheWorker for InMemoryCacheWorker {
    const NAME: &'static str = "inmemory";

    #[instrument(name = "charted.caching.inmemory.get", skip(self))]
    async fn get<O: DeserializeOwned>(&mut self, key: CacheKey) -> Result<Option<O>> {
        match self.pool.get(&key).await {
            Some(obj) => {
                info!(cache.worker = "inmemory", cache.key = %key, "cache hit success");

                serde_json::from_str(&obj)
                    .map(|o| Some(o))
                    .context("unable to deserialize to type `O`")
            }

            None => Ok(None),
        }
    }

    #[instrument(name = "charted.caching.inmemory.put", skip(self, obj))]
    async fn put<O: Serialize + Send + Sync>(&mut self, key: CacheKey, obj: O) -> Result<()> {
        if self.pool.contains_key(&key) {
            return Ok(());
        }

        let serialized = serde_json::to_string(&obj)?;
        self.pool.insert(key.clone(), serialized).await;
        trace!(cache.worker = "inmemory", cache.key = %key, "cache inserted");

        Ok(())
    }

    #[instrument(name = "charted.caching.inmemory.delete", skip(self))]
    async fn delete(&mut self, key: CacheKey) -> Result<()> {
        // continue if no cache key exists
        if !self.pool.contains_key(&key) {
            return Ok(());
        }

        self.pool.remove(&key).await;
        trace!(cache.worker = "inmemory", cache.key = %key, "cache deleted");

        Ok(())
    }
}
