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

use azalia::{hashset, TRUTHY_REGEX};
use noelware_config::{env, merge::Merge, TryFromEnv};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::HashSet};

/// Represents the configuration for configuring Redis for session management
/// and caching (if enabled from [`server.ratelimits.caching`] or [`database.caching`]).
///
/// ## Standalone
/// For standalone connections, [`redis.master_name`][Config::master_name] is not necessary, only for Sentinel connections.
///
/// ## Sentinel
/// Redis Sentinel is first-class supported in `charted-server` as Noelware's deployment
/// for Redis uses Redis Sentinel.
///
/// ## Cluster
/// Cluster support is very experimental and basic as of v0.1.0-beta.
#[derive(Debug, Clone, Serialize, Deserialize, Merge)]
pub struct Config {
    /// The name of the master node that all Sentinel nodes will interact with.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub master_name: Option<String>,

    /// Optional AUTH password to use when authenticating with Redis.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    /// Whether or not if the [`hosts`][Config::hosts] should connect as a clustered
    /// client rather than a regular client.
    ///
    /// NOTE: `master_name` cannot be set while `clustered` is true.
    #[serde(default)]
    #[merge(strategy = noelware_config::merge::strategy::bool::only_if_falsy)]
    pub clustered: bool,

    /// List of all available Redis hosts that the instance will try to connect to for session management,
    /// api key expirations, and caching entities (if configured properly).
    ///
    /// | Scenario                 | Condition                                      | What will happen?                                                                          |
    /// | :----------------------- | :--------------------------------------------- | :----------------------------------------------------------------------------------------- |
    /// | one host available       | `clustered` = false, `master_name` not set     | connect as a standalone client.                                                            |
    /// | one host available       | `clustered` = true, `master_name` not set      | connect as a cluster client as a single node.                                              |
    /// | one host available       | `clustered` = false/true; `master_name` is set | hard error as `master_name` cannot be used.                                                |
    /// | multiple hosts available | `clustered` = false, `master_name` not set     | hard error since either [`clustered`] or [`master_name`] was not set.                      |
    /// | multiple hosts available | `clustered` = true, `master_name` not set      | connect as a cluster client where the list of hosts are the configured nodes.              |
    /// | multiple hosts available | `clustered` = true, `master_name` is set       | warn as `clustered` mode cannot use a `master_name`, but will connect as a cluster client. |
    /// | mulitple hosts available | `clustered` = false, `master_name` is set      | connect as a sentinel client.                                                              |
    #[serde(default, skip_serializing_if = "HashSet::is_empty")]
    #[merge(strategy = __merge_redis_hosts)]
    pub hosts: HashSet<String>,

    /// whether or not if the Redis client supports connecting with TLS.
    #[serde(default)]
    pub tls: bool,

    /// database index to connect as.
    #[serde(default = "__default_db")]
    pub db: u8,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            master_name: None,
            clustered: false,
            password: None,
            hosts: __default_redis_hosts(),
            tls: false,
            db: __default_db(),
        }
    }
}

impl TryFromEnv for Config {
    type Output = Config;
    type Error = eyre::Report;

    fn try_from_env() -> Result<Self::Output, Self::Error> {
        Ok(Config {
            master_name: env!("CHARTED_REDIS_SENTINEL_MASTER_NAME", optional),
            clustered: env!("CHARTED_REDIS_CLUSTERED", |val| TRUTHY_REGEX.is_match(&val); or false),
            password: env!("CHARTED_REDIS_PASSWORD", optional),
            hosts: env!("CHARTED_REDIS_HOSTS", |val| val.split(',').map(String::from).collect(); or __default_redis_hosts()),
            tls: env!("CHARTED_REDIS_TLS", |val| TRUTHY_REGEX.is_match(&val); or false),
            db: charted_common::env("CHARTED_REDIS_PORT", __default_db(), |err| Cow::Owned(err.to_string()))?,
        })
    }
}

const fn __default_db() -> u8 {
    2
}

fn __default_redis_hosts() -> HashSet<String> {
    hashset!["redis://localhost:6379".to_string()]
}

fn __merge_redis_hosts(hosts: &mut HashSet<String>, right: HashSet<String>) {
    // if `right` is the default hosts, then we don't do anything
    if right == __default_redis_hosts() {
        return;
    }

    // overwrite it so it can be reflected correctly
    hosts.clear();
    hosts.extend(right);
}
