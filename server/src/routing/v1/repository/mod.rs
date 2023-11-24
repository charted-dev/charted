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
    models::res::{err, no_content, ok, ErrorCode, Result, INTERNAL_SERVER_ERROR},
    validation::validate,
    Server,
};
use axum::{
    extract::{Path, State},
    handler::Handler,
    http::StatusCode,
    routing, Extension, Router,
};
use charted_cache_worker::{CacheKey, CacheWorker};
use charted_common::{
    models::{
        entities::{ApiKeyScope, Repository},
        payloads::PatchRepositoryPayload,
        Name,
    },
    VERSION,
};
use charted_database::controller::{repositories::RepositoryDatabaseController, DbController};
use serde_json::json;
use sqlx::{query_as, Postgres};
use tower_http::auth::AsyncRequireAuthorizationLayer;
use validator::Validate;

use super::EntrypointResponse;

pub(crate) struct RepositoryResponse;
charted_openapi::generate_response_schema!(RepositoryResponse, schema = "Repository");

pub fn create_router() -> Router<Server> {
    Router::new().route("/", routing::get(MainRestController::run)).route(
        "/:id",
        routing::patch(
            PatchRepositoryRestController::run.layer(AsyncRequireAuthorizationLayer::new(
                SessionAuth::default().scope(ApiKeyScope::RepoUpdate),
            )),
        )
        .get(GetRepositoryRestController::run),
    )
}

/// Generic entrypoint route for the Repositories API.
#[controller(
    id = "repositories",
    tags("Repositories"),
    response(200, "Successful response", ("application/json", response!("EntrypointResponse")))
)]
pub async fn main() {
    ok(
        StatusCode::OK,
        EntrypointResponse {
            message: "Welcome to the Repositories API".into(),
            docs: format!("https://charts.noelware.org/docs/server/{VERSION}/api/repositories"),
        },
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
    response(200, "Successful response", ("application/json", response!("RepositoryResponse")))
)]
pub async fn get_repository(
    State(Server { pool, db_cache, .. }): State<Server>,
    Path(id): Path<i64>,
) -> Result<Repository> {
    let mut worker = db_cache.lock().await;
    let key = CacheKey::repository(id);

    if let Some(cached) = worker.get::<Repository>(key.clone()).await.map_err(|e| {
        error!(error = %e, repo.id = id, "unable to get cached repository");
        sentry_eyre::capture_report(&e);

        err(StatusCode::INTERNAL_SERVER_ERROR, INTERNAL_SERVER_ERROR)
    })? {
        Ok(ok(StatusCode::OK, cached))
    } else {
        match query_as::<Postgres, Repository>("select repositories.* from repositories where id = $1;")
            .bind(id)
            .fetch_optional(&pool)
            .await
        {
            Ok(Some(entity)) => match worker.put(key, &entity).await {
                Ok(()) => Ok(ok(StatusCode::OK, entity)),
                Err(e) => {
                    error!(error = %e, repo.id = id, "unable to put repo in cache; trying again once hit again");
                    sentry_eyre::capture_report(&e);

                    Ok(ok(StatusCode::OK, entity))
                }
            },

            Ok(None) => Err(err(
                StatusCode::NOT_FOUND,
                (
                    ErrorCode::EntityNotFound,
                    format!("repository with id [{id}] was not found"),
                ),
            )),

            Err(e) => {
                error!(error = %e, repo.id = id, "unable to get repository from db");
                sentry::capture_error(&e);

                Err(err(StatusCode::INTERNAL_SERVER_ERROR, INTERNAL_SERVER_ERROR))
            }
        }
    }
}

/// Patch a repository's metadata
#[controller(
    method = patch,
    tags("Repositories"),
    securityRequirements(("ApiKey", ["repo:update"]), ("Bearer", []), ("Basic", [])),
    response(204, "Successful response", ("application/json", response!("EmptyApiResponse"))),
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
) -> Result {
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
                (ErrorCode::AccessNotPermitted, "you do not own this repository"),
            ))
        }

        Ok(None) => {
            return Err(err(
                StatusCode::NOT_FOUND,
                (
                    ErrorCode::EntityNotFound,
                    "repository with id was not found",
                    json!({
                        "id": id,
                    }),
                ),
            ))
        }

        Err(e) => {
            error!(%id, error = %e, "unable to find repository");
            sentry_eyre::capture_report(&e);

            return Err(err(StatusCode::INTERNAL_SERVER_ERROR, INTERNAL_SERVER_ERROR));
        }
    };

    repos.patch(repo.id.try_into().unwrap(), payload.0).await.map_err(|e| {
        error!(%id, error = %e, "unable to patch repository metadata");
        sentry_eyre::capture_report(&e);

        err(StatusCode::INTERNAL_SERVER_ERROR, INTERNAL_SERVER_ERROR)
    })?;

    Ok(no_content())
}
