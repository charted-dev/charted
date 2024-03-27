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

use charted_common::TRUTHY_REGEX;
use noelware_config::{env, merge::Merge, FromEnv};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Merge, Serialize, Deserialize)]
pub struct Config {
    /// whether or not to enable the CDN feature.
    #[serde(default)]
    #[merge(strategy = noelware_config::merge::strategy::bool::only_if_falsy)]
    pub enabled: bool,

    /// route prefix when the cdn is enabled. this is default to `/cdn`, but it can
    /// be anything like `/some/cdn/path/lol`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

impl FromEnv for Config {
    type Output = Config;

    fn from_env() -> Self::Output {
        Config {
            enabled: env!("CHARTED_CDN_ENABLED", |val| TRUTHY_REGEX.is_match(&val)).unwrap_or(false),
            prefix: env!("CHARTED_CDN_PREFIX", optional),
        }
    }
}
