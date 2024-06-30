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
use crate::ServerContext;
use axum::Router;
use charted_core::response::{ok, ApiResponse};
use charted_proc_macros::{controller, generate_response_schema};
use reqwest::StatusCode;

pub struct ApiKeyResponse;
generate_response_schema!(ApiKeyResponse, schema = "ApiKey");

pub fn create_router() -> Router<ServerContext> {
    Router::new()
}

#[controller {
    id = "apikeys",
    tags("API Keys"),
    response(200, "Successful response", ("application/json", response!("EntrypointResponse")))
}]
pub async fn entrypoint() -> ApiResponse<EntrypointResponse> {
    ok(StatusCode::OK, EntrypointResponse::new("apikeys"))
}

/*
/// Entrypoint for the API Keys API
#[controller(id = "apikeys", tags("API Keys"), response(200, "Successful response", ("application/json", response!("EntrypointResponse"))))]
pub async fn entrypoint() {
    ok(StatusCode::OK, EntrypointResponse::new("API Keys"))
}

/// Paginate through all API keys available
#[controller(
    tags("API Keys"),
    securityRequirements(
        ("ApiKey", ["apikeys:list"]),
        ("Bearer", []),
        ("Basic", [])
    ),
    response(200, "Successful response", ("application/json", response!("ApiKeyPaginatedResponse"))),
    response(400, "Unable to process request query parameters", ("application/json", response!("ApiErrorResponse"))),
    response(401, "Unauthorized to process the given session details or if the JWT token had expired", ("application/json", response!("ApiErrorResponse"))),
    response(403, "Received an invalid password from the `Basic` authorization scheme", ("application/json", response!("ApiErrorResponse"))),
    response(406, "Unable to process the session due to some unexpected outcome", ("application/json", response!("ApiErrorResponse"))),
    response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
)]
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
#[controller(
    tags("API Keys"),
    securityRequirements(
        ("ApiKey", ["apikeys:view"]),
        ("Bearer", []),
        ("Basic", [])
    ),
    response(200, "Successful response", ("application/json", response!("ApiKeyResponse"))),
    response(400, "Unable to process request query parameters", ("application/json", response!("ApiErrorResponse"))),
    response(401, "Unauthorized to process the given session details or if the JWT token had expired", ("application/json", response!("ApiErrorResponse"))),
    response(403, "Received an invalid password from the `Basic` authorization scheme", ("application/json", response!("ApiErrorResponse"))),
    response(404, "Entity was not found", ("application/json", response!("ApiErrorResponse"))),
    response(406, "Unable to process the session due to some unexpected outcome", ("application/json", response!("ApiErrorResponse"))),
    response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
)]
pub async fn get_single_apikey(
    State(Instance { pool, .. }): State<Instance>,
    Extension(Session { user, .. }): Extension<Session>,
    charted_server::extract::NameOrSnowflake(nos): charted_server::extract::NameOrSnowflake,
) -> Result<ApiKey> {
    validate(&nos, NameOrSnowflake::validate)?;
    let mut query = QueryBuilder::<sqlx::Postgres>::new("select api_keys.* from api_keys");

    match nos {
        NameOrSnowflake::Snowflake(id) => query.push(" where id = $1").push_bind(i64::try_from(id).unwrap()),
        NameOrSnowflake::Name(ref name) => query.push(" where name = $1").push_bind(name),
    };

    query.push(" and owner = $2").push_bind(user.id);

    let query = query.build();
    match query.fetch_optional(&pool).await {
        Ok(Some(apikey)) => Ok(ok(
            StatusCode::OK,
            ApiKey::from_row(&apikey)
                .inspect_err(|e| {
                    error!(apikey.idOrName = %nos, error = %e, "unable to convert postgres row ~> ApiKey");
                    sentry::capture_error(&e);
                })
                .map_err(|_| internal_server_error())?,
        )),
        Ok(None) => Err(err(
            StatusCode::NOT_FOUND,
            (
                ErrorCode::EntityNotFound,
                "unable to find api key",
                json!({"idOrName":nos}),
            ),
        )),

        Err(e) => {
            error!(apikey.idOrName = %nos, error = %e, "unable to query api key from db");
            sentry::capture_error(&e);

            Err(internal_server_error())
        }
    }
}

/// Create an API key under the current authenticated user's account.
#[controller(
    tags("API Keys"),
    securityRequirements(
        ("ApiKey", ["apikeys:create"]),
        ("Bearer", []),
        ("Basic", [])
    ),
    response(201, "Created API key successfully", ("application/json", response!("ApiKeyResponse"))),
    response(401, "Unauthorized to process the given session details or if the JWT token had expired", ("application/json", response!("ApiErrorResponse"))),
    response(403, "Received an invalid password from the `Basic` authorization scheme", ("application/json", response!("ApiErrorResponse"))),
    response(406, "Unable to process the session due to some unexpected outcome", ("application/json", response!("ApiErrorResponse"))),
    response(409, "API Key with the name already exists on your account", ("application/json", response!("ApiErrorResponse"))),
    response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
)]
pub async fn create_apikey(
    State(Instance {
        controllers, snowflake, ..
    }): State<Instance>,
    Extension(Session { user, .. }): Extension<Session>,
    Json(payload): Json<CreateApiKeyPayload>,
) -> Result<ApiKey> {
    validate(&payload, CreateApiKeyPayload::validate)?;

    // check if the api key under the name already exists
    match controllers
        .apikeys
        .get_by(NameOrSnowflake::Name(payload.name.clone()))
        .await
    {
        Ok(None) => {}
        Ok(Some(_)) => {
            return Err(err(
                StatusCode::CONFLICT,
                (
                    ErrorCode::EntityAlreadyExists,
                    "apikey already exists under this account",
                    json!({"name":payload.name, "user":user.username}),
                ),
            ))
        }

        Err(_) => return Err(internal_server_error()),
    }

    let scopes = ApiKeyScopes::with_iter(payload.scopes.clone());
    let token = rand_string(16);
    let now = Local::now();
    let id = snowflake.generate();
    let apikey = ApiKey {
        description: payload.description.clone(),
        created_at: now,
        updated_at: now,
        expires_in: None,
        scopes: scopes.max().try_into().unwrap(),
        token: Some(token),
        owner: user.id,
        name: payload.name.clone(),
        id: id.value().try_into().unwrap(),
    };

    controllers
        .apikeys
        .create(payload, &apikey)
        .await
        .map(|_| ok(StatusCode::CREATED, apikey))
        .map_err(|_| internal_server_error())
}

/// Patch an API key's metadata
#[controller(tags("API Keys"))]
pub async fn patch_apikey() {}

/// Delete an API key from the server
#[controller(tags("API Keys"))]
pub async fn delete_apikey() {}

*/
