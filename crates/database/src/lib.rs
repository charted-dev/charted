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

pub mod migrations;
pub mod schema;

use charted_config::database::Config;
use diesel::{
    connection::{set_default_instrumentation, InstrumentationEvent},
    prelude::*,
    r2d2::{self, ConnectionManager, Pool},
};
use eyre::Context;
use tracing::{error, trace};

/// [`Pool`] that wraps a [`ConnectionManager`] of our multi-connection type.
pub type DbPool = Pool<ConnectionManager<DbConnection>>;

#[derive(diesel::MultiConnection)]
pub enum DbConnection {
    PostgreSQL(diesel::pg::PgConnection),
    SQLite(diesel::sqlite::SqliteConnection),
}

pub fn create_pool(config: &Config) -> eyre::Result<DbPool> {
    // connection string is the Display impl for `Config`.
    let url = config.to_string();
    trace!(database.url = url, "creating pool for db url");

    let manager = ConnectionManager::new(url);
    set_default_instrumentation(|| Some(Box::new(instrumentation)))?;

    Pool::builder()
        .max_size(config.max_connections())
        .error_handler(Box::new(ErrorHandler))
        .build(manager)
        .context("failed to create db pool")
}

#[derive(Debug)]
struct ErrorHandler;
impl<E: std::error::Error + 'static> r2d2::HandleError<E> for ErrorHandler {
    fn handle_error(&self, error: E) {
        sentry::capture_error(&error);
        error!(%error, "failed to manage connection or perform query");
    }
}

pub fn version(pool: &DbPool) -> eyre::Result<String> {
    connection!(pool, {
        PostgreSQL(conn) {
            diesel::define_sql_function! {
                fn version() -> diesel::sql_types::Text;
            }

            diesel::select(version())
                .get_result::<String>(conn)
                .context("failed to get database version")
        };

        SQLite(conn) {
            diesel::define_sql_function! {
                fn sqlite_version() -> diesel::sql_types::Text;
            }

            diesel::select(sqlite_version())
                .get_result::<String>(conn)
                .context("failed to get database version")
        };
    })
}

fn instrumentation(event: InstrumentationEvent<'_>) {
    match event {
        InstrumentationEvent::BeginTransaction { depth, .. } => {
            trace!("started transation (depth={depth})");
        }

        InstrumentationEvent::CommitTransaction { depth, .. } => {
            trace!("transaction with depth [{depth}] was committed");
        }

        InstrumentationEvent::RollbackTransaction { depth, .. } => {
            trace!("transaction with depth [{depth}] was rolled back");
        }

        InstrumentationEvent::FinishQuery { query, error, .. } => {
            trace!(sql = %query, "finished query{}", if error.is_some() { " with an error" } else { "" });
            if let Some(err) = error {
                sentry::capture_error(err);
            }
        }

        InstrumentationEvent::StartQuery { query, .. } => {
            trace!(sql = %query, "starting query");
        }

        _ => {}
    }
}

#[macro_export]
macro_rules! connection {
    (@raw $conn:ident {
        $(
            $db:ident($c:ident) => $code:expr;
        )*
    }) => {{
        #[allow(unused)]
        use ::diesel::prelude::*;
        match *$conn {
            $(
                $crate::DbConnection::$db(ref mut $c) => $code,
            )*
        }
    }};

    (@raw $conn:ident {
        $(
            $db:ident($c:ident) $code:block;
        )*
    }) => {{
        #[allow(unused)]
        use ::diesel::prelude::*;
        match *$conn {
            $(
                $crate::DbConnection::$db(ref mut $c) => $code,
            )*
        }
    }};

    ($pool:ident, {
        $(
            $db:ident($conn:ident) $code:block;
        )*
    }) => {{
        #[allow(unused)]
        use ::eyre::Context;

        let mut conn = ($pool).get().context("failed to get db connection")?;
        $crate::connection!(@raw conn {
            $(
                $db($conn) $code;
            )*
        })
    }};
}

#[cfg(test)]
mod tests {
    use charted_config::database::{sqlite, Config};
    use std::path::PathBuf;

    #[test]
    fn test_sqlite_version() {
        let db = crate::create_pool(&Config::SQLite(sqlite::Config {
            db_path: PathBuf::from(":memory:"),
            max_connections: 1,
            run_migrations: false,
        }))
        .expect("failed to create in-memory sqlite database");

        let Ok(s) = crate::version(&db) else {
            panic!("failed to get sqlite version")
        };

        assert!(!s.is_empty());
    }
}
