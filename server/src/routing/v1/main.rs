// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

use crate::{models::res::ok, openapi::gen_response_schema};
use axum::{http::StatusCode, response::IntoResponse};
use charted_common::VERSION;
use serde::{Deserialize, Serialize};
use utoipa::{
    openapi::{
        path::{OperationBuilder, PathItemBuilder},
        ContentBuilder, PathItem, PathItemType, Ref, RefOr, ResponseBuilder,
    },
    ToSchema,
};

/// Response object for `GET /`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MainResponse {
    /// The message, which will always be "Hello, world!"
    message: String,

    /// You know, for Helm charts?
    tagline: String,

    /// Documentation URL for this generic entrypoint
    /// response.
    docs: String,
}

gen_response_schema!(MainResponse);

impl Default for MainResponse {
    fn default() -> MainResponse {
        MainResponse {
            message: "Hello, world! 👋".into(),
            tagline: "You know, for Helm charts?".into(),
            docs: format!("https://charts.noelware.org/docs/server/{VERSION}"),
        }
    }
}

pub async fn main() -> impl IntoResponse {
    ok(StatusCode::OK, MainResponse::default())
}

pub fn paths() -> PathItem {
    PathItemBuilder::new()
        .operation(
            PathItemType::Get,
            OperationBuilder::new()
                .description(Some("Generic main entrypoint to charted-server's API server"))
                .response(
                    "200",
                    RefOr::T(
                        ResponseBuilder::new()
                            .description("Successful response.")
                            .content(
                                "application/json",
                                ContentBuilder::new()
                                    .schema(RefOr::Ref(Ref::from_schema_name("MainResponse")))
                                    .build(),
                            )
                            .build(),
                    ),
                )
                .build(),
        )
        .build()
}
