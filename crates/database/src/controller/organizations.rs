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

// use super::DbController;
// use crate::{impl_paginate, impl_patch_for};
// use async_trait::async_trait;
// use charted_common::models::{
//     entities::Organization,
//     payloads::{CreateOrganizationPayload, PatchOrganizationPayload},
// };
// use eyre::{Report, Result};
use sqlx::PgPool;
// use tracing::{error, instrument};

#[derive(Debug, Clone)]
pub struct OrganizationDatabaseController {
    _pool: PgPool,
}

impl OrganizationDatabaseController {
    pub fn new(pool: PgPool) -> OrganizationDatabaseController {
        Self { _pool: pool }
    }
}

// #[async_trait]
// impl DbController for OrganizationDatabaseController {
//     type Patched = PatchOrganizationPayload;
//     type Created = CreateOrganizationPayload;
//     type Entity = Organization;

//     impl_paginate!("organizations" -> Organization);

//     #[instrument(name = "charted.db.organizations.get", skip(self))]
//     async fn get(&self, id: u64) -> Result<Option<Self::Entity>> {
//         panic!("not working")
//         // match sqlx::query_as::<Postgres, Organization>("select organizations.* from organizations where id = $1;")
//         //     .bind(id as i64)
//         //     .execute(&self.pool)
//         //     .await
//         // {
//         //     Ok(opt) => Ok(opt),
//         //     Err(e) => {
//         //         error!(organization.id = %id, error = %e, "unable to query organization by id");
//         //         sentry::capture_error(&e);

//         //         Err(Report::from(e))
//         //     }
//         // }
//     }
// }

/*
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
*/
