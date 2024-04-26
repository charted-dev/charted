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

use noelware_config::merge::Merge;
use serde::{Deserialize, Serialize};

/// Represents the configuration for configuring charted-server's search backend
/// to Meilisearch to allow to search objects.
///
/// ## Example
/// ```toml,filename=config/charted.toml
/// [search.meilisearch]
/// host = "http://localhost:7700"
/// ```
///
/// or
///
/// ```text,filename=.env
/// CHARTED_SEARCH_BACKEND=meilisearch
/// CHARTED_SEARCH_MEILISEARCH_HOST=http://localhost:7700
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Merge)]
pub struct Config {
    /// Master key for authenticating with the Meilisearch server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub master_key: Option<String>,

    /// Single-node host to connect to a Meilisearch server. At the moment, the backend
    /// doesn't support multiple nodes for failover.
    #[serde(default = "__default_meilisearch_host")]
    pub host: String,
}

fn __default_meilisearch_host() -> String {
    String::from("http://localhost:7700")
}
