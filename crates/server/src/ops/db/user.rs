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
use charted_types::User;
use eyre::Report;
use tracing::{instrument, trace};

#[instrument(name = "charted.server.ops.db.getUser", skip_all)]
pub async fn get<ID: Into<NameOrUlid>>(ctx: &ServerContext, id: ID) -> eyre::Result<Option<User>> {
    let name_or_ulid = id.into();
    let mut conn = ctx.pool.get()?;

    connection!(@raw conn {
        PostgreSQL(conn) => conn.build_transaction().read_only().run::<_, eyre::Report, _>(|txn| {
            use postgresql::users::{dsl::*, table};
            use diesel::pg::Pg;

            // We have to box the query since we need to match over either a
            // ULID or Username and we can't do that if it isn't boxed.
            let mut query = table
                .into_boxed()
                .select(<User as SelectableHelper<Pg>>::as_select());

            query = match name_or_ulid {
                NameOrUlid::Ulid(uid) => query.filter(id.eq(uid)),
                NameOrUlid::Name(user_name) => query.filter(username.eq(user_name)),
            };

            match query.first(txn) {
                Ok(user) => Ok(Some(user)),
                Err(diesel::result::Error::NotFound) => Ok(None),
                Err(e) => Err(Report::from(e))
            }
        });

        SQLite(conn) => conn.immediate_transaction(|txn| {
            use sqlite::users::{dsl::*, table};
            use diesel::sqlite::Sqlite;

            let mut query = table
                .into_boxed()
                .select(<User as SelectableHelper<Sqlite>>::as_select());

            query = match name_or_ulid {
                NameOrUlid::Ulid(uid) => query.filter(id.eq(uid)),
                NameOrUlid::Name(user_name) => query.filter(username.eq(user_name)),
            };

            match query.first(txn) {
                Ok(user) => Ok(Some(user)),
                Err(diesel::result::Error::NotFound) => Ok(None),
                Err(e) => Err(Report::from(e))
            }
        });
    })
    .inspect_err(|err| {
        sentry_eyre::capture_report(err);
    })
}

#[instrument(name = "charted.server.users.delete", skip_all, fields(%user.id, %user.username))]
pub async fn delete(ctx: ServerContext, user: User) -> eyre::Result<()> {
    trace!("deleting user from database");

    Ok(())
}

async fn delete_all_repositories(ctx: &ServerContext, user: &User) -> eyre::Result<()> {
    Ok(())
}

async fn delete_all_organizations(ctx: &ServerContext, user: &User) -> eyre::Result<()> {
    Ok(())
}

async fn delete_persistent_metadata(ctx: &ServerContext, user: &User) -> eyre::Result<()> {
    Ok(())
}
