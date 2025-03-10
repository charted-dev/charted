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

use crate::{Context, Yaml, extract::Path, openapi::ApiErrorResponse};
use axum::{extract::State, http::StatusCode};
use charted_core::api;
use charted_database::entities::{OrganizationEntity, UserEntity, organization, user};
use charted_helm_types::ChartIndex;
use charted_types::{NameOrUlid, Organization, Ulid, User, name::Name};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde_json::json;
use std::collections::BTreeMap;
use tracing::instrument;
use utoipa::{
    IntoResponses, PartialSchema, ToSchema,
    openapi::{Content, Ref, RefOr, Response},
};

struct R;
impl IntoResponses for R {
    fn responses() -> BTreeMap<String, RefOr<Response>> {
        azalia::btreemap!(
            "200" => utoipa::openapi::Response::builder()
                .content(
                    "application/yaml",
                    Content::builder()
                        .schema(Some(RefOr::Ref(Ref::from_schema_name(ChartIndex::name()))))
                        .build()
                ),

            "404" => utoipa::openapi::Response::builder()
                .description("User or Organization wasn't found")
                .content("application/json", Content::builder().schema(Some(ApiErrorResponse::schema())).build()),

            "500" => utoipa::openapi::Response::builder()
                .description("Internal Server Error")
                .content("application/json", Content::builder().schema(Some(ApiErrorResponse::schema())).build())
        )
    }
}

/// Retrieve a chart index from a **User** or **Organization**.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    get,

    path = "/v1/indexes/{idOrName}",
    operation_id = "getChartIndex",
    tag = "Main",
    params(
        ("idOrName" = NameOrUlid, Path)
    ),
    responses(R)
)]
pub async fn fetch(
    State(cx): State<Context>,
    Path(id_or_name): Path<NameOrUlid>,
) -> Result<Yaml<ChartIndex>, api::Response> {
    match id_or_name {
        NameOrUlid::Name(name) => fetch_by_name(&cx, name).await,
        NameOrUlid::Ulid(ulid) => fetch_by_id(&cx, ulid).await,
    }
}

#[instrument(name = "charted.server.indexes.get", skip(cx))]
async fn fetch_by_name(cx: &Context, name: Name) -> Result<Yaml<ChartIndex>, api::Response> {
    if let Some(user) = UserEntity::find()
        .filter(user::Column::Username.eq(name.clone()))
        .one(&cx.pool)
        .await
        .map_err(api::system_failure)?
        .map(Into::<User>::into)
    {
        let index = charted_helm_charts::get_chart_index(&cx.storage, user.id)
            .await
            .map_err(api::system_failure_from_report)?
            .unwrap_or_default();

        return Ok((StatusCode::OK, index).into());
    }

    if let Some(org) = OrganizationEntity::find()
        .filter(organization::Column::Name.eq(name.clone()))
        .one(&cx.pool)
        .await
        .map_err(api::system_failure)?
        .map(Into::<Organization>::into)
    {
        let index = charted_helm_charts::get_chart_index(&cx.storage, org.id)
            .await
            .map_err(api::system_failure_from_report)?
            .unwrap_or_default();

        return Ok((StatusCode::OK, index).into());
    }

    Err(api::err(
        StatusCode::NOT_FOUND,
        (
            api::ErrorCode::EntityNotFound,
            "user or organization by name doesn't exist",
            json!({"name":name}),
        ),
    ))
}

#[instrument(name = "charted.server.indexes.get", skip(cx))]
async fn fetch_by_id(cx: &Context, id: Ulid) -> Result<Yaml<ChartIndex>, api::Response> {
    if let Some(user) = UserEntity::find_by_id(id)
        .one(&cx.pool)
        .await
        .map_err(api::system_failure)?
        .map(Into::<User>::into)
    {
        let index = charted_helm_charts::get_chart_index(&cx.storage, user.id)
            .await
            .map_err(api::system_failure_from_report)?
            .unwrap_or_default();

        return Ok((StatusCode::OK, index).into());
    }

    if let Some(org) = OrganizationEntity::find_by_id(id)
        .one(&cx.pool)
        .await
        .map_err(api::system_failure)?
        .map(Into::<Organization>::into)
    {
        let index = charted_helm_charts::get_chart_index(&cx.storage, org.id)
            .await
            .map_err(api::system_failure_from_report)?
            .unwrap_or_default();

        return Ok((StatusCode::OK, index).into());
    }

    Err(api::err(
        StatusCode::NOT_FOUND,
        (
            api::ErrorCode::EntityNotFound,
            "user or organization by id doesn't exist",
            json!({"id":id}),
        ),
    ))
}
