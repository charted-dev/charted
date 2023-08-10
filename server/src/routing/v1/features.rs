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

use crate::{models::res::ok, openapi::gen_response_schema, Server};
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use charted_common::hashmap;
use charted_proc_macros::paths;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

/// Represents the response from the `GET /features` REST handler
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct FeaturesResponse {
    /// Whether if the external OCI registry experimental feature or the home-made implementation registry feature is enabled or not.
    pub docker_registry: bool,

    /// Whether if registrations are enabled on the server
    pub registrations: bool,

    /// Whether if the Audit Logging feature is enabled or not.
    pub audit_logs: bool,

    /// Whether if the Webhooks feature is enabled or not.
    pub webhooks: bool,

    /// Whether if this server instance is invite-only.
    #[deprecated(
        since = "0.1.0-beta",
        note = "`invite_only` is no longer a configuration key, this will always return 'false'"
    )]
    pub is_invite_only: bool,

    /// Object of all the session integrations available.
    pub integrations: HashMap<String, bool>,

    /// Whether if the server has search capabilities with the Elasticsearch or Meilisearch backend
    pub search: bool,
}

gen_response_schema!(FeaturesResponse);

#[allow(deprecated)]
pub async fn features(State(server): State<Server>) -> impl IntoResponse {
    ok(
        StatusCode::OK,
        FeaturesResponse {
            docker_registry: false,
            is_invite_only: false,
            registrations: server.config.registrations,
            audit_logs: false,
            webhooks: false,
            integrations: hashmap!(),
            search: server.config.search.is_some(),
        },
    )
}

pub fn paths() -> PathItem {
    PathItemBuilder::new()
        .operation(
            PathItemType::Get,
            OperationBuilder::new()
                .operation_id(Some("features"))
                .description(Some("REST handler to retrieve this server's features that were enabled or disabled by the server administrators"))
                .response("200", ResponseBuilder::new()
                    .description("Successful response.")
                    .content("application/json", ContentBuilder::new()
                        .schema(response!("ApiFeaturesResponse"))
                        .build()
                    ).build())
                .build(),
        )
        .build()
}

// paths! {
//     Get {
//         description("Hello, world?");
//         operationId("features");
//         tags(["Main"]);

//         requestBody("application/json", {
//             description("Payload for the `GET /features` REST endpoint.");
//             schema(crate::routing::v1::features::FeaturesResponse);
//         });
//     }
// }
