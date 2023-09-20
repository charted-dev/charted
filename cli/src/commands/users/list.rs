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

use charted_common::{cli::AsyncExecute, models::entities::User};
use charted_config::Config;
use cli_table::{format::Border, Table};
use eyre::Result;
use sqlx::{postgres::PgConnectOptions, ConnectOptions, Connection, PgConnection, Postgres};
use std::{path::PathBuf, process::exit, str::FromStr, time::Duration};
use tracing::log::LevelFilter;

#[derive(cli_table::Table)]
struct UserTable {
    #[table(title = "Administrator", order = 2)]
    admin: bool,

    #[table(title = "Verified Publisher", order = 3)]
    verified_publisher: bool,

    #[table(title = "Name", order = 0)]
    name: String,

    #[table(title = "ID", order = 1)]
    id: u64,
}

impl From<User> for UserTable {
    fn from(value: User) -> Self {
        Self {
            admin: value.admin,
            verified_publisher: value.verified_publisher,
            name: match value.name {
                Some(name) => format!("{name} (@{})", value.username),
                None => format!("@{}", value.username),
            },

            id: value.id as u64,
        }
    }
}

/// Lists all users in the database with pagination support
#[derive(Debug, Clone, clap::Parser)]
pub struct List {
    /// Configuration file to locate, this will look in the default locations and
    /// in the `CHARTED_CONFIG_FILE` environment variable. This is used
    /// to connect to a PostgreSQL server to list all users.
    #[arg(long, short = 'c')]
    config: Option<PathBuf>,

    /// Whether if this should be displayed as JSON.
    #[arg(long, short = 'j')]
    json: bool,

    /// The page to flip over like a book.
    page: Option<u64>,
}

#[async_trait]
impl AsyncExecute for List {
    async fn execute(&self) -> Result<()> {
        dotenv::dotenv().unwrap_or_default();
        Config::load(self.config.clone())?;

        let config = Config::get();
        debug!("🛰️   contacting & connecting to PostgreSQL...");

        let mut conn = PgConnection::connect_with(
            &PgConnectOptions::from_str(config.database.to_string().as_str())?
                .log_slow_statements(LevelFilter::Warn, Duration::from_secs(10))
                .log_statements(LevelFilter::Trace),
        )
        .await?;

        let users = sqlx::query_as::<Postgres, User>("select users.* from users offset $1 limit 10")
            .bind(self.page.map(|x| x as i64).unwrap_or(0))
            .fetch_all(&mut conn)
            .await?;

        if users.is_empty() {
            // print empty array
            if self.json {
                eprintln!("[]");
                return Ok(());
            }

            warn!("no users were found.");
            return Ok(());
        }

        if self.json {
            let users = serde_json::to_string(&users).unwrap();
            eprintln!("{users}");

            return Ok(());
        }

        let table = users.into_iter().map(UserTable::from).collect::<Vec<_>>().table();
        eprintln!("{}", table.display().unwrap());

        Ok(())
    }
}
