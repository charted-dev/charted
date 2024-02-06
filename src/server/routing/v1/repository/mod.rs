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
    caching::REPOSITORIES,
    common::models::{
        entities::{ApiKeyScope, ApiKeyScopes, Organization, Repository, User},
        NameOrSnowflake,
    },
    db::controllers::DbController,
    openapi::generate_response_schema,
    server::{
        controller,
        middleware::session::{Middleware, Session},
        models::res::{err, internal_server_error, ok, ErrorCode, Result},
    },
    Instance,
};
use axum::{
    extract::{FromRequestParts, Path, State},
    handler::Handler,
    http::StatusCode,
    routing, Extension, Router,
};
use serde_json::json;
use tower_http::auth::AsyncRequireAuthorizationLayer;

pub struct RepositoryResponse;
generate_response_schema!(RepositoryResponse, schema = "Repository");

pub fn create_router() -> Router<Instance> {
    Router::new()
        .route("/", routing::get(EntrypointRestController::run))
        .route(
            "/:id",
            routing::get(
                GetRepoByIdRestController::run.layer(AsyncRequireAuthorizationLayer::new(Middleware {
                    allow_unauthenticated_requests: true,
                    require_refresh_token: false,
                    scopes: ApiKeyScopes::with_iter([ApiKeyScope::RepoAccess]),
                })),
            ),
        )
        .route(
            "/:owner/:name",
            routing::get(
                GetRepoByOwnerAndNameRestController::run.layer(AsyncRequireAuthorizationLayer::new(Middleware {
                    allow_unauthenticated_requests: true,
                    require_refresh_token: false,
                    scopes: ApiKeyScopes::with_iter([ApiKeyScope::RepoAccess]),
                })),
            ),
        )
}

/// Entrypoint for the Repositories API
#[controller(id = "repositories", tags("Repositories"), response(200, "Successful response", ("application/json", response!("EntrypointResponse"))))]
pub async fn entrypoint() {
    ok(StatusCode::OK, EntrypointResponse::new("Repositories"))
}

/// Finds a repository by its ID.
#[controller(
    tags("Repositories"),
    securityRequirements(
        ("ApiKey", ["repo:access"]),
        ("Bearer", []),
        ("Basic", [])
    ),
    pathParameter("id", snowflake),
    response(200, "Successful response", ("application/json", response!("RepositoryResponse"))),
    response(403, "You are not allowed to see this resource", ("application/json", response!("ApiErrorResponse"))),
    response(404, "Entity was not found", ("application/json", response!("ApiErrorResponse"))),
    response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
)]
pub async fn get_repo_by_id(
    State(Instance { controllers, .. }): State<Instance>,
    Path(id): Path<u64>,
    session: Option<Extension<Session>>,
) -> Result<Repository> {
    let repo = match controllers.repositories.get(id.try_into().unwrap()).await {
        Ok(Some(repo)) => repo,
        Ok(None) => {
            return Err(err(
                StatusCode::NOT_FOUND,
                (
                    ErrorCode::EntityNotFound,
                    "unable to find repository by id",
                    json!({"id":id}),
                ),
            ))
        }

        Err(_) => return Err(internal_server_error()),
    };

    if repo.private {
        if let Some(ref session) = session {
            if repo.owner != session.user.id {
                return Err(err(
                    StatusCode::FORBIDDEN,
                    (
                        ErrorCode::AccessNotPermitted,
                        "you're not allowed to see this resource",
                        json!({"class":"Repository"}),
                    ),
                ));
            }
        }
    }

    Ok(ok(StatusCode::OK, repo))
}

#[derive(FromRequestParts)]
pub struct RepoByOwnerAndNamePathParams {
    owner: crate::server::extract::NameOrSnowflake,
    repo: crate::server::extract::NameOrSnowflake,
}

/// Find a repository by the owner's ID/name and the repository's ID/name
///
/// ## Examples
/// ```http
/// # Acceptable
/// GET /repositories/charted/server
///
/// # Not acceptable
/// GET /repositories/server  # `server` will be treated as a owner's ID or name.
/// GET /repositories/charted # use /[users|organizations]/repositories to list all repositories
/// ```
#[controller(
    tags("Repositories"),
    securityRequirements(
        ("ApiKey", ["repo:patch"]),
        ("Bearer", []),
        ("Basic", [])
    ),
    pathParameter("owner", schema!("NameOrSnowflake"), description = "owner's ID or name"),
    pathParameter("repo", schema!("NameOrSnowflake"), description = "repository's ID or name"),
    response(200, "Successful response", ("application/json", response!("RepositoryResponse"))),
    response(400, "Unable to decode given path parameter", ("application/json", response!("ApiErrorResponse"))),
    response(403, "You are not allowed to patch this resource", ("application/json", response!("ApiErrorResponse"))),
    response(404, "Entity was not found", ("application/json", response!("ApiErrorResponse"))),
    response(406, "Received an invalid `Name`", ("application/json", response!("ApiErrorResponse"))),
    response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
)]
pub async fn get_repo_by_owner_and_name(
    State(Instance { controllers, pool, .. }): State<Instance>,
    RepoByOwnerAndNamePathParams {
        owner: crate::server::extract::NameOrSnowflake(owner),
        repo: crate::server::extract::NameOrSnowflake(repo),
    }: RepoByOwnerAndNamePathParams,
    session: Option<Extension<Session>>,
) -> Result<Repository> {
    let mut owner_id: Option<i64> = None;

    match controllers.users.get_by(&owner).await {
        Ok(Some(User { id, .. })) => {
            owner_id = Some(id);
        }

        Ok(None) => {}
        Err(_) => return Err(internal_server_error()),
    }

    match controllers.organizations.get_by(&owner).await {
        Ok(Some(Organization { id, .. })) => {
            owner_id = Some(id);
        }

        Ok(None) => {}
        Err(_) => return Err(internal_server_error()),
    }

    if owner_id.is_none() {
        return Err(err(
            StatusCode::NOT_FOUND,
            (
                ErrorCode::EntityNotFound,
                "unable to find repository owner",
                json!({"idOrName":owner}),
            ),
        ));
    }

    let owner_id = owner_id.unwrap();
    match repo {
        NameOrSnowflake::Snowflake(id) => match sqlx::query_as::<sqlx::Postgres, Repository>(
            "select repositories.* from repositories where repositories.id = $1 and repositories.owner = $2",
        )
        .bind(i64::try_from(id).unwrap())
        .bind(owner_id)
        .fetch_optional(&pool)
        .await
        {
            Ok(Some(repo)) if !repo.private => {
                let mut cache = controllers.repositories.worker.lock().await;
                let key = REPOSITORIES.join(repo.id.to_string());

                match cache.exists(&key).await {
                    Ok(true) => Ok(ok(StatusCode::OK, repo)),
                    Ok(false) => {
                        warn!(repository.id = id, cache.key = %key, "cache hit miss");
                        let _ = cache.put(key.clone(), repo.clone()).await.map_err(|e| {
                            error!(error = %e, cache.key = %key, repository.id = id, "not placing entity in cache due to error");
                            sentry_eyre::capture_report(&e);
                        });

                        Ok(ok(StatusCode::OK, repo))
                    }

                    Err(e) => {
                        error!(error = %e, cache.key = %key, repository.id = id, "cannot check the existence of entity via cache due to an error, skipping");
                        sentry_eyre::capture_report(&e);

                        Ok(ok(StatusCode::OK, repo))
                    }
                }
            }

            Ok(Some(repo)) if repo.private => {
                if let Some(ref session) = session {
                    if repo.owner == session.user.id {
                        let mut cache = controllers.repositories.worker.lock().await;
                        let key = REPOSITORIES.join(repo.id.to_string());

                        match cache.exists(&key).await {
                            Ok(true) => Ok(ok(StatusCode::OK, repo)),
                            Ok(false) => {
                                warn!(repository.id = id, cache.key = %key, "cache hit miss");
                                let _ = cache.put(key.clone(), repo.clone()).await.map_err(|e| {
                                    error!(error = %e, cache.key = %key, repository.id = id, "not placing entity in cache due to error");
                                    sentry_eyre::capture_report(&e);
                                });

                                Ok(ok(StatusCode::OK, repo))
                            }

                            Err(e) => {
                                error!(error = %e, cache.key = %key, repository.id = id, "cannot check the existence of entity via cache due to an error, skipping");
                                sentry_eyre::capture_report(&e);

                                Ok(ok(StatusCode::OK, repo))
                            }
                        }
                    } else {
                        Err(err(
                            StatusCode::FORBIDDEN,
                            (
                                ErrorCode::AccessNotPermitted,
                                "you're not allowed to see this resource",
                                json!({"class":"Repository"}),
                            ),
                        ))
                    }
                } else {
                    Err(err(
                        StatusCode::FORBIDDEN,
                        (
                            ErrorCode::AccessNotPermitted,
                            "you're not allowed to see this resource",
                            json!({"class":"Repository"}),
                        ),
                    ))
                }
            }

            Ok(Some(_)) => Err(err(
                StatusCode::FORBIDDEN,
                (
                    ErrorCode::AccessNotPermitted,
                    "you're not allowed to see this resource",
                    json!({"class":"Repository"}),
                ),
            )),
            Ok(None) => Err(err(
                StatusCode::NOT_FOUND,
                (
                    ErrorCode::EntityNotFound,
                    "unable to find repository from owner",
                    json!({"owner":owner,"repo":repo}),
                ),
            )),
            Err(e) => {
                error!(error = %e, repository.name = %repo, owner.id = owner_id, "unable to get repository");
                sentry::capture_error(&e);

                Err(internal_server_error())
            }
        },
        NameOrSnowflake::Name(ref name) => match sqlx::query_as::<sqlx::Postgres, Repository>(
            "select repositories.* from repositories where repositories.name = $1 and repositories.owner = $2",
        )
        .bind(name)
        .bind(owner_id)
        .fetch_optional(&pool)
        .await
        {
            Ok(Some(repo)) if !repo.private => Ok(ok(StatusCode::OK, repo)),

            // TODO(@auguwu): determine if the repo is an organization
            Ok(Some(repo)) if repo.private => {
                if let Some(ref session) = session {
                    if repo.owner == session.user.id {
                        return Ok(ok(StatusCode::OK, repo));
                    }

                    Err(err(
                        StatusCode::FORBIDDEN,
                        (
                            ErrorCode::AccessNotPermitted,
                            "you're not allowed to see this resource",
                            json!({"class":"Repository"}),
                        ),
                    ))
                } else {
                    Err(err(
                        StatusCode::FORBIDDEN,
                        (
                            ErrorCode::AccessNotPermitted,
                            "you're not allowed to see this resource",
                            json!({"class":"Repository"}),
                        ),
                    ))
                }
            }
            Ok(Some(_)) => Err(err(
                StatusCode::FORBIDDEN,
                (
                    ErrorCode::AccessNotPermitted,
                    "you're not allowed to see this resource",
                    json!({"class":"Repository"}),
                ),
            )),
            Ok(None) => Err(err(
                StatusCode::NOT_FOUND,
                (
                    ErrorCode::EntityNotFound,
                    "unable to find repository from owner",
                    json!({"owner":owner,"repo":repo}),
                ),
            )),
            Err(e) => {
                error!(error = %e, repository.name = %repo, owner.id = owner_id, "unable to get repository");
                sentry::capture_error(&e);

                Err(internal_server_error())
            }
        },
    }
}
