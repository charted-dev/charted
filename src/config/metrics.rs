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

use crate::TRUTHY_REGEX;
use noelware_config::{env, merge::Merge, FromEnv};
use serde::{Deserialize, Serialize};

/// Represents the configuration for configuring the metrics pipeline.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Merge)]
pub struct Config {
    /// whether or not to enable the metrics pipeline
    #[serde(default)]
    #[merge(strategy = noelware_config::merge::strategy::bool::only_if_falsy)]
    pub enabled: bool,

    /// whether or not if the `/admin/stats` endpoint should be enabled. this can be used
    /// (if enabled) to collect all the metrics as JSON.
    #[serde(default)]
    #[merge(strategy = noelware_config::merge::strategy::bool::only_if_falsy)]
    pub admin: bool,

    /// allows the usage of the `/metrics` endpoint, which can be used to scrape into
    /// a format that [`Prometheus`](https://prometheus.io) knows and uses.
    #[serde(default)]
    #[merge(strategy = noelware_config::merge::strategy::bool::only_if_falsy)]
    pub prometheus: bool,
}

impl FromEnv for Config {
    type Output = Config;

    fn from_env() -> Self::Output {
        Config {
            prometheus: env!("CHARTED_METRICS_PROMETHEUS", |val| TRUTHY_REGEX.is_match(&val); or false),
            enabled: env!("CHARTED_METRICS_ENABLE", |val| TRUTHY_REGEX.is_match(&val); or false),
            admin: env!("CHARTED_METRICS_ENABLE_ADMIN_ENDPOINT", |val| TRUTHY_REGEX.is_match(&val); or false),
        }
    }
}
