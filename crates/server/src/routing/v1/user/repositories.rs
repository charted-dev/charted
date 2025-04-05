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
    Context, commit_patch,
    extract::{Json, Path, Query},
    extract_refor_t,
    middleware::authn::{self, Options, Session},
    modify_property,
    openapi::{ApiErrorResponse, EmptyApiResponse},
    pagination::{Ordering, PaginationRequest},
    util::{self, BuildLinkHeaderOpts},
};
use axum::{
    Extension, Router,
    extract::State,
    handler::Handler,
    http::{
        StatusCode,
        header::{self, HeaderValue},
    },
    routing,
};
use azalia::remi::{
    core::{StorageService as _, UploadRequest},
    fs,
};
use charted_core::{api, bitflags::ApiKeyScope, clamp};
use charted_database::entities::{RepositoryEntity, UserEntity, repository, user};
use charted_types::{
    NameOrUlid, Repository, User,
    payloads::{CreateRepositoryPayload, PatchRepositoryPayload},
};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, Order, PaginatorTrait, QueryFilter,
    QueryOrder,
};
use serde_json::json;
use std::{borrow::Cow, cmp, collections::BTreeMap};
use utoipa::{
    IntoResponses, ToResponse,
    openapi::{Ref, RefOr, Response},
};

pub fn create_router(cx: &Context) -> Router<Context> {
    Router::new()
        .route(
            "/repositories",
            routing::get(
                list_self_user_repositories.layer(authn::new(
                    cx.to_owned(),
                    Options::default()
                        .with_scope(ApiKeyScope::RepoAccess)
                        .with_scope(ApiKeyScope::UserAccess),
                )),
            )
            .put(
                create_user_repository.layer(authn::new(
                    cx.to_owned(),
                    Options::default()
                        .with_scope(ApiKeyScope::RepoCreate)
                        .with_scope(ApiKeyScope::UserAccess),
                )),
            ),
        )
        .route(
            "/repositories/{idOrName}",
            routing::patch(patch_user_repository.layer(authn::new(
                cx.to_owned(),
                Options::default().with_scope(ApiKeyScope::RepoUpdate),
            ))),
        )
}

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

struct PatchRepoR;
impl IntoResponses for PatchRepoR {
    fn responses() -> BTreeMap<String, RefOr<Response>> {
        azalia::btreemap! {
            "204" => {
                let mut response = extract_refor_t!(EmptyApiResponse::response().1);
                modify_property!(response.description("Patch was successful"));

                response
            },

            "4XX" => {
                let mut response = extract_refor_t!(ApiErrorResponse::response().1);
                modify_property!(response.description("Any occurrence when authentication fails or if the patch couldn't be applied"));

                response
            }
        }
    }
}

#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    patch,

    path = "/v1/users/@me/repositories/{idOrName}",
    operation_id = "patchRepository",
    tag = "Users",
    request_body(
        content_type = "application/json",
        description = "Payload object for patching repository metadata",
        content = ref("#/components/schemas/PatchRepositoryPayload")
    ),
    responses(PatchRepoR),
    params(NameOrUlid),
    security(
        ("ApiKey" = ["repo:update"])
    )
)]
pub async fn patch_user_repository(
    State(cx): State<Context>,
    Extension(Session { user, .. }): Extension<Session>,
    Path(id_or_name): Path<NameOrUlid>,
    Json(PatchRepositoryPayload {
        description,
        private,
        readme,
        name,
        ty,
    }): Json<PatchRepositoryPayload>,
) -> api::Result<()> {
    let (mut model, repo) = match id_or_name {
        NameOrUlid::Name(ref name) => match RepositoryEntity::find()
            .filter(repository::Column::Name.eq(name.clone()))
            .filter(repository::Column::Owner.eq(user.id))
            .one(&cx.pool)
            .await
            .map_err(api::system_failure)?
        {
            Some(repo) => (repo.clone().into_active_model(), Repository::from(repo)),
            None => {
                return Err(api::err(
                    StatusCode::NOT_FOUND,
                    (
                        api::ErrorCode::EntityNotFound,
                        "repository with name doesn't exist",
                        json!({"idOrName":id_or_name}),
                    ),
                ));
            }
        },

        NameOrUlid::Ulid(id) => match RepositoryEntity::find_by_id(id)
            .filter(repository::Column::Owner.eq(user.id))
            .one(&cx.pool)
            .await
            .map_err(api::system_failure)?
        {
            Some(repo) => (repo.clone().into_active_model(), Repository::from(repo)),
            None => {
                return Err(api::err(
                    StatusCode::NOT_FOUND,
                    (
                        api::ErrorCode::EntityNotFound,
                        "repository with id doesn't exist",
                        json!({"idOrName":id_or_name}),
                    ),
                ));
            }
        },
    };

    let mut errors = Vec::new();
    commit_patch!(model of string?: old.description => description; validate that len < 140 [errors]);
    commit_patch!(model of bool: old.private => private; [repo]);

    if let Some(name) = name {
        if RepositoryEntity::find()
            .filter(repository::Column::Name.eq(name.clone()))
            .filter(repository::Column::Owner.eq(user.id))
            .one(&cx.pool)
            .await
            .map_err(api::system_failure)?
            .is_some()
        {
            errors.push(api::Error {
                code: api::ErrorCode::EntityAlreadyExists,
                message: Cow::Borrowed("an existing repository already exists with that name"),
                details: Some(json!({
                    "path": "name",
                    "repository": &name
                })),
            });
        } else {
            model.name = ActiveValue::set(name);
        }
    }

    commit_patch!(model of bool: old.type_ => ty; [repo]);

    if let Some(readme) = readme.as_deref() {
        // if it exceeds 16kib, don't allow it
        if readme.len() >= 16382 {
            errors.push(api::Error::from((
                api::ErrorCode::InvalidBody,
                "readme exceeds 16kib of data",
            )));
        }
    }

    if !errors.is_empty() {
        return Err(api::Response {
            headers: axum::http::HeaderMap::new(),
            success: false,
            status: StatusCode::CONFLICT,
            errors,
            data: None::<()>,
        });
    }

    model.updated_at = ActiveValue::set(Utc::now());

    if let Err(e) = model.update(&cx.pool).await {
        error!(error = %e, repository.name = %repo.name, repository.owner = %user.id, "failed to commit changes");
        sentry::capture_error(&e);

        return Err(api::system_failure(e));
    }

    if let Some(readme) = readme {
        let ct = fs::default_resolver(readme.as_bytes());

        if let Err(e) = cx
            .storage
            .upload(
                format!("./repositories/{}/README", repo.id),
                UploadRequest::default().with_content_type(Some(ct)).with_data(readme),
            )
            .await
        {
            error!(error = %e, repository.name = %repo.name, repository.owner = %user.id, "failed to upload README; will be tried again on next request of /repositories/{}/readme", repo.id);
            sentry::capture_error(&e);
        }
    }

    Ok(api::from_default(StatusCode::ACCEPTED))
}

/// Deletes this repository.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    delete,

    path = "/v1/users/@me/repositories/{idOrName}",
    operation_id = "deleteRepository",
    tags = ["Users", "Repositories"],
    params(NameOrUlid),
    responses(
        (
            status = 204,
            description = "Repository has been deleted",
            body = EmptyApiResponse,
            content_type = "application/json"
        )
    )
)]
pub async fn delete(
    State(cx): State<Context>,
    Extension(Session { user, .. }): Extension<Session>,
    Path(id_or_name): Path<NameOrUlid>,
) -> api::Result<()> {
    let repository = (match id_or_name {
        NameOrUlid::Name(ref name) => RepositoryEntity::find().filter(repository::Column::Name.eq(name.clone())),
        NameOrUlid::Ulid(id) => RepositoryEntity::find_by_id(id),
    })
    .filter(repository::Column::Owner.eq(user.id))
    .one(&cx.pool)
    .await
    .inspect_err(|e| {
        error!(error = %e, repository = %id_or_name, owner = %user.id, "failed to query repository");
        sentry::capture_error(e);
    })
    .map_err(api::system_failure)?
    .ok_or_else(|| {
        api::err(
            StatusCode::NOT_FOUND,
            (
                api::ErrorCode::EntityNotFound,
                "repository with id or name doesn't exist",
                json!({"idOrName": id_or_name}),
            ),
        )
    })?;

    // Delete the repository
    RepositoryEntity::delete_by_id(repository.id)
        .exec(&cx.pool)
        .await
        .map_err(api::system_failure)?;

    Ok(api::from_default(StatusCode::ACCEPTED))
}
