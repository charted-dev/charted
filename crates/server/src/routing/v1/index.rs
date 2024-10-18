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

use crate::{extract::Path, openapi::ApiErrorResponse, ops, responses::Yaml, NameOrUlid, ServerContext};
use axum::{extract::State, http::StatusCode};
use charted_core::api;
use charted_types::helm;
use serde_json::json;

/// Retrieve a chart index for a User or Organization.
#[utoipa::path(
    get,
    path = "/v1/indexes/{idOrName}",
    operation_id = "getChartIndex",
    tag = "Main",
    params(
        (
            "idOrName" = NameOrUlid,
            Path,

            description = "Parameter that can take a `Name` or `Ulid`",
            example = json!("noel"),
            example = json!("01J647WVTPF2W5W99H5MBT0YQE")
        ),
    ),
    responses(
        (
            status = 200,
            description = "Chart index for a specific [`User`] or [`Organization`]",
            body = helm::ChartIndex,
            content_type = "application/yaml"
        ),
        (
            status = 404,
            description = "Entity was not found",
            body = ApiErrorResponse,
            content_type = "application/json"
        ),
        (
            status = 500,
            description = "Internal Server Error",
            body = ApiErrorResponse,
            content_type = "application/json"
        )
    )
)]
#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn get_chart_index(
    State(ctx): State<ServerContext>,
    Path(id_or_name): Path<NameOrUlid>,
) -> Result<Yaml<helm::ChartIndex>, api::Response> {
    match ops::db::user::get(&ctx, id_or_name.clone()).await {
        Ok(Some(user)) => {
            let Some(result) = ops::charts::get_index(&ctx, user.id)
                .await
                .map_err(|_| api::internal_server_error())?
            else {
                return Err(api::err(
                    StatusCode::NOT_FOUND,
                    (
                        api::ErrorCode::EntityNotFound,
                        "index for user doesn't exist, this is definitely a bug",
                        json!({"class":"User","id_or_name":id_or_name}),
                    ),
                ));
            };

            Ok((StatusCode::OK, result).into())
        }

        Ok(None) => match ops::db::organization::get(&ctx, id_or_name.clone()).await {
            Ok(Some(org)) => {
                let Some(result) = ops::charts::get_index(&ctx, org.id)
                    .await
                    .map_err(|_| api::internal_server_error())?
                else {
                    return Err(api::err(
                        StatusCode::NOT_FOUND,
                        (
                            api::ErrorCode::EntityNotFound,
                            "index for organization doesn't exist, this is definitely a bug",
                            json!({"class":"Organization","id_or_name":id_or_name}),
                        ),
                    ));
                };

                Ok((StatusCode::OK, result).into())
            }

            Ok(None) => Err(api::err(
                StatusCode::NOT_FOUND,
                (
                    api::ErrorCode::EntityNotFound,
                    "unable to find user or organization",
                    json!({"id_or_name":id_or_name}),
                ),
            )),

            Err(_) => Err(api::internal_server_error()),
        },

        Err(_) => Err(api::internal_server_error()),
    }
}
