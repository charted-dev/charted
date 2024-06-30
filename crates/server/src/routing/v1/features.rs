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

use crate::ServerContext;
use axum::{extract::State, http::StatusCode};
use charted_core::response::ok;
use charted_proc_macros::{controller, generate_response_schema};
use serde::Serialize;
use utoipa::ToSchema;

/// The response datatype for the `GET /features` REST handler.
#[derive(Serialize, ToSchema)]
pub struct FeaturesResponse {
    /// whether if the OCI Registry feature is enabled or not.
    pub oci_registry: bool,

    /// whether if registrations are enabled on the server.
    ///
    /// > [!NOTE]
    /// > Hoshi uses this field to determine if signups are enabled.
    pub registrations: bool,

    /// whether if the Audit Logging feature is enabled.
    pub audit_logs: bool,

    /// Whether if the Webhooks feature is enabled or not.
    pub webhooks: bool,

    /// Whether if the server has search capabilities with the Elasticsearch or Meilisearch backend
    pub search: bool,

    /// whether if server garbage collection is enabled or not
    pub gc: bool,
}

generate_response_schema!(FeaturesResponse);

/// Returns the server's capabilities. For API consumers, this ensures X capability can be used.
#[controller(
    tags("Main"),
    response(200, "Successful response", ("application/json", response!("FeaturesResponse")))
)]
pub async fn features(State(ctx): State<ServerContext>) {
    ok(
        StatusCode::OK,
        FeaturesResponse {
            registrations: ctx.config.registrations,

            // AS OF v0.1.0-beta, these features are not implemented. they will
            // be features soon:tm:
            oci_registry: false,
            audit_logs: false,
            webhooks: false,
            search: false,
            gc: false,
        },
    )
}
