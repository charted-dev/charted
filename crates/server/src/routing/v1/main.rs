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
use charted_core::{api, VERSION};
use serde::Serialize;
use utoipa::{
    openapi::{ContentBuilder, Ref, RefOr, Response, ResponseBuilder},
    ToResponse, ToSchema,
};

/// Response object for the `GET /` REST controller.
#[derive(Serialize, ToSchema)]
pub struct MainResponse {
    /// The message, which will always be "Hello, world!"
    pub message: &'static str,

    /// You know, for Helm charts?
    pub tagline: &'static str,

    /// Documentation URL for this generic entrypoint response.
    pub docs: String,
}

impl Default for MainResponse {
    fn default() -> Self {
        MainResponse {
            message: "Hello, world! 👋",
            tagline: "You know, for Helm charts?",
            docs: format!("https://charts.noelware.org/docs/server/{VERSION}"),
        }
    }
}

impl<'r> ToResponse<'r> for MainResponse {
    fn response() -> (&'r str, RefOr<Response>) {
        (
            "MainResponse",
            RefOr::T(
                ResponseBuilder::new()
                    .description("Response for the `/` REST handler")
                    .content(
                        "application/json",
                        ContentBuilder::new()
                            .schema(Some(RefOr::Ref(Ref::from_schema_name("MainResponse"))))
                            .build(),
                    )
                    .build(),
            ),
        )
    }
}

/// Main entrypoint response to the API. Nothing too important.
#[utoipa::path(
    get,
    path = "/v1",
    operation_id = "main",
    tags = ["Main"],
    responses(
        (
            status = 200,
            description = "Successful response",
            body = inline(api::Response<MainResponse>),
            content_type = "application/json"
        )
    )
)]
#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn main() -> api::Response<MainResponse> {
    api::from_default(StatusCode::OK)
}
