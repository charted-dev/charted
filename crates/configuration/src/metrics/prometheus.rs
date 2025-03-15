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

use crate::util;
use azalia::config::{TryFromEnv, env, merge::Merge};
use charted_core::serde::Duration;
use eyre::bail;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, env::VarError};

pub const STANDALONE_HEADERS: &str = "CHARTED_METRICS_PROMETHEUS_STANDALONE_SERVER_HEADERS";
pub const STANDALONE_HOST: &str = "CHARTED_METRICS_PROMETHEUS_STANDALONE_SERVER_HOST";
pub const BUCKET_DURATION: &str = "CHARTED_METRICS_PROMETHEUS_BUCKET_DURATION";
pub const UPKEEP_INTERVAL: &str = "CHARTED_METRICS_PROMETHEUS_UPKEEP_INTERVAL";
pub const STANDALONE_PORT: &str = "CHARTED_METRICS_PROMETHEUS_STANDALONE_SERVER_PORT";
pub const STANDALONE: &str = "CHARTED_METRICS_PROMETHEUS_STANDALONE";
pub const ENDPOINT: &str = "CHARTED_METRICS_PROMETHEUS_ENDPOINT";

/// Enables metrics collection for Prometheus and exports a scraper endpoint
/// either on the API server itself or standalone.
#[derive(Debug, Clone, Merge, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// The scraper will be in its own standalone HTTP server which can be
    /// accessed internally.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub standalone: Option<Standalone>,

    #[serde(default = "__default_endpoint")]
    pub endpoint: String,

    /// Sets the bucket width when using summaries.
    ///
    /// Summaries are rolling, which means that they are divided into buckets of a fixed
    /// duration (width), and older buckets are dropped as they age out. This means data
    /// from a period as large as the width will be dropped at a time.
    ///
    /// The total amount of data kept for a summary is the number of buckets times the
    /// bucket width. For example, a bucket count of 3 and a bucket width of 20 seconds
    /// would mean that 60 seconds of data is kept at most, with the oldest 20 second
    /// chunk of data being dropped as the summary rolls forward.
    ///
    /// Use more buckets with a smaller width to roll off smaller amounts of data at a
    /// time, or fewer buckets with a larger width to roll it off in larger chunks.
    #[serde(default = "__default_bucket_duration")]
    #[merge(strategy = crate::util::merge_duration)]
    pub bucket_duration: Duration,

    #[serde(default = "__default_upkeep_interval")]
    #[merge(strategy = crate::util::merge_duration)]
    pub upkeep_interval: Duration,
}

impl TryFromEnv for Config {
    type Error = eyre::Report;
    type Output = Self;

    fn try_from_env() -> Result<Self::Output, Self::Error> {
        Ok(Config {
            standalone: match env!(STANDALONE) {
                Ok(value) if azalia::TRUTHY_REGEX.is_match(&value) => Some(Standalone::try_from_env()?),
                Ok(_) | Err(VarError::NotPresent) => None,
                Err(VarError::NotUnicode(_)) => bail!(
                    "environment variable `${}` contained invalid unicode characters",
                    STANDALONE
                ),
            },

            bucket_duration: util::env_from_str(BUCKET_DURATION, __default_bucket_duration())?,
            upkeep_interval: util::env_from_str(UPKEEP_INTERVAL, __default_upkeep_interval())?,
            endpoint: util::env_from_result(env!(ENDPOINT), __default_endpoint())?,
        })
    }
}

#[derive(Debug, Clone, Merge, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Standalone {
    /// A list of headers to append to all responses.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub headers: BTreeMap<String, String>,

    /// The host to bind towards.
    #[serde(default = "__default_host")]
    pub host: String,

    /// Port to listen on.
    #[serde(default = "__default_port")]
    pub port: u16,
}

impl TryFromEnv for Standalone {
    type Error = eyre::Report;
    type Output = Self;

    fn try_from_env() -> Result<Self::Output, Self::Error> {
        Ok(Standalone {
            headers: util::btreemap_env(STANDALONE_HEADERS)?,
            host: util::env_from_result(env!(STANDALONE_HOST), __default_host())?,
            port: util::env_from_str(STANDALONE_PORT, __default_port())?,
        })
    }
}

#[inline]
fn __default_host() -> String {
    String::from("127.0.0.1")
}

#[inline]
fn __default_endpoint() -> String {
    String::from("/_metrics")
}

const fn __default_port() -> u16 {
    50023
}

const fn __default_bucket_duration() -> Duration {
    Duration::from_secs(20)
}

const fn __default_upkeep_interval() -> Duration {
    Duration::from_secs(5)
}
