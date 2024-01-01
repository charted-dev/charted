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

use super::EntrypointResponse;
use crate::{
    macros::controller,
    models::res::{ok, ApiResponse},
    Server,
};
use axum::{http::StatusCode, routing, Router};
use charted_common::VERSION;
use charted_openapi::generate_response_schema;

pub fn create_router() -> Router<Server> {
    Router::new().route("/", routing::get(EntrypointRestController::run))
}

pub(crate) struct ApiKeyResponse;
generate_response_schema!(ApiKeyResponse, schema = "ApiKey");

#[controller(
    id = "apikeys",
    tags("ApiKeys"),
    response(200, "Successful response", ("application/json", response!("EntrypointResponse")))
)]
pub async fn entrypoint() -> ApiResponse<EntrypointResponse> {
    ok(
        StatusCode::OK,
        EntrypointResponse {
            message: "Welcome to the Api Keys API!".into(),
            docs: format!("https://charts.noelware.org/docs/server/{VERSION}/api/apikeys"),
        },
    )
}
