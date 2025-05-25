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
use charted_database::entities::organization;
use charted_types::NameOrUlid;
use sea_orm::{ActiveModelTrait, ColumnTrait, IntoActiveModel, QueryFilter};
use serde_json::json;
use url::Url;

/// Returns the user's current avatar.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    get,
    path = "/v1/organizations/{idOrName}/icon",
    operation_id = "getCurrentOrganizationIcon",
    tag = "Organization/Icons",
    params(NameOrUlid),
    responses(GetAvatarR)
)]
pub async fn get_org_icon(
    State(env): State<Env>,
    Path(id_or_name): Path<NameOrUlid>,
) -> Result<impl IntoResponse, api::Response> {
    let org = db::organization::get(&env.db, id_or_name.clone())
        .await?
        .ok_or_else(|| {
            api::err(
                StatusCode::NOT_FOUND,
                (
                    api::ErrorCode::EntityNotFound,
                    "organization with id or name was not found",
                    json!({"idOrName":id_or_name}),
                ),
            )
        })?;

    env.ds.org_icons(org.id).get(&env, &org).await
}

/// Returns the user's avatar by their hash.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    get,
    path = "/v1/organizations/{idOrName}/icons/{hash}",
    operation_id = "getOrganizationIconByHash",
    tag = "Organization/Icons",
    params(NameOrUlid, Params),
    responses(GetAvatarR)
)]
pub async fn get_org_icon_by_hash(
    State(env): State<Env>,
    Path((id_or_name, hash)): Path<(NameOrUlid, String)>,
) -> Result<impl IntoResponse, api::Response> {
    let org = db::organization::get(&env.db, id_or_name.clone())
        .await?
        .ok_or_else(|| {
            api::err(
                StatusCode::NOT_FOUND,
                (
                    api::ErrorCode::EntityNotFound,
                    "organization with id or name was not found",
                    json!({"idOrName":id_or_name}),
                ),
            )
        })?;

    env.ds.org_icons(org.id).by_hash(hash).await
}

/// Upload an organization icon.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    post,
    path = "/v1/organizations/{idOrName}/icon",
    operation_id = "uploadOrganizationIcon",
    tag = "Organization/Icons",
    request_body(
        description = "Multipart form of a single field being the avatar data",
        content = [u8],
        content_type = "multipart/form-data"
    ),
    params(NameOrUlid),
    responses(UpdateAvatarR)
)]
pub async fn upload_org_icon(
    State(env): State<Env>,
    Extension(Session { user, .. }): Extension<Session>,
    Path(id_or_name): Path<NameOrUlid>,
    data: Multipart,
) -> api::Result<Url> {
    let model = db::organization::as_model_with_additional_bounds(&env.db, id_or_name.clone(), |query| {
        query.filter(organization::Column::Owner.eq(user.id))
    })
    .await?
    .ok_or_else(|| {
        api::err(
            StatusCode::NOT_FOUND,
            (
                api::ErrorCode::EntityNotFound,
                "organization with id or name was not found",
                json!({"idOrName":id_or_name}),
            ),
        )
    })?;

    let mut active = model.clone().into_active_model();
    let hash = env.ds.org_icons(model.id).upload(data).await?;

    active.set(organization::Column::IconHash, Some(hash.clone()).into());
    active.update(&env.db).await.into_system_failure()?;

    let resource = env
        .config
        .base_url
        .unwrap()
        .join(&format!("/organizations/{}/icons/{hash}", model.id))
        .into_system_failure()?;

    Ok(api::ok(StatusCode::ACCEPTED, resource))
}
