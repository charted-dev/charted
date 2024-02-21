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

use crate::{hashset, TRUTHY_REGEX};
use noelware_config::{env, merge::Merge, FromEnv};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Represents the configuration for configuring Redis for session management
/// and caching (if enabled from [`server.ratelimits.caching`] or [`database.caching`]).
///
/// ## Standalone
/// For standalone connections, [`redis.master_name`] is not necessary, only for Sentinel connections.
///
/// ## Sentinel
/// Redis Sentinel is first-class supported in `charted-server` as Noelware's deployment
/// for Redis uses sentinels for failover.
///
/// ## Cluster
/// We don't support Redis Cluster yet, but if people want it, open an issue!
#[derive(Debug, Clone, Serialize, Deserialize, Merge)]
pub struct Config {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub master_name: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    #[serde(default, skip_serializing_if = "HashSet::is_empty")]
    #[merge(strategy = __merge_redis_hosts)]
    pub hosts: HashSet<String>,

    #[serde(default)]
    pub tls: bool,

    #[serde(default = "__default_db")]
    pub db: u8,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            master_name: None,
            password: None,
            hosts: __default_redis_hosts(),
            tls: false,
            db: __default_db(),
        }
    }
}

impl FromEnv for Config {
    type Output = Config;

    fn from_env() -> Self::Output {
        Config {
            master_name: env!("CHARTED_REDIS_SENTINEL_MASTER_NAME", is_optional: true),
            password: env!("CHARTED_REDIS_PASSWORD", is_optional: true),
            db: env!("CHARTED_REDIS_DB", to: u8, or_else: __default_db()),
            tls: env!("CHARTED_REDIS_TLS", {
                or_else: false;
                mapper: |val| TRUTHY_REGEX.is_match(&val);
            }),

            hosts: env!("CHARTED_REDIS_HOSTS", {
                or_else: __default_redis_hosts();
                mapper: |val| val.split(',').map(String::from).collect();
            }),
        }
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
