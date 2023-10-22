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
    models::res::{err, no_content, ApiResponse},
    validation::validate,
    Server,
};
use axum::{
    extract::{Path, State},
    handler::Handler,
    http::StatusCode,
    routing, Extension, Router,
};
use charted_common::models::{entities::ApiKeyScope, payloads::PatchRepositoryPayload, Name};
use charted_database::controller::{repositories::RepositoryDatabaseController, DbController};
use serde_json::json;
use tower_http::auth::AsyncRequireAuthorizationLayer;
use validator::Validate;

pub(crate) struct RepositoryResponse;
charted_openapi::generate_response_schema!(RepositoryResponse, schema = "Repository");

pub fn create_router() -> Router<Server> {
    Router::new().route(
        "/:id",
        routing::patch(
            PatchRepositoryRestController::run.layer(AsyncRequireAuthorizationLayer::new(
                SessionAuth::default().scope(ApiKeyScope::RepoUpdate),
            )),
        )
        .get(GetRepositoryRestController::run),
    )
}

/// Retrieve a repository by the repo ID.
#[controller(
    tags("Repositories"),
    securityRequirements(
        ("ApiKey", ["repo:access"]),
        ("Bearer", []),
        ("Basic", [])
    ),
    response(200, "Successful response", ("application/json", response!("ApiRepositoryResponse")))
)]
pub async fn get_repository() {}

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
    State(Server { controllers, .. }): State<Server>,
    Extension(Session { user, .. }): Extension<Session>,
    Path(id): Path<i64>,
    payload: Json<PatchRepositoryPayload>,
) -> Result<ApiResponse, ApiResponse> {
    let repos = controllers.get::<RepositoryDatabaseController>();
    validate(payload.clone(), PatchRepositoryPayload::validate)?;

    if let Some(name) = payload.name.clone() {
        validate(name, Name::validate)?;
    }

    // get repository and check if the user owns it
    let repo = match repos.get(id.try_into().unwrap()).await {
        // if the user owns it, then they're allowed to edit it
        Ok(Some(repo)) if repo.owner == user.id => repo,
        Ok(Some(_)) => {
            return Err(err(
                StatusCode::NOT_ACCEPTABLE,
                ("UNABLE_TO_PATCH", "you do not own this repository").into(),
            ))
        }

        Ok(None) => {
            return Err(err(
                StatusCode::NOT_FOUND,
                (
                    "REPO_NOT_FOUND",
                    "repository with id was not found",
                    json!({
                        "id": id,
                    }),
                )
                    .into(),
            ))
        }

        Err(e) => {
            error!(%id, error = %e, "unable to find repository");
            sentry_eyre::capture_report(&e);

            return Err(err(
                StatusCode::INTERNAL_SERVER_ERROR,
                ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
            ));
        }
    };

    repos.patch(repo.id.try_into().unwrap(), payload.0).await.map_err(|e| {
        error!(%id, error = %e, "unable to patch repository metadata");
        sentry_eyre::capture_report(&e);

        err(
            StatusCode::INTERNAL_SERVER_ERROR,
            ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
        )
    })?;

    Ok(no_content())
}
