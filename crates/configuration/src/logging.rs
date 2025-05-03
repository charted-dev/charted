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

pub mod loki;

use crate::util;
use azalia::config::{
    env::{self, TryFromEnv, TryFromEnvValue},
    merge::Merge,
};
use serde::{Deserialize, Deserializer, Serialize};
use tracing::Level;
use tracing_subscriber::{EnvFilter, filter};

const FILTER: &str = "CHARTED_LOG_FILTER";
const LEVEL: &str = "CHARTED_LOG_LEVEL";
const JSON: &str = "CHARTED_LOG_JSON";

/// A parsed filter for logs to be filtered out.
#[derive(Debug, Clone, Merge, Serialize)]
pub struct Filter(#[merge(strategy = azalia::config::merge::strategy::string::overwrite_empty)] String);
impl Filter {
    /// Builds a new [`EnvFilter`] from this filter.
    pub fn to_env_filter(&self) -> Result<EnvFilter, filter::ParseError> {
        EnvFilter::try_new(&self.0)
    }

    /// Builds a new [`EnvFilter`] from this filter.
    pub fn into_env_filter(self) -> Result<EnvFilter, filter::ParseError> {
        EnvFilter::try_new(self.0)
    }

    /// Checks if this filter only contains a single level and nothing else.
    ///
    /// It'll check for the possible variants of [`Level`](tracing::Level) and `off`.
    pub fn is_only_level_filter(&self) -> bool {
        let me = self.0.to_string().to_ascii_lowercase();
        matches!(&*me, "info" | "debug" | "trace" | "warn" | "error" | "warning" | "off")
    }
}

impl<'de> Deserialize<'de> for Filter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        let filter = Filter(String::deserialize(deserializer)?);
        filter.to_env_filter().map(|_| filter).map_err(D::Error::custom)
    }
}

impl TryFromEnvValue for Filter {
    type Error = filter::ParseError;

    fn try_from_env_value(value: String) -> Result<Self, Self::Error> {
        let filter = Filter(value);
        filter.to_env_filter().map(|_| filter)
    }
}

#[derive(Debug, Clone, Merge, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// A filter in the style of [`tracing-subscriber`](tracing_subscriber)'s [`EnvFilter`](tracing_subscriber::filter::EnvFilter).
    ///
    /// While we allow directives for only setting the log level (i.e, `filter = "info"`), this will ignore the
    /// filter. This is only recommended to either make or suppress log levels from spans, events, or modules.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<Filter>,

    /// Configures the log level of the API server's logging capabilities. The higher the
    /// level, the more verbose messages you'll get. For production environments, the
    /// default (`INFO`) is fine.
    #[serde(with = "azalia::serde::tracing")]
    pub level: Level,

    /// whether or not emit the log information as JSON blobs or not.
    #[serde(default)]
    #[merge(strategy = azalia::config::merge::strategy::bool::only_if_falsy)]
    pub json: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loki: Option<loki::Config>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            filter: None,
            level: __default_level(),
            json: false,
            loki: None,
        }
    }
}

impl TryFromEnv for Config {
    type Error = eyre::Report;

    fn try_from_env() -> Result<Self, Self::Error> {
        Ok(Config {
            filter: env::try_parse_optional(FILTER)?,
            json: util::bool_env(JSON)?,
            level: env::try_parse_or(LEVEL, __default_level)?,
            loki: match util::bool_env(loki::ENABLE) {
                Ok(true) => Some(loki::Config::try_from_env()?),
                Ok(false) => None,
                Err(e) => return Err(e),
            },
        })
    }
}

const fn __default_level() -> Level {
    Level::INFO
}
