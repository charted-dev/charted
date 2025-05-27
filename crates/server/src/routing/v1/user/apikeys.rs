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
    ext::ResultExt,
    extract::{Json, Path, Query},
    middleware::authn::{Factory, Options, Session},
    mk_into_responses,
    openapi::{ApiKeyResponse, EmptyApiResponse, ListApiKeyResponse},
    ops::db,
    pagination::PaginationRequest,
    util::{self, BuildLinkHeaderOpts},
};
use axum::{
    Extension, Router,
    extract::State,
    handler::Handler,
    http::{HeaderValue, StatusCode, header},
    routing,
};
use charted_core::{api, bitflags::ApiKeyScope, clamp, rand_string};
use charted_database::entities::{ApiKeyEntity, apikey};
use charted_types::{
    ApiKey, NameOrUlid,
    payloads::{CreateApiKeyPayload, PatchApiKeyPayload},
};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, PaginatorTrait, QueryFilter,
    QueryOrder,
};
use serde_json::json;
use std::{borrow::Cow, cmp};

pub fn create_router(env: &Env) -> Router<Env> {
    Router::new()
        .route(
            "/",
            routing::get(list.layer(env.authn(Options::default().with_scope(ApiKeyScope::ApiKeyList))))
                .put(create.layer(env.authn(Options::default().with_scope(ApiKeyScope::ApiKeyCreate)))),
        )
        .route(
            "/{idOrName}",
            routing::get(fetch.layer(env.authn(Options::default().with_scope(ApiKeyScope::ApiKeyView))))
                .patch(patch.layer(env.authn(Options::default().with_scope(ApiKeyScope::ApiKeyUpdate))))
                .delete(delete.layer(env.authn(Options::default().with_scope(ApiKeyScope::ApiKeyDelete)))),
        )
}

struct AllApiKeysR;
mk_into_responses!(for AllApiKeysR {
    "200" => [ref(ListApiKeyResponse)];
});

/// Lists all the avaliable API keys this user has.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    get,
    path = "/v1/users/@me/apikeys",
    operation_id = "getAllApiKeys",
    tag = "API Keys",
    params(PaginationRequest),
    responses(AllApiKeysR)
)]
pub async fn list(
    State(env): State<Env>,
    Extension(Session { user, .. }): Extension<Session>,
    Query(PaginationRequest {
        per_page,
        order_by,
        page,
    }): Query<PaginationRequest>,
) -> api::Result<Vec<ApiKey>> {
    let per_page = clamp(per_page, 10, 100).unwrap_or(10);
    let paginator = ApiKeyEntity::find()
        .filter(apikey::Column::Owner.eq(user.username))
        .order_by(apikey::Column::Id, order_by.into_sea_orm())
        .paginate(&env.db, per_page as u64);

    let pages = paginator.num_pages().await.map_err(api::system_failure)?;
    let entries = paginator
        .fetch_page(cmp::min(0, page as u64))
        .await
        .map_err(api::system_failure)?
        .into_iter()
        .map(Into::<ApiKey>::into)
        .collect::<Vec<_>>();

    let mut link_hdr = String::new();
    util::build_link_header(&mut link_hdr, BuildLinkHeaderOpts {
        entries: entries.len(),
        current: page,
        per_page,
        max_pages: pages,
        resource: env.config.base_url.unwrap().join("/users/@me/apikeys").unwrap(),
    })
    .into_system_failure()?;

    let mut response = api::ok(StatusCode::OK, entries);
    if !link_hdr.is_empty() {
        response = response.with_header(header::LINK, HeaderValue::from_bytes(link_hdr.as_bytes()).unwrap());
    }

    Ok(response)
}

struct SingleApiKeyR;
mk_into_responses!(for SingleApiKeyR {
    "200" => [ref(ApiKeyResponse)];
    "404" => [error(description("api key was not found"))];
});

#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    get,
    path = "/v1/users/@me/apikeys/{idOrName}",
    operation_id = "getSingleApiKey",
    tag = "API Keys",
    params(NameOrUlid),
    responses(SingleApiKeyR)
)]
pub async fn fetch(
    State(env): State<Env>,
    Extension(Session { user, .. }): Extension<Session>,
    Path(id_or_name): Path<NameOrUlid>,
) -> api::Result<ApiKey> {
    match db::apikey::get_with_additional_bounds(&env.db, id_or_name.clone(), |query| {
        query.filter(apikey::Column::Owner.eq(user.id))
    })
    .await?
    {
        Some(apikey) => Ok(api::ok(StatusCode::OK, apikey)),
        None => Err(api::err(
            StatusCode::NOT_FOUND,
            (
                api::ErrorCode::EntityNotFound,
                "api key with name or id was not found",
                json!({"idOrName":id_or_name}),
            ),
        )),
    }
}

struct CreateApiKeyR;
mk_into_responses!(for CreateApiKeyR {
    "201" => [ref(ApiKeyResponse)];
    "409" => [error(description("api key already exists"))];
});

#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    put,
    path = "/v1/users/@me/apikeys",
    operation_id = "createAPIKey",
    tag = "API Keys",
    request_body(
        content = ref("#/components/schemas/CreateApiKeyPayload"),
        description = "Request body for creating a new API key",
        content_type = "application/json"
    ),
    responses(CreateApiKeyR)
)]
pub async fn create(
    State(env): State<Env>,
    Extension(Session { user, .. }): Extension<Session>,
    Json(CreateApiKeyPayload {
        display_name,
        description,
        expires_in,
        scopes,
        name,
    }): Json<CreateApiKeyPayload>,
) -> api::Result<ApiKey> {
    if db::apikey::get_with_additional_bounds(&env.db, NameOrUlid::Name(name.clone()), |query| {
        query.filter(apikey::Column::Owner.eq(user.id))
    })
    .await?
    .is_some()
    {
        return Err(api::err(
            StatusCode::CONFLICT,
            (
                api::ErrorCode::EntityAlreadyExists,
                "apikey with the given name already exists on this account",
                json!({"name": &name, "owner": &user.id}),
            ),
        ));
    }

    let id = env.ulid.generate().into_system_failure()?;
    let now = Utc::now();
    let token = rand_string(32);
    let model = apikey::Model {
        display_name,
        description,
        expires_in: expires_in.map(Into::into),
        created_at: now,
        updated_at: now,
        scopes,
        owner: user.id,
        token,
        name: name.clone(),
        id: id.into(),
    };

    let active_model = model.clone().into_active_model();
    ApiKeyEntity::insert(active_model)
        .exec(&env.db)
        .await
        .inspect_err(|e| {
            error!(error = %e, apikey.name = %name, apikey.owner = %user.id, "failed to create api key");
            sentry::capture_error(e);
        })
        .map_err(api::system_failure)?;

    Ok(api::ok(StatusCode::CREATED, model.into()))
}

struct PatchApiKeyR;
mk_into_responses!(for PatchApiKeyR {
    "204" => [ref(EmptyApiResponse)];
    "409" => [error(description("failed to apply patch"))];
});

/// Patches a API key's metadata.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    patch,
    path = "/v1/users/@me/apikeys/{idOrName}",
    operation_id = "createAPIKey",
    tag = "API Keys",
    request_body(
        content = ref("#/components/schemas/PatchApiKeyPayload"),
        description = "Request body for patching this API key",
        content_type = "application/json"
    ),
    params(NameOrUlid),
    responses(PatchApiKeyR)
)]
pub async fn patch(
    State(env): State<Env>,
    Extension(Session { user, .. }): Extension<Session>,
    Path(id_or_name): Path<NameOrUlid>,
    Json(PatchApiKeyPayload {
        display_name,
        description,
        scopes: _,
        name,
    }): Json<PatchApiKeyPayload>,
) -> api::Result<()> {
    let mut model = db::apikey::get_as_model(&env.db, id_or_name.clone())
        .await?
        .ok_or_else(|| {
            api::err(
                StatusCode::NOT_FOUND,
                (
                    api::ErrorCode::EntityNotFound,
                    "apikey with either name or id wasn't found",
                    json!({"idOrName":id_or_name}),
                ),
            )
        })?
        .into_active_model();

    let mut errors = Vec::new();
    commit_patch!(model of string?: old.display_name => display_name);
    commit_patch!(model of string?: old.description => description; validate that len < 140 [errors]);

    if let Some(name) = name {
        if db::apikey::get_as_model_with_additional_bounds(&env.db, NameOrUlid::Name(name.clone()), |query| {
            query.filter(apikey::Column::Owner.eq(user.id))
        })
        .await?
        .is_some()
        {
            errors.push(api::Error {
                code: api::ErrorCode::EntityAlreadyExists,
                message: Cow::Borrowed("an existing api key under this account already exists with that name"),
                details: Some(json!({
                    "path": "name",
                    "username": &name
                })),
            });
        } else {
            model.name = ActiveValue::set(name);
        }
    }

    if !errors.is_empty() {
        return Err(api::empty(false, StatusCode::CONFLICT));
    }

    model
        .update(&env.db)
        .await
        .map(|_| api::no_content())
        .into_system_failure()
}

struct DeleteApiKeyR;
mk_into_responses!(for DeleteApiKeyR {
    "200" => [ref(EmptyApiResponse)];
    "404" => [error(description("API key by ID or name was not found"))];
});

/// Deletes an API key.
#[axum::debug_handler]
#[utoipa::path(
    delete,
    path = "/v1/users/@me/apikeys/{idOrName}",
    operation_id = "deleteAPIKey",
    tag = "API Keys",
    params(NameOrUlid),
    responses(DeleteApiKeyR)
)]
pub async fn delete(
    State(env): State<Env>,
    Extension(Session { user, .. }): Extension<Session>,
    Path(id_or_name): Path<NameOrUlid>,
) -> api::Result<()> {
    let ApiKey { id, .. } = db::apikey::get_with_additional_bounds(&env.db, id_or_name.clone(), |query| {
        query.filter(apikey::Column::Owner.eq(user.id))
    })
    .await?
    .ok_or_else(|| {
        api::err(
            StatusCode::NOT_FOUND,
            (
                api::ErrorCode::EntityNotFound,
                "apikey with either name or id wasn't found",
                json!({"idOrName":id_or_name}),
            ),
        )
    })?;

    ApiKeyEntity::delete_by_id(id)
        .exec(&env.db)
        .await
        .map(|_| api::from_default(StatusCode::ACCEPTED))
        .map_err(api::system_failure)
}
