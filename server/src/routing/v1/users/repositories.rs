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
    middleware::{Session, SessionAuth},
    models::res::{err, ok, ApiResponse},
    openapi::gen_response_schema,
    Server,
};
use axum::{
    extract::{Path, Query, State},
    handler::Handler,
    http::StatusCode,
    routing, Extension, Json, Router,
};
use charted_common::{
    models::{entities::Repository, payloads::CreateRepositoryPayload, NameOrSnowflake},
    server::pagination::{Pagination, PaginationQuery},
};
use charted_database::controller::{
    repositories::RepositoryDatabaseController, users::UserDatabaseController, DbController, PaginationRequest,
};
use charted_proc_macros::controller;
use tower_http::auth::AsyncRequireAuthorizationLayer;

pub(crate) struct RepositoryResponse;
gen_response_schema!(RepositoryResponse, schema: "Repository");

pub fn create_router() -> Router<Server> {
    Router::new().route(
        "/",
        routing::put(CreateUserRepositoryRestController::run.layer(AsyncRequireAuthorizationLayer::new(SessionAuth))),
    )
}

/// Retrieve a list of all a user's repositories.
#[controller(
    tags("Users", "Repositories"),
    response(200, "List of all the user's repositories", ("application/json", response!("ApiRepositoryPaginatedResponse"))),
    pathParameter("idOrName", schema!("NameOrSnowflake"), description = "Path parameter that can take a [`Name`] or [`Snowflake`] identifier."),
    queryParameter("cursor", snowflake, description = "Cursor to passthrough to proceed into the next or previous page."),
    queryParameter("per_page", int32, description = "How many elements should be present in a page."),
    queryParameter("order", schema!("OrderBy"), description = "Order to sort the entries by.")
)]
pub async fn list_user_repositories(
    State(Server { controllers, .. }): State<Server>,
    Path(nos): Path<NameOrSnowflake>,
    Query(PaginationQuery {
        cursor,
        mut per_page,
        order,
    }): Query<PaginationQuery>,
) -> Result<ApiResponse<Pagination<Repository>>, ApiResponse> {
    let users = controllers.get::<UserDatabaseController>();
    let owner = users
        .get_by_nos(nos.clone())
        .await
        .map_err(|_| {
            err(
                StatusCode::INTERNAL_SERVER_ERROR,
                ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
            )
        })?
        .ok_or_else(|| {
            err(
                StatusCode::NOT_FOUND,
                (
                    "USER_NOT_FOUND",
                    format!("unable to find user by ID or name [{nos}]").as_str(),
                )
                    .into(),
            )
        })?;

    if per_page > 100 {
        return Err(err(
            StatusCode::NOT_ACCEPTABLE,
            ("MAX_PER_PAGE_EXCEEDED", "per_page query parameter can't go over 100").into(),
        ));
    }

    if per_page < 10 {
        per_page = 10;
    }

    let repos = controllers.get::<RepositoryDatabaseController>();
    let data = repos
        .paginate(PaginationRequest {
            cursor,
            per_page,
            order_by: order,
            owner_id: Some(owner.id as u64),
        })
        .await
        .map_err(|_| {
            err(
                StatusCode::INTERNAL_SERVER_ERROR,
                ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
            )
        })?;

    Ok(ok(StatusCode::OK, data))
}

/// Creates a [`Repository`] with the current authenticated user as the owner of the repository.
#[controller(
    method = put,
    tags("Repositories"),
    response(201, "Repository created", ("application/json", response!("ApiRepositoryResponse")))
)]
pub async fn create_user_repository(
    State(Server { controllers, .. }): State<Server>,
    Extension(Session { .. }): Extension<Session>,
    Json(_): Json<CreateRepositoryPayload>,
) -> Result<ApiResponse<Repository>, ApiResponse> {
    let _repos = controllers.get::<RepositoryDatabaseController>();

    Ok(ok(StatusCode::CREATED, Repository::default()))
}
