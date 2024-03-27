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
    db::controllers::{DbController, PaginationRequest},
    hashmap,
    server::middleware::session::{Middleware, Session},
    Instance,
};
use axum::{
    extract::{Query, State},
    handler::Handler,
    http::StatusCode,
    routing, Extension, Router,
};
use charted_entities::{ApiKeyScope, ApiKeyScopes, RepositoryRelease};
use charted_server::{
    controller, err,
    extract::Path,
    internal_server_error, ok,
    pagination::{Pagination, PaginationQuery},
    ErrorCode, Result,
};
use serde_json::{json, Value};
use std::cmp;
use tower_http::auth::AsyncRequireAuthorizationLayer;

/// Creates a [`Router`] that implements the following routes:
///
/// * GET, PUT, DELETE `/repositories/:id/releases/:version/provenance.tgz`
/// * GET, PUT, DELETE `/repositories/:id/releases/:version/tarball.tgz`
/// -------------------------------------------------------------------------
/// * GET, PATCH, DELETE, PUT `/repositories/:id/releases/:version`
/// * GET `/repositories/:id/releases`
pub fn create_router() -> Router<Instance> {
    Router::new().route(
        "/",
        routing::get(
            GetAllRepositoryReleasesRestController::run.layer(AsyncRequireAuthorizationLayer::new(Middleware {
                allow_unauthenticated_requests: true,
                scopes: ApiKeyScopes::with_iter([ApiKeyScope::RepoAccess]),
                ..Default::default()
            })),
        ),
    )
}

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
pub async fn get_repository_release_by_tag() {}

/// Creates a new repository release with the specified tag and update changelog,
/// if present.
///
/// To publish a new release tarball (+ provenance file that can be used via `helm sign`), you
/// can use the [`PUT /repositories/{id}/releases/{version}/tarball`] endpoint to do so.
///
/// [`PUT /repositories/{id}/releases/{version}/tarball`]: https://charts.noelware.org/docs/server/latest/api/reference/repository/releases/#PUT-/{version}/tarball
pub async fn create_repository_release() {}

/// Patch a repository release's update changelog. In the future, this would change other
/// things that is implemented, but for now, it'll only update the release's update changelog.
pub async fn patch_repository_release() {}

/// Deletes the repository release from the server and delete the release tarball and provenance file
/// as well.
pub async fn delete_repository_release() {}

/// Locate a repository releases' release tarball, which is the actual chart itself. This can be uploaded
/// when a release is published and is called via the [`PUT /repositories/{id}/releases/{version}/tarball`] endpoint.
///
/// If no chart has a release tarball, this will return a 404 Not Found indicating that the publisher of the
/// release didn't publish one. If you used the [Helm plugin], when using the `helm charted push` command, it'll
/// do that for you.
///
pub async fn get_release_tarball() {}

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
/// $ curl -fsSL -o ./charted.tgz https://charts.noelware.org/api/repositories/$ID/releases/0.1.0-beta/release.tgz
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
pub async fn get_release_provenance_file() {}

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
pub async fn put_release_tarball() {}

/// Deletes a Helm chart release from a repository release.
pub async fn delete_release_tarball() {}

/// Deletes a Helm chart's provenance file from a repository release.
pub async fn delete_release_provenance_tarball() {}
