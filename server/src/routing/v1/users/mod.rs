// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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
    extract::Json,
    middleware::{Session, SessionAuth},
    models::res::{err, no_content, ok, ApiResponse},
    openapi::gen_response_schema,
    validation::{validate, validate_email},
    Server,
};
use axum::{extract::State, handler::Handler, http::StatusCode, routing, Extension, Router};
use charted_common::{
    extract::NameOrSnowflake,
    models::{
        entities::User,
        payloads::{CreateUserPayload, PatchUserPayload},
        Name,
    },
    server::hash_password,
    VERSION,
};
use charted_proc_macros::controller;
use chrono::Local;
use sqlx::QueryBuilder;
use tower_http::auth::AsyncRequireAuthorizationLayer;
use utoipa::{
    openapi::path::{PathItem, PathItemBuilder},
    ToSchema,
};
use validator::Validate;

macro_rules! impl_patch_for {
    ($txn:expr, $entry:literal, $id:expr, $payload:expr => $value:expr) => {
        if let Some(val) = $payload {
            match sqlx::query(concat!("update users set ", $entry, " = $1 where id = $2;"))
                .bind($value)
                .bind($id)
                .execute(&mut *$txn)
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    ::tracing::error!(user.id = $id, error = %e, concat!("unable to update [", $entry, "] to [{}] for user"), val);
                    ::sentry::capture_error(&e);

                    drop($txn);
                    return Err($crate::models::res::err(
                        ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
                    ));
                }
            }
        }
    };

    ($txn:expr, $entry:literal, $id:expr, $value:expr) => {
        if let Some(val) = $value {
            match sqlx::query(concat!("update users set ", $entry, " = $1 where id = $2;"))
                .bind(val.clone())
                .bind($id)
                .execute(&mut *$txn)
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    ::tracing::error!(user.id = $id, error = %e, concat!("unable to update [", $entry, "] to [{}] for user"), val);
                    ::sentry::capture_error(&e);

                    drop($txn);
                    return Err($crate::models::res::err(
                        ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
                    ));
                }
            }
        }
    };
}

pub fn create_router() -> Router<Server> {
    Router::new()
        .route(
            "/",
            routing::get(MainRestController::run)
                .put(CreateUserRestController::run)
                .patch(PatchUserRestController::run.layer(AsyncRequireAuthorizationLayer::new(SessionAuth)))
                .delete(DeleteUserRestController::run.layer(AsyncRequireAuthorizationLayer::new(SessionAuth))),
        )
        .route(
            "/@me",
            routing::get(GetSelfRestController::run.layer(AsyncRequireAuthorizationLayer::new(SessionAuth))),
        )
        .route("/:idOrName", routing::get(GetUserRestController::run))
}

pub fn paths() -> PathItem {
    let mut paths = PathItemBuilder::new();
    let operations = vec![
        MainRestController::paths().operations.pop_first().unwrap(),
        CreateUserRestController::paths().operations.pop_first().unwrap(),
        PatchUserRestController::paths().operations.pop_first().unwrap(),
    ];

    for (item, op) in operations.iter() {
        paths = paths.operation(item.clone(), op.clone());
    }

    paths.build()
}

/// Generic entrypoint route for the Users API.
#[controller(id = "users", tags("Users"), response(200, "Successful response", ("application/json", response!("ApiEntrypointResponse"))))]
async fn main() {
    ok(
        StatusCode::OK,
        EntrypointResponse {
            message: "Welcome to the Users API".into(),
            docs: format!("https://charts.noelware.org/docs/server/{VERSION}/api/reference/users"),
        },
    )
}

// please do not use this directly
#[derive(ToSchema)]
pub struct UserResponse;
gen_response_schema!(UserResponse, schema: "User");

/// Creates a new user if the server allows registrations.
#[controller(
    method = put,
    tags("Users"),
    requestBody("Payload for creating a new user. `password` can be empty if the server's session manager is not Local", ("application/json", schema!("CreateUserPayload"))),
    response(200, "Successful response", ("application/json", response!("ApiUserResponse"))),
    response(403, "Whether if this server doesn't allow registrations", ("application/json", response!("ApiErrorResponse"))),
    response(406, "If the `username` or `email` was taken.", ("application/json", response!("ApiErrorResponse")))
)]
async fn create_user(
    State(server): State<Server>,
    Json(payload): Json<CreateUserPayload>,
) -> Result<ApiResponse<User>, ApiResponse> {
    if !server.config.registrations {
        return Err(err(
            StatusCode::FORBIDDEN,
            (
                "REGISTRATIONS_DISABLED",
                "This instance is not allowing registrations at this given moment.",
            )
                .into(),
        ));
    }

    if payload.password.is_none() {
        return Err(err(
            StatusCode::NOT_ACCEPTABLE,
            ("MISSING_PASSWORD", "Missing the password to create with.").into(),
        ));
    }

    let username = validate(payload.username, Name::validate)?;
    match sqlx::query("SELECT users.* FROM users WHERE username = $1;")
        .bind(username.as_str())
        .fetch_optional(&server.pool)
        .await
    {
        Ok(None) => {}
        Ok(Some(_)) => {
            return Err(err(
                StatusCode::CONFLICT,
                (
                    "USER_EXISTS",
                    format!("user with username {username} already exists!").as_str(),
                )
                    .into(),
            ))
        }

        Err(e) => {
            error!(username = tracing::field::display(username.clone()), %e, "failed to query user {username}");
            sentry::capture_error(&e);

            return Err(err(
                StatusCode::INTERNAL_SERVER_ERROR,
                ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
            ));
        }
    }

    let email = validate_email(payload.email)?;
    match sqlx::query("select users.* from users where email = $1;")
        .bind(email.clone())
        .fetch_optional(&server.pool)
        .await
    {
        Ok(None) => {}
        Ok(Some(_)) => {
            return Err(err(
                StatusCode::CONFLICT,
                ("USER_EXISTS", "user with email received already exists").into(),
            ))
        }

        Err(e) => {
            error!(email, %e, "failed to query user with");
            sentry::capture_error(&e);

            return Err(err(
                StatusCode::INTERNAL_SERVER_ERROR,
                ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
            ));
        }
    }

    let password = payload.password.unwrap();
    if password.len() < 8 {
        return Err(err(
            StatusCode::NOT_ACCEPTABLE,
            (
                "PASSWORD_LENGTH_NOT_ACCEPTED",
                "password was expected to be 8 or more characters long",
            )
                .into(),
        ));
    }

    let id = server.snowflake.generate();
    let password = charted_common::server::hash_password(password).map_err(|_| {
        err(
            StatusCode::INTERNAL_SERVER_ERROR,
            ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
        )
    })?;

    let user = User {
        created_at: Local::now(),
        updated_at: Local::now(),
        password: Some(password),
        username,
        email,
        id: id.value() as i64,

        ..Default::default()
    };

    match sqlx::query(
        "insert into users(created_at, updated_at, password, username, email, id) values($1, $2, $3, $4, $5, $6);",
    )
    .bind(user.created_at)
    .bind(user.updated_at)
    .bind(user.password.clone())
    .bind(user.username.clone())
    .bind(user.email.clone())
    .bind(user.id)
    .execute(&server.pool)
    .await
    {
        Ok(_) => Ok(ok(StatusCode::OK, user)),
        Err(e) => {
            error!(error = %e, "unable to insert user @{} ({}):", user.username, user.id);
            sentry::capture_error(&e);

            Err(err(
                StatusCode::INTERNAL_SERVER_ERROR,
                ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
            ))
        }
    }
}

/// Retrieve a [`User`] object.
#[controller(
    tags("Users"),
    response(200, "Successful response", ("application/json", response!("ApiUserResponse"))),
    response(400, "Invalid `idOrName` specified", ("application/json", response!("ApiErrorResponse"))),
    response(404, "Unknown User", ("application/json", response!("ApiErrorResponse"))),
    pathParameter("idOrName", schema!("NameOrSnowflake"), description = "Path parameter that can take a [`Name`] or [`Snowflake`] identifier.")
)]
pub async fn get_user(
    State(server): State<Server>,
    NameOrSnowflake(id_or_name): NameOrSnowflake,
) -> Result<ApiResponse<User>, ApiResponse> {
    let mut query = QueryBuilder::<sqlx::Postgres>::new("select users.* from users where");
    match id_or_name {
        charted_common::models::NameOrSnowflake::Snowflake(_) => {
            query.push(" id = ");
        }

        charted_common::models::NameOrSnowflake::Name(_) => {
            query.push(" username = ");
        }
    }

    query.push("$1;");

    let mut query = sqlx::query_as::<sqlx::Postgres, User>(query.sql());
    match id_or_name.clone() {
        charted_common::models::NameOrSnowflake::Snowflake(id) => query = query.bind(id as i64),
        charted_common::models::NameOrSnowflake::Name(name) => query = query.bind(name.to_string()),
    };

    match query.fetch_optional(&server.pool).await {
        Ok(Some(user)) => Ok(ok(StatusCode::OK, user)),
        Ok(None) => Err(err(
            StatusCode::NOT_FOUND,
            (
                "UNKNOWN_USER",
                format!("User with ID or name [{id_or_name}] was not found.").as_str(),
            )
                .into(),
        )),

        Err(e) => {
            error!(idOrName = tracing::field::display(id_or_name), error = %e, "unable to query user with");
            sentry::capture_error(&e);

            Err(err(
                StatusCode::INTERNAL_SERVER_ERROR,
                ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
            ))
        }
    }
}

/// Returns a [User] from an authenticated request.
#[controller(
    tags("Users"),
    response(200, "Returns the current authenticated user's metadata", ("application/json", response!("ApiUserResponse"))),
    response(400, "If the request body was invalid (i.e, validation errors)", ("application/json", response!("ApiErrorResponse"))),
    response(401, "If the session couldn't be validated", ("application/json", response!("ApiErrorResponse"))),
    response(403, "(Bearer token only) - if the JWT was invalid.", ("application/json", response!("ApiErrorResponse"))),
    response(406, "If the request body contained invalid data, or if the session header contained invalid data", ("application/json", response!("ApiErrorResponse"))),
    response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
)]
pub async fn get_self(Extension(Session { user, .. }): Extension<Session>) -> ApiResponse<User> {
    ok(StatusCode::OK, user)
}

/// Patches the current authenticated user's metadata.
#[controller(
    method = patch,
    tags("Users"),
    response(204, "Successful response", ("application/json", response!("ApiEmptyResponse"))),
    response(400, "If the request body was invalid (i.e, validation errors)", ("application/json", response!("ApiErrorResponse"))),
    response(401, "If the session couldn't be validated", ("application/json", response!("ApiErrorResponse"))),
    response(403, "(Bearer token only) - if the JWT was invalid.", ("application/json", response!("ApiErrorResponse"))),
    response(406, "If the request body contained invalid data, or if the session header contained invalid data", ("application/json", response!("ApiErrorResponse"))),
    response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
)]
pub async fn patch_user(
    State(server): State<Server>,
    Extension(Session { user, .. }): Extension<Session>,
    payload: Json<PatchUserPayload>,
) -> Result<ApiResponse, ApiResponse> {
    validate(payload.clone(), PatchUserPayload::validate)?;

    // validate username (since it doesn't validate it D:)
    if let Some(username) = payload.username.clone() {
        validate(username.clone(), Name::validate)?;

        // check if username exists
        match sqlx::query("SELECT users.* FROM users WHERE username = $1;")
            .bind(username.as_str())
            .fetch_optional(&server.pool)
            .await
        {
            Ok(None) => {}
            Ok(Some(_)) => {
                return Err(err(
                    StatusCode::CONFLICT,
                    (
                        "USER_EXISTS",
                        format!("unable to patch: user with username {username} already exists").as_str(),
                    )
                        .into(),
                ))
            }

            Err(e) => {
                error!(username = tracing::field::display(username.clone()), %e, "failed to query user {username}");
                sentry::capture_error(&e);

                return Err(err(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
                ));
            }
        }
    }

    // check if an email already exists by an another user
    if let Some(email) = payload.email.clone() {
        match sqlx::query("select users.* from users where email = $1;")
            .bind(email.clone())
            .fetch_optional(&server.pool)
            .await
        {
            Ok(None) => {}
            Ok(Some(_)) => {
                return Err(err(
                    StatusCode::CONFLICT,
                    ("USER_EXISTS", "user with email received already exists").into(),
                ))
            }

            Err(e) => {
                error!(email, %e, "failed to query user with");
                sentry::capture_error(&e);

                return Err(err(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
                ));
            }
        }
    }

    let span = info_span!("charted.rest.patch", entity = "user", user.id);
    let _guard = span.enter();
    let mut txn = server.pool.begin().await.map_err(|e| {
        error!(user.id, error = %e, "unable to create database transaction for user");
        sentry::capture_error(&e);

        err(
            StatusCode::INTERNAL_SERVER_ERROR,
            ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
        )
    })?;

    impl_patch_for!(txn, "gravatar_email", user.id, payload.gravatar_email.clone());
    impl_patch_for!(txn, "description", user.id, payload.description.clone());
    impl_patch_for!(txn, "username", user.id, payload.username.clone());
    impl_patch_for!(txn, "password", user.id, payload.password.clone() => {
        hash_password(payload.password.clone().unwrap()).map_err(|e| {
            error!(user.id, error = %e, "unable to hash password for user ");
            sentry_eyre::capture_report(&e);

            err(
                StatusCode::INTERNAL_SERVER_ERROR,
                ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
            )
        })?
    });

    impl_patch_for!(txn, "email", user.id, payload.email.clone());
    impl_patch_for!(txn, "name", user.id, payload.name.clone());

    txn.commit().await.map_err(|e| {
        error!(user.id, error = %e, "unable to commit transaction for user");
        sentry::capture_error(&e);

        err(
            StatusCode::INTERNAL_SERVER_ERROR,
            ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
        )
    })?;

    Ok(no_content())
}

#[controller(
    method = delete,
    tags("Users"),
    response(204, "Successful response", ("application/json", response!("ApiEmptyResponse"))),
    response(401, "If the session couldn't be validated", ("application/json", response!("ApiErrorResponse"))),
    response(403, "(Bearer token only) - if the JWT was invalid or expired", ("application/json", response!("ApiErrorResponse"))),
    response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
)]
pub async fn delete_user(
    State(server): State<Server>,
    Extension(Session {
        user,
        session: _session,
    }): Extension<Session>,
) -> Result<ApiResponse, ApiResponse> {
    match sqlx::query("delete from users where id = $1;")
        .bind(user.id)
        .execute(&server.pool)
        .await
    {
        Ok(_) => {}
        Err(e) => {
            error!(user.id, error = %e, "unable to delete user");
            sentry::capture_error(&e);

            return Err(err(
                StatusCode::INTERNAL_SERVER_ERROR,
                ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
            ));
        }
    }

    // in a background task, delete all entities and helm charts
    // that existed with that user.
    tokio::spawn(async move {
        let _helm_charts = server.helm_charts.clone();
        let _pool = server.pool.clone();
        let _user = user.clone();
    });

    Ok(no_content())
}
