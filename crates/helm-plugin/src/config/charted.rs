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

use charted_types::VersionReq;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

const DEFAULT_HELM_VERSION_CONSTRAINT: &str = ">=3.13";

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Charted {
    /// Version constraint to what `charted-helm-plugin` any repository supports.
    ///
    /// This is useful to determine if new features of `charted-helm-plugin` can be used or not.
    ///
    /// The plugin will error if the version constraint is not supported with what
    /// our version is.
    #[serde(default = "__default_charted_version_constraint")]
    pub version: VersionReq,

    /// Version constraint to what version of [Helm] is supported.
    ///
    /// This is useful to use features of Helm that the plugin might use can be enabled.
    ///
    /// The plugin will error if the version constraint is not supported with what
    /// our version is.
    #[serde(default = "__default_helm_version_constraint")]
    pub helm: VersionReq,
}

impl Default for Charted {
    fn default() -> Self {
        Charted {
            version: __default_charted_version_constraint(),
            helm: __default_helm_version_constraint(),
        }
    }
}

fn __default_charted_version_constraint() -> VersionReq {
    VersionReq::parse(&format!(">={}", charted_core::VERSION)).unwrap()
}

fn __default_helm_version_constraint() -> VersionReq {
    VersionReq::parse(DEFAULT_HELM_VERSION_CONSTRAINT).unwrap()
}
