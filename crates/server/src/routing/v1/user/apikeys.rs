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
    middleware::authn::{self, Options, Session},
    modify_property,
    openapi::ApiErrorResponse,
    pagination::{Ordering, PaginationRequest},
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
use charted_types::{ApiKey, NameOrUlid, payloads::CreateApiKeyPayload};
use sea_orm::{
    ColumnTrait, EntityTrait, IntoActiveModel, Order, PaginatorTrait, QueryFilter, QueryOrder, sqlx::types::chrono,
};
use serde_json::json;
use std::{cmp, collections::BTreeMap};
use utoipa::{
    IntoResponses, ToResponse,
    openapi::{Ref, RefOr, Response},
};

pub fn create_router(cx: &Context) -> Router<Context> {
    Router::new()
        .route(
            "/",
            routing::get(list.layer(authn::new(
                cx.clone(),
                Options::default().with_scope(ApiKeyScope::ApiKeyList),
            ))),
        )
        .route(
            "/{idOrName}",
            routing::get(fetch.layer(authn::new(
                cx.clone(),
                Options::default().with_scope(ApiKeyScope::ApiKeyView),
            ))),
        )
}

struct AllApiKeysR;
impl IntoResponses for AllApiKeysR {
    fn responses() -> BTreeMap<String, RefOr<Response>> {
        azalia::btreemap! {
            "200" => Ref::from_response_name("ListApiKeyResponse"),
            "5XX" => {
                let mut response = extract_refor_t!(ApiErrorResponse::response().1);
                modify_property!(response; description("Internal Server Failure"));

                response
            }
        }
    }
}

/// Lists all the avaliable API keys this user has.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[instrument(name = "charted.server.ops.list[apikeys]", skip_all)]
#[utoipa::path(
    get,
    path = "/v1/users/@me/apikeys",
    operation_id = "getAllApiKeys",
    tag = "API Keys",
    params(PaginationRequest),
    responses(AllApiKeysR)
)]
pub async fn list(
    State(cx): State<Context>,
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
        .order_by(apikey::Column::Id, match order_by {
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
        .map(Into::<ApiKey>::into)
        .collect::<Vec<_>>();

    let mut link_hdr = String::new();
    util::build_link_header(&mut link_hdr, BuildLinkHeaderOpts {
        entries: entries.len(),
        current: page,
        per_page,
        max_pages: pages,
        resource: cx.config.base_url.unwrap().join("/users/@me/apikeys").unwrap(),
    })
    .map_err(api::system_failure)?;

    let mut response = api::ok(StatusCode::OK, entries);
    if !link_hdr.is_empty() {
        response = response.with_header(header::LINK, HeaderValue::from_bytes(link_hdr.as_bytes()).unwrap());
    }

    Ok(response)
}

struct SingleApiKeyR;
impl IntoResponses for SingleApiKeyR {
    fn responses() -> BTreeMap<String, RefOr<Response>> {
        azalia::btreemap! {
            "200" => Ref::from_response_name("ApiKeyResponse"),
            "404" => {
                let mut response = extract_refor_t!(ApiErrorResponse::response().1);
                modify_property!(response; description("API key was not found."));

                response
            },

            "5XX" => {
                let mut response = extract_refor_t!(ApiErrorResponse::response().1);
                modify_property!(response; description("Internal Server Failure"));

                response
            }
        }
    }
}

#[cfg_attr(debug_assertions, axum::debug_handler)]
#[instrument(name = "charted.server.ops.fetch[apikeys]", skip_all)]
#[utoipa::path(
    get,
    path = "/v1/users/@me/apikeys/{idOrName}",
    operation_id = "getSingleApiKey",
    tag = "API Keys",
    params(NameOrUlid),
    responses(SingleApiKeyR)
)]
pub async fn fetch(
    State(cx): State<Context>,
    Extension(Session { user, .. }): Extension<Session>,
    Path(id_or_name): Path<NameOrUlid>,
) -> api::Result<ApiKey> {
    let Some(apikey) = (match id_or_name.clone() {
        NameOrUlid::Name(name) => ApiKeyEntity::find().filter(apikey::Column::Name.eq(name)),
        NameOrUlid::Ulid(id) => ApiKeyEntity::find_by_id(id),
    })
    .filter(apikey::Column::Owner.eq(user.id))
    .one(&cx.pool)
    .await
    .map_err(api::system_failure)?
    .map(Into::<ApiKey>::into) else {
        return Err(api::err(
            StatusCode::NOT_FOUND,
            (
                api::ErrorCode::EntityNotFound,
                "api key with name or id was not found",
                json!({"idOrName":id_or_name}),
            ),
        ));
    };

    Ok(api::ok(StatusCode::OK, apikey))
}

pub struct CreateApiKeyR;
impl IntoResponses for CreateApiKeyR {
    fn responses() -> BTreeMap<String, RefOr<Response>> {
        azalia::btreemap! {
            "201" => Ref::from_response_name("ApiKeyResponse"),
            "409" => {
                let mut response = extract_refor_t!(ApiErrorResponse::response().1);
                modify_property!(response; description("API key already exists"));

                response
            },

            "5XX" => {
                let mut response = extract_refor_t!(ApiErrorResponse::response().1);
                modify_property!(response; description("Internal Server Failure"));

                response
            }
        }
    }
}

#[cfg_attr(debug_assertions, axum::debug_handler)]
#[instrument(name = "charted.server.ops.create[apikey]", skip_all)]
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
    State(cx): State<Context>,
    Extension(Session { user, .. }): Extension<Session>,
    Json(CreateApiKeyPayload {
        display_name,
        description,
        expires_in,
        scopes,
        name,
    }): Json<CreateApiKeyPayload>,
) -> api::Result<ApiKey> {
    if ApiKeyEntity::find()
        .filter(apikey::Column::Name.eq(name.clone()))
        .filter(apikey::Column::Owner.eq(user.id))
        .one(&cx.pool)
        .await
        .inspect_err(|e| {
            error!(error = %e, apikey.name = %name, owner = %user.id, "failed to find apikey by name");
            sentry::capture_error(e);
        })
        .map_err(api::system_failure)?
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

    let id = cx
        .ulid_generator
        .generate()
        .inspect_err(|e| {
            error!(error = %e, apikey.name = %name, "received error when generating id for apikey");
            sentry::capture_error(e);
        })
        .map_err(api::system_failure)?;

    let now = chrono::Utc::now();
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
        .exec(&cx.pool)
        .await
        .inspect_err(|e| {
            error!(error = %e, apikey.name = %name, apikey.owner = %user.id, "failed to create api key");
            sentry::capture_error(e);
        })
        .map_err(api::system_failure)?;

    Ok(api::ok(StatusCode::CREATED, model.into()))
}

struct PatchApiKeyR;
impl IntoResponses for PatchApiKeyR {
    fn responses() -> BTreeMap<String, RefOr<Response>> {
        azalia::btreemap! {
            "204" => Ref::from_response_name("EmptyApiResponse"),
            "409" => {
                let mut response = extract_refor_t!(ApiErrorResponse::response().1);
                modify_property!(response; description("Failed to apply patches"));

                response
            },

            "5XX" => {
                let mut response = extract_refor_t!(ApiErrorResponse::response().1);
                modify_property!(response; description("Internal Server Failure"));

                response
            }
        }
    }
}

/// Patches a API key's metadata.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[instrument(name = "charted.server.ops.patch[apikey]", skip_all)]
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
pub async fn patch() -> api::Result<()> {
    todo!()
}

struct DeleteApiKeyR;
impl IntoResponses for DeleteApiKeyR {
    fn responses() -> BTreeMap<String, RefOr<Response>> {
        azalia::btreemap! {
            "204" => Ref::from_response_name("EmptyApiResponse"),
            "5XX" => {
                let mut response = extract_refor_t!(ApiErrorResponse::response().1);
                modify_property!(response; description("Internal Server Failure"));

                response
            }
        }
    }
}

/// Deletes an API key.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[instrument(name = "charted.server.ops.delete[apikey]", skip_all)]
#[utoipa::path(
    delete,
    path = "/v1/users/@me/apikeys/{idOrName}",
    operation_id = "deleteAPIKey",
    tag = "API Keys",
    params(NameOrUlid),
    responses(DeleteApiKeyR)
)]
pub async fn delete() -> api::Result<()> {
    todo!()
}
