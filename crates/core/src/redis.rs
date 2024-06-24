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

use eyre::{eyre, Context};
use redis::sentinel::Sentinel;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, info, trace, warn};

/// Abstraction to allow to configure a standalone, sentinel, or clustered Redis client with
/// a provided Redis configuration. This was made to provide a bridge API without any hacks.
#[derive(Clone)]
pub struct Client {
    inner: Inner,
}

#[derive(Clone)]
enum Inner {
    Standalone(redis::Client),
    #[allow(unused)]
    Clustered(redis::cluster::ClusterClient),
    Sentinel {
        master: String,
        sentinel: Arc<Mutex<Sentinel>>,
    },
}

impl Client {
    /// Construct a new [`Client`] with a given [Redis configuration][charted_config::redis::Config].
    pub fn new(config: &charted_config::redis::Config) -> eyre::Result<Client> {
        info!("attempting to connect to a Redis server...");
        if config.hosts.is_empty() {
            warn!("received no redis clients via `config.redis.hosts`, using default connection info: [redis://localhost:6379]");
            let client = redis::Client::open("redis://localhost:6379")
                .context("failed to connect to Redis via default connection info [redis://localhost:6379]")?;

            return Ok(Client {
                inner: Inner::Standalone(client),
            });
        }

        // if there is only ONE host
        if config.hosts.len() == 1 {
            let host = config.hosts.iter().next().unwrap();

            if config.clustered {
                error!("`config.redis.clustered` is not supported as of this current version!");
                return Err(eyre!("`config.redis.clustered` is not supported yet"));
            }

            return Ok(Client {
                inner: match config.clustered {
                    false => {
                        warn!(%host, "received single host and clustered mode (`config.redis.clustered`) is not enabled. using standalone server instead");
                        Inner::Standalone(
                            redis::Client::open(host.as_ref())
                                .with_context(|| format!("failed to connect to Redis via single host [{host}]"))?,
                        )
                    }

                    true => unreachable!(),
                    // true => {
                    //     warn!(%host, "received single host and clusted mode (`config.redis.clustered`) is enabled. using clustered mode!");
                    //     Inner::Clustered(redis::cluster::ClusterClient::new([host.as_str()]).with_context(|| {
                    //         format!("tried to connect to a Redis cluster with single host [{host}]")
                    //     })?)
                    // }
                },
            });
        }

        if config.clustered {
            error!("`config.redis.clustered` is not supported as of this current version!");
            return Err(eyre!("`config.redis.clustered` is not supported yet"));
        }

        let Some(ref master) = config.master_name else {
            return Err(eyre!("missing `config.redis.master_name` for sentinel connection"));
        };

        trace!(%master, "creating sentinel client with master");
        let hosts = config.hosts.clone().into_iter().collect::<Vec<_>>();

        Ok(Client {
            inner: Inner::Sentinel {
                master: master.clone(),
                sentinel: Arc::new(Mutex::new(Sentinel::build(hosts.clone()).with_context(|| {
                    let hosts = hosts.join(", ");
                    format!(
                        "failed to connect to Redis sentinel server with master [{master}] with given hosts: [{hosts}]"
                    )
                })?)),
            },
        })
    }

    /// Attempt to a master connection, where it has access to read and write to the Redis connection.
    ///
    /// * Standalone: returns itself as it is *standalone*
    /// * Sentinel: returns the configured master by the master name.
    /// * Clustered: not supported as of late
    pub async fn get_master_connection(&mut self) -> eyre::Result<redis::aio::MultiplexedConnection> {
        match self.inner {
            Inner::Standalone(ref client) => client.get_multiplexed_async_connection().await.map_err(Into::into),
            Inner::Sentinel {
                ref master,
                ref sentinel,
            } => sentinel
                .lock()
                .await
                .master_for(master, None)?
                .get_multiplexed_async_connection()
                .await
                .map_err(Into::into),

            _ => unreachable!(),
        }
    }

    /// Attempt to a replica connection, where it is a readonly connection.
    ///
    /// * Standalone: returns itself as it is *standalone*
    /// * Sentinel: returns the configured replicas by the master name.
    /// * Clustered: ???
    pub async fn get_replica_connection(&mut self) -> eyre::Result<redis::aio::MultiplexedConnection> {
        match self.inner {
            Inner::Standalone(ref client) => client.get_multiplexed_async_connection().await.map_err(Into::into),
            Inner::Sentinel {
                ref master,
                ref sentinel,
            } => sentinel
                .lock()
                .await
                .replica_for(master, None)?
                .get_multiplexed_async_connection()
                .await
                .map_err(Into::into),

            _ => unreachable!(),
        }
    }
}

/*
impl Client {
    /// Retrieves the master connection. If this is a standalone client, this will
    /// fast-path to using the main `client` instance. Otherwise, it will try to
    /// call the master of the sentinel.
    pub fn master(&mut self) -> Result<RedisClient> {
        if let Some(client) = self.client() {
            return Ok(client);
        }

        let mut sentinel = self
            .sentinel
            .as_mut()
            .unwrap()
            .lock()
            .expect("unable to acquire mutex lock");

        Ok(sentinel.master_for(self.master_name.as_ref().unwrap(), None)?)
    }

    /// Retrives a replica client from the sentinel list. If this is a standalone connection,
    /// this will just fast-path to the already constructed client as standalone connections
    /// operate alone.
    ///
    /// Otherwise, this will grab a replica from the `master_name` provided
    /// from the 'config.redis.master_name' configuration key.
    ///
    /// This will panic if grabbing the [`Sentinel`]'s mutex was poisoned.
    pub fn replica(&mut self) -> Result<RedisClient> {
        if let Some(client) = self.client() {
            return Ok(client);
        }

        let mut sentinel = self
            .sentinel
            .as_mut()
            .unwrap()
            .try_lock()
            .expect("unable to acquire mutex lock");

        Ok(sentinel.replica_for(self.master_name.as_ref().unwrap(), None)?)
    }

    /// Utility to create a single [command][redis::Cmd] without requiring
    /// to bring in the `redis` dependency.
    pub fn cmd(arg: &str) -> redis::Cmd {
        redis::cmd(arg)
    }

    /// Utility to create a [`Pipeline`][redis::Pipeline].
    pub fn pipeline() -> redis::Pipeline {
        redis::pipe()
    }
}

*/
