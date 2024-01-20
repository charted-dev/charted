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
    common::models::NameOrSnowflake,
    server::pagination::{OrderBy, Pagination, PaginationQuery},
};
use serde::{Deserialize, Serialize};

pub mod apikeys;
pub mod organization;
pub mod repository;
pub mod user;

/// Represents a request object to determine how to paginate
/// queries.
#[derive(Debug, Clone)]
pub struct PaginationRequest {
    /// How many entities should be present per page?
    pub per_page: usize,

    /// [`OrderBy`] on how to order entities ascending or descending by
    /// their ID.
    pub order_by: OrderBy,

    /// Optional cursor to passthrough when flipping through pages. This will be
    /// present from the `page_info.next_cursor`/`page_info.prev_cursor` datapoints.
    pub cursor: Option<u64>,

    /// Owner ID to collect, if from `repositories` or `organizations`.
    pub owner_id: Option<u64>,
}

impl From<PaginationQuery> for PaginationRequest {
    fn from(value: PaginationQuery) -> Self {
        Self {
            per_page: value.per_page,
            order_by: value.order,
            owner_id: None,
            cursor: value.cursor,
        }
    }
}

/// Represents a generic database controller where each db controller controls what data
/// is being created, fetched, deleted, updated, or paginated.
#[async_trait]
pub trait DbController: Send + Sync {
    /// Represents what entity this [`DbController`] controls.
    type Entity: Serialize + for<'de> Deserialize<'de>;

    /// Represents the payload for creating the `Entity`.
    type Created: Serialize;

    /// Represents the payload for patching a `Entity`.
    type Patched: Serialize;

    /// Fetch a single `Entity` by its ID.
    async fn get(&self, id: i64) -> eyre::Result<Option<Self::Entity>>;

    /// Fetch a single `Entity` that is constrained to a [`NameOrSnowflake`], usually from
    /// a REST controller.
    async fn get_by<S: Into<NameOrSnowflake> + Send>(&self, nos: S) -> eyre::Result<Option<Self::Entity>>;

    /// Inserts a new `Entity` with a given `Created` payload and a skeleton of what to use
    /// when inserting it.
    async fn create(&self, payload: Self::Created, skeleton: &Self::Entity) -> eyre::Result<()>;

    /// Patch a given `Entity` by its ID with the specified payload.
    async fn patch(&self, id: i64, payload: Self::Patched) -> eyre::Result<()>;

    /// Deletes a `Entity` with their ID.
    async fn delete(&self, id: i64) -> eyre::Result<()>;

    /// Check if `Entity` by their ID exists in the database.
    async fn exists(&self, id: u64) -> eyre::Result<bool>;

    /// Check if `Entity` by the associated [`NameOrSnowflake`] exists in the database
    async fn exists_by<S: Into<NameOrSnowflake> + Send>(&self, nos: S) -> eyre::Result<bool>;

    /// Implements pagination of chunked entities.
    async fn paginate(&self, _request: PaginationRequest) -> eyre::Result<Pagination<Self::Entity>> {
        unimplemented!("associated type doesn't implement `DbController::paginate`")
    }
}
