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

use crate::{
    extract::Json,
    macros::controller,
    middleware::{Session, SessionAuth},
    models::res::{no_content, ApiResponse},
    validation::validate,
    Server,
};
use axum::{
    extract::{Path, State},
    handler::Handler,
    routing, Extension, Router,
};
use charted_common::models::{entities::ApiKeyScope, payloads::PatchRepositoryPayload};
use tower_http::auth::AsyncRequireAuthorizationLayer;
use validator::Validate;

pub fn create_router() -> Router<Server> {
    Router::new().route(
        "/",
        routing::patch(
            PatchRepositoryRestController::run.layer(AsyncRequireAuthorizationLayer::new(
                SessionAuth::default().scope(ApiKeyScope::RepoUpdate),
            )),
        ),
    )
}

/// Patch a repository's metadata
#[controller(
    method = patch,
    tags("Repositories"),
    securityRequirements(("ApiKey", ["repo:update"]), ("Bearer", []), ("Basic", [])),
    response(204, "Successful response", ("application/json", response!("ApiEmptyResponse"))),
    response(400, "If the request body was invalid (i.e, validation errors)", ("application/json", response!("ApiErrorResponse"))),
    response(401, "If the session couldn't be validated", ("application/json", response!("ApiErrorResponse"))),
    response(403, "(Bearer token only) - if the JWT was invalid.", ("application/json", response!("ApiErrorResponse"))),
    response(406, "If the request body contained invalid data, or if the session header contained invalid data", ("application/json", response!("ApiErrorResponse"))),
    response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
)]
pub async fn patch_repository(
    State(Server { .. }): State<Server>,
    Extension(Session { .. }): Extension<Session>,
    Path(_): Path<u64>,
    payload: Json<PatchRepositoryPayload>,
) -> Result<ApiResponse, ApiResponse> {
    //let repos = controllers.get::<RepositoryDatabaseController>();
    validate(payload.clone(), PatchRepositoryPayload::validate)?;

    Ok(no_content())
}
