// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2025 Noelware, LLC. <team@noelware.org>
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

use crate::{NameOrUlid, ServerContext};
use charted_database::{
    connection,
    schema::{postgresql, sqlite},
};
use charted_types::ApiKey;
use eyre::Report;
use tracing::instrument;

#[instrument(name = "charted.server.ops.db.getApiKey", skip_all)]
pub async fn get(
    ctx: &ServerContext,
    key: String,
    owner: Option<impl Into<NameOrUlid>>,
) -> eyre::Result<Option<ApiKey>> {
    let owner_uid: Option<NameOrUlid> = owner.map(Into::into);
    let mut conn = ctx.pool.get()?;

    connection!(@raw conn {
        PostgreSQL(conn) => conn.build_transaction().read_only().run::<_, eyre::Report, _>(|txn| {
            use postgresql::api_keys::{dsl::*, table};
            use diesel::pg::Pg;

            let mut query = table.into_boxed().select(<ApiKey as SelectableHelper<Pg>>::as_select()).filter(token.eq(key));
            if let Some(uid) = owner_uid {
                query = match uid {
                    NameOrUlid::Ulid(uid) => query.filter(owner.eq(uid)),
                    NameOrUlid::Name(user_name) => query.filter(owner.eq(user_name)),
                };
            }

            match query.first(txn) {
                Ok(user) => Ok(Some(user)),
                Err(diesel::result::Error::NotFound) => Ok(None),
                Err(e) => Err(Report::from(e))
            }
        });

        SQLite(conn) => conn.immediate_transaction(|txn| {
            use sqlite::api_keys::{dsl::*, table};
            use diesel::sqlite::Sqlite;

            let mut query = table.into_boxed().select(<ApiKey as SelectableHelper<Sqlite>>::as_select()).filter(token.eq(key));
            if let Some(uid) = owner_uid {
                query = match uid {
                    NameOrUlid::Ulid(uid) => query.filter(owner.eq(uid)),
                    NameOrUlid::Name(user_name) => query.filter(owner.eq(user_name)),
                };
            }

            match query.first(txn) {
                Ok(user) => Ok(Some(user)),
                Err(diesel::result::Error::NotFound) => Ok(None),
                Err(e) => Err(Report::from(e))
            }
        });
    })
}
