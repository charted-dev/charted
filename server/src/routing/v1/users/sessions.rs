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

use crate::{
    extract::Json,
    macros::controller,
    middleware::SessionAuth,
    models::res::{err, ok, ApiResponse, Empty},
    validation::validate,
    Server,
};
use axum::{extract::State, handler::Handler, http::StatusCode, routing, Extension, Router};
use charted_common::models::{entities::User, payloads::UserLoginPayload, Name};
use charted_config::SessionBackend;
use charted_openapi::generate_response_schema;
use charted_sessions::{Session, SessionProvider};
use serde_json::json;
use sqlx::{query_as, Postgres};
use std::time::Duration;
use tower_http::auth::AsyncRequireAuthorizationLayer;
use validator::Validate;

pub fn create_router() -> Router<Server> {
    Router::new()
        .route(
            "/logout",
            routing::delete(
                LogoutRestController::run.layer(AsyncRequireAuthorizationLayer::new(SessionAuth::default())),
            ),
        )
        .route(
            "/refresh-token",
            routing::post(
                RefreshSessionTokenRestController::run.layer(AsyncRequireAuthorizationLayer::new(
                    SessionAuth::default().require_refresh_token(),
                )),
            ),
        )
}

// this shouldn't be used directly
pub(crate) struct SessionResponse;
generate_response_schema!(SessionResponse, schema = "Session");

/// Creates a new session and returns details about the newly created session.
#[controller(
    method = post,
    tags("Users", "Sessions"),
    response(201, "Successful response", ("application/json", response!("ApiSessionResponse"))),
    response(400, "Invalid payload received.", ("application/json", response!("ApiErrorResponse"))),
    response(403, "Invalid password received", ("application/json", response!("ApiErrorResponse"))),
    response(404, "Unknown User", ("application/json", response!("ApiErrorResponse"))),
    response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
)]
pub async fn login(
    State(Server {
        sessions, pool, config, ..
    }): State<Server>,
    Json(payload): Json<UserLoginPayload>,
) -> Result<ApiResponse<Session>, ApiResponse> {
    // if passwordless is the session backend, then /users/login is no longer
    // available, and you will need to use the /users/passwordless/authenticate
    // REST endpoint instead.
    if let SessionBackend::Passwordless = config.sessions.backend {
        return Err(err(
            StatusCode::NOT_FOUND,
            (
                "HANDLER_NOT_FOUND",
                "Route was not found",
                json!({
                    "method": "post",
                    "url": "/users/login"
                }),
            )
                .into(),
        ));
    }

    let UserLoginPayload {
        password,
        username,
        email,
    } = validate(payload, UserLoginPayload::validate)?;

    if username.is_none() && email.is_none() {
        return Err(err(
            StatusCode::BAD_REQUEST,
            ("INVALID_PAYLOAD", "either `username` or `email` need to be available").into(),
        ));
    }

    if username.is_some() && email.is_some() {
        return Err(err(
            StatusCode::BAD_REQUEST,
            ("INVALID_PAYLOAD", "`username` and `email` cannot be used together.").into(),
        ));
    }

    if let Some(username) = username.clone() {
        validate(username, Name::validate)?;
    }

    let query = match (username.clone(), email.clone()) {
        (Some(ref username), None) => {
            query_as::<Postgres, User>("select users.* from users where username = $1").bind(username.to_string())
        }

        (None, Some(ref email)) => {
            query_as::<Postgres, User>("select users.* from users where email = $1").bind(email.to_string())
        }

        _ => unreachable!(),
    };

    let user = match query.fetch_optional(&pool).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Err(err(
                StatusCode::NOT_FOUND,
                ("UNKNOWN_USER", "User was not found").into(),
            ))
        }

        Err(e) => {
            error!(error = %e, "unable to query user with");
            sentry::capture_error(&e);

            return Err(err(
                StatusCode::INTERNAL_SERVER_ERROR,
                ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
            ));
        }
    };

    let mut sessions = sessions.write().await;
    sessions.authorize(password, &user).await.map_err(|e| {
        error!(user.id, error = %e, "unable to create session for user");
        sentry_eyre::capture_report(&e);

        err(
            StatusCode::INTERNAL_SERVER_ERROR,
            ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
        )
    })?;

    let session = sessions.create_session(user.clone()).await.map_err(|e| {
        error!(user.id, error = %e, "unable to create session for user");
        sentry_eyre::capture_report(&e);

        err(
            StatusCode::INTERNAL_SERVER_ERROR,
            ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
        )
    })?;

    // spawn task
    sessions.create_task(session.session_id, Duration::from_secs(604800));
    Ok(ok(StatusCode::OK, session))
}

/// Attempts to destroy the current authenticated session.
#[controller(
    method = delete,
    tags("Users", "Sessions"),
    response(201, "Session was deleted successfully", ("application/json", response!("ApiEmptyResponse"))),
    response(403, "If the authenticated user didn't provide a session token", ("application/json", response!("ApiErrorResponse"))),
    response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
)]
pub async fn logout(
    State(Server { sessions, .. }): State<Server>,
    Extension(crate::middleware::Session { user, session }): Extension<crate::middleware::Session>,
) -> Result<ApiResponse, ApiResponse> {
    if session.is_none() {
        return Err(err(
            StatusCode::FORBIDDEN,
            (
                "SESSION_ONLY_ROUTE",
                "REST handler only allows session tokens to be used.",
                json!({
                    "method": "delete",
                    "uri": "/users/sessions/logout"
                }),
            )
                .into(),
        ));
    }

    let session = session.unwrap();
    let mut sessions = sessions.write().await;
    sessions.kill_session(session.session_id).map_err(|e| {
        error!(session.id = tracing::field::display(session.session_id), user.id, error = %e, "unable to kill session");
        sentry_eyre::capture_report(&e);

        err(
            StatusCode::INTERNAL_SERVER_ERROR,
            ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
        )
    })?;

    Ok(ok(StatusCode::ACCEPTED, Empty))
}

#[controller(
    method = post,
    tags("Users", "Sessions"),
    response(201, "Session was fully restored with a new one", ("application/json", response!("ApiSessionResponse"))),
    response(403, "If the authenticated user didn't provide a refresh token", ("application/json", response!("ApiErrorResponse"))),
    response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
)]
pub async fn refresh_session_token(
    State(Server { sessions, .. }): State<Server>,
    Extension(crate::middleware::Session { session, user }): Extension<crate::middleware::Session>,
) -> Result<ApiResponse<Session>, ApiResponse> {
    if session.is_none() {
        return Err(err(
            StatusCode::FORBIDDEN,
            (
                "SESSION_ONLY_ROUTE",
                "REST handler only allows session tokens to be used.",
                json!({
                    "method": "delete",
                    "uri": "/users/sessions/refresh-token"
                }),
            )
                .into(),
        ));
    }

    let session = session.unwrap();
    let mut sessions = sessions.write().await;
    sessions.kill_session(session.session_id).map_err(|e| {
        error!(session.id = tracing::field::display(session.session_id), user.id, error = %e, "unable to kill session");
        sentry_eyre::capture_report(&e);

        err(
            StatusCode::INTERNAL_SERVER_ERROR,
            ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
        )
    })?;

    // now create a new one
    let new_session = sessions.create_session(user.clone()).await.map_err(|e| {
        error!(session.id = tracing::field::display(session.session_id), user.id, error = %e, "unable to kill session");
        sentry_eyre::capture_report(&e);

        err(
            StatusCode::INTERNAL_SERVER_ERROR,
            ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
        )
    })?;

    Ok(ok(StatusCode::CREATED, new_session))
}
