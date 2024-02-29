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
    common::models::entities::{ApiKeyScope, ApiKeyScopes, RepositoryRelease},
    db::controllers::{DbController, PaginationRequest},
    hashmap,
    server::{
        controller,
        extract::Path,
        middleware::session::{Middleware, Session},
        models::res::{err, internal_server_error, ok, ErrorCode, Result},
        pagination::{Pagination, PaginationQuery},
    },
    Instance,
};
use axum::{
    extract::{Query, State},
    handler::Handler,
    http::StatusCode,
    routing, Extension, Router,
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
    response(400, "If parsing through the `id` path parameter failed", ("application/json", response!("ApiResponseError")))
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
            } else {
                can_list_private_stuff = true;
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
