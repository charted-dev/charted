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

use crate::controllers::users::UserDatabaseController;
use crate::controllers::DatabaseController;
use async_trait::async_trait;
use charted_common::models::{entities::User, NameOrSnowflake};
use eyre::{Context, Result};
use serde::{de::DeserializeOwned, ser::Serialize};
use sqlx::QueryBuilder;

/// Extension trait to implement fetching a entity with a [`NameOrSnowflake`] enum.
#[async_trait]
pub trait SnowflakeExt: private::Sealed {
    type Entity: Serialize + DeserializeOwned;

    /// Retrieve an entity with a [`NameOrSnowflake`]
    async fn get_with_id_or_name(&self, id_or_name: NameOrSnowflake) -> Result<Option<Self::Entity>>;
}

#[async_trait]
impl SnowflakeExt for UserDatabaseController {
    type Entity = User;

    async fn get_with_id_or_name(&self, id_or_name: NameOrSnowflake) -> Result<Option<Self::Entity>> {
        match id_or_name {
            NameOrSnowflake::Snowflake(id) => self.get(id).await,
            NameOrSnowflake::Name(name) => QueryBuilder::new("SELECT * FROM users WHERE name = ")
                .push_bind(name.clone())
                .build_query_as::<User>()
                .fetch_optional(&self.pool)
                .await
                .context(format!("unable to fetch user with name '{name}'")),
        }
    }
}

mod private {
    pub trait Sealed {}

    impl Sealed for crate::controllers::users::UserDatabaseController {}
}
