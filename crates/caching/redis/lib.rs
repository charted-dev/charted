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

use charted_cache::{CacheKey, MAXIMUM_TIME_TO_LIVE_SPAN, MAXIUMUM_MAX_OBJECT_SIZE};
use charted_common::serde::{Duration, ToSerdeDuration};
use charted_config::caching::Config;
use charted_core::redis::Client;
use eyre::{eyre, Context};
use redis::AsyncCommands;
use serde::{de::DeserializeOwned, Serialize};

/// Represents a [`CacheWorker`][charted_cache::CacheWorker] that uses a Redis server
/// to cache objects.
#[derive(Clone)]
pub struct CacheWorker {
    max_object_size: usize,
    ttl_lifespan: Duration,
    client: Client,
}

impl CacheWorker {
    /// Constructs a new [`CacheWorker`].
    pub fn new(client: Client, config: &Config) -> eyre::Result<CacheWorker> {
        let max_object_size = config.max_object_size.as_u64();
        if max_object_size > MAXIUMUM_MAX_OBJECT_SIZE as u64 {
            return Err(eyre!(
                "`config.caching.max_object_size` was over {} bytes; must be lower than {} bytes (15MB)",
                max_object_size - MAXIUMUM_MAX_OBJECT_SIZE as u64,
                MAXIUMUM_MAX_OBJECT_SIZE
            ));
        }

        let ttl = config
            .ttl
            .unwrap_or_else(|| MAXIMUM_TIME_TO_LIVE_SPAN.to_serde_duration());

        if ttl > MAXIMUM_TIME_TO_LIVE_SPAN.to_serde_duration() {
            return Err(eyre!(
                "`config.caching.ttl` was over {} seconds; must be lower than {} seconds (15 minutes)",
                ttl.as_secs() - MAXIMUM_TIME_TO_LIVE_SPAN.to_serde_duration().as_secs(),
                MAXIMUM_TIME_TO_LIVE_SPAN.as_secs()
            ));
        }

        Ok(CacheWorker {
            max_object_size: max_object_size
                .try_into()
                .context("was unable to fit `u64` -> `usize`")?,

            ttl_lifespan: ttl,
            client,
        })
    }
}

impl<Target: Serialize + DeserializeOwned + Send> charted_cache::CacheWorker<Target> for CacheWorker {
    fn get(&mut self, key: CacheKey) -> charted_common::BoxedFuture<eyre::Result<Option<Target>>> {
        let key = key.as_redis_key();
        Box::pin(async move {
            let key = key.clone();
            let mut conn = self.client.get_replica_connection().await?;

            match conn.get::<_, Option<String>>(&key).await {
                Ok(Some(data)) => {
                    tracing::info!(cache.key = key, "cache hit successful");
                    serde_json::from_str::<Target>(&data)
                        .map(Some)
                        .with_context(|| format!("unable to deserialize from JSON with cache key {key}"))
                }

                Ok(None) => Ok(None),
                Err(e) => Err(Into::into(e)),
            }
        })
    }

    fn put<'a>(&'a mut self, key: CacheKey, obj: Target) -> charted_common::BoxedFuture<'a, eyre::Result<()>>
    where
        Target: 'a,
    {
        let key = key.as_redis_key();
        Box::pin(async move {
            let key = key.clone();
            let mut conn = self.client.get_master_connection().await?;

            if conn.exists(&key).await? {
                return Ok(());
            }

            let serialized = serde_json::to_string(&obj)?;
            if serialized.len() > self.max_object_size {
                return Err(eyre!(
                    "serialized JSON object reached max object size ({} bytes)",
                    self.max_object_size
                ));
            }

            redis::pipe()
                .set(&key, serde_json::to_string(&obj)?)
                .expire(&key, self.ttl_lifespan.as_secs().try_into()?)
                .query_async(&mut conn)
                .await
                .map_err(Into::into)
        })
    }

    fn delete(&mut self, key: CacheKey) -> charted_common::BoxedFuture<eyre::Result<()>> {
        let key = key.as_redis_key();
        Box::pin(async move {
            let key = key.clone();
            let mut conn = self.client.get_master_connection().await?;

            if !conn.exists(&key).await? {
                return Ok(());
            }

            conn.del(&key)
                .await
                .with_context(|| format!("unable to delete object with key [{key}] from Redis"))
        })
    }

    fn exists(&mut self, key: CacheKey) -> charted_common::BoxedFuture<eyre::Result<bool>> {
        let key = key.as_redis_key();
        Box::pin(async move {
            let key = key.clone();
            let mut conn = self.client.get_replica_connection().await?;

            conn.exists(&key)
                .await
                .with_context(|| format!("unable to check existence of key [{key}] from Redis"))
        })
    }
}
