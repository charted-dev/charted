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

use charted::server::version::APIVersion;
use serde::{Deserialize, Serialize};
use url::Url;

/// Represents the registry configuration, which registers a set list
/// of registries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// API version of the registry.
    #[serde(default)]
    pub version: APIVersion,

    /// URL of the registry to point to. This doesn't include the API version
    /// in the URI itself (i.e, `https://charts.noelware.org/api/v1`).
    pub url: Url,
}
