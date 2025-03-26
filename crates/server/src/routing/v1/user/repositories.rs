// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2025 Noelware, LLC. <team@noelware.org>
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
    Context,
    extract::{Path, Query},
    middleware::authn::Session,
    pagination::{Ordering, PaginationRequest},
    util::{self, BuildLinkHeaderOpts},
};
use axum::{
    Extension,
    extract::State,
    http::{
        StatusCode,
        header::{self, HeaderValue},
    },
};
use charted_core::{api, clamp};
use charted_database::entities::{RepositoryEntity, UserEntity, repository, user};
use charted_types::{NameOrUlid, Repository, User};
use sea_orm::{ColumnTrait, EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder};
use serde_json::json;
use std::{cmp, collections::BTreeMap};
use utoipa::{
    IntoResponses,
    openapi::{RefOr, Response},
};

struct ListRepositoriesR;
impl IntoResponses for ListRepositoriesR {
    fn responses() -> BTreeMap<String, RefOr<Response>> {
        azalia::btreemap! {}
    }
}

/// Lists all the avaliable user repositories.
///
/// If the user is logged in with credentials, this will also show their private
/// repositories as well.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    get,
    path = "/v1/users/{idOrName}/repositories",
    operation_id = "listRepositories",
    tag = "Repositories",
    params(PaginationRequest),
    responses(ListRepositoriesR)
)]
pub async fn list_user_repositories(
    State(cx): State<Context>,
    Path(id_or_name): Path<NameOrUlid>,
    Query(PaginationRequest {
        per_page,
        order_by,
        page,
    }): Query<PaginationRequest>,
) -> api::Result<Vec<Repository>> {
    let ion = id_or_name.clone();

    let per_page = clamp(per_page, 10, 100).unwrap_or(10);
    let paginator = (match id_or_name {
        NameOrUlid::Ulid(id) => RepositoryEntity::find().filter(repository::Column::Owner.eq(id)),
        NameOrUlid::Name(ref name) => match UserEntity::find()
            .filter(user::Column::Username.eq(name.to_owned()))
            .one(&cx.pool)
            .await
            .map_err(api::system_failure)?
            .map(Into::<User>::into)
        {
            Some(user) => RepositoryEntity::find().filter(repository::Column::Owner.eq(user.id)),
            None => {
                return Err(api::err(
                    StatusCode::NOT_FOUND,
                    (
                        api::ErrorCode::EntityNotFound,
                        "user with id or name was not found",
                        json!({"idOrName":id_or_name}),
                    ),
                ));
            }
        },
    })
    .filter(repository::Column::Private.eq(false))
    .order_by(
        repository::Column::Id,
        match order_by {
            Ordering::Ascending => Order::Asc,
            Ordering::Descending => Order::Desc,
        },
    )
    .paginate(&cx.pool, per_page as u64);

    let pages = paginator.num_pages().await.map_err(api::system_failure)?;
    let entries = paginator
        .fetch_page(cmp::min(0, page as u64))
        .await
        .map_err(api::system_failure)?
        .into_iter()
        .map(Into::<Repository>::into)
        .collect::<Vec<_>>();

    let mut link_hdr = String::new();
    util::build_link_header(
        &mut link_hdr,
        BuildLinkHeaderOpts {
            entries: entries.len(),
            current: page,
            per_page,
            max_pages: pages,
            resource: cx
                .config
                .base_url
                .unwrap()
                .join(&format!("/users/{ion}/repositories"))
                .unwrap(),
        },
    )
    .map_err(api::system_failure)?;

    let mut response = api::ok(StatusCode::OK, entries);
    if !link_hdr.is_empty() {
        response = response.with_header(header::LINK, HeaderValue::from_bytes(link_hdr.as_bytes()).unwrap());
    }

    Ok(response)
}

/// Lists all of this user's repositories.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    get,
    path = "/v1/users/@me/repositories",
    operation_id = "listMyRepositories",
    tag = "Repositories"
)]
pub async fn list_self_user_repositories(
    State(cx): State<Context>,
    Extension(Session { user, .. }): Extension<Session>,
    Query(PaginationRequest {
        per_page,
        order_by,
        page,
    }): Query<PaginationRequest>,
) -> api::Result<Vec<Repository>> {
    let per_page = clamp(per_page, 10, 100).unwrap_or(10);
    let paginator = RepositoryEntity::find()
        .filter(repository::Column::Owner.eq(user.id))
        .filter(repository::Column::Private.eq(true))
        .order_by(
            repository::Column::Id,
            match order_by {
                Ordering::Ascending => Order::Asc,
                Ordering::Descending => Order::Desc,
            },
        )
        .paginate(&cx.pool, per_page as u64);

    let pages = paginator.num_pages().await.map_err(api::system_failure)?;
    let entries = paginator
        .fetch_page(cmp::min(0, page as u64))
        .await
        .map_err(api::system_failure)?
        .into_iter()
        .map(Into::<Repository>::into)
        .collect::<Vec<_>>();

    let mut link_hdr = String::new();
    util::build_link_header(
        &mut link_hdr,
        BuildLinkHeaderOpts {
            entries: entries.len(),
            current: page,
            per_page,
            max_pages: pages,
            resource: cx.config.base_url.unwrap().join("/users/@me/repositories").unwrap(),
        },
    )
    .map_err(api::system_failure)?;

    let mut response = api::ok(StatusCode::OK, entries);
    if !link_hdr.is_empty() {
        response = response.with_header(header::LINK, HeaderValue::from_bytes(link_hdr.as_bytes()).unwrap());
    }

    Ok(response)
}

/// Creates a repository under this user.
#[utoipa::path(
    put,
    path = "/v1/users/@me/repositories",
    operation_id = "createRepository",
    tag = "Repositories"
)]
#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn create_user_repository() {}

/*
/// Create a repository with the current authenticated user as the owner of the repository
#[controller(
    method = put,
    tags("Repositories"),
    requestBody("Payload for creating a repository", ("application/json", schema!("CreateRepositoryPayload"))),
    response(201, "Repository created", ("application/json", response!("RepositoryResponse"))),
    response(400, "Bad Request", ("application/json", response!("ApiErrorResponse"))),
    response(409, "Conflict: repository with that name already exists on the user's account", ("application/json", response!("ApiErrorResponse"))),
    response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
)]
pub async fn create_user_repository(
    State(Instance {
        controllers,
        snowflake,
        pool,
        ..
    }): State<Instance>,
    Extension(Session { user, .. }): Extension<Session>,
    Json(payload): Json<CreateRepositoryPayload>,
) -> Result<Repository> {
    validate(&payload, CreateRepositoryPayload::validate)?;

    match sqlx::query_as::<Postgres, Repository>(
        "select repositories.id from repositories where name = $1 and owner = $2;",
    )
    .bind(&payload.name)
    .bind(user.id)
    .fetch_optional(&pool)
    .await
    {
        Ok(None) => {}
        Ok(Some(_)) => {
            return Err(err(
                StatusCode::CONFLICT,
                (
                    ErrorCode::EntityAlreadyExists,
                    "repository with given name already exists on your account",
                    json!({"name":payload.name}),
                ),
            ))
        }

        Err(e) => {
            error!(error = %e, user.id, %payload.name, "unable to find a user repository with name");
            sentry::capture_error(&e);

            return Err(internal_server_error());
        }
    }

    let id = snowflake.generate();
    let now = Local::now();
    let repo = Repository {
        description: payload.description.clone(),
        created_at: now,
        updated_at: now,
        private: payload.private,
        r#type: payload.r#type,
        owner: user.id,
        name: payload.name.clone(),
        id: i64::try_from(id.value()).unwrap(),

        ..Default::default()
    };

    controllers
        .repositories
        .create(payload, &repo)
        .await
        .map(|_| ok(StatusCode::CREATED, repo))
        .map_err(|_| internal_server_error())
}
*/
