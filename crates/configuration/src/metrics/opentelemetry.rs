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

use super::DRIVER;
use azalia::config::{
    env::{self, TryFromEnv, TryParseError},
    merge::Merge,
};
use eyre::bail;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, env::VarError};
use url::Url;

pub const LABELS: &str = "CHARTED_METRICS_OTEL_LABELS";
pub const URL: &str = "CHARTED_METRICS_OTEL_ENDPOINT";

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

    /// URL to the supported OpenTelemetry collector.
    pub url: Url,
}

impl TryFromEnv for Config {
    type Error = eyre::Report;

    fn try_from_env() -> Result<Self, Self::Error> {
        Ok(Config {
            labels: env::try_parse_or_else(LABELS, Default::default()).unwrap_or_default(),
            url: match env::try_parse::<_, Url>(URL) {
                Ok(url) => url,

                Err(TryParseError::Parse(err)) => bail!("failed to parse url: {}", err),
                Err(TryParseError::System(VarError::NotPresent)) => bail!(
                    "environment variable `${}` is required when environment variable `${}` is set to \"opentelemetry\"",
                    URL,
                    DRIVER,
                ),

                Err(TryParseError::System(VarError::NotUnicode(_))) => {
                    bail!("environment variable `${}` contained invalid unicode characters", URL)
                }
            },
        })
    }
}
