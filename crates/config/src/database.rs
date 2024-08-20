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

pub mod postgresql;
pub mod sqlite;

use azalia::config::TryFromEnv;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// The `database {}` block allows to configure the database that charted-server
/// uses to store persistent data like users, repositories, and more.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Config {
    /// Uses [PostgreSQL] as the database driver. This is recommended
    /// for production use.
    ///
    /// [PostgreSQL]: https://postgresql.org
    PostgreSQL(postgresql::Config),

    /// Uses [SQLite] as the database driver. This is the recommended
    /// driver for development and evaluation use or don't need
    /// PostgreSQL running.
    ///
    /// [SQLite]: https://sqlite.org
    SQLite(sqlite::Config),
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Config::PostgreSQL(psql) => {
                f.write_str("postgres://")?;

                match (psql.username.as_ref(), psql.password.as_ref()) {
                    (Some(user), Some(pass)) => write!(f, "{user}:{pass}@")?,
                    (Some(user), None) => write!(f, "{user}:@")?,
                    (None, Some(pass)) => write!(f, "postgres:{pass}@")?,
                    _ => {}
                }

                write!(f, "{}:{}/{}", psql.host, psql.port, psql.database)
            }

            Config::SQLite(sqlite) => write!(f, "sqlite://{}", sqlite.db_path.display()),
        }
    }
}

impl TryFromEnv for Config {
    type Output = Config;
    type Error = eyre::Report;

    fn try_from_env() -> Result<Self::Output, Self::Error> {
        todo!()
    }
}

impl Config {
    pub fn max_connections(&self) -> u32 {
        match self {
            Config::PostgreSQL(psql) => psql.max_connections,
            Config::SQLite(sqlite) => sqlite.max_connections,
        }
    }
}
