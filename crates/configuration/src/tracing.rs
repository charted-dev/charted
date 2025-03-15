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
use eyre::bail;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, env::VarError};
use url::Url;

pub const MAX_ATTRIBUTES_PER_EVENT: &str = "CHARTED_TRACING_MAX_ATTRIBUTES_PER_EVENT";
pub const MAX_ATTRIBUTES_PER_SPAN: &str = "CHARTED_TRACING_MAX_ATTRIBUTES_PER_SPAN";
pub const MAX_ATTRIBUTES_PER_LINK: &str = "CHARTED_TRACING_MAX_ATTRIBUTES_PER_LINK";
pub const MAX_EVENTS_PER_SPAN: &str = "CHARTED_TRACING_MAX_EVENTS_PER_SPAN";
pub const MAX_LINKS_PER_SPAN: &str = "CHARTED_TRACING_MAX_LINKS_PER_SPAN";
pub const SAMPLER: &str = "CHARTED_TRACING_SAMPLER";
pub const ENABLED: &str = "CHARTED_TRACING_ENABLE";
pub const LABELS: &str = "CHARTED_TRACING_LABELS";
pub const URL: &str = "CHARTED_TRACING_ENDPOINT";

/// Configures how traces are sampled.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum Sampler {
    RatioOf(f64),
    Toggle(bool),
}

impl Default for Sampler {
    fn default() -> Self {
        Sampler::Toggle(true)
    }
}

impl Merge for Sampler {
    fn merge(&mut self, other: Self) {
        match (self, other) {
            (Self::RatioOf(r1), Self::RatioOf(r2)) => {
                azalia::config::merge::strategy::f64::non_negative(r1, r2);
            }

            (Self::Toggle(t1), Self::Toggle(t2)) => {
                azalia::config::merge::strategy::bool::only_if_falsy(t1, t2);
            }

            (me, other) => {
                *me = other;
            }
        }
    }
}

impl TryFromEnv for Sampler {
    type Error = eyre::Report;
    type Output = Self;

    fn try_from_env() -> Result<Self::Output, Self::Error> {
        match env!(SAMPLER) {
            Ok(value) => {
                if value.is_empty() {
                    return Ok(Sampler::Toggle(true));
                }

                if let Ok(ratio) = value.parse::<f64>() {
                    return Ok(Sampler::RatioOf(ratio));
                }

                Ok(Sampler::Toggle(azalia::TRUTHY_REGEX.is_match(&value)))
            }

            Err(VarError::NotPresent) => Ok(Sampler::Toggle(true)),
            Err(VarError::NotUnicode(_)) => {
                bail!(
                    "environment variable `${}` contained invalid unicode characters",
                    SAMPLER
                )
            }
        }
    }
}

/// ## `[tracing]` table
/// Allows the API server to report traces to any OpenTelemetry supported
/// service like [OpenTelemetry Collector], [Elastic APM], etc.
///
/// [OpenTelemetry Collector]: https://opentelemetry.io/docs/collector/
/// [Elastic APM]: https://www.elastic.co/observability/application-performance-monitoring
#[derive(Debug, Clone, Merge, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// A list of labels to use to detect this instance.
    ///
    /// By default, the API server will add the following labels:
    ///
    /// * `charted.version`
    /// * `service.name`
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub labels: BTreeMap<String, String>,

    /// Configures the maximum amount of events per span.
    #[serde(default = "__maximum")]
    pub max_events_per_span: u32,

    /// Configures the maximum amount of attributes per span.
    #[serde(default = "__maximum")]
    pub max_attributes_per_span: u32,

    /// Configures the maximum amount of links per span.
    #[serde(default = "__maximum")]
    pub max_links_per_span: u32,

    /// Configures the maximum amount of attributes per event.
    #[serde(default = "__maximum")]
    pub max_attributes_per_event: u32,

    /// Configures the maximum amount of attributes per link.
    #[serde(default = "__maximum")]
    pub max_attributes_per_link: u32,

    /// Configures how traces are sampled by the OpenTelemetry SDK.
    #[serde(default)]
    pub sampler: Sampler,

    /// URL to the supported OpenTelemetry collector.
    pub url: Url,
}

impl TryFromEnv for Config {
    type Error = eyre::Report;
    type Output = Self;

    fn try_from_env() -> Result<Self::Output, Self::Error> {
        Ok(Config {
            max_attributes_per_event: util::env_from_str(MAX_ATTRIBUTES_PER_EVENT, __maximum())?,
            max_attributes_per_link: util::env_from_str(MAX_ATTRIBUTES_PER_LINK, __maximum())?,
            max_attributes_per_span: util::env_from_str(MAX_ATTRIBUTES_PER_SPAN, __maximum())?,
            max_events_per_span: util::env_from_str(MAX_EVENTS_PER_SPAN, __maximum())?,
            max_links_per_span: util::env_from_str(MAX_LINKS_PER_SPAN, __maximum())?,
            sampler: Sampler::try_from_env()?,
            labels: util::btreemap_env(LABELS)?,
            url: match env!(URL) {
                Ok(url) => Url::parse(&url)?,
                Err(VarError::NotPresent) => bail!(
                    "environment variable `${}` is required when environment variable `${}` is set to true",
                    URL,
                    ENABLED
                ),

                Err(VarError::NotUnicode(_)) => {
                    bail!("environment variable `${}` contained invalid unicode characters", URL)
                }
            },
        })
    }
}

const fn __maximum() -> u32 {
    128
}
