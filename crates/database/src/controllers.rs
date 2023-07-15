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

pub mod users;

use async_trait::async_trait;
use eyre::Result;
use serde::{de::DeserializeOwned, ser::Serialize};
use sqlx::{
    postgres::{PgQueryResult, PgRow},
    FromRow,
};

/// Represents a controller that is used to fetch, create, modify, or delete
/// data from the PostgreSQL database.
#[async_trait]
pub trait DatabaseController: Send + Sync {
    type Patched: Serialize + DeserializeOwned;
    type Created: Serialize + DeserializeOwned;
    type Entity: Serialize + DeserializeOwned + FromRow<'static, PgRow>;

    /// Retrieves the [entity][Entity] from the database with the selected
    /// snowflake, and returns it.
    ///
    /// ## Example
    /// ```no_run
    /// # use charted_database::controllers::users::UserDatabaseController;
    /// #
    /// # let pool = ...;
    /// let users = UserDatabaseController::new(pool.clone());
    /// users.get(1234).await;
    /// // => Result(Some(User { id: 1234, ..Default::default() }))
    /// ```
    async fn get(&self, id: u64) -> Result<Option<Self::Entity>>;

    /// Inserts a new [entity][Entity] into the database with the payload to
    /// specify on how to create this user.
    ///
    /// ## Example
    /// ```no_run
    /// # use charted_database::controllers::users::UserDatabaseController;
    /// #
    /// # let pool = ...;
    /// let users = UserDatabaseController::new(pool.clone());
    /// ```
    async fn create(&self, payload: Self::Created) -> Result<PgQueryResult>;
    // async fn patch(&self, payload: Self::Patched) -> Result<()>;
    // async fn delete(&self, id: u64) -> Result<()>;
}
