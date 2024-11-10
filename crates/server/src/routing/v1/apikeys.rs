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

use super::Entrypoint;
use crate::{extract::Json, middleware::session::Session, openapi::ApiErrorResponse, ServerContext};
use axum::{extract::State, http::StatusCode, routing, Extension, Router};
use charted_core::{api, bitflags::ApiKeyScopes, rand_string};
use charted_database::schema::{postgresql, sqlite};
use charted_types::{payloads::apikey::CreateApiKeyPayload, ApiKey};
use chrono::Local;
use eyre::Context;
use serde_json::json;
use tracing::{error, instrument};

pub fn create_router() -> Router<ServerContext> {
    Router::new().route("/", routing::get(entrypoint))
}

/// Entrypoint response for the `/apikeys` route.
#[utoipa::path(
    get,
    path = "/v1/apikeys",
    operation_id = "apikeys",
    tag = "API Keys",
    responses(
        (
            status = 200,
            description = "Entrypoint response",
            body = api::Response<Entrypoint>,
            content_type = "application/json"
        )
    )
)]
#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn entrypoint() -> api::Response<Entrypoint> {
    api::ok(StatusCode::OK, Entrypoint::new("API Keys"))
}

/// Generate an API key from the current authenticated user.
#[utoipa::path(
    put,
    path = "/v1/apikeys",
    operation_id = "createApiKey",
    tag = "API Keys",
    request_body(
        content = ref("CreateApiKeyPayload"),
        description = "Request body for creating a new API key.",
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
    State(cx): State<ServerContext>,
    Extension(Session { user, .. }): Extension<Session>,
    Json(CreateApiKeyPayload {
        name,
        description,
        expires_in,
        scopes,
    }): Json<CreateApiKeyPayload>,
) -> api::Result<ApiKey> {
    // check if an apikey already exists under the user
    let mut conn = cx
        .pool
        .get()
        .inspect_err(|e| {
            sentry::capture_error(e);
            error!(error = %e, "failed to get connection");
        })
        .map_err(|_| api::internal_server_error())?;

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
                Err(e) => Err(eyre::Report::from(e))
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
                Err(e) => Err(eyre::Report::from(e))
            }
        });
    })
    .inspect_err(|e| {
        sentry_eyre::capture_report(e);
        error!(error = %e, "failed to query api key with given name and owner");
    })
    .map_err(|_| api::internal_server_error())?;

    if exists {
        return Err(api::err(
            StatusCode::CONFLICT,
            (
                api::ErrorCode::EntityAlreadyExists,
                "api key with given name on this account already exists",
                json!({"user": user.username.as_str(), "apikey": name.as_str()}),
            ),
        ));
    }

    let scopes = scopes.into_iter().collect::<ApiKeyScopes>();
    let token = rand_string(16);
    let id = cx
        .ulid_gen
        .generate()
        .inspect_err(|e| {
            sentry::capture_error(e);
            error!("received monotonic overflow -- please inspect this as fast you can!!!!!");
        })
        .map_err(|_| api::internal_server_error())?;

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

            diesel::insert_into(table).values(&key).execute(txn).context("failed to insert api key into database")
        });

        SQLite(conn) => conn.immediate_transaction(|txn| {
            use sqlite::api_keys::table;

            diesel::insert_into(table).values(&key).execute(txn).context("failed to insert api key into database")
        });
    });

    todo!()
}
