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

use crate::{extract::Path, ops, ServerContext, Yaml};
use axum::extract::State;
use charted_core::response::{internal_server_error, ApiResponse};
use charted_database::controllers::{users, DbController};
use charted_entities::{helm::ChartIndex, NameOrSnowflake};
use charted_proc_macros::{controller, generate_response_schema};

pub struct ChartIndexResponse;
generate_response_schema!(ChartIndexResponse, schema = "ChartIndex");

/// Returns a chart index for a user or organization.
#[controller(
    tags("Main", "Users", "Organizations"),
    pathParameter("idOrName", schema!("NameOrSnowflake"), description = "Parameter that intakes a `Name` or `Snowflake` identifier."),
    response(200, "Chart index for a specific user or organization", ("text/yaml", response!("ChartIndexResponse"))),
    response(404, "User or Organization was not found", ("application/json", response!("ApiErrorResponse"))),
    response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
)]
pub async fn get_chart_index(
    State(ctx): State<ServerContext>,
    Path(nos): Path<NameOrSnowflake>,
) -> Result<Yaml<ChartIndex>, ApiResponse> {
    let users = ctx.controllers.get::<users::DbController>().unwrap();
    match users.get_by(&nos).await {
        Ok(Some(user)) => return ops::pull_index(&ctx.storage, "User", user.id).await.map(Yaml::ok),
        Ok(None) => {} // continue if `nos` points to an organization
        Err(_) => return Err(internal_server_error()),
    };

    todo!()
}
