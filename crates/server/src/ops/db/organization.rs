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

use crate::{NameOrUlid, ServerContext};
use charted_database::{
    connection,
    schema::{postgresql, sqlite},
};
use charted_types::Organization;
use eyre::Report;
use tracing::instrument;

#[instrument(name = "charted.server.ops.db.getOrganization", skip_all)]
pub async fn get<ID: Into<NameOrUlid>>(
    ServerContext { pool, .. }: &ServerContext,
    id: ID,
) -> eyre::Result<Option<Organization>> {
    let nou = id.into();
    let mut conn = pool.get()?;

    connection!(@raw conn {
        PostgreSQL(conn) => conn.build_transaction().read_only().run::<_, eyre::Report, _>(|txn| {
            use postgresql::organizations::{dsl::*, table};
            use diesel::pg::Pg;

            let mut query = table.into_boxed().select(<Organization as SelectableHelper<Pg>>::as_select());
            query = match nou {
                NameOrUlid::Ulid(ulid) => query.filter(id.eq(ulid)),
                NameOrUlid::Name(n) => query.filter(name.eq(n))
            };

            match query.first(txn) {
                Ok(org) => Ok(Some(org)),
                Err(diesel::result::Error::NotFound) => Ok(None),
                Err(e) => Err(Report::from(e))
            }
        });

        SQLite(conn) => conn.immediate_transaction(|txn| {
            use sqlite::organizations::{dsl::*, table};
            use diesel::sqlite::Sqlite;

            let mut query = table.into_boxed().select(<Organization as SelectableHelper<Sqlite>>::as_select());
            query = match nou {
                NameOrUlid::Ulid(ulid) => query.filter(id.eq(ulid)),
                NameOrUlid::Name(n) => query.filter(name.eq(n))
            };

            match query.first(txn) {
                Ok(org) => Ok(Some(org)),
                Err(diesel::result::Error::NotFound) => Ok(None),
                Err(e) => Err(Report::from(e))
            }
        });
    })
    .inspect_err(|err| {
        sentry_eyre::capture_report(err);
    })
}
