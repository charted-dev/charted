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

use axum::http::StatusCode;
use charted_common::{BUILD_DATE, COMMIT_HASH, VERSION};
use charted_core::response::ok;
use charted_entities::Distribution;
use charted_proc_macros::{controller, generate_response_schema};
use serde::Serialize;
use utoipa::ToSchema;

/// Represents the response for the `GET /info` REST handler.
#[derive(Serialize, ToSchema)]
pub struct InfoResponse {
    /// The distribution the server is running off from
    pub distribution: Distribution,

    /// The commit hash from the Git repository.
    pub commit_sha: String,

    /// Build date in RFC3339 format
    pub build_date: String,

    /// Product name. Will always be "charted-server"
    pub product: String,

    /// Valid SemVer 2 of the current version of this instance
    pub version: String,

    /// Vendor of charted-server, will always be "Noelware, LLC."
    pub vendor: String,
}

generate_response_schema!(InfoResponse);

impl Default for InfoResponse {
    fn default() -> InfoResponse {
        InfoResponse {
            distribution: Distribution::detect(),
            commit_sha: COMMIT_HASH.to_string(),
            build_date: BUILD_DATE.to_string(),
            product: "charted-server".into(),
            version: VERSION.to_string(),
            vendor: "Noelware, LLC.".into(),
        }
    }
}

/// REST handler for getting more information about this instance that can be visible for API consumers.
#[controller(tags("Main"), response(200, "Successful response", ("application/json", response!("InfoResponse"))))]
pub async fn info() {
    ok(StatusCode::OK, InfoResponse::default())
}
