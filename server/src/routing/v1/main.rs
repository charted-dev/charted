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

use crate::models::res::ok;
use axum::{http::StatusCode, response::IntoResponse};
use charted_common::VERSION;
use serde::{Deserialize, Serialize};
use utoipa::{
    openapi::{
        path::{OperationBuilder, PathItemBuilder},
        ContentBuilder, PathItemType, Paths, PathsBuilder, RefOr, ResponseBuilder,
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

pub fn paths() -> Paths {
    PathsBuilder::new()
        .path(
            "/",
            PathItemBuilder::new()
                .operation(
                    PathItemType::Get,
                    OperationBuilder::new()
                        .description(Some("Generic main entrypoint to charted-server's API server"))
                        .response(
                            "200",
                            RefOr::T(
                                ResponseBuilder::new()
                                    .content(
                                        "application/json",
                                        ContentBuilder::new().schema(MainResponse::schema().1).build(),
                                    )
                                    .build(),
                            ),
                        )
                        .build(),
                )
                .build(),
        )
        .build()
}
