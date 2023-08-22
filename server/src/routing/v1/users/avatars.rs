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
    models::res::{err, ApiResponse},
    Server,
};
use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::IntoResponse,
    routing, Router,
};
use charted_common::models::{entities::User, NameOrSnowflake};
use charted_proc_macros::controller;
use sqlx::{query_as, Postgres};

pub fn create_router() -> Router<Server> {
    Router::new().route("/", routing::get(GetCurrentUserAvatarRestController::run))
}

pub fn create_me_router() -> Router<Server> {
    Router::new().route("/", routing::get(me::GetMyAvatarRestController::run))
}

/// Returns the user's current avatar. Use the [`GET /users/{idOrName}/avatar/{hash}.png`] REST handler
/// to grab by a specific hash.
///
/// [`GET /users/{idOrName}/avatar/{hash}.png`]: https://charts.noelware.org/docs/server/latest/api/reference/users#GET-/users/{idOrName}/avatar/{hash}.png
#[controller(
    tags("Users", "Avatars"),
    pathParameter("idOrName", schema!("NameOrSnowflake"), description = "Path parameter that can take a [`Name`] or [`Snowflake`] identifier."),
    response(200, "Successful response", ("image/*", binary)),
    response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
)]
pub async fn get_current_user_avatar(
    State(Server { avatars, pool, .. }): State<Server>,
    Path(id_or_name): Path<NameOrSnowflake>,
) -> Result<impl IntoResponse, ApiResponse> {
    let query = match id_or_name {
        NameOrSnowflake::Snowflake(id) => {
            query_as::<Postgres, User>("select users.* from users where id = $1;").bind(id as i64)
        }

        NameOrSnowflake::Name(ref name) => {
            query_as::<Postgres, User>("select users.* from users where username = $1;").bind(name.to_string())
        }
    };

    let user = match query.fetch_optional(&pool).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Err(err(
                StatusCode::NOT_FOUND,
                (
                    "UNKNOWN_USER",
                    format!("User with ID or name [{id_or_name}] was not found.").as_str(),
                )
                    .into(),
            ))
        }

        Err(e) => {
            error!(idOrName = tracing::field::display(id_or_name), error = %e, "unable to query user with");
            sentry::capture_error(&e);

            return Err(err(
                StatusCode::INTERNAL_SERVER_ERROR,
                ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
            ));
        }
    };

    // 1. Check if they have an avatar already.
    match avatars.user(user.id as u64, None).await {
        Ok(Some(bytes)) => {
            let headers = [(header::CONTENT_TYPE, "image/png")];
            return Ok((headers, bytes).into_response());
        }

        Ok(None) => {}
        Err(e) => {
            error!(user.id, error = %e, "unable to grab current avatar for user");
            sentry_eyre::capture_report(&e);

            return Err(err(
                StatusCode::INTERNAL_SERVER_ERROR,
                ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
            ));
        }
    }

    // 2. Check if their Gravatar exists
    if let Some(gravatar_email) = user.gravatar_email.clone() {
        match avatars.gravatar(gravatar_email).await {
            Ok(Some(bytes)) => {
                let headers = [(header::CONTENT_TYPE, "image/png")];
                return Ok((headers, bytes).into_response());
            }

            Ok(None) => {}
            Err(e) => {
                error!(user.id, error = %e, "unable to grab gravatar for user");
                sentry_eyre::capture_report(&e);

                return Err(err(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
                ));
            }
        }
    }

    // 3. Revert to Dicebear
    match avatars.identicons(user.id as u64).await {
        Ok(bytes) => {
            let headers = [(header::CONTENT_TYPE, "image/png")];
            Ok((headers, bytes).into_response())
        }

        Err(e) => {
            error!(user.id, error = %e, "unable to grab current avatar for user");
            sentry_eyre::capture_report(&e);

            Err(err(
                StatusCode::INTERNAL_SERVER_ERROR,
                ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
            ))
        }
    }
}

pub mod me {
    use crate::{
        middleware::Session,
        models::res::{err, ApiResponse},
        Server,
    };
    use axum::{
        extract::State,
        http::{header, StatusCode},
        response::IntoResponse,
        Extension,
    };
    use charted_proc_macros::controller;

    /// Returns the current authenticated user's current avatar. Use the [`GET /users/@me/avatar/{hash}.png`] REST handler
    /// to grab by a specific hash.
    ///
    /// [`GET /users/@me/avatar/{hash}.png`]: https://charts.noelware.org/docs/server/latest/api/reference/users#GET-/users/@me/avatar/{hash}.png
    #[controller(
        tags("Users", "Avatars"),
        response(200, "Successful response", ("image/*", binary)),
        response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
    )]
    pub async fn get_my_avatar(
        State(Server { avatars, .. }): State<Server>,
        Extension(Session { user, .. }): Extension<Session>,
    ) -> Result<impl IntoResponse, ApiResponse> {
        // 1. Check if they have an avatar already.
        match avatars.user(user.id as u64, None).await {
            Ok(Some(bytes)) => {
                let headers = [(header::CONTENT_TYPE, "image/png")];
                return Ok((headers, bytes).into_response());
            }

            Ok(None) => {}
            Err(e) => {
                error!(user.id, error = %e, "unable to grab current avatar for user");
                sentry_eyre::capture_report(&e);

                return Err(err(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
                ));
            }
        }

        // 2. Check if their Gravatar exists
        if let Some(gravatar_email) = user.gravatar_email.clone() {
            match avatars.gravatar(gravatar_email).await {
                Ok(Some(bytes)) => {
                    let headers = [(header::CONTENT_TYPE, "image/png")];
                    return Ok((headers, bytes).into_response());
                }

                Ok(None) => {}
                Err(e) => {
                    error!(user.id, error = %e, "unable to grab gravatar for user");
                    sentry_eyre::capture_report(&e);

                    return Err(err(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
                    ));
                }
            }
        }

        // 3. Revert to Dicebear
        match avatars.identicons(user.id as u64).await {
            Ok(bytes) => {
                let headers = [(header::CONTENT_TYPE, "image/png")];
                Ok((headers, bytes).into_response())
            }

            Err(e) => {
                error!(user.id, error = %e, "unable to grab current avatar for user");
                sentry_eyre::capture_report(&e);

                Err(err(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
                ))
            }
        }
    }
}
