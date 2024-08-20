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

// TODO(@auguwu/@spotlightishere): determine how we should implement Redis Cluster for this

use eyre::Context;
use redis::{aio::MultiplexedConnection, sentinel::Sentinel};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
enum Inner {
    Standalone(redis::Client),
    Sentinel {
        master_name: String,
        sentinel: Arc<Mutex<Sentinel>>,
    },
}

/// Represents a generic Redis client that can map to a standalone, clustered
/// or a sentinel client.
#[derive(Debug, Clone)]
pub struct Client(Inner);

impl Client {
    pub fn new(_: &()) {}

    /// Retrieves the master connection. If this is a standalone client, this will
    /// fast-path to using the main `client` instance. Otherwise, it will try to
    /// connect to the master of the sentinel.
    pub fn master(&mut self) -> eyre::Result<redis::Client> {
        match self.0 {
            Inner::Standalone(client) => Ok(client),
            Inner::Sentinel { sentinel, master_name } => {
                let mut sentinel = sentinel
                    .try_lock()
                    .context("mutex for sentinel was poisioned somehow?")?;

                sentinel
                    .master_for(&master_name, None)
                    .context("failed to get master connection from sentinel")
            }
        }
    }

    /// Retrives a replica client from the sentinel list. If this is a standalone connection,
    /// this will just fast-path to the already constructed client as standalone connections
    /// operate alone.
    pub fn replica(&mut self) -> eyre::Result<redis::Client> {
        match self.0 {
            Inner::Standalone(client) => Ok(client),
            Inner::Sentinel { sentinel, master_name } => {
                let mut sentinel = sentinel
                    .try_lock()
                    .context("mutex for sentinel was poisioned somehow?")?;

                sentinel
                    .replica_for(&master_name, None)
                    .context("failed to get master connection from sentinel")
            }
        }
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
