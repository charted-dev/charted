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

use super::{CacheKey, CacheWorker, DEFAULT_TTL_LIFESPAN};
use crate::config;
use eyre::Context;
use redis::{AsyncCommands, Commands};
use serde::{de::DeserializeOwned, Serialize};

/// Represents a [`CacheWorker`] that uses Redis as the backend.
pub struct RedisCache {
    config: config::caching::Config,
    client: crate::redis::Client,
}

impl RedisCache {
    /// Creates a new [`RedisCache`] instance.
    pub fn new(client: crate::redis::Client, config: config::caching::Config) -> RedisCache {
        trace!(cache.worker = "redis", "configured redis cache");
        RedisCache { config, client }
    }
}

#[async_trait]
impl<'a, Target: Serialize + DeserializeOwned + Send + Sync + 'a> CacheWorker<Target> for RedisCache {
    #[instrument(name = "charted.caching.redis.get", skip(self))]
    async fn get(&mut self, key: CacheKey) -> eyre::Result<Option<Target>> {
        let key = key.as_redis_key();
        let client = self.client.replica()?;
        let mut conn = client.get_async_connection().await?;

        match conn.get::<_, Option<String>>(key).await {
            Ok(Some(data)) => Ok(Some(serde_json::from_str::<Target>(&data)?)),
            Ok(None) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    async fn put(&mut self, key: CacheKey, obj: Target) -> eyre::Result<()>
    where
        Target: 'async_trait,
    {
        let key = key.as_redis_key();
        let client = self.client.master()?;
        let mut conn = client.get_connection()?;

        if conn.exists(&key)? {
            return Ok(());
        }

        redis::pipe()
            .set(&key, serde_json::to_string(&obj)?)
            .expire(
                &key,
                self.config
                    .ttl
                    .unwrap_or(DEFAULT_TTL_LIFESPAN.into())
                    .as_secs()
                    .try_into()?,
            )
            .query::<()>(&mut conn)
            .context(format!("unable to run SET/EXPIRE on key [{key}]"))
    }

    #[instrument(name = "charted.caching.redis.delete", skip(self))]
    async fn delete(&mut self, key: CacheKey) -> eyre::Result<()> {
        let key = key.as_redis_key();
        let client = self.client.master()?;
        let mut conn = client.get_async_connection().await?;

        if !conn.exists(&key).await? {
            return Ok(());
        }

        conn.del(&key).await.context("unable to delete Redis key [{key}]")
    }
}
