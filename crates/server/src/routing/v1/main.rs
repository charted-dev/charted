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

use crate::openapi::ApiResponse;
use axum::http::StatusCode;
use charted_core::{BuildInfo, Distribution, api};
use serde::Serialize;
use std::collections::BTreeMap;
use utoipa::{
    IntoResponses, ToResponse, ToSchema,
    openapi::{Ref, RefOr, Response},
};

#[derive(Serialize, ToSchema)]
pub struct Main {
    /// current distribution this instance is running as.
    #[schema(read_only)]
    distribution: Distribution,

    /// build information.
    build_info: BuildInfo,
}

impl Default for Main {
    fn default() -> Self {
        Main {
            distribution: Distribution::detect(),
            build_info: BuildInfo::new(),
        }
    }
}

impl IntoResponses for ApiResponse<Main> {
    fn responses() -> BTreeMap<String, RefOr<Response>> {
        azalia::btreemap! {
            "200" => RefOr::Ref(Ref::from_response_name(ApiResponse::<Main>::response().0))
        }
    }
}

/// Main entrypoint of the API server.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    get,

    path = "/v1",
    operation_id = "main",
    tag = "Main",
    responses(ApiResponse<Main>)
)]
pub async fn main() -> api::Response<Main> {
    api::ok(StatusCode::OK, Main::default())
}
