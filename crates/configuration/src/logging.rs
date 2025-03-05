// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2025 Noelware, LLC. <team@noelware.org>
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

use azalia::{
    config::{env, merge::Merge, FromEnv},
    TRUTHY_REGEX,
};
use serde::{Deserialize, Serialize};
use tracing::Level;

const LEVEL: &str = "CHARTED_LOG_LEVEL";
const JSON: &str = "CHARTED_LOG_JSON";

#[derive(Debug, Clone, Merge, Serialize, Deserialize)]
pub struct Config {
    /// Configures the log level of the API server's logging capabilities. The higher the
    /// level, the more verbose messages you'll get. For production environments, the
    /// default (`INFO`) is fine.
    #[serde(with = "azalia::serde::tracing")]
    #[merge(strategy = __merge_level)]
    pub level: Level,

    /// whether or not emit the log information as JSON blobs or not.
    #[serde(default)]
    #[merge(strategy = azalia::config::merge::strategy::bool::only_if_falsy)]
    pub json: bool,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            level: __default_level(),
            json: false,
        }
    }
}

impl FromEnv for Config {
    type Output = Config;

    fn from_env() -> Self::Output {
        Config {
            json: env!(JSON, |val| TRUTHY_REGEX.is_match(&val); or false),
            level: env!(LEVEL, |val| match &*val.to_ascii_lowercase() {
                "trace" => Level::TRACE,
                "debug" => Level::DEBUG,
                "error" => Level::ERROR,
                "warn"  => Level::WARN,
                "info"  => Level::INFO,
                _ => Level::INFO
            }; or __default_level()),
        }
    }
}

fn __merge_level(level: &mut Level, other: Level) {
    if *level != other {
        *level = other;
    }
}

const fn __default_level() -> Level {
    Level::INFO
}
