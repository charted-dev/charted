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
    models::{
        res::{err, ApiResponse},
        yaml::Yaml,
    },
    Server,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use charted_common::models::{helm::ChartIndex, NameOrSnowflake};
use charted_database::controller::{users::UserDatabaseController, DbController};
use remi_core::StorageService;

/// Returns a `ChartIndex` for a specific user or organization.
#[controller(
    tags("Main", "Users", "Organizations"),
    pathParameter("idOrName", schema!("NameOrSnowflake"), description = "Path parameter that can take a [`Name`] or [`Snowflake`] identifier."),
    response(200, "Helm index for the user or organization", ("text/yaml", response!("ChartIndex"))),
    response(404, "User or Organization was not found", ("application/json", response!("ApiErrorResponse"))),
    response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
)]
pub async fn get_index(
    State(Server {
        controllers, storage, ..
    }): State<Server>,
    Path(nos): Path<NameOrSnowflake>,
) -> Result<Yaml<ChartIndex>, ApiResponse> {
    let users = controllers.get::<UserDatabaseController>();
    match users.get_by_nos(nos.clone()).await {
        Ok(Some(user)) => {
            let contents = storage
                .open(format!("./metadata/{}/index.yaml", user.id))
                .await
                .map_err(|e| {
                    error!(idOrName = tracing::field::display(nos.clone()), error = %e, "unable to perform cdn query");
                    sentry::capture_error(&e);

                    err(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
                    )
                })?
                .unwrap();

            let contents: ChartIndex = serde_yaml::from_slice(contents.as_ref()).map_err(|e| {
                error!(idOrName = tracing::field::display(nos), error = %e, "unable to deserialize contents to `ChartIndex`");
                sentry::capture_error(&e);

                err(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
                )
            })?;

            Ok(Yaml(StatusCode::OK, contents))
        }

        Ok(None) => {
            return Err(err(
                StatusCode::NOT_FOUND,
                (
                    "USER_NOT_FOUND",
                    format!("unable to find user with idOrName {nos}").as_str(),
                )
                    .into(),
            ))
        }

        Err(e) => {
            error!(idOrName = tracing::field::display(nos), error = %e, "unable to find user");
            sentry::capture_error(&*e);

            Err(err(
                StatusCode::INTERNAL_SERVER_ERROR,
                ("INTERNAL_SERVER_ERROR", "Internal Server Error").into(),
            ))
        }
    }
}
