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

use charted_config::{Config, RedisConfig};
use eyre::{eyre, Context, Result};
use redis::{sentinel::Sentinel, Client};
use std::{
    fmt::Debug,
    iter::FromIterator,
    sync::{Arc, Mutex},
};
use tracing::{info, warn};

/// An abstraction over to provide both a standalone [Redis client][Client] and a [sentinel connection][Client]
/// with the provided [`RedisConfig`].
///
/// This abstraction was made so we can wrap over [Redis clients][Client] and provide a bridge API to overlap
/// over a standalone and sentinel connections.
///
/// ## Example
/// ```no_run
/// # use charted_config::RedisConfig;
/// # use charted_redis::RedisClient;
/// #
/// // uses the default connection info
/// let config = Config::default();
/// let mut client = RedisClient::new();
/// assert!(client.is_ok());
/// ```
#[derive(Clone)]
pub struct RedisClient {
    sentinel: Option<Arc<Mutex<Sentinel>>>,
    client: Option<Client>,
    config: RedisConfig,
}

impl Debug for RedisClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RedisClient").finish_non_exhaustive()
    }
}

impl RedisClient {
    /// Creates a new [`RedisClient`].
    pub fn new() -> Result<RedisClient> {
        let config = Config::get();
        let redis = config.redis;
        if redis.hosts.is_empty() {
            warn!("no redis hosts were configured, using default connection info");
            let client = Client::open("redis://localhost:6379").context(
                "used default redis connection info (redis://localhost:6379) due to no hosts being configured",
            )?;

            return Ok(RedisClient {
                sentinel: None,
                client: Some(client),
                config: redis.clone(),
            });
        }

        let hosts = Vec::from_iter(redis.hosts.clone());
        if hosts.len() == 1 {
            let first = hosts.first().unwrap();
            let client =
                Client::open(first.clone()).context(format!("used connection info [{first}] from configuration"))?;

            // check if it is a standalone connection or not
            // let mut conn = client
            //     .get_connection()
            //     .context(format!("unable to grab connection from redis client (info: {first})"))?;

            info!("assuming that first entry in 'config.redis.hosts' ({first}) is a standalone server");
            return Ok(RedisClient {
                sentinel: None,
                client: Some(client),
                config: redis.clone(),
            });
        }

        info!("received {} hosts, using sentinel as the connection type!", hosts.len());

        let master_name = redis.master_name().context("unable to parse secure setting")?;
        if master_name.is_none() {
            return Err(eyre!(
                "Missing `config.redis.master_name` configuration key. Required for Sentinel connections"
            ));
        }

        let sentinel = Sentinel::build(hosts)?;
        Ok(RedisClient {
            sentinel: Some(Arc::new(Mutex::new(sentinel))),
            client: None,
            config: redis,
        })
    }

    /// Returns a standalone [`Client`] that is meant to open a single connection
    /// if the connection is standalone.
    pub fn client(&self) -> Option<Client> {
        match self.client.clone() {
            Some(client) if self.sentinel.is_none() => Some(client),
            _ => None,
        }
    }

    /// Retrieves the master connection. If this is a standalone client, this will
    /// fast-path to using the main `client` instance. Otherwise, it will try to
    /// call the master of the sentinel.
    pub fn master(&mut self) -> Result<Client> {
        if let Some(client) = self.client.clone() {
            return Ok(client);
        }

        let master_name = self.config.master_name()?.unwrap();
        let mut sentinel = self
            .sentinel
            .as_mut()
            .unwrap()
            .lock()
            .expect("unable to acquire mutex lock");

        Ok(sentinel.master_for(&master_name, None)?)
    }

    /// Retrives a replica client from the sentinel list. If this is a standalone connection,
    /// this will just fast-path to the already constructed client as standalone connections
    /// operate alone.
    ///
    /// Otherwise, this will grab a replica from the `master_name` provided
    /// from the 'config.redis.master_name' configuration key.
    ///
    /// This will panic if grabbing the [`Sentinel`]'s mutex was poisoned.
    pub fn replica(&mut self) -> Result<Client> {
        if let Some(client) = self.client.clone() {
            return Ok(client);
        }

        let master_name = self.config.master_name()?.unwrap();
        let mut sentinel = self
            .sentinel
            .as_mut()
            .unwrap()
            .try_lock()
            .expect("unable to acquire mutex lock");

        Ok(sentinel.replica_for(&master_name, None)?)
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
