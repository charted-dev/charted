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
use charted_common::{models::Distribution, BUILD_DATE, COMMIT_HASH, VERSION};
use serde::{Deserialize, Serialize};
use utoipa::{
    openapi::{
        path::{OperationBuilder, PathItemBuilder},
        ContentBuilder, PathItem, PathItemType, Ref, RefOr, ResponseBuilder,
    },
    ToSchema,
};

/// Represents the response for the `GET /info` REST handler.
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
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

    /// Vendor of charted-server, will always be "Noelware"
    pub vendor: String,
}

gen_response_schema!(InfoResponse);

pub async fn info() -> impl IntoResponse {
    ok(
        StatusCode::OK,
        InfoResponse {
            distribution: Distribution::default(),
            commit_sha: COMMIT_HASH.to_string(),
            build_date: BUILD_DATE.to_string(),
            product: "charted-server".into(),
            version: VERSION.to_string(),
            vendor: "Noelware, LLC.".into(),
        },
    )
}

pub fn paths() -> PathItem {
    PathItemBuilder::new()
        .operation(
            PathItemType::Get,
            OperationBuilder::new()
                .description(Some(
                    "REST handler for getting more information about this instance that can be publically be visible.",
                ))
                .response(
                    "200",
                    RefOr::T(
                        ResponseBuilder::new()
                            .description("Successful response")
                            .content(
                                "application/json",
                                ContentBuilder::new()
                                    .schema(RefOr::Ref(Ref::from_schema_name("InfoResponse")))
                                    .build(),
                            )
                            .build(),
                    ),
                )
                .build(),
        )
        .build()
}
