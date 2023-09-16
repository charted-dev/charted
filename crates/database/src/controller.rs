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

pub mod apikeys;
pub mod connections;
pub mod organizations;
pub mod repositories;
pub mod users;

use self::{
    connections::UserConnectionsDatabaseController, organizations::OrganizationDatabaseController,
    repositories::RepositoryDatabaseController, users::UserDatabaseController,
};
use async_trait::async_trait;
use charted_common::{
    models::NameOrSnowflake,
    server::pagination::{OrderBy, Pagination, PaginationQuery},
};
use charted_storage::MultiStorageService;
use eyre::{eyre, Result};
use serde::{de::DeserializeOwned, ser::Serialize};
use sqlx::PgPool;
use std::{any::Any, sync::Arc};

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

#[async_trait]
pub trait DbController: Send + Sync {
    /// Type that represents a way to "patch" the given [`Entity`] in this
    /// database controller.
    type Patched: Serialize + DeserializeOwned;

    /// Type that represents a way to create the given [`Entity`] in this
    /// database controller.
    type Created: Serialize + DeserializeOwned;

    /// Type that represents the "entity" that it belongs towards.
    type Entity: Serialize + DeserializeOwned;

    async fn paginate(&self, _request: PaginationRequest) -> Result<Pagination<Self::Entity>> {
        Err(eyre!("pagination is not supported on this DbController"))
    }

    /// Fetch a single row and returns an Option variant of the entity to determine
    /// its existence.
    async fn get(&self, id: u64) -> Result<Option<Self::Entity>>;

    /// Fetch a single row by the [`NameOrSnowflake`].
    async fn get_by_nos(&self, nos: NameOrSnowflake) -> Result<Option<Self::Entity>>;

    /// Creates a new entity and returns it from the `skeleton`. The `skeleton` is provided
    /// to provide a "base" on how to insert it into the database.
    async fn create(&self, payload: Self::Created, skeleton: Self::Entity) -> Result<Self::Entity>;

    /// Patches a given entity with the payload.
    async fn patch(&self, id: u64, payload: Self::Patched) -> Result<()>;

    /// Deletes the entity by its ID.
    async fn delete(&self, id: u64) -> Result<()>;
}

#[derive(Clone)]
pub struct DbControllerRegistry(Vec<Arc<dyn Any + Send + Sync>>);

impl DbControllerRegistry {
    pub fn new(storage: MultiStorageService, pool: PgPool) -> DbControllerRegistry {
        let users = UserDatabaseController::new(pool.clone());
        let connections = UserConnectionsDatabaseController::new(pool.clone(), users.clone());

        DbControllerRegistry(vec![
            Arc::new(users),
            Arc::new(connections),
            Arc::new(RepositoryDatabaseController::new(storage.clone(), pool.clone())),
            Arc::new(OrganizationDatabaseController::new(pool.clone())),
        ])
    }

    pub fn get<DB: DbController + 'static>(&self) -> &DB {
        let Some(db) = self.0.iter().find(|s| s.is::<DB>()) else {
            panic!("unable to find controller");
        };

        db.downcast_ref().unwrap()
    }
}
