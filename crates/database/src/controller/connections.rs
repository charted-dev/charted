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

use super::{users::UserDatabaseController, DbController};
use crate::impl_patch_for;
use async_trait::async_trait;
use charted_common::models::{entities::UserConnections, payloads::PatchUserConnectionsPayload, NameOrSnowflake};
use eyre::{Context, Report, Result};
use sqlx::{PgPool, Postgres};
use tracing::{error, instrument};

pub struct UserConnectionsDatabaseController {
    pool: PgPool,
    users: UserDatabaseController,
}

impl UserConnectionsDatabaseController {
    pub fn new(pool: PgPool, users: UserDatabaseController) -> UserConnectionsDatabaseController {
        UserConnectionsDatabaseController { pool, users }
    }
}

#[async_trait]
impl DbController for UserConnectionsDatabaseController {
    type Patched = PatchUserConnectionsPayload;
    type Created = u64;
    type Entity = UserConnections;

    #[instrument(name = "charted.db.users.connections.get", skip(self))]
    async fn get(&self, id: u64) -> Result<Option<Self::Entity>> {
        match sqlx::query_as::<Postgres, UserConnections>(
            "select user_connections.* from user_connections where id = $1;",
        )
        .bind(i64::try_from(id).unwrap())
        .fetch_optional(&self.pool)
        .await
        {
            Ok(opt) => Ok(opt),
            Err(e) => {
                error!(user.id = id, error = %e, "unable to query connections for user");
                sentry::capture_error(&e);

                Err(Report::from(e))
            }
        }
    }

    #[instrument(name = "charted.db.users.connections.get_nos", skip(self))]
    async fn get_by_nos(&self, nos: NameOrSnowflake) -> Result<Option<Self::Entity>> {
        match nos {
            NameOrSnowflake::Snowflake(uid) => self.get(uid).await,
            NameOrSnowflake::Name(ref name) => {
                let user = self.users.get_by_nos(NameOrSnowflake::Name(name.clone())).await?;
                match user {
                    Some(user) => self.get(u64::try_from(user.id).unwrap()).await,
                    None => Ok(None),
                }
            }
        }
    }

    #[instrument(name = "charted.db.users.connections.create", skip(self, id, skeleton))]
    async fn create(&self, id: u64, skeleton: Self::Entity) -> Result<Self::Entity> {
        match sqlx::query("insert into user_connections(created_at, updated_at, id) values($1, $2, $3);")
            .bind(skeleton.created_at)
            .bind(skeleton.updated_at)
            .bind(skeleton.id)
            .execute(&self.pool)
            .await
        {
            Ok(_) => Ok(skeleton),
            Err(e) => {
                error!(user.id = id, error = %e, "unable to create connections for user");
                sentry::capture_error(&e);

                Err(Report::from(e))
            }
        }
    }

    #[instrument(name = "charted.db.users.connections.patch", skip(self, payload))]
    async fn patch(&self, id: u64, payload: Self::Patched) -> Result<()> {
        let mut txn = self.pool.begin().await.map_err(|e| {
            error!(user.id = id, error = %e, "unable to create db transaction");
            sentry::capture_error(&e);

            Report::from(e)
        })?;

        impl_patch_for!(txn, {
            payload: payload.noelware_account_id;
            column:  "noelware_account_id";
            table:   "user_connections";
            as_:     i64;
            id:      i64::try_from(id).unwrap();
        });

        impl_patch_for!(txn, {
            payload: payload.google_account_id.clone();
            column:  "google_account_id";
            table:   "user_connections";
            id:      i64::try_from(id).unwrap();
        });

        impl_patch_for!(txn, {
            payload: payload.github_account_id.clone();
            column:  "github_account_id";
            table:   "user_connections";
            id:      i64::try_from(id).unwrap();
        });

        txn.commit().await.map(|_| ()).map_err(|e| {
            error!(user.id = %id, error = %e, "unable to commit transaction for user");
            sentry::capture_error(&e);

            Report::from(e)
        })
    }

    #[instrument(name = "charted.db.users.connections.delete", skip(self))]
    async fn delete(&self, id: u64) -> Result<()> {
        sqlx::query("delete from user_connections where id = $1;")
            .bind(i64::try_from(id).unwrap())
            .execute(&self.pool)
            .await
            .map(|_| ())
            .context("unable to delete user")
    }
}
