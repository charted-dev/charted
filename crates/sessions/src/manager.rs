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

use crate::{Session, SessionProvider, UserWithPassword};
use charted_common::hashmap;
use charted_redis::RedisClient;
use eyre::{Context, Result};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::task::JoinHandle;

/// An abstraction over handling sessions.
#[derive(Debug, Clone)]
pub struct SessionManager {
    initialized: bool,
    provider: Arc<Box<dyn SessionProvider>>,
    handles: HashMap<u64, Arc<Mutex<JoinHandle<Result<()>>>>>,
    redis: RedisClient,
}

impl SessionManager {
    /// Creates a new [`SessionManager`].
    pub fn new(redis: RedisClient, provider: Box<dyn SessionProvider>) -> SessionManager {
        SessionManager {
            initialized: false,
            provider: Arc::new(provider),
            handles: hashmap!(),
            redis,
        }
    }

    /// Initializes this [`SessionManager`].
    pub fn init(&mut self) -> Result<()> {
        if self.initialized {
            return Ok(());
        }

        let mut client = self
            .redis
            .client()
            .unwrap_or(self.redis.replica()?)
            .get_connection_with_timeout(Duration::from_millis(150))?;

        let mut master = match self.redis.client() {
            Some(_) => None,
            None => Some(
                self.redis
                    .master()?
                    .get_connection_with_timeout(Duration::from_millis(150))?,
            ),
        };

        let mapping: HashMap<u64, String> = RedisClient::cmd("HGETALL").arg("charted:sessions").query(&mut client)?;
        let mut handles = hashmap!();

        for (id, payload) in mapping.into_iter() {
            let Ok(_) = serde_json::from_str::<Session>(&payload) else {
                RedisClient::cmd("HDEL")
                    .arg("charted:sessions")
                    .arg(id)
                    .query(&mut master.as_mut().unwrap_or(&mut client))?;

                continue;
            };

            let ttl: i32 = RedisClient::cmd("TTL")
                .arg(format!("charted:sessions:{id}"))
                .query(&mut client)?;

            match ttl {
                -2 => { /* do nothing */ }
                -1 => {
                    RedisClient::cmd("HDEL")
                        .arg("charted:sessions")
                        .arg(id)
                        .query(&mut master.as_mut().unwrap_or(&mut client))?;

                    continue;
                }

                _ => {
                    let mut redis = self.redis.clone();
                    let handle: JoinHandle<Result<()>> = tokio::task::spawn(async move {
                        tokio::time::sleep(Duration::from_secs(ttl as u64)).await;

                        // We will need to create a connection to Redis once
                        // the task is completed since it'll be dropped after
                        // we are done initializing.
                        let mut client = redis
                            .client()
                            .unwrap_or(redis.master()?)
                            .get_connection_with_timeout(Duration::from_millis(150))?;

                        RedisClient::cmd("HDEL")
                            .arg("charted:sessions")
                            .arg(id)
                            .query(&mut client)
                            .context("unable to delete session {id} from Redis")
                    });

                    handles.insert(id, Arc::new(Mutex::new(handle)));
                }
            }
        }

        self.initialized = true;
        self.handles = handles;

        Ok(())
    }

    pub fn destroy(&mut self) -> Result<usize> {
        let mut failed = 0usize;
        for handle in self.handles.values() {
            match handle.try_lock() {
                Ok(handle) => {
                    handle.abort();

                    // drop it immedidately so we can continue
                    //
                    // TODO(@auguwu): wait until Mutex::unlock is stable enough,
                    // then switch to that
                    drop(handle);
                }

                Err(_) => {
                    failed += 1;
                }
            }
        }

        Ok(failed)
    }
}

#[async_trait::async_trait]
impl SessionProvider for SessionManager {
    async fn authorize(&self, password: String, user: &dyn UserWithPassword) -> Result<Session> {
        self.provider.authorize(password, user).await
    }
}
