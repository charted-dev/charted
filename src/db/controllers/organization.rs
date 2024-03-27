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
    caching::{CacheWorker, ORGANIZATIONS},
    db::{impl_paginate, impl_patch_for},
};
use charted_entities::{
    payloads::{CreateOrganizationPayload, PatchOrganizationPayload},
    NameOrSnowflake, Organization,
};
use eyre::{Context, Report};
use sqlx::{PgPool, Postgres};
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod member;

#[derive(Clone)]
pub struct DbController {
    pub(in crate::db) worker: Arc<Mutex<dyn CacheWorker<Organization>>>,
    pub(in crate::db) pool: PgPool,
}

#[async_trait]
impl super::DbController for DbController {
    type Patched = PatchOrganizationPayload;
    type Created = CreateOrganizationPayload;
    type Entity = Organization;

    impl_paginate!("organizations");

    #[instrument(name = "charted.database.organizations.get", skip(self))]
    async fn get(&self, id: i64) -> eyre::Result<Option<Self::Entity>> {
        let mut cache = self.worker.lock().await;
        let key = ORGANIZATIONS.join(id.to_string());

        if let Some(cached) = cache.get(key.clone()).await? {
            return Ok(Some(cached));
        }

        let query =
            sqlx::query_as::<Postgres, Organization>("select organizations.* from organizations where id = $1;")
                .bind(id);

        match query.fetch_optional(&self.pool).await {
            Ok(Some(org)) => {
                warn!(organization.id = org.id, cache.key = %key, "cache hit miss");
                cache.put(key, org.clone()).await.map(|()| Some(org))
            }

            Ok(None) => Ok(None),
            Err(e) => {
                error!(organization.id = id, error = %e, "unable to query organization from db");
                sentry::capture_error(&e);

                Err(e.into())
            }
        }
    }

    #[instrument(name = "charted.database.organizations.get_by", skip_all)]
    async fn get_by<S: Into<NameOrSnowflake> + Send>(&self, nos: S) -> eyre::Result<Option<Self::Entity>> {
        let nos = nos.into();
        match nos {
            NameOrSnowflake::Snowflake(id) => self.get(id.try_into()?).await,
            NameOrSnowflake::Name(name) => {
                let query = sqlx::query_as::<Postgres, Organization>(
                    "select organizations.* from organizations where name = $1;",
                )
                .bind(&name);

                query
                    .fetch_optional(&self.pool)
                    .await
                    .inspect_err(|e| {
                        error!(repository.name = %name, error = %e, "unable to query repository from db");
                        sentry::capture_error(&e);
                    })
                    .context("unable to query repository by name")
            }
        }
    }

    #[instrument(name = "charted.database.organizations.create", skip_all)]
    async fn create(&self, _payload: Self::Created, skeleton: &Self::Entity) -> eyre::Result<()> {
        sqlx::query("insert into organizations(display_name, created_at, updated_at, private, owner, name, id) values($1, $2, $3, $4, $5, $6, $7)")
            .bind(skeleton.display_name.as_ref())
            .bind(skeleton.created_at)
            .bind(skeleton.updated_at)
            .bind(skeleton.private)
            .bind(skeleton.owner)
            .bind(skeleton.name.clone())
            .bind(skeleton.id)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .inspect_err(|e| {
                error!(organization.id = skeleton.id, error = %e, "unable to create organization");
                sentry::capture_error(&e);
            })
            .map_err(From::from)
    }

    #[instrument(name = "charted.database.organizations.path", skip(self, payload))]
    async fn patch(&self, id: i64, payload: Self::Patched) -> eyre::Result<()> {
        let mut txn = self.pool.begin().await.map_err(|e| {
            error!(organization.id = id, "unable to open db transaction for organization");
            sentry::capture_error(&e);

            Report::from(e)
        })?;

        impl_patch_for!([txn]: update on [payload.twitter_handle] in table "organizations", in column "twitter_handle" where id = id);
        impl_patch_for!([txn]: update on [payload.gravatar_email] in table "organizations", in column "organizations" where id = id);
        impl_patch_for!([txn]: update on [payload.display_name]   in table "organizations", in column "display_name" where id = id);
        impl_patch_for!([txn]: update on [payload.private]        in table "organizations", in column "private" where id = id; if |_val| true);
        impl_patch_for!([txn]: update on [payload.name]           in table "organizations", in column "name" where id = id);

        txn.commit()
            .await
            .inspect_err(|e| {
                error!(organization.id = id, "unable to commit transaction for db update");
                sentry::capture_error(&e);
            })
            .map_err(From::from)
    }

    #[instrument(name = "charted.db.repositories.delete", skip(self))]
    async fn delete(&self, id: i64) -> eyre::Result<()> {
        let mut cache = self.worker.lock().await;
        cache.delete(ORGANIZATIONS.join(id.to_string())).await?;

        sqlx::query("delete from repositories where id = $1;")
            .bind(id)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .context("unable to delete repository")
    }

    #[instrument(name = "charted.database.organizations.exists", skip(self))]
    async fn exists(&self, id: i64) -> eyre::Result<bool> {
        // look up through cache to make it easier
        let mut cache = self.worker.lock().await;
        if (cache.get(ORGANIZATIONS.join(id.to_string())).await?).is_some() {
            return Ok(true);
        }

        match sqlx::query("select count(1) from repositories where id = $1;")
            .bind(id)
            .execute(&self.pool)
            .await
        {
            Ok(_) => Ok(true),
            Err(sqlx::Error::ColumnNotFound(_)) => Ok(false),
            Err(e) => Err(e.into()),
        }
    }

    #[instrument(name = "charted.database.organizations.exists_by_nos", skip_all)]
    async fn exists_by<S: Into<NameOrSnowflake> + Send>(&self, nos: S) -> eyre::Result<bool> {
        let nos = nos.into();
        match nos {
            NameOrSnowflake::Snowflake(id) => self.exists(id.try_into()?).await,
            NameOrSnowflake::Name(name) => {
                match sqlx::query("select count(1) from repositories where name = $1;")
                    .bind(name)
                    .execute(&self.pool)
                    .await
                {
                    Ok(_) => Ok(true),
                    Err(sqlx::Error::ColumnNotFound(_)) => Ok(false),
                    Err(e) => Err(e.into()),
                }
            }
        }
    }
}
