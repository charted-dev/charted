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
use charted_redis::RedisClient;
use eyre::{Context, Report, Result};
use redis::AsyncCommands;
use serde::{de::DeserializeOwned, ser::Serialize};
use tracing::instrument;

#[derive(Debug, Clone)]
pub struct RedisCacheWorker {
    client: RedisClient,
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
    async fn put<O: Serialize + Send + Sync>(&mut self, key: CacheKey, obj: O) -> Result<()> {
        let redis_key = key.as_redis_key();
        let client = self.client.master()?;
        let mut conn = client.get_async_connection().await?;

        if conn.exists(redis_key.clone()).await? {
            return Ok(());
        }

        conn.set::<_, _, ()>(redis_key.clone(), serde_json::to_string(&obj)?)
            .await
            .map(|_| ())
            .context(format!("unable to run 'SET {redis_key}'"))
    }
}
