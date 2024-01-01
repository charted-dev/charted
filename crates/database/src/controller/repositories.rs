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

mod member;
mod releases;

pub use member::*;
pub use releases::*;

use super::DbController;
use crate::{impl_paginate, impl_patch_for};
use async_trait::async_trait;
use charted_common::models::{
    entities::Repository,
    payloads::{CreateRepositoryPayload, PatchRepositoryPayload},
    NameOrSnowflake,
};
use charted_storage::{Bytes, MultiStorageService, StorageService, UploadRequest};
use eyre::{Context, Result};
use sqlx::{PgPool, Postgres};
use tracing::{error, instrument};

#[derive(Debug, Clone)]
pub struct RepositoryDatabaseController {
    storage: MultiStorageService,
    pool: PgPool,
}

impl RepositoryDatabaseController {
    pub fn new(storage: MultiStorageService, pool: PgPool) -> RepositoryDatabaseController {
        RepositoryDatabaseController { storage, pool }
    }
}

#[async_trait]
impl DbController for RepositoryDatabaseController {
    type Patched = PatchRepositoryPayload;
    type Created = CreateRepositoryPayload;
    type Entity = Repository;

    impl_paginate!("repositories" -> Repository);

    #[instrument(name = "charted.db.repositories.get", skip(self))]
    async fn get(&self, id: u64) -> Result<Option<Self::Entity>> {
        match sqlx::query_as::<Postgres, Repository>("select repositories.* from repositories where id = $1;")
            .bind(i64::try_from(id).unwrap())
            .fetch_optional(&self.pool)
            .await
        {
            Ok(entity) => Ok(entity),
            Err(e) => {
                error!(repository.id = id, error = %e, "unable to query repository");
                sentry::capture_error(&e);

                Err(e.into())
            }
        }
    }

    #[instrument(name = "charted.db.repositories.get_by_name_or_snowflake", skip(self))]
    async fn get_by_nos(&self, nos: NameOrSnowflake) -> Result<Option<Self::Entity>> {
        match nos {
            NameOrSnowflake::Snowflake(uid) => self.get(uid).await,
            NameOrSnowflake::Name(ref name) => {
                name.is_valid()?; // validate it just in case
                match sqlx::query_as::<Postgres, Repository>("select repositories.* from repositories where name = $1;")
                    .bind(name.to_string())
                    .fetch_optional(&self.pool)
                    .await
                {
                    Ok(entity) => Ok(entity),
                    Err(e) => {
                        error!(repository.name = name.to_string(), error = %e, "unable to query repository");
                        sentry::capture_error(&e);

                        Err(e.into())
                    }
                }
            }
        }
    }

    #[instrument(name = "charted.db.repositories.create", skip(self, payload, skeleton))]
    async fn create(&self, payload: Self::Created, skeleton: Self::Entity) -> Result<Self::Entity> {
        // upload the README immediately, if we have it
        if let Some(readme) = &payload.readme {
            self.storage
                .upload(
                    format!("./repositories/{}/README.md", skeleton.id),
                    UploadRequest::default()
                        .with_content_type(Some("text/markdown; charset=utf-8".into()))
                        .with_data(Bytes::from(readme.clone()))
                        .seal(),
                )
                .await?;
        };

        match sqlx::query("insert into repositories(created_at, deprecated, description, id, name, owner, private, type, updated_at) values($1, false, $2, $3, $4, $5, $6, $7, $8);")
            .bind(skeleton.created_at)
            .bind(skeleton.description.clone())
            .bind(skeleton.id)
            .bind(skeleton.name.to_string())
            .bind(skeleton.owner)
            .bind(skeleton.private)
            .bind(skeleton.r#type)
            .bind(skeleton.updated_at)
            .execute(&self.pool)
            .await {
                Ok(_) => Ok(skeleton),
                Err(e) => {
                    error!(repository.id = skeleton.id, error = %e, "unable to create repository");
                    sentry::capture_error(&e);

                    Err(e.into())
                }
            }
    }

    #[instrument(name = "charted.db.repositories.patch", skip(self, payload))]
    async fn patch(&self, id: u64, payload: Self::Patched) -> Result<()> {
        let mut txn = self.pool.begin().await.map_err(|e| {
            error!(repository.id = id, error = %e, "unable to create db transaction");
            sentry::capture_error(&e);

            e
        })?;

        impl_patch_for!(txn, {
            payload: payload.description.clone();
            column:  "description";
            table:   "repositories";
            id:      i64::try_from(id).unwrap();
        });

        impl_patch_for!(txn, {
            payload: payload.private;
            column:  "description";
            table:   "repositories";
            id:      i64::try_from(id).unwrap();
        });

        // update the README immediately, or just delete it if it is a empty
        // string.
        if let Some(readme) = &payload.readme {
            match readme.is_empty() {
                false => {
                    self.storage
                        .upload(
                            format!("./repositories/{}/README.md", id),
                            UploadRequest::default()
                                .with_content_type(Some("text/markdown; charset=utf-8".into()))
                                .with_data(Bytes::from(readme.clone()))
                                .seal(),
                        )
                        .await?
                }

                true => self.storage.delete(format!("./repositories/{}/README.md", id)).await?,
            }
        };

        impl_patch_for!(txn, {
            payload: payload.name.clone();
            column:  "name";
            table:   "repositories";
            id:      i64::try_from(id).unwrap();
        });

        impl_patch_for!(txn, {
            payload: payload.r#type;
            column:  "type";
            table:   "repositories";
            id:      i64::try_from(id).unwrap();
        });

        match txn.commit().await {
            Ok(_) => Ok(()),
            Err(e) => {
                error!(repository.id = id, error = %e, "unable to commit transaction for repository");
                sentry::capture_error(&e);

                Err(e.into())
            }
        }
    }

    #[instrument(name = "charted.db.repositories.delete", skip(self))]
    async fn delete(&self, id: u64) -> Result<()> {
        sqlx::query("delete from repositories where id = $1;")
            .bind(i64::try_from(id).unwrap())
            .execute(&self.pool)
            .await
            .map(|_| ())
            .context("unable to delete user")
    }
}
