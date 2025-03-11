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

use charted_types::VersionReq;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Global configuration that applies for all operations.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Global {
    /// A SemVer version requirement constraint of what version of **charted-helm-plugin**
    /// to use for Helm operations.
    ///
    /// **charted-helm-plugin** uses the [`semver`] crate to parse version
    /// requirements, which abides how [Cargo parses and evaluates semantic versioning].
    ///
    /// [Cargo parses and evaluates semantic versioning]: https://doc.rust-lang.org/cargo/reference/semver.html
    /// [`semver`]: https://crates.io/crates/semver
    #[serde(
        default = "__default_plugin_constraint",
        skip_serializing_if = "VersionReq::is_wildcard"
    )]
    pub plugin_constraint: VersionReq,

    /// A SemVer version requirement constraint of what version of Helm is supported.
    ///
    /// **charted-helm-plugin** uses the [`semver`] crate to parse version
    /// requirements, which abides how [Cargo parses and evaluates semantic versioning].
    ///
    /// [Cargo parses and evaluates semantic versioning]: https://doc.rust-lang.org/cargo/reference/semver.html
    /// [`semver`]: https://crates.io/crates/semver
    #[serde(
        default = "__default_helm_constraint",
        skip_serializing_if = "VersionReq::is_wildcard"
    )]
    pub helm_constraint: VersionReq,
}

impl Default for Global {
    fn default() -> Self {
        Global {
            plugin_constraint: __default_plugin_constraint(),
            helm_constraint: __default_helm_constraint(),
        }
    }
}

fn __default_plugin_constraint() -> VersionReq {
    VersionReq::parse(&format!(">={}", charted_core::VERSION)).unwrap()
}

fn __default_helm_constraint() -> VersionReq {
    VersionReq::parse(">= 3.12").unwrap()
}
