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

pub mod member;
pub mod release;

use crate::{
    caching::{CacheWorker, REPOSITORIES},
    common::models::{
        entities::Repository,
        payloads::{CreateRepositoryPayload, PatchRepositoryPayload},
        NameOrSnowflake,
    },
    db::{impl_paginate, impl_patch_for},
};
use eyre::Context;
use sqlx::{PgPool, Postgres};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct DbController {
    worker: Arc<Mutex<dyn CacheWorker<Repository>>>,
    pool: PgPool,
}

impl DbController {
    /// Creates a new [`DbController`].
    pub fn new<W: CacheWorker<Repository> + 'static>(worker: W, pool: PgPool) -> DbController {
        DbController {
            pool,
            worker: Arc::new(Mutex::new(worker)),
        }
    }
}

#[async_trait]
impl super::DbController for DbController {
    type Patched = PatchRepositoryPayload;
    type Created = CreateRepositoryPayload;
    type Entity = Repository;

    impl_paginate!("repositories");

    #[instrument(name = "charted.database.users.get", skip(self))]
    async fn get(&self, id: i64) -> eyre::Result<Option<Self::Entity>> {
        let mut cache = self.worker.lock().await;
        let key = REPOSITORIES.join(id.to_string());

        if let Some(cached) = cache.get(key.clone()).await? {
            return Ok(Some(cached));
        }

        let query =
            sqlx::query_as::<Postgres, Repository>("select repositories.* from repositories where id = $1;").bind(id);

        match query.fetch_optional(&self.pool).await {
            Ok(Some(repo)) => {
                warn!(repo.id, cache.key = %key, "cache hit miss");
                cache.put(key, repo.clone()).await.map(|()| Some(repo))
            }

            Ok(None) => Ok(None),
            Err(e) => {
                error!(repository.id = id, error = %e, "unable to query repository from db");
                sentry::capture_error(&e);

                Err(e.into())
            }
        }
    }

    #[instrument(name = "charted.database.repositories.get_by_nos", skip_all)]
    async fn get_by<S: Into<NameOrSnowflake> + Send>(&self, nos: S) -> eyre::Result<Option<Self::Entity>> {
        let nos = nos.into();
        match nos {
            NameOrSnowflake::Snowflake(id) => self.get(id.try_into()?).await,
            NameOrSnowflake::Name(name) => {
                // since we can't get the snowflake by their name (which is used to cache the object
                // without duplication), we will have to hit the db each time, unless we keep a cache
                // of pointers (name -> id) that lives temporarily until it needs to be re-cached.
                //
                // TODO(@auguwu/@spotlightishere): is there any way we can do that or? ^
                let query =
                    sqlx::query_as::<Postgres, Repository>("select repositories.* from repositories where name = $1")
                        .bind(&name);

                query
                    .fetch_optional(&self.pool)
                    .await
                    .map_err(|e| {
                        error!(repository.name = %name, error = %e, "unable to query repository from db");
                        sentry::capture_error(&e);

                        e
                    })
                    .context("unable to query repository by name")
            }
        }
    }

    #[instrument(name = "charted.database.repositories.create", skip_all)]
    async fn create(&self, _payload: Self::Created, skeleton: &Self::Entity) -> eyre::Result<()> {
        // We don't care about the payload that is sent on each new user invocation as
        // it is validated and created by its skeleton so we don't bring down any more
        // dependencies for this db controller.

        match sqlx::query("insert into repositories(created_at, deprecated, description, id, name, owner, private, type, updated_at) values($1, false, $2, $3, $4, $5, $6, $7, $8);")
            .bind(skeleton.created_at)
            .bind(skeleton.description.as_ref())
            .bind(skeleton.id)
            .bind(skeleton.name.clone())
            .bind(skeleton.owner)
            .bind(skeleton.private)
            .bind(skeleton.r#type)
            .bind(skeleton.updated_at)
            .execute(&self.pool)
            .await {
                Ok(_) => Ok(()),
                Err(e) => {
                    error!(repository.id = skeleton.id, error = %e, "unable to create repository");
                    sentry::capture_error(&e);

                    Err(e.into())
                }
            }
    }

    #[instrument(name = "charted.db.repositories.patch", skip(self, payload))]
    async fn patch(&self, id: i64, payload: Self::Patched) -> eyre::Result<()> {
        let mut txn = self
            .pool
            .begin()
            .await
            .map_err(|e| {
                error!(repository.id = id, error = %e, "unable to create db transaction");
                sentry::capture_error(&e);

                e
            })
            .context("unable to create db transaction")?;

        impl_patch_for!(txn, optional, {
            payload: payload.description;
            column:  "description";
            table:   "repositories";
            id:      id;
        });

        impl_patch_for!(txn, optional, {
            payload: payload.private;
            column:  "private";
            table:   "repositories";
            cond:    |_val| true; // just update it either way
            id:      id;
        });

        impl_patch_for!(txn, optional, {
            payload: payload.name;
            column:  "name";
            table:   "repositories";
            id:      id;
        });

        impl_patch_for!(txn, optional, {
            payload: payload.r#type;
            column:  "type";
            table:   "repositories";
            cond:    |_val| true;
            id:      id;
        });

        txn.commit()
            .await
            .map_err(|e| {
                error!(repository.id = id, error = %e, "unable to commit db transaction for repository");
                sentry::capture_error(&e);

                e
            })
            .context("unable to commit db transaction")
    }

    #[instrument(name = "charted.db.repositories.delete", skip(self))]
    async fn delete(&self, id: i64) -> eyre::Result<()> {
        let mut cache = self.worker.lock().await;
        cache.delete(REPOSITORIES.join(id.to_string())).await?;

        sqlx::query("delete from repositories where id = $1;")
            .bind(id)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .context("unable to delete repository")
    }

    #[instrument(name = "charted.db.repositories.exists", skip(self))]
    async fn exists(&self, id: i64) -> eyre::Result<bool> {
        // look up through cache to make it easier
        let mut cache = self.worker.lock().await;
        if (cache.get(REPOSITORIES.join(id.to_string())).await?).is_some() {
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

    #[instrument(name = "charted.db.repositories.exists_by_nos", skip_all)]
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

/*
#[async_trait]
impl super::DbController for DbController {
    type Patched = PatchUserPayload;
    type Created = CreateUserPayload;
    type Entity = User;

    #[instrument(name = "charted.database.users.get", skip(self))]
    async fn get(&self, id: i64) -> eyre::Result<Option<Self::Entity>> {
        let mut cache = self.worker.lock().await;
        let key = USERS.join(id.to_string());

        if let Some(cached) = cache.get(key.clone()).await? {
            return Ok(Some(cached));
        }

        let query = sqlx::query_as::<Postgres, User>("select users.* from users where id = $1;").bind(id);
        match query.fetch_optional(&self.pool).await {
            Ok(Some(user)) => {
                warn!(user.id, cache.key = %key, "cache hit miss");
                cache.put(key, user.clone()).await?;

                Ok(Some(user))
            }

            Ok(None) => Ok(None),
            Err(e) => {
                error!(user.id = id, error = %e, "unable to query user from db");
                sentry::capture_error(&e);

                Err(e.into())
            }
        }
    }

    #[instrument(name = "charted.database.users.get_by_nos", skip_all)]
    async fn get_by<S: Into<NameOrSnowflake> + Send>(&self, nos: S) -> eyre::Result<Option<Self::Entity>> {
        let nos = nos.into();
        match nos {
            NameOrSnowflake::Snowflake(id) => self.get(id.try_into()?).await,
            NameOrSnowflake::Name(name) => {
                // since we can't get the snowflake by their name (which is used to cache the object
                // without duplication), we will have to hit the db each time, unless we keep a cache
                // of pointers (name -> id) that lives temporarily until it needs to be re-cached.
                //
                // TODO(@auguwu/@spotlightishere): is there any way we can do that or? ^
                let query = sqlx::query_as::<Postgres, User>("select users.* from users where name = $1").bind(&name);
                query
                    .fetch_optional(&self.pool)
                    .await
                    .map_err(|e| {
                        error!(user.name = %name, error = %e, "unable to query user from db");
                        sentry::capture_error(&e);

                        e
                    })
                    .context("unable to query user by name")
            }
        }
    }

    #[instrument(name = "charted.database.users.create", skip_all)]
    async fn create(&self, _payload: Self::Created, skeleton: &Self::Entity) -> eyre::Result<()> {
        // We don't care about the payload that is sent on each new user invocation as
        // it is validated and created by its skeleton so we don't bring down any more
        // dependencies for this db controller.

        sqlx::query(
            "insert into users(created_at, updated_at, password, username, email, id) values($1, $2, $3, $4, $5, $6);",
        )
        .bind(skeleton.created_at)
        .bind(skeleton.updated_at)
        .bind(skeleton.password.as_ref())
        .bind(skeleton.username.clone())
        .bind(skeleton.email.clone())
        .bind(skeleton.id)
        .execute(&self.pool)
        .await
        .map(|_| ())
        .map_err(|e| {
            error!(user.id = skeleton.id, error = %e, "unable to create user");
            sentry::capture_error(&e);

            Report::from(e)
        })
    }

    #[instrument(name = "charted.db.users.patch", skip(self, payload))]
    async fn patch(&self, id: i64, payload: Self::Patched) -> eyre::Result<()> {
        let mut txn = self
            .pool
            .begin()
            .await
            .map_err(|e| {
                error!(user.id = id, error = %e, "unable to create db transaction");
                sentry::capture_error(&e);

                e
            })
            .context("unable to create db transaction")?;

        impl_patch_for!(txn, optional, {
            payload: payload.gravatar_email;
            column:  "gravatar_email";
            table:   "users";
            id:      id;
        });

        impl_patch_for!(txn, optional, {
            payload: payload.description;
            column:  "description";
            table:   "users";
            id:      id;
        });

        impl_patch_for!(txn, optional, {
            payload: payload.username;
            column:  "username";
            table:   "users";
            id:      id;
        });

        impl_patch_for!(txn, optional, {
            payload: payload.email;
            column:  "email";
            table:   "users";
            id:      id;
        });

        impl_patch_for!(txn, optional, {
            payload: payload.name;
            column:  "name";
            table:   "users";
            id:      id;
        });

        txn.commit()
            .await
            .map_err(|e| {
                error!(user.id = id, error = %e, "unable to commit db transaction for user");
                sentry::capture_error(&e);

                e
            })
            .context("unable to commit db transaction")
    }

    #[instrument(name = "charted.db.users.delete", skip(self))]
    async fn delete(&self, id: i64) -> eyre::Result<()> {
        let mut cache = self.worker.lock().await;
        cache.delete(USERS.join(id.to_string())).await?;

        sqlx::query("delete from users where id = $1;")
            .bind(id)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .context("unable to delete user")
    }

    /// Check if `Entity` by their ID exists in the database.
    #[instrument(name = "charted.db.users.exists", skip(self))]
    async fn exists(&self, id: i64) -> eyre::Result<bool> {
        // look up through cache to make it easier
        let mut cache = self.worker.lock().await;
        if (cache.get(USERS.join(id.to_string())).await?).is_some() {
            return Ok(true);
        }

        match sqlx::query("select count(1) from users where id = $1;")
            .bind(id)
            .execute(&self.pool)
            .await
        {
            Ok(_) => Ok(true),
            Err(sqlx::Error::ColumnNotFound(_)) => Ok(false),
            Err(e) => Err(e.into()),
        }
    }

    #[instrument(name = "charted.db.users.exists_by_nos", skip_all)]
    async fn exists_by<S: Into<NameOrSnowflake> + Send>(&self, nos: S) -> eyre::Result<bool> {
        let nos = nos.into();
        match nos {
            NameOrSnowflake::Snowflake(id) => self.exists(id.try_into()?).await,
            NameOrSnowflake::Name(name) => match sqlx::query("select count(1) from users where username = $1;")
                .bind(name)
                .execute(&self.pool)
                .await
            {
                Ok(_) => Ok(true),
                Err(sqlx::Error::ColumnNotFound(_)) => Ok(false),
                Err(e) => Err(e.into()),
            },
        }
    }
}

*/
