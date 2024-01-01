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
    extract::Json,
    macros::controller,
    middleware::{Session, SessionAuth},
    models::res::{err, ok, ApiResponse},
    validation::validate,
    Server,
};
use axum::{
    extract::{Path, Query, State},
    handler::Handler,
    http::StatusCode,
    routing, Extension, Router,
};
use charted_common::{
    models::{
        entities::{ApiKeyScope, Repository},
        payloads::CreateRepositoryPayload,
        Name, NameOrSnowflake,
    },
    server::pagination::{Pagination, PaginationQuery},
};
use charted_database::controller::{
    repositories::RepositoryDatabaseController, users::UserDatabaseController, DbController, PaginationRequest,
};
use charted_openapi::generate_response_schema;
use chrono::Local;
use tower_http::auth::AsyncRequireAuthorizationLayer;
use validator::Validate;

pub(crate) struct RepositoryResponse;
generate_response_schema!(RepositoryResponse, schema = "Repository");

pub fn create_router() -> Router<Server> {
    Router::new().route(
        "/",
        routing::put(
            CreateUserRepositoryRestController::run.layer(AsyncRequireAuthorizationLayer::new(
                SessionAuth::default().scope(ApiKeyScope::RepoCreate),
            )),
        ),
    )
}

/// Retrieve a list of all a user's repositories.
#[controller(
    tags("Users", "Repositories"),
    response(200, "List of all the user's repositories", ("application/json", response!("RepositoryPaginatedResponse"))),
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
            owner_id: Some(u64::try_from(owner.id).unwrap()),
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
    response(201, "Repository created", ("application/json", response!("RepositoryResponse"))),
    response(400, "Bad Request", ("application/json", response!("ApiErrorResponse"))),
    response(409, "Conflict: repository with that name already exists on the user's account", ("application/json", response!("ApiErrorResponse"))),
    response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
)]
pub async fn create_user_repository(
    State(Server {
        controllers, snowflake, ..
    }): State<Server>,
    Extension(Session { user, .. }): Extension<Session>,
    Json(payload): Json<CreateRepositoryPayload>,
) -> Result<ApiResponse<Repository>, ApiResponse> {
    let repos = controllers.get::<RepositoryDatabaseController>();
    validate(payload.clone(), CreateRepositoryPayload::validate)?;

    // validate the name (since the first one won't go through)
    let name = validate(payload.name.clone(), Name::validate)?;

    // check if the owner has a repo with the same name
    match repos.get_by_nos(NameOrSnowflake::Name(name.clone())).await {
        Ok(None) => {}
        Ok(Some(_)) => {
            return Err(err(
                StatusCode::CONFLICT,
                (
                    "REPO_ALREADY_EXISTS",
                    format!(
                        "repository with name {} already exists under your account",
                        payload.name.clone()
                    ),
                )
                    .into(),
            ));
        }

        Err(_) => {
            return Err(err(
                StatusCode::INTERNAL_SERVER_ERROR,
                ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
            ));
        }
    }

    let id = snowflake.generate();
    let now = Local::now();
    let repo = repos
        .create(
            payload.clone(),
            Repository {
                description: payload.description.clone(),
                created_at: now,
                updated_at: now,
                private: payload.private,
                r#type: payload.r#type,
                owner: user.id,
                name,
                id: i64::try_from(id.value()).unwrap(),

                ..Default::default()
            },
        )
        .await
        .map_err(|_| {
            err(
                StatusCode::INTERNAL_SERVER_ERROR,
                ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
            )
        })?;

    Ok(ok(StatusCode::CREATED, repo))
}
