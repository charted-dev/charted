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

use crate::util;
use charted_config::database::Config;
use diesel::{backend::Backend, migration::MigrationSource};
use diesel_migrations::MigrationHarness;
use eyre::{eyre, Context};
use std::path::PathBuf;
use tracing::info;

/// Runs all the pending migrations.
#[derive(Debug, Clone, clap::Parser)]
pub struct Args {
    /// location to a relative/absolute path to a configuration file. by default, this will locate
    /// in `./config/charted.hcl`/`./config.hcl` if found.
    #[arg(short = 'c', long, env = "CHARTED_CONFIG_FILE")]
    config: Option<PathBuf>,
}

pub fn run(Args { config }: Args) -> eyre::Result<()> {
    info!("running all pending migrations!");

    let mut config = util::load_config(config)?;

    // Don't run pending migrations as this command will run pending migrations.
    config.database = match config.database {
        Config::PostgreSQL(mut cfg) => {
            cfg.run_migrations = false;
            Config::PostgreSQL(cfg)
        }

        Config::SQLite(mut cfg) => {
            cfg.run_migrations = false;
            Config::SQLite(cfg)
        }
    };

    let pool = charted_app::create_db_pool(&config)?;
    let mut conn = pool.get().context("failed to get db connection")?;

    charted_database::connection!(@raw conn {
        PostgreSQL(conn) => run_all_migrations(conn, charted_database::migrations::POSTGRESQL_MIGRATIONS);
        SQLite(conn) => run_all_migrations(conn, charted_database::migrations::SQLITE_MIGRATIONS);
    })?;

    Ok(())
}

fn run_all_migrations<DB: Backend, S: MigrationSource<DB>, H: MigrationHarness<DB>>(
    harness: &mut H,
    source: S,
) -> eyre::Result<()> {
    harness
        .run_pending_migrations(source)
        .map(|_| ())
        .map_err(|e| eyre!("failed to run pending migrations: {e}"))
}
