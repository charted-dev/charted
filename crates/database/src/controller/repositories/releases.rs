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

use super::DbController;
use async_trait::async_trait;
use charted_cache_worker::DynamicCacheWorker;
use charted_common::models::{
    entities::RepositoryRelease,
    payloads::{CreateRepositoryReleasePayload, PatchRepositoryReleasePayload},
    NameOrSnowflake,
};
use eyre::Result;
use sqlx::PgPool;
use std::sync::Arc;
use tracing::instrument;

#[derive(Debug, Clone)]
pub struct RepositoryReleasesDatabaseController {
    cache: Arc<DynamicCacheWorker>,
    pool: PgPool,
}

impl RepositoryReleasesDatabaseController {
    pub fn new(pool: PgPool, cache: Arc<DynamicCacheWorker>) -> RepositoryReleasesDatabaseController {
        RepositoryReleasesDatabaseController { cache, pool }
    }
}

#[async_trait]
impl DbController for RepositoryReleasesDatabaseController {
    type Patched = PatchRepositoryReleasePayload;
    type Created = CreateRepositoryReleasePayload;
    type Entity = RepositoryRelease;

    //impl_paginate!("repository_releases" -> RepositoryRelease);

    #[instrument(name = "charted.db.repository.releases.get", skip(self))]
    async fn get(&self, id: u64) -> Result<Option<Self::Entity>> {
        panic!("unimplemented")
    }

    #[instrument(name = "charted.db.repository.releases.get", skip(self))]
    async fn get_by_nos(&self, nos: NameOrSnowflake) -> Result<Option<Self::Entity>> {
        panic!("unimplemented")
    }

    #[instrument(name = "charted.db.repository.releases.get", skip(self))]
    async fn create(&self, payload: Self::Created, skeleton: Self::Entity) -> Result<Self::Entity> {
        panic!("unimplemented")
    }

    #[instrument(name = "charted.db.repository.releases.get", skip(self))]
    async fn patch(&self, id: u64, payload: Self::Patched) -> Result<()> {
        panic!("unimplemented")
    }

    #[instrument(name = "charted.db.repository.releases.delete", skip(self))]
    async fn delete(&self, id: u64) -> Result<()> {
        panic!("unimplemented")
    }
}
