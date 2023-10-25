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

use crate::{CacheKey, CacheWorker};
use async_trait::async_trait;
use eyre::{Context, Result};
use moka::future::Cache;
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;
use tracing::{instrument, trace};

#[derive(Clone)]
pub struct InMemoryCacheWorker {
    pool: Cache<CacheKey, String>,
}

impl Default for InMemoryCacheWorker {
    fn default() -> InMemoryCacheWorker {
        InMemoryCacheWorker {
            pool: Cache::builder()
                .weigher(|_, value: &String| -> u32 { value.len().try_into().unwrap_or(u32::MAX) })
                .max_capacity(15 * 1024 * 1024)
                .time_to_live(Duration::from_secs(15 * 60)) // ~15 minute ttl
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
            Some(obj) => serde_json::from_str(&obj)
                .map(|o| Some(o))
                .context("unable to deserialize to type `O`"),

            None => Ok(None),
        }
    }

    #[instrument(name = "charted.caching.inmemory.put", skip(self, obj))]
    async fn put<O: Serialize + Send + Sync>(&mut self, key: CacheKey, obj: O) -> Result<()> {
        if self.pool.contains_key(&key) {
            return Ok(());
        }

        let serialized = serde_json::to_string(&obj)?;
        self.pool.insert(key, serialized).await;

        Ok(())
    }
}
