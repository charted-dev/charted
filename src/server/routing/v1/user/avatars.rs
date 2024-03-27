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

use crate::{
    db::controllers::DbController,
    server::{middleware::session::Session, validation::validate},
    Instance,
};
use axum::{
    extract::State,
    http::{header, StatusCode},
    response::IntoResponse,
    Extension,
};
use charted_entities::NameOrSnowflake;
use charted_entities::User;
use charted_server::{
    controller, err, extract::Path, internal_server_error, multipart::Multipart, ok, ApiResponse, ErrorCode, Result,
};
use noelware_remi::StorageService;
use remi_fs::default_resolver;
use serde_json::json;
use validator::Validate;

/// Returns the user's current avatar. Use the [`GET /users/{idOrName}/avatar/{hash}.png`] REST route
/// to grab an user avatar by a specific hash.
///
/// [`GET /users/{idOrName}/avatar/{hash}.png`]: https://charts.noelware.org/docs/server/latest/api/reference/users#GET-/users/{idOrName}/avatar/{hash}.png
#[controller(
    tags("Users", "Avatars"),
    pathParameter("idOrName", schema!("NameOrSnowflake"), description = "Path parameter that can take a `Name` or Snowflake ID"),
    response(200, "Successful response", ("image/*", binary)),
    response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
)]
pub async fn get_current_user_avatar(
    State(Instance {
        controllers,
        ref storage,
        ..
    }): State<Instance>,
    charted_server::extract::NameOrSnowflake(nos): charted_server::extract::NameOrSnowflake,
) -> std::result::Result<impl IntoResponse, ApiResponse> {
    validate(&nos, NameOrSnowflake::validate)?;
    match controllers.users.get_by(&nos).await {
        Ok(Some(user)) => fetch_avatar_impl(user, storage).await,
        Ok(None) => Err::<_, ApiResponse>(err(
            StatusCode::NOT_FOUND,
            (
                ErrorCode::EntityNotFound,
                "user with id or name was not found",
                json!({"idOrName":nos}),
            ),
        )),

        Err(_) => Err(internal_server_error()),
    }
}

/// Return a user avatar by the avatar hash.
#[controller(
    tags("Users", "Avatars"),
    pathParameter("idOrName", schema!("NameOrSnowflake"), description = "Path parameter that can take a `Name` or Snowflake ID"),
    pathParameter("hash", string, description = "the hash to lookup for"),
    response(200, "Successful response", ("image/*", binary)),
    response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
)]
pub async fn get_user_avatar_by_hash(
    State(Instance {
        controllers,
        ref storage,
        ..
    }): State<Instance>,
    charted_server::extract::NameOrSnowflake(nos): charted_server::extract::NameOrSnowflake,
    Path(hash): Path<String>,
) -> std::result::Result<impl IntoResponse, ApiResponse> {
    validate(&nos, NameOrSnowflake::validate)?;
    match controllers.users.get_by(&nos).await {
        Ok(Some(user)) => fetch_avatar_by_hash_impl(user.id.try_into().unwrap(), hash, storage).await,
        Ok(None) => Err::<_, ApiResponse>(err(
            StatusCode::NOT_FOUND,
            (
                ErrorCode::EntityNotFound,
                "user with id or name was not found",
                json!({"idOrName":nos}),
            ),
        )),

        Err(_) => Err(internal_server_error()),
    }
}

/// Uploads a user avatar.
#[controller(
    method = post,
    tags("Users", "Avatars"),
    response(201, "Successful response", ("application/json", response!("EmptyApiResponse"))),
    response(400, "Bad Request", ("application/json", response!("ApiErrorResponse"))),
    response(406, "Not Acceptable", ("application/json", response!("ApiErrorResponse"))),
    response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
)]
pub async fn upload_avatar(
    State(Instance { pool, ref storage, .. }): State<Instance>,
    Extension(Session { user, .. }): Extension<Session>,
    mut data: Multipart,
) -> Result<()> {
    let Some(field) = data.next_field().await.inspect_err(|e| {
        error!(error = %e, user.id, "unable to fetch next multipart field");
        sentry::capture_error(&e);
    })?
    else {
        return Err(err(
            StatusCode::NOT_ACCEPTABLE,
            (
                ErrorCode::MissingMultipartField,
                "didn't find the next multipart field, is it empty?",
            ),
        ));
    };

    let data = field.bytes().await.map_err(|e| {
        error!(error = %e, user.id, "unable to collect inner data from multipart field");
        sentry::capture_error(&e);

        e
    })?;

    let ct = default_resolver(data.as_ref());
    let mime = ct
        .parse::<mime::Mime>()
        .inspect_err(|e| {
            error!(error = %e, "received invalid content type, this is a bug");
            sentry::capture_error(&e);
        })
        .map_err(|_| {
            err(
                StatusCode::UNPROCESSABLE_ENTITY,
                (
                    ErrorCode::InvalidContentType,
                    "received an invalid content type, this is a bug",
                ),
            )
        })?;

    if mime.type_() != mime::IMAGE {
        return Err(err(
            StatusCode::NOT_ACCEPTABLE,
            (
                ErrorCode::InvalidContentType,
                "expected a image-based content type",
                json!({"contentType":ct}),
            ),
        ));
    }

    match mime.subtype() {
        mime::PNG => "png",
        mime::JPEG => "jpg",
        mime::GIF => "gif",
        mime::SVG => "svg",
        _ => {
            return Err(err(
                StatusCode::NOT_ACCEPTABLE,
                (
                    ErrorCode::InvalidContentType,
                    "expected `png`, `svg`, `jpeg`, or `gif` as subtype",
                    json!({"contentType":ct, "subType": mime.subtype().to_string()}),
                ),
            ))
        }
    };

    let hash = crate::avatars::upload_user_avatar(storage, user.id.try_into().unwrap(), data)
        .await
        .inspect_err(|e| {
            error!(error = %e, user.id, "unable to upload user avatar");
        })
        .map_err(|_| internal_server_error())?;

    // update it in the database
    match sqlx::query("update users set avatar_hash = $1 where id = $2")
        .bind(hash)
        .bind(user.id)
        .execute(&pool)
        .await
    {
        Ok(_) => Ok(ok(StatusCode::ACCEPTED, ())),
        Err(e) => {
            error!(error = %e, user.id, "unable to update column [avatar_hash] on table [users]");
            sentry::capture_error(&e);

            Err(internal_server_error())
        }
    }
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
pub async fn get_self_user_avatar(
    State(Instance { ref storage, .. }): State<Instance>,
    Extension(Session { user, .. }): Extension<Session>,
) -> std::result::Result<impl IntoResponse, ApiResponse> {
    fetch_avatar_impl(user, storage).await
}

/// Returns the current authenticated user's current avatar by the specific hash
#[controller(
    tags("Users", "Avatars"),
    pathParameter("hash", string, description = "avatar hash to look up for"),
    response(200, "Successful response", ("image/*", binary)),
    response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
)]
pub async fn get_self_user_avatar_by_hash(
    State(Instance { ref storage, .. }): State<Instance>,
    Extension(Session { user, .. }): Extension<Session>,
    Path(hash): Path<String>,
) -> std::result::Result<impl IntoResponse, ApiResponse> {
    fetch_avatar_by_hash_impl(user.id.try_into().unwrap(), hash, storage).await
}

async fn fetch_avatar_by_hash_impl(
    id: u64,
    hash: String,
    storage: &StorageService,
) -> std::result::Result<impl IntoResponse, ApiResponse> {
    match crate::avatars::user(storage, id, Some(&hash)).await {
        Ok(Some(data)) => {
            let ct = default_resolver(data.as_ref());
            let mime = ct.parse::<mime::Mime>().inspect_err(|e| {
                error!(error = %e, id, user.avatar = hash, "unable to validate `Content-Type` as a valid media type");
                sentry::capture_error(&e);
            }).map_err(|_| internal_server_error())?;

            if mime.type_() != mime::IMAGE {
                return Err(err(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    (
                        ErrorCode::UnableToProcess,
                        "media type for given avatar is invalid for some reason",
                        json!({"mediaType": mime.to_string()}),
                    ),
                ));
            }

            Ok(([(header::CONTENT_TYPE, ct.as_str())], data).into_response())
        }

        // skip if we can't find it
        Ok(None) => Err(err(
            StatusCode::NOT_FOUND,
            (
                ErrorCode::EntityNotFound,
                "user avatar with hash was not found",
                json!({"hash": hash}),
            ),
        )),

        Err(e) => {
            error!(error = %e, id, "unable to get current avatar for user");
            sentry_eyre::capture_report(&e);

            Err(internal_server_error())
        }
    }
}

async fn fetch_avatar_impl(
    user: User,
    storage: &StorageService,
) -> std::result::Result<impl IntoResponse, ApiResponse> {
    if let Some(ref hash) = user.avatar_hash {
        match crate::avatars::user(storage, user.id.try_into().unwrap(), Some(hash)).await {
            Ok(Some(data)) => {
                let ct = default_resolver(data.as_ref());
                let mime = ct.parse::<mime::Mime>().map_err(|e| {
                    error!(error = %e, user.id, user.avatar = hash, "unable to validate `Content-Type` as a valid media type");
                    sentry::capture_error(&e);

                    internal_server_error()
                })?;

                if mime.type_() != mime::IMAGE {
                    return Err(err(
                        StatusCode::UNPROCESSABLE_ENTITY,
                        (
                            ErrorCode::UnableToProcess,
                            "media type for given avatar is invalid for some reason",
                            json!({"mediaType": mime.to_string()}),
                        ),
                    ));
                }

                return Ok(([(header::CONTENT_TYPE, ct.as_str())], data).into_response());
            }

            // skip if we can't find it
            Ok(None) => {}
            Err(e) => {
                error!(error = %e, user.id, "unable to get current avatar for user");
                sentry_eyre::capture_report(&e);

                return Err(internal_server_error());
            }
        }
    }

    if let Some(ref gravatar) = user.gravatar_email {
        match crate::avatars::gravatar(gravatar).await {
            Ok(Some(data)) => {
                let ct = default_resolver(data.as_ref());
                let mime = ct.parse::<mime::Mime>().map_err(|e| {
                    error!(error = %e, user.id, "unable to validate `Content-Type` as a valid media type from Gravatar");
                    sentry::capture_error(&e);

                    internal_server_error()
                })?;

                if mime.type_() != mime::IMAGE {
                    return Err(err(
                        StatusCode::UNPROCESSABLE_ENTITY,
                        (
                            ErrorCode::UnableToProcess,
                            "media type for given avatar is invalid for some reason",
                            json!({"mediaType": mime.to_string()}),
                        ),
                    ));
                }

                return Ok(([(header::CONTENT_TYPE, ct.as_str())], data).into_response());
            }

            // skip if we can't find it
            Ok(None) => {}
            Err(e) => {
                error!(error = %e, user.id, "unable to get current avatar for user");
                sentry_eyre::capture_report(&e);

                return Err(internal_server_error());
            }
        }
    }

    match crate::avatars::identicons(user.id.try_into().unwrap()).await {
        Ok(data) => {
            let ct = default_resolver(data.as_ref());
            let mime = ct.parse::<mime::Mime>().map_err(|e| {
                error!(error = %e, user.id, "unable to validate `Content-Type` as a valid media type");
                sentry::capture_error(&e);

                internal_server_error()
            })?;

            if mime.type_() != mime::IMAGE {
                return Err(err(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    (
                        ErrorCode::UnableToProcess,
                        "media type for given avatar is invalid for some reason",
                        json!({"mediaType": mime.to_string()}),
                    ),
                ));
            }

            Ok(([(header::CONTENT_TYPE, ct.as_str())], data).into_response())
        }

        Err(e) => {
            error!(error = %e, user.id, "unable to get current avatar for user");
            sentry_eyre::capture_report(&e);

            Err(internal_server_error())
        }
    }
}
