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
    caching::{CacheWorker, USERS},
    common::models::{
        entities::User,
        payloads::{CreateUserPayload, PatchUserPayload},
        NameOrSnowflake,
    },
};
use eyre::Context;
use sqlx::{PgPool, Postgres};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct DbController {
    pool: PgPool,
    worker: Arc<Mutex<dyn CacheWorker<User>>>,
}

impl DbController {
    /// Creates a new [`DbController`].
    pub fn new<W: CacheWorker<User> + 'static>(worker: W, pool: PgPool) -> DbController {
        DbController {
            pool,
            worker: Arc::new(Mutex::new(worker)),
        }
    }
}

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

    /// Fetch a single `Entity` that is constrained to a [`NameOrSnowflake`], usually from
    /// a REST controller.
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

    /// Inserts a new `Entity` with a given `Created` payload and a skeleton of what to use
    /// when inserting it.
    async fn create(&self, payload: Self::Created, skeleton: &Self::Entity) -> eyre::Result<()> {
        todo!()
    }

    /// Patch a given `Entity` by its ID with the specified payload.
    async fn patch(&self, id: i64, payload: Self::Patched) -> eyre::Result<()> {
        todo!()
    }

    /// Deletes a `Entity` with their ID.
    async fn delete(&self, id: i64) -> eyre::Result<()> {
        todo!()
    }

    /// Check if `Entity` by their ID exists in the database.
    async fn exists(&self, id: u64) -> eyre::Result<bool> {
        todo!()
    }

    /// Check if `Entity` by the associated [`NameOrSnowflake`] exists in the database
    async fn exists_by<S: Into<NameOrSnowflake> + Send>(&self, nos: S) -> eyre::Result<bool> {
        todo!()
    }
}

/*
#[async_trait]
impl DbController for UserDatabaseController {
    #[instrument(name = "charted.db.users.get_nos", skip(self))]
    async fn get_by_nos(&self, nos: NameOrSnowflake) -> Result<Option<Self::Entity>> {
        let mut cache = self.worker.lock().await;

        match nos {
            NameOrSnowflake::Snowflake(uid) => {
                let key = CacheKey::user(uid.try_into().unwrap());
                if let Some(cached) = cache.get(key).await? {
                    return Ok(Some(cached));
                }

                self.get(uid).await
            }

            // we can't do cache calucations on Names since we don't need
            // duplication, but maybe a "pointer" to their ID?
            //
            // TODO(@auguwu): determine if pointers (point name -> id) to a resource is acceptable
            NameOrSnowflake::Name(ref name) => {
                match sqlx::query_as::<sqlx::Postgres, User>("select users.* from users where username = $1;")
                    .bind(name.to_string())
                    .fetch_optional(&self.pool)
                    .await
                {
                    Ok(opt) => Ok(opt),
                    Err(e) => {
                        error!(user.name = name.to_string(), error = %e, "unable to query user");
                        sentry::capture_error(&e);

                        Err(e).context("unable to query user")
                    }
                }
            }
        }
    }

    #[instrument(name = "charted.db.users.create", skip(self, _payload, skeleton))]
    async fn create(&self, _payload: Self::Created, skeleton: Self::Entity) -> Result<Self::Entity> {
        match sqlx::query(
            "insert into users(created_at, updated_at, password, username, email, id) values($1, $2, $3, $4, $5, $6);",
        )
        .bind(skeleton.created_at)
        .bind(skeleton.updated_at)
        .bind(skeleton.password.clone())
        .bind(skeleton.username.clone())
        .bind(skeleton.email.clone())
        .bind(skeleton.id)
        .execute(&self.pool)
        .await
        {
            Ok(_) => Ok(skeleton),
            Err(e) => {
                error!(user.id = skeleton.id, error = %e, "unable to create user");
                sentry::capture_error(&e);

                Err(e).context("unable to create user")
            }
        }
    }

    #[instrument(name = "charted.db.users.patch", skip(self, payload))]
    async fn patch(&self, id: u64, payload: Self::Patched) -> Result<()> {
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

        impl_patch_for!(txn, {
            payload: payload.gravatar_email.clone();
            column:  "gravatar_email";
            table:   "users";
            id:      i64::try_from(id).unwrap();
        });

        impl_patch_for!(txn, {
            payload: payload.description.clone();
            column:  "description";
            table:   "users";
            id:      i64::try_from(id).unwrap();
        });

        impl_patch_for!(txn, {
            payload: payload.username.clone();
            column:  "username";
            table:   "users";
            id:      i64::try_from(id).unwrap();
        });

        impl_patch_for!(txn, {
            payload: payload.password.clone();
            column:  "password";
            table:   "users";
            id:      i64::try_from(id).unwrap();

            {
                hash_password(payload.password.unwrap()).map_err(|e| {
                    error!(user.id = id, error = %e, "unable to hash password");
                    sentry::capture_error(&*e);

                    e
                })?
            };
        });

        impl_patch_for!(txn, {
            payload: payload.email.clone();
            column:  "email";
            table:   "users";
            id:      i64::try_from(id).unwrap();
        });

        impl_patch_for!(txn, {
            payload: payload.name.clone();
            column:  "name";
            table:   "users";
            id:      i64::try_from(id).unwrap();
        });

        txn.commit()
            .await
            .map_err(|e| {
                error!(error = %e, "unable to commit transaction for user");
                sentry::capture_error(&e);

                e
            })
            .context("unable to commit transaction")?;

        Ok(())
    }

    #[instrument(name = "charted.db.users.delete", skip(self))]
    async fn delete(&self, id: u64) -> Result<()> {
        // drop the cached value from cache so we don't keep it around
        let mut cache = self.worker.lock().await;
        cache.delete(CacheKey::user(id.try_into().unwrap())).await?;

        sqlx::query("delete from users where id = $1;")
            .bind(i64::try_from(id).unwrap())
            .execute(&self.pool)
            .await
            .map(|_| ())
            .context("unable to delete user")
    }
}

*/
