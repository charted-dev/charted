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
    extract::Query,
    extract_refor_t,
    middleware::sessions::{Middleware, Session},
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
use charted_core::{api, bitflags::ApiKeyScope, clamp};
use charted_database::entities::{ApiKeyEntity, apikey};
use charted_types::{ApiKey, NameOrUlid};
use sea_orm::{ColumnTrait, EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder};
use std::{cmp, collections::BTreeMap};
use tower_http::auth::AsyncRequireAuthorizationLayer;
use utoipa::{
    IntoResponses, ToResponse,
    openapi::{Ref, RefOr, Response},
};

pub fn create_router() -> Router<Context> {
    Router::new().route(
        "/",
        routing::get(list.layer(AsyncRequireAuthorizationLayer::new(
            Middleware::default().with_scope(ApiKeyScope::ApiKeyList),
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
pub async fn fetch() -> api::Result<Option<ApiKey>> {
    todo!()
}

/*
    let mut conn = ctx
        .pool
        .get()
        .inspect_err(|e| {
            sentry::capture_error(e);
            error!(error = %e, "failed to get db connection");
        })
        .map_err(|e| api::system_failure::<eyre::Report>(e.into()))?;

    let Some(apikey) = charted_database::connection!(@raw conn {
        PostgreSQL(conn) => conn.build_transaction().read_only().run::<_, diesel::result::Error, _>(|txn| {
            use postgresql::api_keys::{dsl, table};
            use diesel::pg::Pg;

            let mut query = table
                .into_boxed()
                .select(<ApiKey as SelectableHelper<Pg>>::as_select())
                .filter(dsl::owner.eq(&user.id));

            query = match &id_or_name {
                NameOrUlid::Name(name) => query.filter(dsl::name.eq(name)),
                NameOrUlid::Ulid(id) => query.filter(dsl::id.eq(id))
            };

            match query.first(txn) {
                Ok(apikey) => Ok(Some(apikey)),
                Err(diesel::result::Error::NotFound) => Ok(None),
                Err(e) => Err(e),
            }
        });

        SQLite(conn) => conn.immediate_transaction(|txn| {
            use sqlite::api_keys::{dsl, table};
            use diesel::sqlite::Sqlite;

            let mut query = table
                .into_boxed()
                .select(<ApiKey as SelectableHelper<Sqlite>>::as_select())
                .filter(dsl::owner.eq(&user.id));

            query = match &id_or_name {
                NameOrUlid::Name(name) => query.filter(dsl::name.eq(name)),
                NameOrUlid::Ulid(id) => query.filter(dsl::id.eq(id)),
            };

            match query.first(txn) {
                Ok(apikey) => Ok(Some(apikey)),
                Err(diesel::result::Error::NotFound) => Ok(None),
                Err(e) => Err(e),
            }
        });
    })
    .inspect_err(|e| {
        sentry::capture_error(e);
        error!(error = %e, "failed to query api key");
    })
    .map_err(|e| api::system_failure::<eyre::Report>(e.into()))?
    else {
        return Err(api::err(
            StatusCode::NOT_FOUND,
            (
                api::ErrorCode::EntityNotFound,
                "api key with given ID or name doesn't exist",
                json!({
                    "idOrName": id_or_name
                }),
            ),
        ));
    };

    Ok(api::ok(StatusCode::OK, apikey.sanitize()))
*/

pub struct CreateApiKeyR;
impl IntoResponses for CreateApiKeyR {
    fn responses() -> BTreeMap<String, RefOr<Response>> {
        azalia::btreemap! {
            "200" => Ref::from_response_name("ApiKeyResponse"),
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
pub async fn create() -> api::Result<ApiKey> {
    todo!()
}

/*
    let mut conn = ctx
        .pool
        .get()
        .inspect_err(|e| {
            sentry::capture_error(e);
            error!(error = %e, "failed to get db connection");
        })
        .map_err(|e| api::system_failure::<eyre::Report>(e.into()))?;

    let exists = charted_database::connection!(@raw conn {
        PostgreSQL(conn) => conn.build_transaction().read_only().run(|txn| {
            use postgresql::api_keys::{dsl, table};
            use diesel::pg::Pg;

            let query = table
                .select(<ApiKey as SelectableHelper<Pg>>::as_select())
                .filter(dsl::owner.eq(&user.username))
                .filter(dsl::name.eq(&name));

            match query.first(txn) {
                Ok(_) => Ok(true),
                Err(diesel::result::Error::NotFound) => Ok(false),
                Err(e) => Err(e)
            }
        });

        SQLite(conn) => conn.immediate_transaction(|txn| {
            use sqlite::api_keys::{dsl, table};
            use diesel::sqlite::Sqlite;

            let query = table
                .select(<ApiKey as SelectableHelper<Sqlite>>::as_select())
                .filter(dsl::owner.eq(&user.username))
                .filter(dsl::name.eq(&name));

            match query.first(txn) {
                Ok(_) => Ok(true),
                Err(diesel::result::Error::NotFound) => Ok(false),
                Err(e) => Err(e)
            }
        });
    })
    .inspect_err(|e| {
        sentry::capture_error(e);
        error!(error = %e, "failed to query api key with given name and owner");
    })
    .map_err(|e| api::system_failure::<eyre::Report>(e.into()))?;

    if exists {
        return Err(api::err(
            StatusCode::CONFLICT,
            (
                api::ErrorCode::EntityAlreadyExists,
                "api key with name already exists",
                json!({"name": name.as_str()}),
            ),
        ));
    }

    let scopes = scopes.into_iter().collect::<ApiKeyScopes>();
    let token = rand_string(16);
    let id = ctx
        .ulid_gen
        .generate()
        .inspect_err(|e| {
            sentry::capture_error(e);
            error!("received monotonic overflow -- please inspect this as fast you can!!!!!");
        })
        .map_err(api::system_failure)?;

    let now: charted_types::DateTime = chrono::DateTime::from(Local::now()).into();
    let key = ApiKey {
        description,
        created_at: now,
        updated_at: now,
        expires_in: None,
        scopes: scopes.value().try_into().unwrap(),
        token,
        owner: user.id,
        name,
        id: id.into(),
    };

    charted_database::connection!(@raw conn {
        PostgreSQL(conn) => conn.build_transaction().read_write().run(|txn| {
            use postgresql::api_keys::table;

            diesel::insert_into(table).values(&key).execute(txn)
        });

        SQLite(conn) => conn.immediate_transaction(|txn| {
            use sqlite::api_keys::table;

            diesel::insert_into(table).values(&key).execute(txn)
        });
    })
    .inspect_err(|e| {
        sentry::capture_error(e);
        error!(error = %e, "failed to insert api key into database");
    })
    .map_err(|_| api::internal_server_error())?;

    // TODO(@auguwu): register api key to a scheduled background job
    // to be deleted within today + `expires_in`

    Ok(api::ok(StatusCode::CREATED, key))
*/

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
