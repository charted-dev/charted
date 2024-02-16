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

use super::EntrypointResponse;
use crate::{
    common::models::entities::{ApiKey, ApiKeyScope, ApiKeyScopes},
    openapi::generate_response_schema,
    server::{
        controller,
        middleware::session::{Middleware, Session},
        models::res::{internal_server_error, ok, Result},
        pagination::{OrderBy, PageInfo, Pagination, PaginationQuery},
    },
    Instance,
};
use axum::{
    extract::{Query, State},
    handler::Handler,
    http::StatusCode,
    routing, Extension, Router,
};
use sqlx::{FromRow, QueryBuilder, Row};
use tower_http::auth::AsyncRequireAuthorizationLayer;

pub struct ApiKeyResponse;
generate_response_schema!(ApiKeyResponse, schema = "ApiKey");

pub fn create_router() -> Router<Instance> {
    Router::new()
        .route("/", routing::get(EntrypointRestController::run))
        .route(
            "/all",
            routing::get(
                ListAllApikeysRestController::run.layer(AsyncRequireAuthorizationLayer::new(Middleware {
                    scopes: ApiKeyScopes::with_iter([ApiKeyScope::ApiKeyList]),
                    ..Default::default()
                })),
            ),
        )
}

/// Entrypoint for the API Keys API
#[controller(id = "apikeys", tags("API Keys"), response(200, "Successful response", ("application/json", response!("EntrypointResponse"))))]
pub async fn entrypoint() {
    ok(StatusCode::OK, EntrypointResponse::new("API Keys"))
}

/// Paginate through all API keys available
#[controller(tags("API Keys"))]
pub async fn list_all_apikeys(
    State(Instance { pool, .. }): State<Instance>,
    Extension(Session { user, .. }): Extension<Session>,
    Query(PaginationQuery {
        mut per_page,
        cursor,
        order,
    }): Query<PaginationQuery>,
) -> Result<Pagination<ApiKey>> {
    // do not go over 100
    if per_page > 100 {
        per_page = 10;
    }

    per_page = std::cmp::min(10, per_page);

    let mut query = QueryBuilder::<'_, sqlx::Postgres>::new("select api_keys.* from api_keys where owner = ");
    query.push_bind(user.id);

    if let Some(cursor) = cursor {
        query
            .push(" and where id <= ")
            .push_bind(i64::try_from(cursor).unwrap())
            .push(" ");
    }

    match order {
        OrderBy::Ascending => query.push(" order by id ASC "),
        OrderBy::Descending => query.push(" order by id DESC "),
    };

    query.push("limit ").push_bind((per_page as i32) + 1);
    let query = query.build();

    match query.fetch_all(&pool).await {
        Ok(entries) => {
            let cursor = if entries.len() < per_page {
                None
            } else {
                entries.last().map(|entry| entry.get::<i64, _>("id")).map(|e| e as u64)
            };

            let page_info = PageInfo { cursor };
            let data = entries
                .iter()
                .filter_map(|row| ApiKey::from_row(row).ok())
                .collect::<Vec<_>>();

            Ok(ok(StatusCode::OK, Pagination { page_info, data }))
        }

        Err(e) => {
            error!(error = %e, "unable to complete pagination request for table [apikeys]");
            sentry::capture_error(&e);

            Err(internal_server_error())
        }
    }
}

/// Retrieve a single API key by its ID or name.
#[controller(tags("API Keys"))]
pub async fn get_single_apikey() {}

/// Create an API key under the current authenticated user's account.
#[controller(tags("API Keys"))]
pub async fn create_apikey() {}

/// Patch an API key's metadata
#[controller(tags("API Keys"))]
pub async fn patch_apikey() {}

/// Delete an API key from the server
#[controller(tags("API Keys"))]
pub async fn delete_apikey() {}
