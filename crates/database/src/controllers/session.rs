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

use charted_common::{box_pin, BoxedFuture};
use charted_entities::Session;
use eyre::{eyre, Context};
use sqlx::{types::Uuid, PgPool, Postgres};
use tracing::{error, instrument};

#[derive(Clone)]
pub struct DbController {
    pool: PgPool,
}

impl super::DbController for DbController {
    type Created = Session;
    type Patched = ();
    type Entity = Session;
    type ID = Uuid;

    #[instrument(
        name = "charted.db.sessions.get",
        skip_all,
        fields(
            session.id = %id
        )
    )]
    fn get(&self, id: Uuid) -> BoxedFuture<eyre::Result<Option<Self::Entity>>> {
        box_pin!([id: copyable id] {
            sqlx::query_as::<Postgres, Session>("select sessions.* from sessions where id = $1;")
                .bind(id)
                .fetch_optional(&self.pool)
                .await
                .inspect_err(|e| {
                    error!(error = %e, session.id = %id, "failed to query session");
                    sentry::capture_error(e);
                })
                .with_context(|| format!("failed to query session [{id}]"))
        })
    }

    fn get_by<'a, S: Into<charted_entities::NameOrSnowflake> + Send + 'a>(
        &'a self,
        _nos: S,
    ) -> BoxedFuture<eyre::Result<Option<Self::Entity>>> {
        box_pin!({ Err(eyre!("`get_by` operation for sessions is not supported")) })
    }

    fn create<'a>(&'a self, _payload: Self::Created, skeleton: &'a Self::Entity) -> BoxedFuture<eyre::Result<()>> {
        box_pin!([session: copyable skeleton] {
            sqlx::query("insert into sessions(id, user_id, access_token, refresh_token) values($1, $2, $3, $4);")
                .bind(session.id)
                .bind(session.user_id)
                .bind(session.access_token.as_ref())
                .bind(session.refresh_token.as_ref())
                .execute(&self.pool)
                .await
                .map(|_| ())
                .inspect_err(|e| {
                    error!(error = %e, %session.id, "failed to create session");
                    sentry::capture_error(e);
                })
                .with_context(|| format!("failed to create session [{}]", session.id))
        })
    }

    fn patch(&self, _id: Self::ID, _payload: Self::Patched) -> BoxedFuture<eyre::Result<()>> {
        box_pin!({ Err(eyre!("`patch` operation for sessions is not supported")) })
    }

    fn delete(&self, id: Self::ID) -> BoxedFuture<eyre::Result<()>> {
        box_pin!([id: copyable id] {
            sqlx::query("delete from sessions where id = $1").bind(id).execute(&self.pool).await.map(|_| ()).inspect_err(|e| {
                error!(error = %e, session.id = %id, "failed to delete session from db");
                sentry::capture_error(e);
            }).with_context(|| format!("failed to delete session [{id}]"))
        })
    }

    fn exists(&self, id: Self::ID) -> BoxedFuture<eyre::Result<bool>> {
        box_pin!([id: copyable id] {
            match sqlx::query("select count(1) from sessions where id = $1").bind(id).execute(&self.pool).await {
                Ok(_) => Ok(true),
                Err(sqlx::Error::ColumnNotFound(_)) => Ok(false),
                Err(e) => Err(e.into())
            }
        })
    }

    fn exists_by<'a, S: Into<charted_entities::NameOrSnowflake> + Send + 'a>(
        &'a self,
        _nos: S,
    ) -> BoxedFuture<eyre::Result<bool>> {
        box_pin!({ Err(eyre!("`exists_by` operation for sessions is not supported")) })
    }
}
