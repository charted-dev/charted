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

use std::time::Duration;

use crate::{
    extract::Json,
    models::res::{err, ok, ApiResponse},
    openapi::gen_response_schema,
    validation::validate,
    Server,
};
use axum::{extract::State, http::StatusCode, Router};
use charted_common::models::{entities::User, payloads::UserLoginPayload, Name};
use charted_proc_macros::controller;
use charted_sessions::{Session, SessionProvider};
use sqlx::{query_as, Postgres};
use validator::Validate;

pub fn create_router() -> Router<Server> {
    Router::new()
}

// this shouldn't be used directly
pub(crate) struct SessionResponse;
gen_response_schema!(SessionResponse, schema: "Session");

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
    State(Server { sessions, pool, .. }): State<Server>,
    Json(payload): Json<UserLoginPayload>,
) -> Result<ApiResponse<Session>, ApiResponse> {
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
    let session = sessions.authorize(password, &user).await.map_err(|e| {
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
