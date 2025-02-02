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

use serde::Serialize;
use std::{env, fmt::Display, fs, path::PathBuf, sync::OnceLock};

const KUBERNETES_SERVICE_TOKEN_FILE: &str = "/run/secrets/kubernetes.io/serviceaccount/token";
const KUBERNETES_NAMESPACE_FILE: &str = "/run/secrets/kubernetes.io/serviceaccount/namespace";

/// Automatic detection to check if the distribution of charted-server is running on a Kubernetes
/// cluster as a pod or not. It'll check in the following paths and check if they exist:
///
/// * `/run/secrets/kubernetes.io/serviceaccount/token`
/// * `/run/secrets/kubernetes.io/serviceaccount/namespace`
fn is_in_k8s() -> bool {
    if env::var("KUBERNETES_SERVICE_HOST").is_ok() {
        return true;
    }

    PathBuf::from(KUBERNETES_SERVICE_TOKEN_FILE)
        .try_exists()
        .or_else(|_| PathBuf::from(KUBERNETES_NAMESPACE_FILE).try_exists())
        .unwrap_or_default()
}

/// Detects if charted-server is running as a Docker container, it'll check if `/.dockerenv` exists or if
/// `/proc/self/cgroup` contains `docker` in it.
fn is_in_docker_container() -> bool {
    let has_dockerenv = PathBuf::from("/.dockerenv").try_exists().unwrap_or_default();
    let has_cgroup = {
        let cgroup = PathBuf::from("/proc/self/cgroup");
        let Ok(contents) = fs::read_to_string(cgroup) else {
            return false;
        };

        contents.contains("docker")
    };

    has_dockerenv || has_cgroup
}

#[derive(Debug, Clone, Copy, Serialize, Default, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "lowercase")]
pub enum Distribution {
    /// Running on a Kubernetes cluster.
    Kubernetes,

    /// This build of charted-server was built from source.
    #[serde(rename = "from_source")]
    #[default]
    FromSource,

    /// Running as a Docker container.
    Docker,

    /// Running from a Nix flake.
    Nix,

    /// Uses a locally built binary from the host.
    Git,
}

impl Distribution {
    pub fn detect() -> Distribution {
        static ONCE: OnceLock<Distribution> = OnceLock::new();
        *ONCE.get_or_init(|| {
            if is_in_k8s() {
                return Distribution::Kubernetes;
            }

            if is_in_docker_container() {
                return Distribution::Docker;
            }

            match option_env!("CHARTED_DISTRIBUTION_KIND") {
                Some(s) => match s {
                    "docker" => Distribution::Docker,
                    "git" => Distribution::Git,
                    "nix" => Distribution::Nix,
                    _ => Distribution::FromSource,
                },

                None => Distribution::FromSource,
            }
        })
    }
}

impl Display for Distribution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Distribution::Kubernetes => f.write_str("kubernetes"),
            Distribution::Docker => f.write_str("docker"),
            Distribution::Nix => f.write_str("nix package manager"),
            Distribution::Git => f.write_str("git"),
            _ => f.write_str("«from source»"),
        }
    }
}
