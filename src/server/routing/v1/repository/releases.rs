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

use crate::{
    db::controllers::{Controllers, DbController, PaginationRequest},
    server::{
        middleware::session::{Middleware, Session},
        validation::validate,
    },
    Instance,
};
use axum::{
    extract::{Query, State},
    handler::Handler,
    http::StatusCode,
    routing, Extension, Router,
};
use azalia::hashmap;
use charted_entities::{
    payloads::{CreateRepositoryReleasePayload, PatchRepositoryReleasePayload},
    ApiKeyScope, ApiKeyScopes, Repository, RepositoryRelease, Version,
};
use charted_helm_charts::UploadReleaseTarballRequest;
use charted_server::{
    controller, err,
    extract::{Json, Path},
    internal_server_error,
    multipart::Multipart,
    ok,
    pagination::{Pagination, PaginationQuery},
    ApiResponse, ErrorCode, Result,
};
use chrono::Local;
use remi::Bytes;
use serde_json::{json, Value};
use sqlx::PgPool;
use std::cmp;
use tower_http::auth::AsyncRequireAuthorizationLayer;
use validator::Validate;

/// Creates a [`Router`] that implements the following routes:
///
/// * GET, PUT, DELETE `/repositories/:id/releases/:version/provenance`
/// * GET, PUT, DELETE `/repositories/:id/releases/:version/tarball`
/// -------------------------------------------------------------------------
/// * GET, PUT, PATCH, DELETE `/repositories/:id/releases`
/// * GET `/repositories/:id/releases/:version`
pub fn create_router() -> Router<Instance> {
    Router::new()
        .route(
            "/",
            routing::get(
                GetAllRepositoryReleasesRestController::run.layer(AsyncRequireAuthorizationLayer::new(Middleware {
                    allow_unauthenticated_requests: true,
                    scopes: ApiKeyScopes::with_iter([ApiKeyScope::RepoAccess]),
                    ..Default::default()
                })),
            )
            .put(
                CreateRepositoryReleaseRestController::run.layer(AsyncRequireAuthorizationLayer::new(Middleware {
                    scopes: ApiKeyScopes::with_iter([ApiKeyScope::RepoReleaseCreate]),
                    ..Default::default()
                })),
            ),
        )
        .route(
            "/:version",
            routing::get(
                GetRepositoryReleaseByTagRestController::run.layer(AsyncRequireAuthorizationLayer::new(Middleware {
                    allow_unauthenticated_requests: true,
                    scopes: ApiKeyScopes::with_iter([ApiKeyScope::RepoAccess]),
                    ..Default::default()
                })),
            )
            .patch(
                PatchRepositoryReleaseRestController::run.layer(AsyncRequireAuthorizationLayer::new(Middleware {
                    scopes: ApiKeyScopes::with_iter([ApiKeyScope::RepoReleaseUpdate]),
                    ..Default::default()
                })),
            )
            .delete(
                DeleteRepositoryReleaseRestController::run.layer(AsyncRequireAuthorizationLayer::new(Middleware {
                    scopes: ApiKeyScopes::with_iter([ApiKeyScope::RepoReleaseDelete]),
                    ..Default::default()
                })),
            ),
        )
        .route(
            "/:version/tarball",
            routing::get(
                GetReleaseTarballRestController::run.layer(AsyncRequireAuthorizationLayer::new(Middleware {
                    allow_unauthenticated_requests: true,
                    ..Default::default()
                })),
            )
            .put(PutReleaseTarballRestController::run.layer(AsyncRequireAuthorizationLayer::new(Middleware::default())))
            .delete(
                DeleteReleaseTarballRestController::run
                    .layer(AsyncRequireAuthorizationLayer::new(Middleware::default())),
            ),
        )
    // .route(
    //     "/:version/provenance",
    //     routing::get(
    //         GetReleaseProvenanceFileRestController::run.layer(AsyncRequireAuthorizationLayer::new(Middleware {
    //             allow_unauthenticated_requests: true,
    //             ..Default::default()
    //         })),
    //     )
    //     .put(
    //         PutReleaseProvenanceTarballRestController::run
    //             .layer(AsyncRequireAuthorizationLayer::new(Middleware::default())),
    //     ),
    // )
}

/// Fetches all the releases for a specific repository.
#[controller(
    tags("Repository", "Releases"),
    securityRequirements(
        ("ApiKey", ["repo:access"]),
        ("Bearer", []),
        ("Basic", [])
    ),
    pathParameter("id", snowflake, description = "Repository ID to locate the releases from"),
    queryParameter("cursor", snowflake, description = "ID that acts like a cursor to passthrough pages"),
    queryParameter("per_page", int32, description = "Elements that should be present in a singular page"),
    queryParameter("order", schema!("OrderBy"), description = "Orders elements in page in ascending (default) or descending order by the entity ID"),
    response(200, "Successful response", ("application/json", response!("RepositoryReleasePaginatedResponse"))),
    response(400, "If parsing through the `id` path parameter failed", ("application/json", response!("ApiErrorResponse")))
)]
pub async fn get_all_repository_releases(
    State(Instance { controllers, .. }): State<Instance>,
    Path(id): Path<i64>,
    Query(PaginationQuery {
        per_page,
        order,
        cursor,
    }): Query<PaginationQuery>,
    session: Option<Extension<Session>>,
) -> Result<Pagination<RepositoryRelease>> {
    let repo = match controllers.repositories.get(id).await {
        Ok(Some(repo)) => repo,
        Ok(None) => {
            return Err(err(
                StatusCode::NOT_FOUND,
                (
                    ErrorCode::EntityNotFound,
                    "repository with id doesn't exist",
                    json!({"id":id}),
                ),
            ))
        }

        Err(_) => return Err(internal_server_error()),
    };

    // check if the owner (user) / creator (org) is from the session
    // to determine if the repository can be viewed if the repository
    // is private
    let mut can_list_private_stuff = false;
    if repo.private {
        if let Some(ref session) = session {
            let compare = if let Some(creator) = repo.creator {
                creator == session.user.id
            } else {
                repo.owner == session.user.id
            };

            if !compare {
                return Err(err(
                    StatusCode::FORBIDDEN,
                    (
                        ErrorCode::AccessNotPermitted,
                        "access is not permitted to this resource",
                    ),
                ));
            }

            can_list_private_stuff = true;
        } else {
            return Err(err(
                StatusCode::FORBIDDEN,
                (
                    ErrorCode::AccessNotPermitted,
                    "access is not permitted to this resource",
                ),
            ));
        }
    }

    if per_page > 100 {
        return Err(err(
            StatusCode::NOT_ACCEPTABLE,
            (
                ErrorCode::MaxPerPageExceeded,
                "`per_page` query parameter can't go over 100 entries",
                json!({"perPage": per_page}),
            ),
        ));
    }

    controllers
        .releases
        .paginate(PaginationRequest {
            list_private_stuff: can_list_private_stuff,
            owner_id: Some(repo.owner.try_into().map_err(|_| internal_server_error())?),
            order_by: order,
            per_page: cmp::min(10, per_page),
            cursor,
            metadata: hashmap! {
                "repository" => Value::Number(repo.id.into())
            },
        })
        .await
        .map(|data| ok(StatusCode::OK, data))
        .map_err(|_| internal_server_error())
}

/// Retrieve a repository release via its semver tag.
#[controller(
    tags("Repository", "Releases"),
    securityRequirements(
        ("ApiKey", ["repo:access"]),
        ("Bearer", []),
        ("Basic", [])
    ),
)]
pub async fn get_repository_release_by_tag(
    State(Instance { controllers, pool, .. }): State<Instance>,
    Path((id, version)): Path<(i64, Version)>,
    session: Option<Extension<Session>>,
) -> Result<RepositoryRelease> {
    // ensures that a repository does exist with that ID
    let repo = match controllers.repositories.get(id).await {
        Ok(Some(repo)) => repo,
        Ok(None) => {
            return Err(err(
                StatusCode::NOT_FOUND,
                (
                    ErrorCode::EntityNotFound,
                    "repository with id was not found",
                    json!({"id":id}),
                ),
            ))
        }

        Err(_) => return Err(internal_server_error()),
    };

    // check if the owner (user) / creator (org) is from the session
    // to determine if the repository can be viewed if the repository
    // is private
    if repo.private {
        if let Some(ref session) = session {
            let compare = if let Some(creator) = repo.creator {
                creator == session.user.id
            } else {
                repo.owner == session.user.id
            };

            if !compare {
                return Err(err(
                    StatusCode::FORBIDDEN,
                    (
                        ErrorCode::AccessNotPermitted,
                        "access is not permitted to this resource",
                    ),
                ));
            }
        } else {
            return Err(err(
                StatusCode::FORBIDDEN,
                (
                    ErrorCode::AccessNotPermitted,
                    "access is not permitted to this resource",
                ),
            ));
        }
    }

    match sqlx::query_as::<_, RepositoryRelease>(
        "select repository_releases.* from repository_releases where repository = $1 and tag = $2;",
    )
    .bind(id)
    .bind(&version)
    .fetch_optional(&pool)
    .await
    {
        Ok(Some(release)) => Ok(ok(StatusCode::OK, release)),
        Ok(None) => Err(err(
            StatusCode::NOT_FOUND,
            (
                ErrorCode::EntityNotFound,
                "repository release with tag doesn't exist",
                json!({"repository":id,"tag":version}),
            ),
        )),

        Err(_) => Err(internal_server_error()),
    }
}

/// Creates a new repository release with the specified tag and update changelog,
/// if present.
///
/// To publish a new release tarball (+ provenance file that can be used via `helm package --sign`), you
/// can use the [`PUT /repositories/{id}/releases/{version}/tarball`] endpoint to do so.
///
/// [`PUT /repositories/{id}/releases/{version}/tarball`]: https://charts.noelware.org/docs/server/latest/api/reference/repository/releases/#PUT-/{version}/tarball
#[controller(
    method = put,
    tags("Repository", "Releases"),
    securityRequirements(
        ("ApiKey", ["repo:releases:create"]),
        ("Bearer", []),
        ("Basic", [])
    ),
)]
pub async fn create_repository_release(
    State(Instance {
        controllers,
        pool,
        snowflake,
        ..
    }): State<Instance>,
    Path(id): Path<i64>,
    Extension(session): Extension<Session>,
    Json(payload): Json<CreateRepositoryReleasePayload>,
) -> Result<RepositoryRelease> {
    validate(&payload, CreateRepositoryReleasePayload::validate)?;

    // ensures that a repository does exist with that ID
    let repo = match controllers.repositories.get(id).await {
        Ok(Some(repo)) => repo,
        Ok(None) => {
            return Err(err(
                StatusCode::NOT_FOUND,
                (
                    ErrorCode::EntityNotFound,
                    "repository with id was not found",
                    json!({"id":id}),
                ),
            ))
        }

        Err(_) => return Err(internal_server_error()),
    };

    // check if the owner (user) / creator (org) is from the session
    // to determine if the repository can be viewed if the repository
    // is private
    if repo.private {
        let compare = if let Some(creator) = repo.creator {
            creator == session.user.id
        } else {
            repo.owner == session.user.id
        };

        if !compare {
            return Err(err(
                StatusCode::FORBIDDEN,
                (
                    ErrorCode::AccessNotPermitted,
                    "access is not permitted to this resource",
                ),
            ));
        }
    }

    match sqlx::query_as::<_, RepositoryRelease>(
        "select repository_releases.* from repository_releases where repository = $1 and tag = $2;",
    )
    .bind(id)
    .bind(&payload.tag)
    .fetch_optional(&pool)
    .await
    {
        Ok(Some(_)) => {
            return Err(err(
                StatusCode::CONFLICT,
                (
                    ErrorCode::EntityAlreadyExists,
                    "repository release already exists",
                    json!({"repository":id,"tag":payload.tag}),
                ),
            ))
        }

        Ok(None) => {}
        Err(_) => return Err(internal_server_error()),
    }

    let id = snowflake.generate();
    let skeleton = RepositoryRelease {
        is_prerelease: !payload.tag.pre.is_empty(),
        update_text: payload.update_text.clone(),
        repository: repo.id,
        created_at: Local::now(),
        updated_at: Local::now(),
        tag: payload.tag.clone(),
        id: id.value().try_into().unwrap(),
    };

    controllers
        .releases
        .create(payload, &skeleton)
        .await
        .map(|_| ok(StatusCode::CREATED, skeleton))
        .map_err(|_| internal_server_error())
}

/// Patch a repository release's update changelog. In the future, this would change other
/// things that is implemented, but for now, it'll only update the release's update changelog.
#[controller(
    method = patch,
    tags("Repository", "Releases"),
    securityRequirements(
        ("ApiKey", ["repo:releases:update"]),
        ("Bearer", []),
        ("Basic", [])
    ),
)]
pub async fn patch_repository_release(
    State(Instance { controllers, pool, .. }): State<Instance>,
    Path((id, version)): Path<(i64, Version)>,
    Extension(Session { user, .. }): Extension<Session>,
    Json(payload): Json<PatchRepositoryReleasePayload>,
) -> Result<()> {
    validate(&payload, PatchRepositoryReleasePayload::validate)?;
    let (_, release) = ensure_repository_and_release_exist(
        &controllers,
        &pool,
        id,
        &version,
        Some(&Session { user, session: None }),
        true,
    )
    .await?;

    controllers
        .releases
        .patch(release.id, payload)
        .await
        .map(|_| ok(StatusCode::ACCEPTED, ()))
        .map_err(|_| internal_server_error())
}

/// Deletes the repository release from the server and delete the release tarball and provenance file
/// as well.
#[controller(
    method = delete,
    tags("Repository", "Releases"),
    securityRequirements(
        ("ApiKey", ["repo:releases:delete"]),
        ("Bearer", []),
        ("Basic", [])
    ),
)]
pub async fn delete_repository_release(
    State(Instance { controllers, pool, .. }): State<Instance>,
    Path((id, version)): Path<(i64, Version)>,
    Extension(Session { user, .. }): Extension<Session>,
) -> Result<()> {
    // Ensure that a release with the version specified already exists
    let (_, release) = ensure_repository_and_release_exist(
        &controllers,
        &pool,
        id,
        &version,
        Some(&Session { user, session: None }),
        true,
    )
    .await?;

    controllers
        .releases
        .delete(release.id)
        .await
        .map(|_| ok(StatusCode::ACCEPTED, ()))
        .map_err(|_| internal_server_error())
}

/// Locate a repository releases' release tarball, which is the actual chart itself. This can be uploaded
/// when a release is published and is called via the [`PUT /repositories/{id}/releases/{version}/tarball`] endpoint.
///
/// If no chart has a release tarball, this will return a 404 Not Found indicating that the publisher of the
/// release didn't publish one. If you used the [Helm plugin], when using the `helm charted push` command, it'll
/// do that for you.
///
/// [Helm plugin]: https://charts.noelware.org/docs/helm-plugin
/// [`PUT /repositories/{id}/releases/{version}/tarball`]: https://charts.noelware.org/docs/server/api/reference/repository-releases#PUT-/{id}/releases/{version}/tarball
#[controller(
    tags("Repository", "Releases"),
    securityRequirements(
        ("ApiKey", []),
        ("Bearer", []),
        ("Basic", [])
    ),
)]
pub async fn get_release_tarball(
    State(Instance {
        controllers, charts, ..
    }): State<Instance>,
    Path((id, version)): Path<(i64, Version)>,
    session: Option<Extension<Session>>,
) -> std::result::Result<Bytes, ApiResponse> {
    let repo = match controllers.repositories.get(id).await {
        Ok(Some(repo)) => repo,
        Ok(None) => {
            return Err(err(
                StatusCode::NOT_FOUND,
                (
                    ErrorCode::EntityNotFound,
                    "repository with id was not found",
                    json!({"id":id}),
                ),
            ))
        }

        Err(_) => return Err(internal_server_error()),
    };

    if repo.private {
        if let Some(ref session) = session {
            let compare = if let Some(creator) = repo.creator {
                creator == session.user.id
            } else {
                repo.owner == session.user.id
            };

            if !compare {
                return Err(err(
                    StatusCode::FORBIDDEN,
                    (
                        ErrorCode::AccessNotPermitted,
                        "access is not permitted to this resource",
                    ),
                ));
            }
        } else {
            return Err(err(
                StatusCode::FORBIDDEN,
                (
                    ErrorCode::AccessNotPermitted,
                    "access is not permitted to this resource",
                ),
            ));
        }
    }

    match charts
        .get_tarball(
            repo.owner.try_into().map_err(|_| internal_server_error())?,
            repo.id.try_into().map_err(|_| internal_server_error())?,
            version.to_string().trim(),
            !version.pre.is_empty(),
        )
        .await
    {
        Ok(Some(content)) => Ok(content),
        Ok(None) => Err(err(
            StatusCode::NOT_FOUND,
            (ErrorCode::EntityNotFound, "release doesn't have a chart linked to it"),
        )),
        Err(_) => Err(internal_server_error()),
    }
}

/// Locate a repository releases' provenance file, which is used when signing Helm charts. This can be uploaded
/// when a release is published and is called via the [`PUT /repositories/{id}/releases/{version}/provenance`] endpoint.
///
/// If the chart didn't include uploading it from the specific endpoint, then it'll return a 404 Not Found, indicating
/// that the publisher didn't include one.
///
/// If you used the [Helm plugin] and used `helm charted push --provenance`, it'll upload the provenance file to the API server
/// and updates it accordingly.
///
/// ## Using the provenance file
/// To check the integrity of the Helm chart, you can use the [`helm verify`] command:
///
/// ```sh
/// # Since Helm requires to verify a *file*, we will need to get the Helm chart
/// # from the API server. (we need `charted`'s ID to locate since we don't perform lookups from `owner/name` references)
/// $ export ID=$(curl -fsSL https://charts.noelware.org/api/organizations/charted | jq '.data.id')
/// $ curl -fsSL -o ./charted.tgz https://charts.noelware.org/api/repositories/$ID/releases/0.1.0-beta/tarball
///
/// # Validates the `charted/server` Helm chart, since Noelware upload provenance files
/// # when new releases occur.
/// $ helm verify charted.tgz
/// ```
///
/// To use it from [`helm install`], we can use `helm install --verify` and it'll verify its integrity. This is easier
/// than performing [`helm verify`] manually.
///
/// ```sh
/// # If you have the Helm plugin for charted installed, we can use `charted://`, which will
/// # do the necessary lookups when using `helm install`.
/// $ helm install --verify charted charted://charted/server
/// ```
///
/// [`PUT /repositories/{id}/releases/{version}/tarball`]: https://charts.noelware.org/docs/server/latest/api/reference/repository/releases/#PUT-/{version}/tarball
/// [`helm install`]: https://helm.sh/docs/helm/helm_install/#helm-install
/// [`helm verify`]: https://helm.sh/docs/helm/helm_verify/#helm-verify
/// [Helm plugin]: https://charts.noelware.org/docs/helm-plugin
#[controller(
    tags("Repository", "Releases"),
    securityRequirements(
        ("ApiKey", ["repo:releases:delete"]),
        ("Bearer", []),
        ("Basic", [])
    ),
)]
pub async fn get_release_provenance_file(
    State(Instance {
        controllers, charts, ..
    }): State<Instance>,
    Path((id, version)): Path<(i64, Version)>,
    session: Option<Extension<Session>>,
) {
    let repo = match controllers.repositories.get(id).await {
        Ok(Some(repo)) => repo,
        Ok(None) => {
            return Err(err(
                StatusCode::NOT_FOUND,
                (
                    ErrorCode::EntityNotFound,
                    "repository with id was not found",
                    json!({"id":id}),
                ),
            ))
        }

        Err(_) => return Err(internal_server_error()),
    };

    if repo.private {
        if let Some(ref session) = session {
            let compare = if let Some(creator) = repo.creator {
                creator == session.user.id
            } else {
                repo.owner == session.user.id
            };

            if !compare {
                return Err(err(
                    StatusCode::FORBIDDEN,
                    (
                        ErrorCode::AccessNotPermitted,
                        "access is not permitted to this resource",
                    ),
                ));
            }
        } else {
            return Err(err(
                StatusCode::FORBIDDEN,
                (
                    ErrorCode::AccessNotPermitted,
                    "access is not permitted to this resource",
                ),
            ));
        }
    }

    match charts
        .get_provenance(
            repo.owner.try_into().map_err(|_| internal_server_error())?,
            repo.id.try_into().map_err(|_| internal_server_error())?,
            version.to_string().trim(),
            !version.pre.is_empty(),
        )
        .await
    {
        Ok(Some(content)) => Ok(content),
        Ok(None) => Err(err(
            StatusCode::NOT_FOUND,
            (ErrorCode::EntityNotFound, "release doesn't have a chart linked to it"),
        )),
        Err(_) => Err(internal_server_error()),
    }
}

/// Initiate a upload request for adding a Helm chart from a created repository release. Do be warned that
/// this can be any arbitrary Helm chart, so put in care of what you upload to this endpoint.
///
/// This endpoint uses `multipart/form-data` as a way for uploading, you are required to send the Helm chart's
/// package itself as the first field and an optional second field, which will be the provenance file that was
/// generated either from `helm package --sign` or from `charted helm push --provenance`.
///
/// ## Usage
/// ### API Server
/// > [!IMPORTANT]
/// > Please handle this endpoint with care as this will accept any arbitrary Helm chart!
///
/// ```sh
/// # Create your Helm chart archive
/// $ helm package .
///
/// # Now, we send a request to the API server. You will need to handle
/// # authentication on your own.
/// $ curl -L -F "chart=@./package-version.tgz" https://charts.noelware.org/api/repositories/{id}/releases/{version}/release.tgz
/// ```
///
/// ### Helm plugin
/// All you need to call is `helm charted push [repository or '.' for all]` and it'll do it for you!
#[controller(
    method = put,
    tags("Repository", "Releases"),
    securityRequirements(
        ("ApiKey", []),
        ("Bearer", []),
        ("Basic", [])
    ),
)]
pub async fn put_release_tarball(
    State(Instance {
        controllers, charts, ..
    }): State<Instance>,
    Path((id, version)): Path<(i64, Version)>,
    Extension(Session { user, .. }): Extension<Session>,
    data: Multipart,
) -> Result<()> {
    let repo = match controllers.repositories.get(id).await {
        Ok(Some(repo)) => repo,
        Ok(None) => {
            return Err(err(
                StatusCode::NOT_FOUND,
                (
                    ErrorCode::EntityNotFound,
                    "repository with id was not found",
                    json!({"id":id}),
                ),
            ))
        }

        Err(_) => return Err(internal_server_error()),
    };

    if repo.private {
        let compare = if let Some(creator) = repo.creator {
            creator == user.id
        } else {
            repo.owner == user.id
        };

        if !compare {
            return Err(err(
                StatusCode::FORBIDDEN,
                (
                    ErrorCode::AccessNotPermitted,
                    "access is not permitted to this resource",
                ),
            ));
        }
    }

    charts
        .upload(
            UploadReleaseTarballRequest {
                owner: repo.owner.try_into().unwrap(),
                version: version.to_string(),
                repo: repo.id.try_into().unwrap(),
            },
            data.into_inner(),
        )
        .await
        .map(|_| ok(StatusCode::ACCEPTED, ()))
        .inspect_err(|e| {
            error!(error = %e, repository.id = repo.id, "failed to upload tarball");
            sentry::capture_error(&e);
        })
        .map_err(|_| internal_server_error())
}

// #[controller(
//     tags("Repository", "Releases"),
//     securityRequirements(
//         ("ApiKey", []),
//         ("Bearer", []),
//         ("Basic", [])
//     ),
// )]
// pub async fn put_release_provenance_tarball(
//     State(Instance { controllers: _, .. }): State<Instance>,
//     Path((_, _)): Path<(i64, Version)>,
//     Extension(Session { user: _, .. }): Extension<Session>,
//     _: Multipart,
// ) {
// }

/// Deletes a Helm chart release from a repository release.
#[controller(
    method = delete,
    tags("Repository", "Releases"),
    securityRequirements(
        ("ApiKey", []),
        ("Bearer", []),
        ("Basic", [])
    ),
)]
pub async fn delete_release_tarball(
    State(Instance {
        controllers, charts, ..
    }): State<Instance>,
    Path((id, version)): Path<(i64, Version)>,
    Extension(Session { user, .. }): Extension<Session>,
) {
    let repo = match controllers.repositories.get(id).await {
        Ok(Some(repo)) => repo,
        Ok(None) => {
            return Err(err(
                StatusCode::NOT_FOUND,
                (
                    ErrorCode::EntityNotFound,
                    "repository with id was not found",
                    json!({"id":id}),
                ),
            ))
        }

        Err(_) => return Err(internal_server_error()),
    };

    if repo.private {
        let compare = if let Some(creator) = repo.creator {
            creator == user.id
        } else {
            repo.owner == user.id
        };

        if !compare {
            return Err(err(
                StatusCode::FORBIDDEN,
                (
                    ErrorCode::AccessNotPermitted,
                    "access is not permitted to this resource",
                ),
            ));
        }
    }

    charts
        .delete_chart(
            repo.owner.try_into().unwrap(),
            repo.id.try_into().unwrap(),
            version.to_string(),
        )
        .await
        .map(|()| ok(StatusCode::ACCEPTED, ()))
        .inspect_err(|e| {
            error!(error = %e, repo.id, "failed to delete chart for repository");
            sentry_eyre::capture_report(e);
        })
        .map_err(|_| internal_server_error())
}

// /// Deletes a Helm chart's provenance file from a repository release.
// #[controller(
//     tags("Repository", "Releases"),
//     securityRequirements(
//         ("ApiKey", []),
//         ("Bearer", []),
//         ("Basic", [])
//     ),
// )]
// pub async fn delete_release_provenance_tarball(
//     State(Instance { controllers: _, .. }): State<Instance>,
//     Path((_, _)): Path<(i64, Version)>,
//     Extension(Session { user: _, .. }): Extension<Session>,
// ) {
// }

async fn ensure_repository_and_release_exist(
    controllers: &Controllers,
    pool: &PgPool,
    id: i64,
    tag: &Version,
    session: Option<&Session>,
    check_perms: bool,
) -> std::result::Result<(Repository, RepositoryRelease), ApiResponse> {
    let repo = match controllers.repositories.get(id).await {
        Ok(Some(repo)) => repo,
        Ok(None) => {
            return Err(err(
                StatusCode::NOT_FOUND,
                (
                    ErrorCode::EntityNotFound,
                    "repository with id was not found",
                    json!({"id":id}),
                ),
            ))
        }

        Err(_) => return Err(internal_server_error()),
    };

    if check_perms && repo.private {
        // check if the owner (user) / creator (org) is from the session
        // to determine if the repository can be viewed if the repository
        // is private

        if let Some(session) = session {
            let compare = if let Some(creator) = repo.creator {
                creator == session.user.id
            } else {
                repo.owner == session.user.id
            };

            if !compare {
                return Err(err(
                    StatusCode::FORBIDDEN,
                    (
                        ErrorCode::AccessNotPermitted,
                        "access is not permitted to this resource",
                    ),
                ));
            }
        } else {
            return Err(err(
                StatusCode::FORBIDDEN,
                (
                    ErrorCode::AccessNotPermitted,
                    "access is not permitted to this resource",
                ),
            ));
        }
    }

    match sqlx::query_as::<_, RepositoryRelease>(
        "select repository_releases.* from repository_releases where repository = $1 and tag = $2;",
    )
    .bind(id)
    .bind(tag)
    .fetch_optional(pool)
    .await
    {
        Ok(Some(release)) => Ok((repo, release)),
        Ok(None) => Err(err(
            StatusCode::NOT_FOUND,
            (
                ErrorCode::EntityNotFound,
                "repository release with tag doesn't exist",
                json!({"repository":repo.id,"tag":tag}),
            ),
        )),

        Err(_) => Err(internal_server_error()),
    }
}
