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

use serde::{Deserialize, Serialize};
use std::{env::var, fmt::Display, fs, path::PathBuf};
use tracing::error;
use utoipa::{
    openapi::{ObjectBuilder, RefOr, Schema, SchemaType},
    ToSchema,
};

/// Represents the distribution that this instance is running off from.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum Distribution {
    /// Running on a Kubernetes cluster, it can also be running
    /// from the [official Helm chart](https://charts.noelware.org/~/charted/server).
    Kubernetes,

    /// Unknown distribution, be cautious!
    #[default]
    Unknown,

    /// Running on the official [Docker image](https://cr.noelware.cloud/~/charted/server).
    Docker,

    /// Running from the official RPM distribution
    /// from [Noelware's Artifacts Registry](https://artifacts.noelware.cloud)
    RPM,

    /// Running from the official Debian distribution
    /// from [Noelware's Artifacts Registry](https://artifacts.noelware.cloud)
    Deb,

    /// Running from the Git repository
    Git,
}

impl Distribution {
    fn is_kubernetes() -> bool {
        if var("KUBERNETES_SERVICE_HOST").is_ok() {
            return true;
        }

        let mut has_service_acc_token = false;
        let mut has_service_acc_ns = false;
        match PathBuf::from("/run/secrets/kubernetes.io/serviceaccount/token").try_exists() {
            Ok(true) => {
                has_service_acc_token = true;
            }

            Ok(false) => {}
            Err(e) => {
                error!("unable to detect if we are in a Kubernetes pod (tried to read /run/secrets/kubernetes.io/serviceaccount/token): {e}");
                return false;
            }
        }

        match PathBuf::from("/run/secrets/kubernetes.io/serviceaccount/namespace").try_exists() {
            Ok(true) => {
                has_service_acc_ns = true;
            }

            Ok(false) => {}
            Err(e) => {
                error!("unable to detect if we are in a Kubernetes pod (tried to read /run/secrets/kubernetes.io/serviceaccount/namespace): {e}");
                return false;
            }
        }

        let mut has_cluster_local_in_resolv = false;
        let resolv_conf = PathBuf::from("/etc/resolv.conf");
        match resolv_conf.try_exists() {
            Ok(true) => {
                let Ok(contents) = fs::read_to_string(resolv_conf) else {
                    error!("unable to detect if we are in a Kubernetes pod (tried to read /etc/resolv.conf)");
                    return has_service_acc_token && has_service_acc_ns;
                };

                if contents.contains("cluster.local") {
                    has_cluster_local_in_resolv = true;
                }
            }

            Ok(false) => {}
            Err(e) => {
                error!(error = %e, "unable to detect if we are in a Kubernetes pod (tried to read /run/secrets/kubernetes.io/serviceaccount/namespace):");
                return false;
            }
        }

        (has_service_acc_token && has_service_acc_ns) || has_cluster_local_in_resolv
    }

    fn is_docker() -> bool {
        let has_dockerenv = match PathBuf::from("/.dockerenv").try_exists() {
            Ok(res) => res,
            Err(e) => {
                error!(error = %e, "unable to detect if we are in a Docker container (tried to stat /.dockerenv):");
                false
            }
        };

        let has_docker_cgroup = {
            let cgroup = PathBuf::from("/proc/self/cgroup");
            let Ok(contents) = fs::read_to_string(cgroup) else {
                error!("unable to detect if we are in a Docker container (tried to read /proc/self/cgroup)");
                return false;
            };

            contents.contains("docker")
        };

        has_dockerenv || has_docker_cgroup
    }

    // TODO(@auguwu): should we cache this information?
    pub fn detect() -> Distribution {
        if Distribution::is_kubernetes() {
            return Distribution::Kubernetes;
        }

        if Distribution::is_docker() {
            return Distribution::Docker;
        }

        match var("CHARTED_DISTRIBUTION_KIND") {
            Ok(s) => match s.as_str() {
                // rpm and deb are automatically set in the systemd service
                // so we don't need to do any detection
                "rpm" => Distribution::RPM,
                "deb" => Distribution::Deb,

                // git is applied when built from source (i.e, ./dev server)
                "git" => Distribution::Git,

                // disallow any other value
                _ => Distribution::Unknown,
            },
            Err(_) => Distribution::Unknown,
        }
    }
}

impl<'s> ToSchema<'s> for Distribution {
    fn schema() -> (&'s str, RefOr<Schema>) {
        (
            "Distribution",
            RefOr::T(Schema::Object(
                ObjectBuilder::new()
                    .description(Some(
                        "Represents the distribution that this instance is running off from.",
                    ))
                    .schema_type(SchemaType::String)
                    .enum_values(Some(vec!["kubernetes", "docker", "rpm", "deb", "git", "unknown"]))
                    .default(Some("unknown".into()))
                    .build(),
            )),
        )
    }
}

impl Display for Distribution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Distribution::Kubernetes => f.write_str("kubernetes"),
            Distribution::Docker => f.write_str("docker"),
            Distribution::Git => f.write_str("git"),
            Distribution::Deb => f.write_str("debian"),
            Distribution::RPM => f.write_str("rpm"),
            _ => f.write_str("«unknown»"),
        }
    }
}
