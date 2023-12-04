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

use crate::{CacheKey, CacheWorker, DEFAULT_TTL_LIFESPAN};
use async_trait::async_trait;
use charted_common::serde::duration::Duration;
use charted_config::caching::RedisCacheConfig;
use charted_redis::RedisClient;
use eyre::{Context, Report, Result};
use redis::{AsyncCommands, Commands};
use serde::{de::DeserializeOwned, ser::Serialize};
use tracing::{info, instrument, trace};

#[derive(Debug, Clone)]
pub struct RedisCacheWorker {
    client: RedisClient,
    ttl: Duration,
}

impl RedisCacheWorker {
    pub fn new(client: RedisClient, config: RedisCacheConfig) -> RedisCacheWorker {
        let ttl = config.time_to_live.unwrap_or(Duration(DEFAULT_TTL_LIFESPAN));
        info!(
            cache.worker = "inmemory",
            config.ttl = tracing::field::display(ttl),
            "using configured ttl"
        );

        RedisCacheWorker { client, ttl }
    }
}

#[async_trait]
impl CacheWorker for RedisCacheWorker {
    const NAME: &'static str = "redis";

    #[instrument(name = "charted.caching.redis.get", skip(self))]
    async fn get<O: DeserializeOwned>(&mut self, key: CacheKey) -> Result<Option<O>> {
        let redis_key = key.as_redis_key();
        let client = self.client.replica()?;
        let mut conn = client.get_async_connection().await?;

        match conn.get::<_, Option<String>>(redis_key).await {
            Ok(Some(json)) => Ok(Some(serde_json::from_str::<O>(&json)?)),
            Ok(None) => Ok(None),
            Err(e) => Err(Report::from(e)),
        }
    }

    #[instrument(name = "charted.caching.redis.put", skip(self, obj))]
    async fn put<O: Serialize + Send + Sync>(&mut self, key: CacheKey, obj: &O) -> Result<()> {
        let redis_key = key.as_redis_key();
        let client = self.client.master()?;
        let mut conn = client.get_connection()?;

        if conn.exists(&redis_key)? {
            return Ok(());
        }

        let mut pipeline = RedisClient::pipeline();
        pipeline
            .set(&redis_key, serde_json::to_string(obj)?)
            .expire(&redis_key, self.ttl.as_secs().try_into()?)
            .query::<()>(&mut conn)
            .context(format!("unable to run 'SET {redis_key}'"))
    }

    #[instrument(name = "charted.caching.redis.delete", skip(self))]
    async fn delete(&mut self, key: CacheKey) -> Result<()> {
        let client = self.client.master()?;
        let mut conn = client.get_async_connection().await?;
        let redis_key = key.as_redis_key();

        if !conn.exists(&redis_key).await? {
            return Ok(());
        }

        conn.del::<_, ()>(&redis_key)
            .await
            .map(|_| {
                trace!(cache.worker = "redis", key = %redis_key, "deleted cache key in Redis successfully");
            })
            .with_context(|| "unable to delete cache key from Redis")
    }
}
