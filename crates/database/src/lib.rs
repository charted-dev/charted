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
    prelude::*,
    r2d2::{self, ConnectionManager, Pool},
};
use eyre::Context;
use tracing::error;

pub type DbPool = Pool<ConnectionManager<DbConnection>>;

#[derive(diesel::MultiConnection)]
pub enum DbConnection {
    PostgreSQL(diesel::pg::PgConnection),
    SQLite(diesel::sqlite::SqliteConnection),
}

pub fn create_pool(config: &Config) -> eyre::Result<DbPool> {
    // connection string is the Display impl for `Config`.
    let url = config.to_string();
    let manager = ConnectionManager::new(url);

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

#[macro_export]
macro_rules! connection {
    ($pool:ident, {
        $(
            $db:ident => |$conn:ident| $code:block;
        )*
    }) => {{
        #[allow(unused)]
        use ::diesel::prelude::*;
        use ::eyre::Context;

        let mut conn = ($pool).get().context("failed to get connection from pool")?;
        match *conn {
            $(
                $crate::DbConnection::$db(ref mut $conn) => $code,
            )*
        }
    }};
}
