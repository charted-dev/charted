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
    Env, commit_patch,
    ext::{DataStoreExt, ResultExt},
    extract::{Json, Path, Query},
    middleware::authn::Session,
    mk_into_responses,
    openapi::{EmptyApiResponse, ListRepositoryResponse, RepositoryResponse},
    ops::db,
    pagination::PaginationRequest,
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
use charted_database::entities::{RepositoryEntity, repository};
use charted_datastore::{
    fs,
    remi::{StorageService, UploadRequest},
};
use charted_types::{
    NameOrUlid, Repository,
    payloads::{CreateRepositoryPayload, PatchRepositoryPayload},
};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, PaginatorTrait, QueryFilter,
    QueryOrder,
};
use serde_json::json;
use std::{borrow::Cow, cmp};

struct ListRepositoriesR;
mk_into_responses!(for ListRepositoriesR {
    "200" => [ref(ListRepositoryResponse)];
});

/// Lists all the avaliable user repositories.
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
    State(env): State<Env>,
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
        NameOrUlid::Name(name) => match db::user::get(&env.db, NameOrUlid::Name(name)).await? {
            Some(user) => RepositoryEntity::find().filter(repository::Column::Owner.eq(user.id)),
            None => {
                return Err(api::err(
                    StatusCode::NOT_FOUND,
                    (
                        api::ErrorCode::EntityNotFound,
                        "user with id or name was not found",
                        json!({"idOrName":&ion}),
                    ),
                ));
            }
        },
    })
    .filter(repository::Column::Private.eq(false))
    .order_by(repository::Column::Id, order_by.into_sea_orm())
    .paginate(&env.db, per_page as u64);

    let pages = paginator.num_pages().await.into_system_failure()?;
    let entries = paginator
        .fetch_page(cmp::min(0, page as u64))
        .await
        .into_system_failure()?
        .into_iter()
        .map(Into::<Repository>::into)
        .collect::<Vec<_>>();

    let mut link_hdr = String::new();
    util::build_link_header(&mut link_hdr, BuildLinkHeaderOpts {
        entries: entries.len(),
        current: page,
        per_page,
        max_pages: pages,
        resource: env
            .config
            .base_url
            .unwrap()
            .join(&format!("/users/{ion}/repositories"))
            .unwrap(),
    })
    .into_system_failure()?;

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
    State(env): State<Env>,
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
        .order_by(repository::Column::Id, order_by.into_sea_orm())
        .paginate(&env.db, per_page as u64);

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
        resource: env.config.base_url.unwrap().join("/users/@me/repositories").unwrap(),
    })
    .into_system_failure()?;

    let mut response = api::ok(StatusCode::OK, entries);
    if !link_hdr.is_empty() {
        response = response.with_header(header::LINK, HeaderValue::from_bytes(link_hdr.as_bytes()).unwrap());
    }

    Ok(response)
}

struct CreateRepositoryR;
mk_into_responses!(for CreateRepositoryR {
    "201" => [ref(RepositoryResponse)];
    "409" => [error(description("repository already exists"))];
});

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
    State(env): State<Env>,
    Extension(Session { user, .. }): Extension<Session>,
    Json(CreateRepositoryPayload {
        description,
        private,
        readme,
        name,
        ty,
    }): Json<CreateRepositoryPayload>,
) -> api::Result<Repository> {
    if db::repository::get_as_model_with_additional_bounds(&env.db, NameOrUlid::Name(name.clone()), |query| {
        query.filter(repository::Column::Owner.eq(user.id))
    })
    .await?
    .is_some()
    {
        return Err(api::err(
            StatusCode::CONFLICT,
            (
                api::ErrorCode::EntityAlreadyExists,
                "repository with the given name already exists on this account",
                json!({"name": &name, "owner": &user.id}),
            ),
        ));
    }

    let readme_repr = if let Some(readme) = readme {
        let content_type = azalia::remi::fs::default_resolver(readme.as_bytes());
        if content_type.starts_with("text/html") || content_type.starts_with("text/plain") {
            Some((readme, content_type))
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

    let id = env.ulid.generate().into_system_failure()?;
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
        .exec(&env.db)
        .await
        .inspect_err(|e| {
            error!(error = %e, repository.name = %name, repository.owner = %user.id, "failed to create repository");
            sentry::capture_error(e);
        })
        .map_err(api::system_failure)?;

    if let Some((content, ty)) = readme_repr {
        use azalia::remi::core::StorageService;

        let ns = env.ds.repositories(model.id);

        // if it ever fails, once a request is inflight for /repositories/:id/README, then
        // we can just re-create it if it doesn't exist.
        //
        // for filesystem, this should always work no matter what
        //
        // for other storages, it could possibly fail so this is why we'll just
        // re-create it later if it fails
        if let Err(e) = ns
            .upload(
                "README",
                UploadRequest::default().with_content_type(Some(ty)).with_data(content),
            )
            .await
        {
            error!(error = %e, repository.name = %model.name, repository.owner = %model.owner, "failed to upload README: will retry again");
            sentry::capture_error(&e);
        }
    }

    Ok(api::ok(StatusCode::CREATED, model.into()))
}

struct PatchRepoR;
mk_into_responses!(for PatchRepoR {
    "204" => [ref(with "application/json" => EmptyApiResponse;
        description("patch was applied to object");
    )];

    "4XX" => [error(description("authentication failures"))];
});

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
    State(env): State<Env>,
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
    let model = db::repository::get_as_model_with_additional_bounds(&env.db, id_or_name.clone(), |query| {
        query.filter(repository::Column::Owner.eq(user.id))
    })
    .await?
    .ok_or_else(|| {
        api::err(
            StatusCode::NOT_FOUND,
            (
                api::ErrorCode::EntityNotFound,
                "repository with name doesn't exist",
                json!({"idOrName":id_or_name}),
            ),
        )
    })?;

    let mut active = model.clone().into_active_model();
    let mut errors = Vec::new();
    commit_patch!(active of string?: old.description => description; validate that len < 140 [errors]);

    if let Some(field) = private {
        active.private.set_if_not_equals(field);
    }

    if let Some(name) = name {
        if db::repository::get_as_model_with_additional_bounds(&env.db, NameOrUlid::Name(name.clone()), |query| {
            query.filter(repository::Column::Owner.eq(user.id))
        })
        .await?
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
            active.name = ActiveValue::set(name);
        }
    }

    if let Some(ty) = ty {
        active.type_.set_if_not_equals(ty);
    }

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

    active.updated_at = ActiveValue::set(Utc::now());
    active.update(&env.db).await.into_system_failure()?;

    if let Some(readme) = readme.as_deref() {
        let content_type = fs::default_resolver(readme.as_bytes());
        let ns = env.ds.repositories(model.id);

        if let Err(e) = ns
            .upload(
                "README",
                UploadRequest::default()
                    .with_content_type(Some(content_type))
                    .with_data(readme.to_owned()),
            )
            .await
        {
            error!(error = %e, repository.name = %model.name, repository.owner = %model.owner, "failed to upload README: will retry again");
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
    State(env): State<Env>,
    Extension(Session { user, .. }): Extension<Session>,
    Path(id_or_name): Path<NameOrUlid>,
) -> api::Result<()> {
    let repository = db::repository::get_with_additional_bounds(&env.db, id_or_name.clone(), |query| {
        query.filter(repository::Column::Owner.eq(user.id))
    })
    .await?
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
        .exec(&env.db)
        .await
        .into_system_failure()?;

    Ok(api::from_default(StatusCode::ACCEPTED))
}
