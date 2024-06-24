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

pub mod releases;

/*
use charted_cache::{CacheWorker, USER};
use charted_common::BoxedFuture;
use charted_entities::{
    payloads::{CreateUserPayload, PatchUserPayload},
    NameOrSnowflake, User,
};
use eyre::Context;
use sqlx::{PgPool, Postgres};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, warn};

use crate::controllers::perform_patch;

pub struct DbController {
    pool: PgPool,
    worker: Arc<Mutex<dyn CacheWorker<User>>>,
}

impl DbController {
    pub fn new<W: CacheWorker<User> + 'static>(pool: PgPool, worker: W) -> DbController {
        DbController {
            pool,
            worker: Arc::new(Mutex::new(worker)),
        }
    }
}

impl super::DbController for DbController {
    type Patched = PatchUserPayload;
    type Created = CreateUserPayload;
    type Entity = User;

    fn get(&self, id: i64) -> BoxedFuture<eyre::Result<Option<Self::Entity>>> {
        Box::pin(async move {
            let mut cache = self.worker.lock().await;
            if let Some(cached) = cache.get(USER.join(id.to_string())).await? {
                return Ok(Some(cached));
            }

            match sqlx::query_as::<Postgres, User>("select users.* from users where id = $1;")
                .bind(id)
                .fetch_optional(&self.pool)
                .await
            {
                Ok(Some(user)) => {
                    let cache_key = USER.join(id.to_string());

                    warn!(user.id, cache.key = %cache_key, "cache hit miss");
                    cache.put(cache_key, user.clone()).await?;

                    Ok(Some(user))
                }

                Ok(None) => Ok(None),
                Err(e) => Err(Into::into(e)),
            }
        })
    }

    fn get_by<'a, S: Into<NameOrSnowflake> + Send + 'a>(
        &'a self,
        nos: S,
    ) -> BoxedFuture<eyre::Result<Option<Self::Entity>>> {
        Box::pin(async move {
            let nos = nos.into();
            match nos {
                NameOrSnowflake::Snowflake(id) => self.get(id.try_into()?).await,
                NameOrSnowflake::Name(name) => {
                    // We have to perform a database request every time a `Name` is passed
                    // since we only cache users by their ID. We could do a pointer table
                    // where names can be pointed to a snowflake and see if a cached
                    // version already exists, but I don't really know.
                    //
                    // TODO(@auguwu): is this possible or a bad idea?
                    sqlx::query_as::<Postgres, User>("select users.* from users where username = $1;")
                        .bind(&name)
                        .fetch_optional(&self.pool)
                        .await
                        .inspect_err(|e| {
                            error!(user.name = %name, error = %e, "failed to get user by username");
                            sentry::capture_error(e);
                        })
                        .with_context(|| format!("unable to get user by username [@{name}]"))
                }
            }
        })
    }

    fn create<'a>(&'a self, _payload: Self::Created, skeleton: &'a Self::Entity) -> BoxedFuture<eyre::Result<()>> {
        Box::pin(async move {
            // We can care less about the actual payload that was sent through here. It is already
            // validated in when a user is created anyway.
            sqlx::query(
                "insert into users(created_at, updated_at, password, username, email, id) values($1, $2, $3, $4, $5, $6);",
            )
            .bind(skeleton.created_at)
            .bind(skeleton.updated_at)
            .bind(skeleton.password.as_ref())
            .bind(&skeleton.username)
            .bind(&skeleton.email)
            .bind(skeleton.id)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .inspect_err(|e| {
                error!(error = %e, user.id = skeleton.id, "failed to create user");
                sentry::capture_error(e);
            })
            .with_context(|| format!("failed to create user @{} ({})", skeleton.username, skeleton.id))
        })
    }

    fn patch(&self, id: i64, payload: Self::Patched) -> BoxedFuture<eyre::Result<()>> {
        Box::pin(async move {
            let mut txn = self
                .pool
                .begin()
                .await
                .inspect_err(|e| {
                    error!(user.id = id, error = %e, "unable to start db transaction");
                    sentry::capture_error(e);
                })
                .context("was unable to create db transaction")?;

            super::patch!(optional [txn, payload.gravatar_email]: table "users", column "gravatar_email", where id = id; if |val| val.is_empty());
            super::patch!(optional [txn, payload.description]: table "users", column "description", where id = id; if |val| val.is_empty());

            // We use `true` to short-circuit to automatically apply since it'll either return `Some(<value>)` or `None`
            // in the main implementation of the route on the three queries.
            super::patch!(optional [txn, payload.password]: table "users", column "password", where id = id; if |_value| true);
            super::patch!(optional [txn, payload.username]: table "users", column "username", where id = id; if |_value| true);
            super::patch!(optional [txn, payload.email]: table "users", column "email", where id = id; if |_value| true);
            super::patch!(optional [txn, payload.name]: table "users", column "name", where id = id; if |val| val.is_empty());

            txn.commit()
                .await
                .inspect_err(|e| {
                    error!(user.id = id, error = %e, "failed to commit user update transaction");
                    sentry::capture_error(e);
                })
                .context("failed to commit transaction")
        })
    }

    fn delete(&self, id: i64) -> BoxedFuture<eyre::Result<()>> {
        Box::pin(async move {
            let mut cache = self.worker.lock().await;

            // forget about the result since a cached result might not exist,
            // so we could care less tbh -- unless it DOES exist and the service
            // has failed to remove it, then we should care... but that's a problem
            // for future Noel, not present Noel
            let _ = cache.delete(USER.join(id.to_string())).await;

            sqlx::query("delete from users where id = $1")
                .bind(id)
                .execute(&self.pool)
                .await
                .map(|_| ())
                .with_context(|| format!("unable to delete user with id [{id}]"))
        })
    }

    fn exists(&self, id: i64) -> BoxedFuture<eyre::Result<bool>> {
        Box::pin(async move {
            // lookup if a user exists in cache first as a "fast" way
            let mut cache = self.worker.lock().await;
            if (cache.get(USER.join(id.to_string())).await?).is_some() {
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
        })
    }

    fn exists_by<'a, S: Into<NameOrSnowflake> + Send + 'a>(&'a self, nos: S) -> BoxedFuture<eyre::Result<bool>> {
        Box::pin(async move {
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
        })
    }
}

*/
