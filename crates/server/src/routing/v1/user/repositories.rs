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
    extract::{Json, Path, Query},
    extract_refor_t,
    middleware::authn::Session,
    modify_property,
    openapi::ApiErrorResponse,
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
use azalia::remi::core::UploadRequest;
use charted_core::{api, clamp};
use charted_database::entities::{RepositoryEntity, UserEntity, repository, user};
use charted_types::{NameOrUlid, Repository, User, payloads::CreateRepositoryPayload};
use chrono::Utc;
use sea_orm::{ColumnTrait, EntityTrait, IntoActiveModel, Order, PaginatorTrait, QueryFilter, QueryOrder};
use serde_json::json;
use std::{cmp, collections::BTreeMap};
use utoipa::{
    IntoResponses, ToResponse,
    openapi::{Ref, RefOr, Response},
};

struct ListRepositoriesR;
impl IntoResponses for ListRepositoriesR {
    fn responses() -> BTreeMap<String, RefOr<Response>> {
        azalia::btreemap! {
            "200" => Ref::from_response_name("ListRepositoryResponse"),
            "5XX" => {
                let mut response = extract_refor_t!(ApiErrorResponse::response().1);
                modify_property!(response.description("Internal Server Failure"));

                response
            }
        }
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
    .order_by(repository::Column::Id, match order_by {
        Ordering::Ascending => Order::Asc,
        Ordering::Descending => Order::Desc,
    })
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
    util::build_link_header(&mut link_hdr, BuildLinkHeaderOpts {
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
    })
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
    tag = "Repositories",
    params(PaginationRequest),
    responses(ListRepositoriesR)
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
        .order_by(repository::Column::Id, match order_by {
            Ordering::Ascending => Order::Asc,
            Ordering::Descending => Order::Desc,
        })
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
    util::build_link_header(&mut link_hdr, BuildLinkHeaderOpts {
        entries: entries.len(),
        current: page,
        per_page,
        max_pages: pages,
        resource: cx.config.base_url.unwrap().join("/users/@me/repositories").unwrap(),
    })
    .map_err(api::system_failure)?;

    let mut response = api::ok(StatusCode::OK, entries);
    if !link_hdr.is_empty() {
        response = response.with_header(header::LINK, HeaderValue::from_bytes(link_hdr.as_bytes()).unwrap());
    }

    Ok(response)
}

struct CreateRepositoryR;
impl IntoResponses for CreateRepositoryR {
    fn responses() -> BTreeMap<String, RefOr<Response>> {
        azalia::btreemap! {
            "201" => Ref::from_response_name("RepositoryResponse"),
            "409" => {
                let mut response = extract_refor_t!(ApiErrorResponse::response().1);
                modify_property!(response.description("repository already exists"));

                response
            },

            "5XX" => {
                let mut response = extract_refor_t!(ApiErrorResponse::response().1);
                modify_property!(response.description("Internal Server Failure"));

                response
            }
        }
    }
}

/// Creates a repository under this user.
#[utoipa::path(
    put,
    path = "/v1/users/@me/repositories",
    operation_id = "createRepository",
    tags = ["Users", "Repositories"],
    responses(CreateRepositoryR),
    request_body(
        content = ref("#/components/schemas/CreateRepositoryPayload"),
        description = "Request body for creating a new repository",
        content_type = "application/json"
    ),
)]
#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn create_user_repository(
    State(cx): State<Context>,
    Extension(Session { user, .. }): Extension<Session>,
    Json(CreateRepositoryPayload {
        description,
        private,
        readme,
        name,
        ty,
    }): Json<CreateRepositoryPayload>,
) -> api::Result<Repository> {
    if RepositoryEntity::find()
        .filter(repository::Column::Name.eq(name.clone()))
        .filter(repository::Column::Owner.eq(user.id))
        .one(&cx.pool)
        .await
        .inspect_err(|e| {
            error!(error = %e, repository.name = %name, repository.owner = %user.id, "failed to find repository by name");
            sentry::capture_error(e);
        })
        .map_err(api::system_failure)?
        .is_some()
    {
        return Err(api::err(
            StatusCode::CONFLICT,
            (
                api::ErrorCode::EntityAlreadyExists,
                "repository with the given name already exists on this account",
                json!({"name": &name, "owner": &user.id}),
            )
        ));
    }

    let readme_repr = if let Some(readme) = readme {
        let content_type = azalia::remi::fs::default_resolver(readme.as_bytes());
        if content_type.starts_with("text/html") {
            Some((readme, ".html", content_type))
        } else if content_type.starts_with("text/plain") {
            Some((readme, ".txt", content_type))
        } else {
            return Err(api::err(
                StatusCode::FORBIDDEN,
                (
                    api::ErrorCode::InvalidBody,
                    "only accepting `text/html` or `text/plain` as the content type for readme",
                    json!({"contentType":content_type}),
                ),
            ));
        }
    } else {
        None
    };

    let id = cx
        .ulid_generator
        .generate()
        .inspect_err(|e| {
            error!(error = %e, apikey.name = %name, "received error when generating id for apikey");
            sentry::capture_error(e);
        })
        .map_err(api::system_failure)?;

    let now = Utc::now();
    let model = repository::Model {
        description,
        deprecated: false,
        created_at: now,
        updated_at: now,
        icon_hash: None,
        private,
        creator: None,
        owner: user.id,
        type_: ty,
        name: name.clone(),
        id: id.into(),
    };

    let active_model = model.clone().into_active_model();
    RepositoryEntity::insert(active_model)
        .exec(&cx.pool)
        .await
        .inspect_err(|e| {
            error!(error = %e, repository.name = %name, repository.owner = %user.id, "failed to create repository");
            sentry::capture_error(e);
        })
        .map_err(api::system_failure)?;

    if let Some((content, ext, ty)) = readme_repr {
        use azalia::remi::core::StorageService;

        // if it ever fails, once a request is inflight for /repositories/:id/README, then
        // we can just re-create it if it doesn't exist.
        //
        // for filesystem, this should always work no matter what
        //
        // for other storages, it could possibly fail so this is why we'll just
        // re-create it later if it fails
        let _ = cx
            .storage
            .upload(
                format!("./repositories/{id}/README{ext}"),
                UploadRequest::default().with_content_type(Some(ty)).with_data(content),
            )
            .await
            .inspect_err(|e| {
                error!(error = %e, repository.id = %id, "failed to upload readme");
                sentry::capture_error(e);
            });
    }

    Ok(api::ok(StatusCode::CREATED, model.into()))
}
