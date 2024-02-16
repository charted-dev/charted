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

use eyre::Context;
use sqlx::{PgPool, Postgres};

use crate::{
    common::models::{
        entities::ApiKey,
        payloads::{CreateApiKeyPayload, PatchApiKeyPayload},
        NameOrSnowflake,
    },
    db::impl_patch_for,
};

#[derive(Clone)]
pub struct DbController {
    pool: PgPool,
}

impl DbController {
    pub fn new(pool: PgPool) -> DbController {
        DbController { pool }
    }
}

#[async_trait]
impl super::DbController for DbController {
    type Patched = PatchApiKeyPayload;
    type Created = CreateApiKeyPayload;
    type Entity = ApiKey;

    #[instrument(name = "charted.database.api_keys.get", skip_all)]
    async fn get(&self, id: i64) -> eyre::Result<Option<Self::Entity>> {
        let query = sqlx::query_as::<Postgres, ApiKey>("select api_keys.* from api_keys where id = $1;").bind(id);

        match query.fetch_optional(&self.pool).await {
            Ok(Some(key)) => Ok(Some(key)),
            Ok(None) => Ok(None),
            Err(e) => {
                error!(apikey.id = id, error = %e, "unable to query api key from db");
                sentry::capture_error(&e);

                Err(e.into())
            }
        }
    }

    #[instrument(name = "charted.database.api_keys.get_by_nos", skip_all)]
    async fn get_by<S: Into<NameOrSnowflake> + Send>(&self, nos: S) -> eyre::Result<Option<Self::Entity>> {
        let nos = nos.into();
        match nos {
            NameOrSnowflake::Snowflake(id) => self.get(id.try_into()?).await,
            NameOrSnowflake::Name(name) => {
                let query =
                    sqlx::query_as::<Postgres, ApiKey>("select api_keys.* from api_keys where name = $1").bind(&name);

                query
                    .fetch_optional(&self.pool)
                    .await
                    .inspect_err(|e| {
                        error!(apikey.name = %name, error = %e, "unable to query repository from db");
                        sentry::capture_error(&e);
                    })
                    .context("unable to query repository by name")
            }
        }
    }

    #[instrument(name = "charted.database.api_keys.create", skip_all)]
    async fn create(&self, _payload: Self::Created, skeleton: &Self::Entity) -> eyre::Result<()> {
        match sqlx::query("insert into api_keys(description, created_at, updated_at, expires_in, scopes, owner, token, name, id) values($1, false, $2, $3, $4, $5, $6, $7, $8);")
            .bind(skeleton.description.as_ref())
            .bind(skeleton.created_at)
            .bind(skeleton.updated_at)
            .bind(skeleton.expires_in)
            .bind(skeleton.scopes)
            .bind(skeleton.owner)
            .bind(skeleton.token.as_ref().unwrap())
            .bind(skeleton.name.clone())
            .bind(skeleton.id)
            .execute(&self.pool)
            .await {
                Ok(_) => Ok(()),
                Err(e) => {
                    error!(apikey.id = skeleton.id, error = %e, "unable to create apikey");
                    sentry::capture_error(&e);

                    Err(e.into())
                }
            }
    }

    #[instrument(name = "charted.db.api_keys.patch", skip(self, payload))]
    async fn patch(&self, id: i64, payload: Self::Patched) -> eyre::Result<()> {
        let mut txn = self
            .pool
            .begin()
            .await
            .inspect_err(|e| {
                error!(apikey.id = id, error = %e, "unable to create db transaction");
                sentry::capture_error(&e);
            })
            .context("unable to create db transaction")?;

        impl_patch_for!(txn, optional, {
            payload: payload.description;
            column:  "description";
            table:   "api_keys";
            id:      id;
        });

        impl_patch_for!(txn, optional, {
            payload: payload.name;
            column:  "name";
            table:   "api_keys";
            id:      id;
        });

        txn.commit()
            .await
            .inspect_err(|e| {
                error!(apikey.id = id, error = %e, "unable to commit db transaction for repository");
                sentry::capture_error(&e);
            })
            .context("unable to commit db transaction")
    }

    #[instrument(name = "charted.db.api_keys.delete", skip(self))]
    async fn delete(&self, id: i64) -> eyre::Result<()> {
        sqlx::query("delete from api_keys where id = $1;")
            .bind(id)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .context("unable to delete repository")
    }

    #[instrument(name = "charted.db.api_keys.exists", skip(self))]
    async fn exists(&self, id: i64) -> eyre::Result<bool> {
        match sqlx::query("select count(1) from api_keys where id = $1;")
            .bind(id)
            .execute(&self.pool)
            .await
        {
            Ok(_) => Ok(true),
            Err(sqlx::Error::ColumnNotFound(_)) => Ok(false),
            Err(e) => Err(e.into()),
        }
    }

    #[instrument(name = "charted.db.api_keys.exists_by_nos", skip_all)]
    async fn exists_by<S: Into<NameOrSnowflake> + Send>(&self, nos: S) -> eyre::Result<bool> {
        let nos = nos.into();
        match nos {
            NameOrSnowflake::Snowflake(id) => self.exists(id.try_into()?).await,
            NameOrSnowflake::Name(name) => {
                match sqlx::query("select count(1) from api_keys where name = $1;")
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
