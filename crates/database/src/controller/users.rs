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
use crate::impl_patch_for;
use async_trait::async_trait;
use charted_common::{
    models::{
        entities::User,
        payloads::{CreateUserPayload, PatchUserPayload},
        NameOrSnowflake,
    },
    server::hash_password,
};
use eyre::{Context, Result};
use sqlx::PgPool;
use tracing::{error, instrument};

#[derive(Debug, Clone)]
pub struct UserDatabaseController {
    pool: PgPool,
}

impl UserDatabaseController {
    pub fn new(pool: PgPool) -> UserDatabaseController {
        UserDatabaseController { pool }
    }
}

#[async_trait]
impl DbController for UserDatabaseController {
    type Patched = PatchUserPayload;
    type Created = CreateUserPayload;
    type Entity = User;

    #[instrument(name = "charted.db.users.get", skip(self))]
    async fn get(&self, id: u64) -> Result<Option<Self::Entity>> {
        match sqlx::query_as::<sqlx::Postgres, User>("select users.* from users where id = $1;")
            .bind(i64::try_from(id).unwrap())
            .fetch_optional(&self.pool)
            .await
        {
            Ok(opt) => Ok(opt),
            Err(e) => {
                error!(user.id = id, error = %e, "unable to query user");
                sentry::capture_error(&e);

                Err(e).context("unable to query user")
            }
        }
    }

    #[instrument(name = "charted.db.users.get_nos", skip(self))]
    async fn get_by_nos(&self, nos: NameOrSnowflake) -> Result<Option<Self::Entity>> {
        match nos {
            NameOrSnowflake::Snowflake(uid) => self.get(uid).await,
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
        sqlx::query("delete from users where id = $1;")
            .bind(i64::try_from(id).unwrap())
            .execute(&self.pool)
            .await
            .map(|_| ())
            .context("unable to delete user")
    }
}
