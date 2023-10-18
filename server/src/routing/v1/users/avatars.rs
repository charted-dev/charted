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
    macros::controller,
    middleware::Session,
    models::res::{err, ApiResponse},
    Multipart, Server,
};
use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::IntoResponse,
    Extension,
};
use charted_common::models::NameOrSnowflake;
use charted_database::controller::{users::UserDatabaseController, DbController};
use charted_storage::{ContentTypeResolver, DefaultContentTypeResolver};

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
    State(Server {
        avatars, controllers, ..
    }): State<Server>,
    Path(id_or_name): Path<NameOrSnowflake>,
) -> Result<impl IntoResponse, ApiResponse> {
    let users = controllers.get::<UserDatabaseController>();
    let user = match users.get_by_nos(id_or_name.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Err(err(
                StatusCode::NOT_FOUND,
                (
                    "UNKNOWN_USER",
                    format!("User with ID or name [{id_or_name}] was not found."),
                )
                    .into(),
            ))
        }

        Err(e) => {
            error!(idOrName = tracing::field::display(id_or_name), error = %e, "unable to query user with");
            sentry::capture_error(&*e);

            return Err(err(
                StatusCode::INTERNAL_SERVER_ERROR,
                ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
            ));
        }
    };

    // 1. Check if they have an avatar already.
    if let Some(hash) = user.avatar_hash.clone() {
        match avatars.user(user.id as u64, hash).await {
            Ok(Some(bytes)) => {
                let ct = DefaultContentTypeResolver::resolve(&DefaultContentTypeResolver, bytes.as_ref());
                let mime = ct.parse::<mime::Mime>().expect("valid content-type");
                assert!(mime.type_() == mime::IMAGE);

                let mime = mime.to_string();
                let headers = [(header::CONTENT_TYPE, mime.as_str())];

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
    }

    // 2. Check if their Gravatar exists
    if let Some(gravatar_email) = user.gravatar_email.clone() {
        match avatars.gravatar(gravatar_email).await {
            Ok(Some(bytes)) => {
                let ct = DefaultContentTypeResolver::resolve(&DefaultContentTypeResolver, bytes.as_ref());
                let mime = ct.parse::<mime::Mime>().expect("valid content-type");
                assert!(mime.type_() == mime::IMAGE);

                let mime = mime.to_string();
                let headers = [(header::CONTENT_TYPE, mime.as_str())];

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

/// Uploads a user avatar.
#[controller(
    method = post,
    tags("Users", "Avatars"),
    response(201, "Successful response", ("application/json", response!("ApiEmptyResponse"))),
    response(400, "Bad Request", ("application/json", response!("ApiErrorResponse"))),
    response(406, "Not Acceptable", ("application/json", response!("ApiErrorResponse"))),
    response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
)]
pub async fn upload_user_avatar(
    Extension(Session { user, .. }): Extension<Session>,
    State(Server { avatars, pool, .. }): State<Server>,
    mut data: Multipart,
) -> Result<ApiResponse, ApiResponse> {
    let Some(field) = data.next_field().await.map_err(|e| {
        error!(user.id, error = %e, "unable to grab next multipart field");
        sentry::capture_error(&e);

        err(
            StatusCode::INTERNAL_SERVER_ERROR,
            ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
        )
    })?
    else {
        return Err(err(
            StatusCode::NOT_ACCEPTABLE,
            ("MISSING_MULTIPART_FIELD", "missing a single multipart field").into(),
        ));
    };

    let data = field.bytes().await.map_err(|e| {
        error!(user.id, error = %e, "unable to collect the data from field");
        sentry::capture_error(&e);

        err(
            StatusCode::INTERNAL_SERVER_ERROR,
            ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
        )
    })?;

    let actual_ct = DefaultContentTypeResolver::resolve(&DefaultContentTypeResolver, data.as_ref());
    let mime = actual_ct.parse::<mime::Mime>().map_err(|e| {
        err(
            StatusCode::NOT_ACCEPTABLE,
            ("INVALID_CONTENT_TYPE", e.to_string()).into(),
        )
    })?;

    if mime.type_() != mime::IMAGE {
        return Err(err(
            StatusCode::NOT_ACCEPTABLE,
            (
                "INVALID_CONTENT_TYPE",
                format!("expected `image/` as type, received {}", mime.type_()),
            )
                .into(),
        ));
    }

    let ext = match mime.subtype() {
        mime::PNG => "png",
        mime::JPEG => "jpg",
        mime::GIF => "gif",
        _ => {
            return Err(err(
                StatusCode::NOT_ACCEPTABLE,
                (
                    "INVALID_CONTENT_TYPE",
                    format!(
                        "expected `png`, `jpeg`, or `gif` for the content-type's subtype, received {}",
                        mime.subtype()
                    ),
                )
                    .into(),
            ))
        }
    };

    info!(user.id, "now performing avatar update...");
    let hash = avatars
        .upload_user_avatar(user.id as u64, data, actual_ct, ext)
        .await
        .map_err(|e| {
            error!(user.id, error = %e, "unable to upload user avatar");
            sentry::capture_error(&*e);

            err(
                StatusCode::INTERNAL_SERVER_ERROR,
                ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
            )
        })?;

    // update it in the database
    match sqlx::query("update users set avatar_hash = $1 where id = $2")
        .bind(hash)
        .bind(user.id)
        .execute(&pool)
        .await
    {
        Ok(_) => {}
        Err(e) => {
            error!(user.id, error = %e, "unable to update column [avatar_hash] for table [users]");
            sentry::capture_error(&e);

            // TODO(@auguwu): push to a background task and keep trying for 5 times?
            return Err(err(
                StatusCode::INTERNAL_SERVER_ERROR,
                ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
            ));
        }
    }

    Ok(ApiResponse {
        status: StatusCode::ACCEPTED,
        success: true,
        data: None,
        errors: vec![],
    })
}

pub mod me {
    use super::UploadUserAvatarRestController;
    use crate::{
        macros::controller,
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
    use utoipa::openapi::{path::PathItemBuilder, PathItem};

    pub fn paths() -> PathItem {
        let mut builder = PathItemBuilder::new();
        let ops = vec![
            GetMyCurrentAvatarRestController::paths()
                .operations
                .pop_first()
                .unwrap(),
            UploadUserAvatarRestController::paths().operations.pop_first().unwrap(),
        ];

        for (ty, op) in ops {
            builder = builder.operation(ty, op);
        }

        builder.build()
    }

    /// Returns the current authenticated user's current avatar. Use the [`GET /users/@me/avatar/{hash}.png`] REST handler
    /// to grab by a specific hash.
    ///
    /// [`GET /users/@me/avatar/{hash}.png`]: https://charts.noelware.org/docs/server/latest/api/reference/users#GET-/users/@me/avatar/{hash}.png
    #[controller(
        tags("Users", "Avatars"),
        response(200, "Successful response", ("image/*", binary)),
        response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
    )]
    pub async fn get_my_current_avatar(
        State(Server { avatars, .. }): State<Server>,
        Extension(Session { user, .. }): Extension<Session>,
    ) -> Result<impl IntoResponse, ApiResponse> {
        // 1. Check if they have an avatar already.
        if let Some(hash) = user.avatar_hash.clone() {
            match avatars.user(user.id as u64, hash).await {
                Ok(Some(bytes)) => {
                    let headers = [(header::CONTENT_TYPE, "image/*")];
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
