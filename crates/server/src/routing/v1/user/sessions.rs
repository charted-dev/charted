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

#[utoipa::path(
    post,
    path = "/v1/users/@me/sessions",
    operation_id = "login",
    tag = "Users/Sessions"
)]
#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn login() {}

#[utoipa::path(
    delete,
    path = "/v1/users/@me/sessions",
    operation_id = "logout",
    tag = "Users/Sessions"
)]
#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn logout() {}

#[utoipa::path(
    post,
    path = "/v1/users/@me/sessions/refresh",
    operation_id = "refreshSessionToken",
    tag = "Users/Sessions"
)]
#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn refresh_session_token() {}

/*
use crate::{
    authz, db::controllers::DbController, openapi::generate_response_schema, server::validation::validate,
    sessions::Session, Instance,
};
use axum::{extract::State, http::StatusCode, Extension};
use charted_entities::payloads::UserLoginPayload;
use charted_server::{controller, err, extract::Json, internal_server_error, ok, ErrorCode, Result};
use serde_json::json;
use sqlx::Postgres;
use validator::Validate;

pub struct SessionResponse;
generate_response_schema!(SessionResponse, schema = "Session");

/// Creates a new session and returns details about the newly created session.
#[controller(
    method = post,
    tags("Users", "Sessions"),
    requestBody("Payload for creating a new user. `password` can be empty if the server's session manager is not Local", ("application/json", schema!("UserLoginPayload"))),
    response(201, "Successful response", ("application/json", response!("SessionResponse"))),
    response(400, "Invalid payload received.", ("application/json", response!("ApiErrorResponse"))),
    response(403, "Invalid password received", ("application/json", response!("ApiErrorResponse"))),
    response(404, "Unknown User", ("application/json", response!("ApiErrorResponse"))),
    response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
)]
pub async fn login(
    State(Instance {
        controllers,
        sessions,
        authz,
        pool,
        ..
    }): State<Instance>,
    Json(payload): Json<UserLoginPayload>,
) -> Result<Session> {
    validate(&payload, UserLoginPayload::validate)?;

    let user = match (payload.username, payload.email) {
        (Some(ref username), None) => match controllers.users.get_by(username).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                return Err(err(
                    StatusCode::NOT_FOUND,
                    (
                        ErrorCode::EntityNotFound,
                        "user with username doesn't exist",
                        json!({"username": username}),
                    ),
                ))
            }

            Err(_) => return Err(internal_server_error()),
        },

        (None, Some(ref email)) => match sqlx::query_as::<Postgres, _>("select users.* from users where email = $1;")
            .bind(email)
            .fetch_optional(&pool)
            .await
        {
            Ok(Some(user)) => user,
            Ok(None) => {
                return Err(err(
                    StatusCode::NOT_FOUND,
                    (
                        ErrorCode::EntityNotFound,
                        "user with username doesn't exist",
                        json!({"email": email}),
                    ),
                ))
            }

            Err(e) => {
                error!(error = %e, "unable to query user by email");
                sentry::capture_error(&e);

                return Err(internal_server_error());
            }
        },

        (Some(_), Some(_)) => {
            return Err(err(
                StatusCode::BAD_REQUEST,
                (
                    ErrorCode::InvalidJsonPayload,
                    "`username` and `email` are mutually exclusive",
                ),
            ))
        }

        (None, None) => {
            return Err(err(
                StatusCode::BAD_REQUEST,
                (
                    ErrorCode::InvalidJsonPayload,
                    "either `username` or `email` needs to be available",
                ),
            ))
        }
    };

    // check if we can authenticate
    authz
        .authenticate(user.clone(), payload.password)
        .await
        .map_err(|e| match e {
            authz::Error::InvalidPassword => err(
                StatusCode::FORBIDDEN,
                (ErrorCode::InvalidPassword, "password given was not correct"),
            ),

            authz::Error::Eyre(e) => {
                error!(error = %e, user.id, "unable to complete authentication from authz backend");
                sentry_eyre::capture_report(&e);

                internal_server_error()
            }

            authz::Error::Ldap(e) => {
                error!(error = %e, user.id, "unable to complete authentication from LDAP authz backend");
                sentry::capture_error(&e);

                internal_server_error()
            }
        })?;

    let mut sessions = sessions.lock().await;
    let session = sessions.create(user).await.map_err(|_| internal_server_error())?;
    sessions.create_task(session.session, std::time::Duration::from_secs(604800));

    Ok(ok(StatusCode::CREATED, session))
}

/// Destroy the current authenticated session.
#[controller(
    method = delete,
    tags("Users", "Sessions"),
    response(201, "Session was deleted successfully", ("application/json", response!("EmptyApiResponse"))),
    response(403, "If the authenticated user didn't provide a session token", ("application/json", response!("ApiErrorResponse"))),
    response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
)]
pub async fn destroy_session(
    State(Instance { sessions, .. }): State<Instance>,
    Extension(crate::server::middleware::session::Session { session, .. }): Extension<
        crate::server::middleware::session::Session,
    >,
) -> Result<()> {
    let Some(session) = session else {
        return Err(err(
            StatusCode::FORBIDDEN,
            (
                ErrorCode::SessionOnlyRoute,
                "this REST route requires only session tokens to be used",
                json!({"method": "delete", "uri": "/users/sessions/logout"}),
            ),
        ));
    };

    let mut mgr = sessions.lock().await;
    mgr.kill(session.session)
        .map(|_| ok(StatusCode::ACCEPTED, ()))
        .map_err(|e| {
            sentry_eyre::capture_report(&e);
            internal_server_error()
        })
}

/// Refresh a session with the given refresh token upon creation.
#[controller(
    method = post,
    tags("Users", "Sessions"),
    response(201, "Session was fully restored with a new one", ("application/json", response!("SessionResponse"))),
    response(403, "If the authenticated user didn't provide a refresh token", ("application/json", response!("ApiErrorResponse"))),
    response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
)]
pub async fn refresh_session_token(
    State(Instance { sessions, .. }): State<Instance>,
    Extension(crate::server::middleware::session::Session { session, user, .. }): Extension<
        crate::server::middleware::session::Session,
    >,
) -> Result<Session> {
    let Some(session) = session else {
        return Err(err(
            StatusCode::FORBIDDEN,
            (
                ErrorCode::SessionOnlyRoute,
                "this REST route requires only session tokens to be used",
                json!({"method": "post", "uri": "/users/sessions/refresh"}),
            ),
        ));
    };

    let mut mgr = sessions.lock().await;
    mgr.kill(session.session).map_err(|e| {
        sentry_eyre::capture_report(&e);
        internal_server_error()
    })?;

    // create a new session since the old one is destroyed
    mgr.create(user)
        .await
        .map(|sess| {
            mgr.create_task(sess.session, std::time::Duration::from_secs(604800));
            ok(StatusCode::CREATED, sess)
        })
        .map_err(|e| {
            sentry_eyre::capture_report(&e);
            internal_server_error()
        })
}
*/
