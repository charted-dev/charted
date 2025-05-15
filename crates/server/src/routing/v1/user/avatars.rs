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
    ext::ResultExt,
    extract::{Multipart, Path},
    middleware::authn::Session,
    ops::{
        avatars::{DataStoreExt, GetAvatarR, Params, UpdateAvatarR},
        db,
    },
};
use axum::{Extension, extract::State, http::StatusCode, response::IntoResponse};
use charted_core::api;
use charted_database::entities::user;
use charted_types::NameOrUlid;
use sea_orm::{ActiveModelTrait, IntoActiveModel};
use serde_json::json;
use url::Url;

/// Returns the user's current avatar.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    get,
    path = "/v1/users/{idOrName}/avatar",
    operation_id = "getCurrentUserAvatar",
    tag = "Users/Avatars",
    params(NameOrUlid),
    responses(GetAvatarR)
)]
pub async fn get_user_avatar(
    State(env): State<Env>,
    Path(id_or_name): Path<NameOrUlid>,
) -> Result<impl IntoResponse, api::Response> {
    let user = db::user::get(&env.db, id_or_name.clone()).await?.ok_or_else(|| {
        api::err(
            StatusCode::NOT_FOUND,
            (
                api::ErrorCode::EntityNotFound,
                "user with id or name was not found",
                json!({"idOrName":id_or_name}),
            ),
        )
    })?;

    env.ds.user_avatars(user.id).get(&env, &user).await
}

/// Returns the user's avatar by their hash.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    get,
    path = "/v1/users/{idOrName}/avatars/{hash}",
    operation_id = "getUserAvatar",
    tag = "Users/Avatars",
    params(NameOrUlid, Params),
    responses(GetAvatarR)
)]
pub async fn get_user_avatar_by_hash(
    State(env): State<Env>,
    Path((id_or_name, hash)): Path<(NameOrUlid, String)>,
) -> Result<impl IntoResponse, api::Response> {
    let user = db::user::get(&env.db, id_or_name.clone()).await?.ok_or_else(|| {
        api::err(
            StatusCode::NOT_FOUND,
            (
                api::ErrorCode::EntityNotFound,
                "user with id or name was not found",
                json!({"idOrName":id_or_name}),
            ),
        )
    })?;

    env.ds.user_avatars(user.id).by_hash(hash).await
}

/// Returns the authenticated user's current avatar.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    get,
    path = "/v1/users/@me/avatar",
    operation_id = "getSelfUserAvatar",
    tag = "Users/Avatars",
    responses(GetAvatarR)
)]
pub async fn get_self_user_avatar(
    State(env): State<Env>,
    Extension(Session { user, .. }): Extension<Session>,
) -> Result<impl IntoResponse, api::Response> {
    env.ds.user_avatars(user.id).get(&env, &user).await
}

/// Returns the authenticated user's avatar by their hash.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    get,
    path = "/v1/users/@me/avatars/{hash}",
    operation_id = "getSelfUserAvatarByHash",
    tag = "Users/Avatars",
    responses(GetAvatarR)
)]
pub async fn get_self_user_avatar_by_hash(
    State(env): State<Env>,
    Path(hash): Path<String>,
    Extension(Session { user, .. }): Extension<Session>,
) -> Result<impl IntoResponse, api::Response> {
    env.ds.user_avatars(user.id).by_hash(hash).await
}

/// Upload an avatar.
#[cfg_attr(debug_assertions, axum::debug_handler)]
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
    State(env): State<Env>,
    Extension(Session { user, .. }): Extension<Session>,
    data: Multipart,
) -> api::Result<Url> {
    let mut model = db::user::get_as_model(&env.db, NameOrUlid::Ulid(user.id))
        .await?
        .unwrap()
        .into_active_model();

    let ns = env.ds.user_avatars(user.id);
    let hash = ns.upload(data).await?;

    model.set(user::Column::AvatarHash, Some(hash.clone()).into());
    model.update(&env.db).await.into_system_failure()?;

    let resource = env
        .config
        .base_url
        .unwrap()
        .join(&format!("/users/{}/avatars/{hash}", user.id))
        .into_system_failure()?;

    Ok(api::ok(StatusCode::ACCEPTED, resource))
}
