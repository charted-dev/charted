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

use axum::http::StatusCode;
use charted_core::{api, Distribution, BUILD_DATE, COMMIT_HASH, VERSION};
use serde::Serialize;
use utoipa::ToSchema;

/// Represents the response for the `GET /info` REST handler.
#[derive(Serialize, ToSchema)]
pub struct Info {
    /// The distribution the server is running off from
    pub distribution: Distribution,

    /// The commit hash from the Git repository.
    pub commit_sha: &'static str,

    /// Build date in RFC3339 format
    pub build_date: &'static str,

    /// Product name. Will always be "charted-server"
    pub product: &'static str,

    /// Valid SemVer 2 of the current version of this instance
    pub version: &'static str,

    /// Vendor of charted-server, will always be "Noelware, LLC."
    pub vendor: &'static str,
}

impl Default for Info {
    fn default() -> Self {
        Self {
            distribution: Distribution::detect(),
            commit_sha: COMMIT_HASH,
            build_date: BUILD_DATE,
            product: "charted-server",
            version: VERSION,
            vendor: "Noelware, LLC.",
        }
    }
}

/// Shows information about this running instance.
#[utoipa::path(
    get,
    path = "/v1/info",
    operation_id = "info",
    tags = ["Main"],
    responses(
        (
            status = 200,
            description = "Successful response",
            body = api::Response<Info>,
            content_type = "application/json"
        )
    )
)]
#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn info() -> api::Response<Info> {
    api::from_default(StatusCode::OK)
}
