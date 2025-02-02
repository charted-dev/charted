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
    extract::{Json, Path, Query},
    middleware::session::{Middleware, Session},
    openapi::ApiErrorResponse,
    NameOrUlid, ServerContext,
};
use axum::{extract::State, handler::Handler, http::StatusCode, routing, Extension, Router};
use charted_core::{
    api,
    bitflags::{ApiKeyScope, ApiKeyScopes},
    rand_string,
};
use charted_database::{
    paginate::Paginated,
    schema::{postgresql, sqlite},
};
use charted_types::{
    payloads::apikey::{CreateApiKeyPayload, PatchApiKeyPayload},
    ApiKey,
};
use chrono::Local;
use serde_json::json;
use tower_http::auth::AsyncRequireAuthorizationLayer;
use tracing::{error, instrument};

crate::macros::impl_list_response!(ListApiKeyResponse as "ApiKey");

pub fn create_router() -> Router<ServerContext> {
    Router::new()
        .route(
            "/",
            routing::get(list.layer(AsyncRequireAuthorizationLayer::new(
                Middleware::default().scopes([ApiKeyScope::ApiKeyList]),
            )))
            .put(create.layer(AsyncRequireAuthorizationLayer::new(
                Middleware::default().scopes([ApiKeyScope::ApiKeyCreate]),
            ))),
        )
        .route(
            "/:idOrName",
            routing::get(get.layer(AsyncRequireAuthorizationLayer::new(
                Middleware::default().scopes([ApiKeyScope::ApiKeyView]),
            )))
            .patch(patch.layer(AsyncRequireAuthorizationLayer::new(
                Middleware::default().scopes([ApiKeyScope::ApiKeyUpdate]),
            )))
            .delete(delete.layer(AsyncRequireAuthorizationLayer::new(
                Middleware::default().scopes([ApiKeyScope::ApiKeyDelete]),
            ))),
        )
}

/// Lists all the user's API keys avaliable.
#[utoipa::path(
    get,
    path = "/v1/users/@me/apikeys",
    operation_id = "listAPIKeys",
    tag = "API Keys",
    responses(
        (
            status = 200,
            description = "Successful request",
            body = ListApiKeyResponse,
            content_type = "application/json"
        )
    )
)]
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[instrument(name = "charted.server.ops.user.listAPIKeys", skip_all, fields(user.name = %user.username, %user.id))]
pub async fn list(
    State(ctx): State<ServerContext>,
    Extension(Session { user, .. }): Extension<Session>,
) -> api::Result<Vec<ApiKey>> {
    let mut conn = ctx
        .pool
        .get()
        .inspect_err(|e| {
            sentry::capture_error(e);
            tracing::error!(error = %e, "failed to get db connection");
        })
        .map_err(|e| api::system_failure(eyre::Report::from(e)))?;

    let apikeys = charted_database::connection!(@raw conn {
        PostgreSQL(conn) => conn.build_transaction().read_only().run::<Vec<ApiKey>, diesel::result::Error, _>(|txn| {
            use postgresql::api_keys::{dsl, table};
            use diesel::pg::Pg;

            table.select(<ApiKey as SelectableHelper<Pg>>::as_select())
                .filter(dsl::owner.eq(&user.id))
                .load(txn)
        });

        SQLite(conn) => conn.immediate_transaction::<_, diesel::result::Error, _>(|txn| {
            use sqlite::api_keys::{dsl, table};
            use diesel::sqlite::Sqlite;

            table.select(<ApiKey as SelectableHelper<Sqlite>>::as_select())
                .filter(dsl::owner.eq(&user.id))
                .load(txn)
        });
    })
    .inspect_err(|e| {
        sentry::capture_error(e);
        error!(error = %e, "failed to query API key");
    })
    .map_err(|e| api::system_failure(eyre::Report::from(e)))?;

    Ok(api::ok(
        StatusCode::OK,
        apikeys.into_iter().map(|x| x.sanitize()).collect::<Vec<_>>(),
    ))
}

/// Retrieve a single API key's metadata
#[utoipa::path(
    get,
    path = "/v1/users/@me/apikeys/{idOrName}",
    operation_id = "getAPIKey",
    tag = "API Keys",
    responses(
        (
            status = 200,
            description = "Successful request",
            body = api::Response<ApiKey>,
            content_type = "application/json"
        ),
        (
            status = 404,
            description = "API key was not found",
            body = ApiErrorResponse,
            content_type = "application/json"
        )
    )
)]
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[instrument(name = "charted.server.ops.user.getAPIKey", skip_all, fields(user.name = %user.username, %user.id, apikey.name = %id_or_name))]
pub async fn get(
    State(ctx): State<ServerContext>,
    Extension(Session { user, .. }): Extension<Session>,
    Path(id_or_name): Path<NameOrUlid>,
) -> api::Result<ApiKey> {
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
}

/// Generate an API key from the current authenticated user.
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
    responses(
        (
            status = 201,
            description = "API key was created",
            body = api::Response<ApiKey>,
            content_type = "application/json"
        ),
        (
            status = 409,
            description = "If the API key with the name is already registered under the user",
            body = ApiErrorResponse,
            content_type = "application/json"
        )
    )
)]
#[instrument(
    name = "charted.server.ops.v1.createApiKey",
    skip_all,
    fields(
        apikey.owner = %user.username,
        apikey.name = %name,
    )
)]
#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn create(
    State(ctx): State<ServerContext>,
    Extension(Session { user, .. }): Extension<Session>,
    Json(CreateApiKeyPayload {
        name,
        description,
        expires_in: _,
        scopes,
    }): Json<CreateApiKeyPayload>,
) -> api::Result<ApiKey> {
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
}

/// Patch metadata about a API key.
#[utoipa::path(
    patch,
    path = "/v1/users/@me/apikeys/{idOrName}",
    operation_id = "createAPIKey",
    tag = "API Keys"
)]
#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn patch(
    State(ctx): State<ServerContext>,
    Extension(Session { user, .. }): Extension<Session>,
    Path(id_or_name): Path<NameOrUlid>,
    Json(PatchApiKeyPayload { description, name, .. }): Json<PatchApiKeyPayload>,
) -> api::Result<()> {
    todo!()
}

/// Wipes the API key off the system.
#[utoipa::path(
    delete,
    path = "/v1/users/@me/apikeys/{idOrName}",
    operation_id = "createAPIKey",
    tag = "API Keys"
)]
#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn delete(
    State(ctx): State<ServerContext>,
    Extension(Session { user, .. }): Extension<Session>,
    Path(id_or_name): Path<NameOrUlid>,
) -> api::Result<()> {
    todo!()
}
