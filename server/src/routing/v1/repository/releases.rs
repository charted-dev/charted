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
    middleware::Session,
    models::res::{err, ok, Result},
    Server,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Extension, Router,
};
use charted_common::{models::entities::RepositoryRelease, server::pagination::PaginationQuery, ID};
use charted_database::controller::repositories::{RepositoryDatabaseController, RepositoryReleasesDatabaseController};
use charted_server_proc_macro::controller;

pub fn create_router() -> Router<Server> {
    Router::new()
}

/// Retrieve a list of all a repository's releases.
#[controller(
    id = "get_repository_releases",
    tags("Repositories"),
    securityRequirements(
        ("ApiKey", ["repo:releases:access"]),
        ("Bearer", []),
        ("Basic", [])
    ),
    response(200, "Successful response", ("application/json", response!("RepositoryReleasePaginatedResponse"))),
    pathParameter("id", snowflake, description = "[`Snowflake`] identifier that resolves a repository ID"),
    queryParameter("cursor", snowflake, description = "Cursor to passthrough to proceed into the next or previous page."),
    queryParameter("per_page", int32, description = "How many elements should be present in a page."),
    queryParameter("order", schema!("OrderBy"), description = "Order to sort the entries by.")
)]
pub async fn get_releases(
    State(Server { controllers, .. }): State<Server>,
    Path(id): Path<ID>,
    Query(PaginationQuery {
        mut per_page,
        cursor,
        order,
    }): Query<PaginationQuery>,
    session: Option<Extension<Session>>,
) -> Result<Vec<RepositoryRelease>> {
    Ok(ok(StatusCode::OK, vec![]))
}
