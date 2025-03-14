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

mod ops;

use crate::{
    Context, extract::Path, extract_refor_t, middleware::sessions::Session, modify_property, multipart::Multipart,
    openapi::ApiErrorResponse,
};
use axum::{Extension, extract::State, response::IntoResponse};
use azalia::remi::{
    core::{StorageService, UploadRequest},
    fs,
};
use charted_core::{api, rand_string};
use charted_database::entities::{UserEntity, user};
use charted_types::{NameOrUlid, User};
use reqwest::StatusCode;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
use serde_json::json;
use std::{borrow::Borrow, collections::BTreeMap};
use url::Url;
use utoipa::{
    IntoParams, IntoResponses, PartialSchema, ToResponse,
    openapi::{
        Content, Ref, RefOr, Response,
        path::{Parameter, ParameterIn},
    },
};

struct GetUserAvatarR;
impl IntoResponses for GetUserAvatarR {
    fn responses() -> BTreeMap<String, RefOr<Response>> {
        azalia::btreemap! {
            "200" => Response::builder()
                .description("Avatar data as a raw image")
                .content("image/*", Content::builder().schema(Some(<[u8] as PartialSchema>::schema())).build())
                .build(),

            "404" => {
                let mut response = extract_refor_t!(ApiErrorResponse::response().1);
                modify_property!(response; description("Avatar by hash was not found"));

                response
            },

            "5XX" => {
                let mut response = extract_refor_t!(ApiErrorResponse::response().1);
                modify_property!(response; description("Internal Server Error"));

                response
            }
        }
    }
}

struct HashParams;
impl IntoParams for HashParams {
    fn into_params(parameter_in_provider: impl Fn() -> Option<ParameterIn>) -> Vec<Parameter> {
        vec![
            Parameter::builder()
                .name("hash")
                .required(utoipa::openapi::Required::True)
                .parameter_in(parameter_in_provider().unwrap_or_default())
                .description(Some("The hash that the request will check for"))
                .schema(Some(String::schema()))
                .build(),
        ]
    }
}

/// Returns the user's current avatar.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[instrument(name = "charted.server.ops.fetch[user/avatars]", skip_all)]
#[utoipa::path(
    get,
    path = "/v1/users/{idOrName}/avatar",
    operation_id = "getCurrentUserAvatar",
    tag = "Users/Avatars",
    params(NameOrUlid),
    responses(GetUserAvatarR)
)]
pub async fn get_user_avatar(
    State(cx): State<Context>,
    Path(id_or_name): Path<NameOrUlid>,
) -> Result<impl IntoResponse, api::Response> {
    match id_or_name {
        NameOrUlid::Name(ref name) => match UserEntity::find()
            .filter(user::Column::Username.eq(name.to_owned()))
            .one(&cx.pool)
            .await
            .map_err(api::system_failure)?
            .map(Into::<User>::into)
        {
            Some(user) => ops::fetch(cx, user).await,
            None => Err(api::err(
                StatusCode::NOT_FOUND,
                (
                    api::ErrorCode::EntityNotFound,
                    "user with id or name was not found",
                    json!({"idOrName":id_or_name}),
                ),
            )),
        },

        NameOrUlid::Ulid(id) => match UserEntity::find_by_id(id)
            .one(&cx.pool)
            .await
            .map_err(api::system_failure)?
            .map(Into::<User>::into)
        {
            Some(user) => ops::fetch(cx, user).await,
            None => Err(api::err(
                StatusCode::NOT_FOUND,
                (
                    api::ErrorCode::EntityNotFound,
                    "user with id or name was not found",
                    json!({"idOrName":id_or_name}),
                ),
            )),
        },
    }
}

/// Returns the user's avatar by their hash.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[instrument(name = "charted.server.ops.fetch[user/avatar/byHash]", skip_all)]
#[utoipa::path(
    get,
    path = "/v1/users/{idOrName}/avatars/{hash}",
    operation_id = "getUserAvatar",
    tag = "Users/Avatars",
    params(NameOrUlid, HashParams),
    responses(GetUserAvatarR)
)]
pub async fn get_user_avatar_by_hash(
    State(cx): State<Context>,
    Path((id_or_name, hash)): Path<(NameOrUlid, String)>,
) -> Result<impl IntoResponse, api::Response> {
    match id_or_name {
        NameOrUlid::Name(ref name) => match UserEntity::find()
            .filter(user::Column::Username.eq(name.to_owned()))
            .one(&cx.pool)
            .await
            .map_err(api::system_failure)?
            .map(Into::<User>::into)
        {
            Some(user) => ops::fetch_by_hash(cx, user.id, hash).await,
            None => Err(api::err(
                StatusCode::NOT_FOUND,
                (
                    api::ErrorCode::EntityNotFound,
                    "user with id or name was not found",
                    json!({"idOrName":id_or_name}),
                ),
            )),
        },

        NameOrUlid::Ulid(id) => match UserEntity::find_by_id(id)
            .one(&cx.pool)
            .await
            .map_err(api::system_failure)?
            .map(Into::<User>::into)
        {
            Some(user) => ops::fetch_by_hash(cx, user.id, hash).await,
            None => Err(api::err(
                StatusCode::NOT_FOUND,
                (
                    api::ErrorCode::EntityNotFound,
                    "user with id or name was not found",
                    json!({"idOrName":id_or_name}),
                ),
            )),
        },
    }
}

/// Returns the authenticated user's current avatar.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[instrument(name = "charted.server.ops.fetch[user/self/avatars]", skip_all)]
#[utoipa::path(
    get,
    path = "/v1/users/@me/avatar",
    operation_id = "getSelfUserAvatar",
    tag = "Users/Avatars",
    responses(GetUserAvatarR)
)]
pub async fn get_self_user_avatar(
    State(cx): State<Context>,
    Extension(Session { user, .. }): Extension<Session>,
) -> Result<impl IntoResponse, api::Response> {
    ops::fetch(cx, user).await
}

/// Returns the authenticated user's avatar by their hash.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[instrument(name = "charted.server.ops.fetch[user/self/avatar/byHash]", skip_all)]
#[utoipa::path(
    get,
    path = "/v1/users/@me/avatars/{hash}",
    operation_id = "getSelfUserAvatarByHash",
    tag = "Users/Avatars",
    responses(GetUserAvatarR)
)]
pub async fn get_self_user_avatar_by_hash(
    State(cx): State<Context>,
    Path(hash): Path<String>,
    Extension(Session { user, .. }): Extension<Session>,
) -> Result<impl IntoResponse, api::Response> {
    ops::fetch_by_hash(cx, user.id, hash).await
}

struct UpdateAvatarR;
impl IntoResponses for UpdateAvatarR {
    fn responses() -> BTreeMap<String, RefOr<Response>> {
        azalia::btreemap! {
            "201" => Ref::from_response_name("UrlResponse"),
            "5XX" => {
                let mut response = extract_refor_t!(ApiErrorResponse::response().1);
                modify_property!(response; description("Internal Server Failure"));

                response
            }
        }
    }
}

/// Upload an avatar.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[instrument(name = "charted.server.ops.create[user/avatar]", skip_all)]
#[utoipa::path(
    post,
    path = "/v1/users/@me/avatar",
    operation_id = "uploadSelfUserAvatar",
    tag = "Users/Avatars",
    request_body(
        description = "Multipart form of a single field being the avatar data",
        content = [u8],
        content_type = "multipart/form-data"
    ),
    responses(UpdateAvatarR)
)]
pub async fn upload_user_avatar(
    State(cx): State<Context>,
    Extension(Session { user, .. }): Extension<Session>,
    mut data: Multipart,
) -> api::Result<Url> {
    let Some(field) = data
        .next_field()
        .await
        .inspect_err(|e| {
            error!(error = %e, %user.id, "unable to get next multipart field");
            sentry::capture_error(e);
        })
        .map_err(api::system_failure)?
    else {
        return Err(api::err(
            StatusCode::NOT_ACCEPTABLE,
            (
                api::ErrorCode::MissingMultipartField,
                "didn't find the next multipart field, is it empty?",
            ),
        ));
    };

    let data = field
        .bytes()
        .await
        .inspect_err(|e| {
            error!(error = %e, %user.id, "unable to collect inner data from multipart field");
            sentry::capture_error(&e);
        })
        .map_err(api::system_failure)?;

    let ct = fs::default_resolver(&data);
    let mime = ct
        .parse::<mime::Mime>()
        .inspect_err(|e| {
            error!(error = %e, "received invalid content type, this is a bug");
            sentry::capture_error(&e);
        })
        .map_err(|_| {
            api::err(
                StatusCode::UNPROCESSABLE_ENTITY,
                (
                    api::ErrorCode::InvalidContentType,
                    "received an invalid content type, this is a bug",
                ),
            )
        })?;

    if mime.type_() != mime::IMAGE {
        return Err(api::err(
            StatusCode::NOT_ACCEPTABLE,
            (
                api::ErrorCode::InvalidContentType,
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
            return Err(api::err(
                StatusCode::NOT_ACCEPTABLE,
                (
                    api::ErrorCode::InvalidContentType,
                    "expected `png`, `svg`, `jpeg`, or `gif` as subtype",
                    json!({"contentType": ct, "subType": mime.subtype().to_string()}),
                ),
            ));
        }
    };

    let hash = rand_string(5);
    let request = UploadRequest::default()
        .with_content_type(Some(ct.clone()))
        .with_data(data)
        .with_metadata(azalia::hashmap! {
            "charts.noelware.org/user" => user.id.as_str()
        });

    let ext = match ct.borrow() {
        "image/png" => "png",
        "image/jpeg" => "jpg",
        "image/gif" => "gif",
        "image/svg" => "svg",
        _ => unreachable!(),
    };

    cx.storage
        .upload(format!("./avatars/users/{}/{hash}.{ext}", user.id), request)
        .await
        .map_err(api::system_failure)?;

    let mut model = UserEntity::find_by_id(user.id)
        .one(&cx.pool)
        .await
        .map_err(api::system_failure)?
        .map(Into::<user::ActiveModel>::into)
        .unwrap();

    model.set(user::Column::AvatarHash, Some(format!("{hash}.{ext}")).into());
    model
        .update(&cx.pool)
        .await
        .inspect_err(|e| {
            error!(error = %e, "failed to apply user update patch");
            sentry::capture_error(e);
        })
        .map_err(api::system_failure)?;

    let resource = cx
        .config
        .base_url
        .unwrap()
        .join(&format!("/users/{}/avatars/{hash}.{ext}", user.id))
        .map_err(api::system_failure)?;

    Ok(api::ok(StatusCode::ACCEPTED, resource))
}
