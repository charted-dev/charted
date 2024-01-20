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

use semver::VersionReq;
use serde::{Deserialize, Serialize};

/// Configures the Helm plugin with the `charted {}` block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Version requirement for the charted Helm plugin.
    #[serde(default = "__default_charted_req")]
    pub version: VersionReq,

    /// Version requirement for Helm itself.
    #[serde(default = "__default_helm_req")]
    pub helm: VersionReq,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            version: __default_charted_req(),
            helm: __default_helm_req(),
        }
    }
}

#[inline]
fn __default_charted_req() -> VersionReq {
    VersionReq::parse(format!(">={}", crate::VERSION).as_str()).unwrap()
}

#[inline]
fn __default_helm_req() -> VersionReq {
    VersionReq::parse(">=3.7").unwrap()
}
