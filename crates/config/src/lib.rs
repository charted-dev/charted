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

use azalia::config::merge::Merge;
use rand::distributions::{Alphanumeric, DistString};
use sentry_types::Dsn;
use serde::{Deserialize, Serialize};

pub(crate) mod helpers;

pub mod database;
pub mod logging;
pub mod metrics;
pub mod redis;
pub mod server;
pub mod sessions;
pub mod storage;

#[derive(Debug, Clone, Serialize, Deserialize, Merge)]
pub struct Config {
    /// whether or not if users can be registered on this instance
    #[serde(default = "__truthy")]
    #[merge(strategy = azalia::config::merge::strategy::bool::only_if_falsy)]
    pub registrations: bool,

    /// Secret key for encoding JWT tokens. This must be set once and never touched again.
    #[serde(default)]
    #[merge(skip)] // don't even attempt to merge jwt secret keys
    pub jwt_secret_key: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentry_dsn: Option<Dsn>,

    /// whether or not if the API server should act like a single organization, where most features
    /// are disabled like repository/organization members and audit logging.
    #[serde(default)]
    #[merge(strategy = azalia::config::merge::strategy::bool::only_if_falsy)]
    pub single_org: bool,

    /// whether or not if the API server should act like a single user, where *most* features
    /// are disabled and only one user is allowed to roam.
    ///
    /// all publically available features like Audit Logging can be enabled but repository and
    /// organization members are disabled. most endpoints will be also disabled.
    #[serde(default)]
    #[merge(strategy = azalia::config::merge::strategy::bool::only_if_falsy)]
    pub single_user: bool,
}

fn __generated_secret_key() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 16)
}

const fn __truthy() -> bool {
    true
}
