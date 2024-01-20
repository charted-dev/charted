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

pub mod charted;
pub mod registry;
pub mod repository;

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Represents the HCL configuration file (`.charted.hcl`)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Configuration for the `charted {}` block, which configures
    /// the Helm plugin itself.
    #[serde(default)]
    pub charted: charted::Config,

    /// List of registries that are available for repositories.
    #[serde(
        default = "__default_registries",
        rename = "registry",
        serialize_with = "hcl::ser::labeled_block"
    )]
    pub registries: BTreeMap<String, registry::Config>,

    /// List of repositories available.
    #[serde(
        default,
        skip_serializing_if = "BTreeMap::is_empty",
        rename = "repository",
        serialize_with = "hcl::ser::labeled_block"
    )]
    pub repositories: BTreeMap<String, repository::Config>,
}

/*
charted {
    version = "~> 0.1.0-beta"
}

registry "default" {
    version = 1
    url     = "https://charts.noelware.org/api"
}

repository "jetbrains-hub" {
    readme = "./charts/jetbrains/hub/README.md"
    source = "./charts/jetbrains/hub"

    config {
        repository = "noelware/jetbrains-hub"
        registry   = [registry.default]
    }
}

repository "jetbrains-youtrack" {
    readme = "./charts/jetbrains/youtrack/README.md"
    source = "./charts/jetbrains/youtrack"

    config {
        repository = "noelware/youtrack"
        registry   = [registry.default]
    }
}

repository "hazel" {
    readme = "./charts/noelware/hazel/README.md"
    source = "./charts/noelware/hazel"

    config {
        repository = "noelware/hazel"
        registry   = [registry.default]
    }
}

repository "petal" {
    readme = "./charts/noelware/petal/README.md"
    source = "./charts/noelware/petal"

    config {
        repository = "noelware/petal"
        registry   = [registry.default]
    }
}
*/
