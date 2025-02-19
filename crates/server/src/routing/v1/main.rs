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
use charted_core::{api, Distribution};
use serde_json::{json, Value};
use utoipa::ToSchema;

#[derive(ToSchema)]
#[allow(unused)]
struct Response {
    #[schema(read_only)]
    distribution: Distribution,
    build_info: BuildInfo,
}

#[derive(ToSchema)]
#[allow(unused)]
struct BuildInfo {
    #[schema(read_only)]
    version: String,

    #[schema(read_only)]
    commit_hash: String,

    #[schema(read_only)]
    build_timestamp: String,

    #[schema(read_only)]
    rustc: String,
}

#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    get,

    path = "/",
    operation_id = "main",
    tag = "Main",
    responses(
        (
            status = OK,
            description = "200 OK",
            body = inline(Response)
        )
    )
)]
pub async fn main() -> api::Response<Value> {
    api::ok(
        StatusCode::OK,
        json!({
            "distribution": Distribution::detect(),
            "build_info": {
                "version": charted_core::VERSION,
                "commit_hash": charted_core::COMMIT_HASH,
                "build_timestamp": charted_core::BUILD_DATE,
                "rustc": charted_core::RUSTC_VERSION
            }
        }),
    )
}
