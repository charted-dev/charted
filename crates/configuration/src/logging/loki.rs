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
use eyre::Context;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use url::Url;

pub const HEADERS: &str = "CHARTED_LOG_LOKI_HEADERS";
pub const FIELDS: &str = "CHARTED_LOG_LOKI_FIELDS";
pub const LABELS: &str = "CHARTED_LOG_LOKI_LABELS";
pub const ENABLE: &str = "CHARTED_LOG_LOKI";
pub const URL: &str = "CHARTED_LOG_LOKI_URL";

/// **[logging.loki]**: Enables sending logs to [Grafana Loki].
///
/// [Grafana Loki]: https://grafana.com/loki
#[derive(Debug, Clone, Merge, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// A list of HTTP headers to send to Grafana Loki.
    ///
    /// ## Example
    /// ### Specifying the tenant ID
    /// ```toml
    /// [logging.loki]
    /// headers = { "X-Scope-OrgID" = "7662a206-fa0f-407f-abe9-261d652c750b" }
    /// ```
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub headers: BTreeMap<String, String>,

    /// A list of fields to prepend to all logs.
    ///
    /// **charted-server** will have the following fields:
    ///
    /// - `server.pid` to the server's process ID
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub fields: BTreeMap<String, String>,

    /// Additional labels to prepend to all logs.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub labels: BTreeMap<String, String>,

    /// URL to your Loki instance.
    pub url: Url,
}

impl TryFromEnv for Config {
    type Error = eyre::Report;
    type Output = Self;

    fn try_from_env() -> Result<Self::Output, Self::Error> {
        Ok(Self {
            url: env!(URL)
                .map(|s| Url::parse(&s))
                .with_context(|| format!("environment variable `${URL}` is required"))??,

            headers: util::btreemap_env(HEADERS)?,
            fields: util::btreemap_env(FIELDS)?,
            labels: util::btreemap_env(LABELS)?,
        })
    }
}
