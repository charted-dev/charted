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
    common::models::{helm::ChartIndex, NameOrSnowflake},
    db::controllers::DbController,
    openapi::generate_response_schema,
    server::{
        controller,
        models::{
            res::{err, internal_server_error, ApiResponse, ErrorCode},
            yaml::Yaml,
        },
    },
    Instance,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use remi::StorageService;
use serde_json::json;

pub struct ChartIndexResponse;
generate_response_schema!(ChartIndexResponse, schema = "ChartIndex");

/// Returns a index of a user or organization's Helm charts
#[controller(
    tags("Main", "Users", "Organizations"),
    pathParameter("idOrName", schema!("NameOrSnowflake"), description = "Path parameter that can take a `Name` or `Snowflake` identifier"),
    response(200, "Chart index for a specific user or organization", ("text/yaml", response!("ChartIndexResponse"))),
    response(404, "User or Organization was not found", ("application/json", response!("ApiErrorResponse"))),
    response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
)]
pub async fn get_chart_index(
    State(Instance {
        controllers, storage, ..
    }): State<Instance>,
    Path(nos): Path<NameOrSnowflake>,
) -> Result<Yaml<ChartIndex>, ApiResponse> {
    match controllers.users.get_by(&nos).await {
        Ok(Some(user)) => {
            let Some(contents) = storage
                .open(format!("./metadata/{}/index.yaml", user.id))
                .await
                .inspect_err(|e| {
                    error!(idOrName = %nos, error = %e, "unable to perform chart index lookup");
                    sentry::capture_error(&e);
                })
                .map_err(|_| internal_server_error())?
            else {
                return Err(err(
                    StatusCode::NOT_FOUND,
                    (
                        ErrorCode::EntityNotFound,
                        "index doesn't exist, this is most definitely a bug",
                        json!({"class":"User","idOrName": nos}),
                    ),
                ));
            };

            let contents: ChartIndex = serde_yaml::from_slice(contents.as_ref()).inspect_err(|e| {
                error!(idOrName = %nos, error = %e, "unable to deserialize contents into `ChartIndex`, was it tampered with?");
                sentry::capture_error(&e);
            }).map_err(|_| internal_server_error())?;

            Ok(Yaml(StatusCode::OK, contents))
        }

        Ok(None) => match controllers.organizations.get_by(&nos).await {
            Ok(Some(org)) => {
                let Some(contents) = storage
                    .open(format!("./metadata/{}/index.yaml", org.id))
                    .await
                    .inspect_err(|e| {
                        error!(idOrName = %nos, error = %e, "unable to perform chart index lookup");
                        sentry::capture_error(&e);
                    })
                    .map_err(|_| internal_server_error())?
                else {
                    return Err(err(
                        StatusCode::NOT_FOUND,
                        (
                            ErrorCode::EntityNotFound,
                            "index doesn't exist, this is most definitely a bug",
                            json!({"class":"Organization","idOrName": nos}),
                        ),
                    ));
                };

                let contents: ChartIndex = serde_yaml::from_slice(contents.as_ref()).inspect_err(|e| {
                    error!(idOrName = %nos, error = %e, "unable to deserialize contents into `ChartIndex`, was it tampered with?");
                    sentry::capture_error(&e);
                }).map_err(|_| internal_server_error())?;

                Ok(Yaml(StatusCode::OK, contents))
            }
            Ok(None) => Err(err(
                StatusCode::NOT_FOUND,
                (
                    ErrorCode::EntityNotFound,
                    "user or organization doesn't exist",
                    json!({"idOrName": nos}),
                ),
            )),

            Err(_) => Err(internal_server_error()),
        },

        Err(_) => Err(internal_server_error()),
    }
}
