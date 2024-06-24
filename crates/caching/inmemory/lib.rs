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
use charted_common::serde::ToSerdeDuration;
use charted_config::caching::Config;
use eyre::eyre;
use moka::{future::Cache, notification::RemovalCause};
use serde::{de::DeserializeOwned, Serialize};

/// Represents a cache worker that keeps object in-memory of each API server process.
#[derive(Clone)]
pub struct CacheWorker {
    max_object_size: usize,
    cache: Cache<CacheKey, String>,
}

impl CacheWorker {
    pub fn new(config: &Config) -> eyre::Result<CacheWorker> {
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
            max_object_size: max_object_size.try_into()?,
            cache: Cache::builder()
                .max_capacity(max_object_size)
                .time_to_live(*ttl)
                .weigher(|_key, value: &String| -> u32 { value.len().try_into().unwrap_or(u32::MAX) })
                .eviction_listener(|key, _value, cause| {
                    tracing::trace!(cache.key = %key, cause = cause_to_str(&cause), "cache key was evicted");
                })
                .build(),
        })
    }
}

impl<Target: Serialize + DeserializeOwned + Send> charted_cache::CacheWorker<Target> for CacheWorker {
    fn get(&mut self, key: CacheKey) -> charted_common::BoxedFuture<eyre::Result<Option<Target>>> {
        Box::pin(async move {
            match self.cache.get(&key).await.map(|x| serde_json::from_str::<Target>(&x)) {
                Some(Ok(target)) => Ok(Some(target)),
                Some(Err(e)) => Err(Into::into(e)),
                None => Ok(None),
            }
        })
    }

    fn put<'a>(&'a mut self, key: CacheKey, obj: Target) -> charted_common::BoxedFuture<'a, eyre::Result<()>>
    where
        Target: 'a,
    {
        Box::pin(async move {
            let serialized = serde_json::to_string(&obj)?;
            if serialized.len() > self.max_object_size {
                return Err(eyre!(
                    "object size in bytes was over {} bytes; must be less than {} bytes",
                    serialized.len() - self.max_object_size,
                    self.max_object_size
                ));
            }

            self.cache.insert(key, serialized).await;
            Ok(())
        })
    }

    fn delete(&mut self, key: CacheKey) -> charted_common::BoxedFuture<eyre::Result<()>> {
        Box::pin(async move {
            let _ = self.cache.remove(&key).await;
            Ok(())
        })
    }

    fn exists(&mut self, key: CacheKey) -> charted_common::BoxedFuture<eyre::Result<bool>> {
        Box::pin(async move { Ok(self.cache.contains_key(&key)) })
    }
}

fn cause_to_str(cause: &RemovalCause) -> &str {
    match cause {
        RemovalCause::Expired => "expired",
        RemovalCause::Explicit => "removed by server",
        RemovalCause::Replaced => "replaced",
        RemovalCause::Size => "size of object reached max capacity",
    }
}
