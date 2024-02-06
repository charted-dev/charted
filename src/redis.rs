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

use eyre::{Context, Result};
use redis::{sentinel::Sentinel, Client as RedisClient};
use std::{
    iter::FromIterator,
    sync::{Arc, Mutex},
};

/// An abstraction over to provide both a standalone [Redis client][RedisClient] and a [sentinel connection][Sentinel]
/// with the provided [`RedisConfig`].
///
/// This abstraction was made so we can wrap over [Redis clients][RedisClient] and provide a bridge API to overlap
/// over a standalone and sentinel connections.
#[derive(Clone)]
pub struct Client {
    master_name: Option<String>,
    sentinel: Option<Arc<Mutex<Sentinel>>>,
    client: Option<RedisClient>,
}

impl Client {
    /// Creates a new [`Client`].
    pub fn new(config: &crate::config::redis::Config) -> Result<Client> {
        if config.hosts.is_empty() {
            warn!("received no redis hosts, using default connection info [redis://localhost:6379]");
            let client = RedisClient::open("redis://localhost:6379")
                .context("used default Redis connection info (redis://localhost:6379)")?;

            return Ok(Client {
                master_name: None,
                sentinel: None,
                client: Some(client),
            });
        }

        // we need to use the `.first()` function, which is why it was
        // implicilly converted into a Vec from a HashSet.
        let hosts = Vec::from_iter(config.hosts.clone());
        if config.hosts.len() == 1 {
            let url = hosts.first().unwrap();
            warn!(
                url,
                "assuming that the only entry in 'config.redis.hosts' is a Redis standalone server"
            );

            let client = RedisClient::open(url.as_str())?;
            return Ok(Client {
                master_name: None,
                sentinel: None,
                client: Some(client),
            });
        }

        info!(
            "received {} hosts, using Redis Sentinel as the connection type",
            hosts.len()
        );

        let Some(master_name) = config.master_name.clone() else {
            return Err(eyre!("missing required `config.redis.master_name` property"));
        };

        let sentinel = Sentinel::build(hosts)?;
        Ok(Client {
            master_name: Some(master_name),
            sentinel: Some(Arc::new(Mutex::new(sentinel))),
            client: None,
        })
    }

    /// Returns a standalone [`Client`] that is meant to open a single connection
    /// if the connection is standalone.
    pub fn client(&self) -> Option<RedisClient> {
        match self.client.clone() {
            Some(client) if self.sentinel.is_none() => Some(client),
            _ => None,
        }
    }

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
