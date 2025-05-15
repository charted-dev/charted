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
    Env,
    extract::Json,
    middleware::authn::Session,
    mk_into_responses,
    openapi::{EmptyApiResponse, SessionResponse},
    ops::db,
};
use axum::{Extension, extract::State, http::StatusCode};
use charted_core::api;
use charted_types::payloads::UserLoginPayload;

struct LoginR;
mk_into_responses!(for LoginR {
    "201" => [ref(SessionResponse)];
    "403" => [error(description("invalid password"))];
    "404" => [error(description("user was not found by username or email address"))];
    "406" => [error(description("email was not properly formatted"))];
});

/// Creates a new session.
#[utoipa::path(
    post,

    path = "/v1/users/login",
    tags = ["Users", "Users/Sessions"],
    operation_id = "login",
    request_body(
        content = ref("#/components/schemas/UserLoginPayload"),
        content_type = "application/json"
    ),
    responses(LoginR)
)]
#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn login(
    State(env): State<Env>,
    Json(payload): Json<UserLoginPayload>,
) -> api::Result<charted_types::Session> {
    db::session::login(&env, &payload)
        .await
        .map(|session| api::ok(StatusCode::CREATED, session))
}

struct FetchSessionR;
mk_into_responses!(for FetchSessionR {
    "200" => [ref(SessionResponse)];
    "4XX" => [error(description("authentication failures"))];
});

/// Retrieve information about this session.
///
/// Useless on its own but useful for testing out session authentication.
#[utoipa::path(
    get,

    path = "/v1/users/@me/session",
    tags = ["Users", "Users/Sessions"],
    operation_id = "getUserSession",
    responses(FetchSessionR)
)]
pub async fn fetch(Extension(Session { session, .. }): Extension<Session>) -> api::Result<charted_types::Session> {
    let Some(session) = session else {
        return Err(api::err(
            StatusCode::NOT_FOUND,
            (
                api::ErrorCode::EntityNotFound,
                "authentication didn't use a bearer token",
            ),
        ));
    };

    Ok(api::ok(StatusCode::OK, session.sanitize()))
}

struct LogoutR;
mk_into_responses!(for LogoutR {
    "204" => [ref(with "application/json" => EmptyApiResponse;
        description("session was successfully deleted");
    )];

    "404" => [error(description("session was not found"))];
});

/// Logs you out from the session and destroys it.
#[utoipa::path(
    delete,

    path = "/v1/users/@me/session",
    tags = ["Users", "Users/Sessions"],
    operation_id = "logout",
    responses(LogoutR)
)]
#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn logout(
    State(env): State<Env>,
    Extension(Session { session, .. }): Extension<Session>,
) -> api::Result<()> {
    let Some(session) = session else {
        return Err(api::err(
            StatusCode::NOT_FOUND,
            (
                api::ErrorCode::EntityNotFound,
                "authentication didn't use a bearer token",
            ),
        ));
    };

    db::session::logout(&env, session).await.map(|()| api::no_content())
}

struct RefreshSessionR;
mk_into_responses!(for RefreshSessionR {
    "201" => [ref(SessionResponse)];
});

/// Refresh a session by destroying the old session and creating a new one.
#[utoipa::path(
    post,

    path = "/v1/users/@me/session/refresh",
    tags = ["Users", "Users/Sessions"],
    operation_id = "refreshSessionToken",
    responses(RefreshSessionR)
)]
#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn refresh_session(
    State(env): State<Env>,
    Extension(Session { session, user }): Extension<Session>,
) -> api::Result<charted_types::Session> {
    let Some(session) = session else {
        return Err(api::err(
            StatusCode::NOT_FOUND,
            (
                api::ErrorCode::EntityNotFound,
                "authentication didn't use a bearer token",
            ),
        ));
    };

    db::session::refresh_session(&env, session, user)
        .await
        .map(|session| api::ok(StatusCode::CREATED, session))
}
