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
    caching::{CacheWorker, REPOSITORIES},
    common::models::{
        entities::RepositoryRelease,
        payloads::{CreateRepositoryReleasePayload, PatchRepositoryReleasePayload},
        NameOrSnowflake,
    },
    db::{controllers::PaginationRequest, impl_patch_for},
    server::pagination::{PageInfo, Pagination},
};
use eyre::{Context, ContextCompat};
use sqlx::{FromRow, PgPool, Postgres, QueryBuilder, Row};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct DbController {
    pub(in crate::db) pool: PgPool,
    pub(in crate::db) worker: Arc<Mutex<dyn CacheWorker<RepositoryRelease>>>,
}

#[async_trait]
impl crate::db::controllers::DbController for DbController {
    type Patched = PatchRepositoryReleasePayload;
    type Created = CreateRepositoryReleasePayload;
    type Entity = RepositoryRelease;

    async fn paginate(&self, request: PaginationRequest) -> eyre::Result<Pagination<Self::Entity>> {
        let mut query = QueryBuilder::<Postgres>::new("select repository_releases.* from repository_releases ");
        if let Some(cursor) = request.cursor {
            query
                .push("where repository_releases.id <= ")
                .push_bind(i64::try_from(cursor).context("cannot convert to `i64`")?)
                .push(" and ");
        } else {
            query.push("where ");
        }

        query
            .push("repository_releases.repository = ")
            .push_bind(
                request
                    .metadata
                    .get("repository")
                    .context("missing `repository` metadata field")?
                    .as_number()
                    .context("wanted `serde_json::Number`")?
                    .as_i64()
                    .unwrap(),
            )
            .push(" ")
            .push(format!("order by id {} ", request.order_by))
            .push("limit ")
            .push_bind((request.per_page as i32) + 1);

        let query = query.build();
        match query.fetch_all(&self.pool).await {
            Ok(entries) => {
                let cursor = if entries.len() < request.per_page {
                    None
                } else {
                    entries.last().map(|entry| entry.get::<i64, _>("id")).map(|e| e as u64)
                };

                Ok(Pagination {
                    data: entries
                        .iter()
                        .filter_map(|row| RepositoryRelease::from_row(row).ok())
                        .collect(),

                    page_info: PageInfo { cursor },
                })
            }

            Err(e) => {
                error!(error = %e, "unable to complete pagination request");
                sentry::capture_error(&e);

                Err(e).context("unable to complete pagination request for [repository_releases] table")
            }
        }
    }

    async fn get(&self, id: i64) -> eyre::Result<Option<Self::Entity>> {
        let mut cache = self.worker.lock().await;
        let key = REPOSITORIES.join("releases").join(id.to_string());

        if let Some(cached) = cache.get(key.clone()).await? {
            return Ok(Some(cached));
        }

        match sqlx::query_as::<Postgres, RepositoryRelease>(
            "select repository_releases.* from repository_releases where id = $1;",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        {
            Ok(Some(release)) => {
                warn!(repo.release.id = release.id, cache.key = %key, "cache hit miss");
                cache.put(key, release.clone()).await.map(|()| Some(release))
            }

            Ok(None) => Ok(None),
            Err(e) => {
                error!(repo.release.id = id, error = %e, "unable to query repository release from database");
                sentry::capture_error(&e);

                Err(e.into())
            }
        }
    }

    async fn get_by<S: Into<NameOrSnowflake> + Send>(&self, nos: S) -> eyre::Result<Option<Self::Entity>> {
        match nos.into() {
            NameOrSnowflake::Snowflake(id) => self.get(id.try_into()?).await,
            NameOrSnowflake::Name(_) => unimplemented!("repository releases do not contain `Name`s"),
        }
    }

    async fn create(&self, _payload: Self::Created, skeleton: &Self::Entity) -> eyre::Result<()> {
        match sqlx::query("insert into repository_releases(repository, update_text, created_at, updated_at, tag, id) values($1, $2, $3, $4, $5, $6, $7);")
            .bind(skeleton.repository)
            .bind(&skeleton.update_text)
            .bind(skeleton.created_at)
            .bind(skeleton.updated_at)
            .bind(&skeleton.tag)
            .bind(skeleton.id)
            .execute(&self.pool)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => {
                error!(apikey.id = skeleton.id, error = %e, "unable to create apikey");
                sentry::capture_error(&e);

                Err(e.into())
            }
        }
    }

    async fn patch(&self, id: i64, payload: Self::Patched) -> eyre::Result<()> {
        let mut txn = self
            .pool
            .begin()
            .await
            .inspect_err(|e| {
                error!(repo.release.id = id, error = %e,"unable to create db transaction");
                sentry::capture_error(&e);
            })
            .context("unable to create db transaction")?;

        impl_patch_for!([txn]: update on [payload.update_text] in table "repository_releases", in column "update_text" where id = id);
        txn.commit()
            .await
            .inspect_err(|e| {
                error!(apikey.id = id, error = %e, "unable to commit db transaction for repository");
                sentry::capture_error(&e);
            })
            .context("unable to commit db transaction")
    }

    async fn delete(&self, id: i64) -> eyre::Result<()> {
        let mut cache = self.worker.lock().await;
        cache.delete(REPOSITORIES.join("releases").join(id.to_string())).await?;

        sqlx::query("delete from repository_releases where id = $1;")
            .bind(id)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .with_context(|| "unable to delete repository release")
    }

    async fn exists(&self, id: i64) -> eyre::Result<bool> {
        let mut cache = self.worker.lock().await;
        if cache
            .exists(&REPOSITORIES.join("releases").join(id.to_string()))
            .await?
        {
            return Ok(true);
        }

        match sqlx::query("select count(1) from repository_releases where id = $1;")
            .bind(id)
            .execute(&self.pool)
            .await
        {
            Ok(_) => Ok(true),
            Err(sqlx::Error::ColumnNotFound(_)) => Ok(false),
            Err(e) => Err(e.into()),
        }
    }

    async fn exists_by<S: Into<NameOrSnowflake> + Send>(&self, nos: S) -> eyre::Result<bool> {
        match nos.into() {
            NameOrSnowflake::Snowflake(id) => self.exists(id.try_into()?).await,
            NameOrSnowflake::Name(_) => unimplemented!("repository releases do not contain `Name`s"),
        }
    }
}
