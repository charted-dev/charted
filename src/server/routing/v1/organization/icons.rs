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
    avatars::AvatarsModule,
    common::models::{entities::Organization, NameOrSnowflake},
    db::controllers::DbController,
    server::{
        controller,
        models::res::{err, internal_server_error, ApiResponse, ErrorCode},
        validation::validate,
    },
    Instance,
};
use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::IntoResponse,
};
use remi_fs::default_resolver;
use serde_json::json;
use validator::Validate;

/// Returns the organization's current icon. Use the [`GET /organizations/{idOrName}/icons/{hash}.png`] REST route
/// to grab an organization icon by a specific hash.
///
/// [`GET /organizations/{idOrName}/icons/{hash}.png`]: https://charts.noelware.org/docs/server/latest/api/reference/organizations#GET-/organizations/{idOrName}/icon/{hash}.png
#[controller(
    tags("Organizations", "Icons"),
    pathParameter("idOrName", schema!("NameOrSnowflake"), description = "Path parameter that can take a `Name` or Snowflake ID"),
    response(200, "Successful response", ("image/*", binary)),
    response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
)]
pub async fn get_current_org_icon(
    State(Instance {
        controllers, avatars, ..
    }): State<Instance>,
    crate::server::extract::NameOrSnowflake(nos): crate::server::extract::NameOrSnowflake,
) -> std::result::Result<impl IntoResponse, ApiResponse> {
    validate(&nos, NameOrSnowflake::validate)?;
    match controllers.organizations.get_by(&nos).await {
        Ok(Some(org)) => fetch_icon_impl(org, avatars).await,
        Ok(None) => Err::<_, ApiResponse>(err(
            StatusCode::NOT_FOUND,
            (
                ErrorCode::EntityNotFound,
                "organization with id or name was not found",
                json!({"idOrName":nos}),
            ),
        )),

        Err(_) => Err(internal_server_error()),
    }
}

/// Return a organization icon by the icon hash.
#[controller(
    tags("Organizations", "Icons"),
    pathParameter("idOrName", schema!("NameOrSnowflake"), description = "Path parameter that can take a `Name` or Snowflake ID"),
    pathParameter("hash", string, description = "the hash to lookup for"),
    response(200, "Successful response", ("image/*", binary)),
    response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
)]
pub async fn get_org_icon_by_hash(
    State(Instance {
        controllers, avatars, ..
    }): State<Instance>,
    crate::server::extract::NameOrSnowflake(nos): crate::server::extract::NameOrSnowflake,
    Path(hash): Path<String>,
) -> std::result::Result<impl IntoResponse, ApiResponse> {
    validate(&nos, NameOrSnowflake::validate)?;
    match controllers.organizations.get_by(&nos).await {
        Ok(Some(org)) => fetch_icon_by_hash_impl(org.id.try_into().unwrap(), hash, avatars).await,
        Ok(None) => Err::<_, ApiResponse>(err(
            StatusCode::NOT_FOUND,
            (
                ErrorCode::EntityNotFound,
                "organization with id or name was not found",
                json!({"idOrName":nos}),
            ),
        )),

        Err(_) => Err(internal_server_error()),
    }
}

// /// Uploads a organization icon.
// #[controller(
//     method = post,
//     tags("Organizations", "icons"),
//     response(201, "Successful response", ("application/json", response!("EmptyApiResponse"))),
//     response(400, "Bad Request", ("application/json", response!("ApiErrorResponse"))),
//     response(406, "Not Acceptable", ("application/json", response!("ApiErrorResponse"))),
//     response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
// )]
// pub async fn upload_icon(
//     State(Instance { icons, pool, .. }): State<Instance>,
//     Extension(Session { organization, .. }): Extension<Session>,
//     mut data: Multipart,
// ) -> Result<()> {
//     let Some(field) = data.next_field().await.map_err(|e| {
//         error!(error = %e, org.id, "unable to fetch next multipart field");
//         sentry::capture_error(&e);

//         e
//     })?
//     else {
//         return Err(err(
//             StatusCode::NOT_ACCEPTABLE,
//             (
//                 ErrorCode::MissingMultipartField,
//                 "didn't find the next multipart field, is it empty?",
//             ),
//         ));
//     };

//     let data = field.bytes().await.map_err(|e| {
//         error!(error = %e, org.id, "unable to collect inner data from multipart field");
//         sentry::capture_error(&e);

//         e
//     })?;

//     let ct = default_resolver(data.as_ref());
//     let mime = ct.parse::<mime::Mime>().map_err(|e| {
//         error!(error = %e, "received invalid content type, this is a bug");
//         sentry::capture_error(&e);

//         err(
//             StatusCode::UNPROCESSABLE_ENTITY,
//             (
//                 ErrorCode::InvalidContentType,
//                 "received an invalid content type, this is a bug",
//             ),
//         )
//     })?;

//     if mime.type_() != mime::IMAGE {
//         return Err(err(
//             StatusCode::NOT_ACCEPTABLE,
//             (
//                 ErrorCode::InvalidContentType,
//                 "expected a image-based content type",
//                 json!({"contentType":ct}),
//             ),
//         ));
//     }

//     match mime.subtype() {
//         mime::PNG => "png",
//         mime::JPEG => "jpg",
//         mime::GIF => "gif",
//         mime::SVG => "svg",
//         _ => {
//             return Err(err(
//                 StatusCode::NOT_ACCEPTABLE,
//                 (
//                     ErrorCode::InvalidContentType,
//                     "expected `png`, `svg`, `jpeg`, or `gif` as subtype",
//                     json!({"contentType":ct, "subType": mime.subtype().to_string()}),
//                 ),
//             ))
//         }
//     };

//     let hash = icons
//         .upload_organization_icon(org.id.try_into().unwrap(), data)
//         .await
//         .map_err(|e| {
//             error!(error = %e, org.id, "unable to upload organization icon");
//             sentry_eyre::capture_report(&e);

//             internal_server_error()
//         })?;

//     // update it in the database
//     match sqlx::query("update organizations set icon_hash = $1 where id = $2")
//         .bind(hash)
//         .bind(org.id)
//         .execute(&pool)
//         .await
//     {
//         Ok(_) => Ok(ok(StatusCode::ACCEPTED, ())),
//         Err(e) => {
//             error!(error = %e, org.id, "unable to update column [icon_hash] on table [organizations]");
//             sentry::capture_error(&e);

//             Err(internal_server_error())
//         }
//     }
// }

async fn fetch_icon_by_hash_impl(
    id: u64,
    hash: String,
    icons: AvatarsModule,
) -> std::result::Result<impl IntoResponse, ApiResponse> {
    match icons.organization(id, Some(&hash)).await {
        Ok(Some(data)) => {
            let ct = default_resolver(data.as_ref());
            let mime = ct.parse::<mime::Mime>().map_err(|e| {
                error!(error = %e, id, org.icon = hash, "unable to validate `Content-Type` as a valid media type");
                sentry::capture_error(&e);

                internal_server_error()
            })?;

            if mime.type_() != mime::IMAGE {
                return Err(err(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    (
                        ErrorCode::UnableToProcess,
                        "media type for given icon is invalid for some reason",
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
                "organization icon with hash was not found",
                json!({"hash": hash}),
            ),
        )),

        Err(e) => {
            error!(error = %e, id, "unable to get current icon for organization");
            sentry_eyre::capture_report(&e);

            Err(internal_server_error())
        }
    }
}

async fn fetch_icon_impl(
    org: Organization,
    icons: AvatarsModule,
) -> std::result::Result<impl IntoResponse, ApiResponse> {
    if let Some(ref hash) = org.icon_hash {
        match icons.organization(org.id.try_into().unwrap(), Some(hash)).await {
            Ok(Some(data)) => {
                let ct = default_resolver(data.as_ref());
                let mime = ct.parse::<mime::Mime>().map_err(|e| {
                    error!(error = %e, org.id, org.icon = hash, "unable to validate `Content-Type` as a valid media type");
                    sentry::capture_error(&e);

                    internal_server_error()
                })?;

                if mime.type_() != mime::IMAGE {
                    return Err(err(
                        StatusCode::UNPROCESSABLE_ENTITY,
                        (
                            ErrorCode::UnableToProcess,
                            "media type for given icon is invalid for some reason",
                            json!({"mediaType": mime.to_string()}),
                        ),
                    ));
                }

                return Ok(([(header::CONTENT_TYPE, ct.as_str())], data).into_response());
            }

            // skip if we can't find it
            Ok(None) => {}
            Err(e) => {
                error!(error = %e, org.id, "unable to get current icon for organization");
                sentry_eyre::capture_report(&e);

                return Err(internal_server_error());
            }
        }
    }

    if let Some(ref gravatar) = org.gravatar_email {
        match icons.gravatar(gravatar).await {
            Ok(Some(data)) => {
                let ct = default_resolver(data.as_ref());
                let mime = ct.parse::<mime::Mime>().map_err(|e| {
                    error!(error = %e, org.id, "unable to validate `Content-Type` as a valid media type from Gricon");
                    sentry::capture_error(&e);

                    internal_server_error()
                })?;

                if mime.type_() != mime::IMAGE {
                    return Err(err(
                        StatusCode::UNPROCESSABLE_ENTITY,
                        (
                            ErrorCode::UnableToProcess,
                            "media type for given icon is invalid for some reason",
                            json!({"mediaType": mime.to_string()}),
                        ),
                    ));
                }

                return Ok(([(header::CONTENT_TYPE, ct.as_str())], data).into_response());
            }

            // skip if we can't find it
            Ok(None) => {}
            Err(e) => {
                error!(error = %e, org.id, "unable to get current icon for organization");
                sentry_eyre::capture_report(&e);

                return Err(internal_server_error());
            }
        }
    }

    match icons.identicons(org.id.try_into().unwrap()).await {
        Ok(data) => {
            let ct = default_resolver(data.as_ref());
            let mime = ct.parse::<mime::Mime>().map_err(|e| {
                error!(error = %e, org.id, "unable to validate `Content-Type` as a valid media type");
                sentry::capture_error(&e);

                internal_server_error()
            })?;

            if mime.type_() != mime::IMAGE {
                return Err(err(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    (
                        ErrorCode::UnableToProcess,
                        "media type for given icon is invalid for some reason",
                        json!({"mediaType": mime.to_string()}),
                    ),
                ));
            }

            Ok(([(header::CONTENT_TYPE, ct.as_str())], data).into_response())
        }

        Err(e) => {
            error!(error = %e, org.id, "unable to get current icon for organization");
            sentry_eyre::capture_report(&e);

            Err(internal_server_error())
        }
    }
}
