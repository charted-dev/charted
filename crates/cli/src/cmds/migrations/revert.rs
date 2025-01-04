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
use diesel::{backend::Backend, migration::MigrationSource};
use diesel_migrations::{MigrationError, MigrationHarness};
use eyre::{eyre, Context};
use std::path::PathBuf;
use tracing::info;

/// Reverts `N` or all database migrations.
#[derive(Debug, Clone, clap::Parser)]
pub struct Args {
    /// Reverts `n` amount of migrations.
    #[arg(default_value_t = 1, conflicts_with = "all")]
    amount: u64,

    /// location to a relative/absolute path to a configuration file. by default, this will locate
    /// in `./config/charted.hcl`/`./config.hcl` if found.
    #[arg(short = 'c', long, env = "CHARTED_CONFIG_FILE")]
    config: Option<PathBuf>,

    /// If all migrations should be reverted.
    #[arg(short = 'a', long, default_value_t = false, conflicts_with = "amount")]
    all: bool,
}

// Code for this is from diesel's implementation of `diesel migration revert`:
// https://github.com/diesel-rs/diesel/blob/2a3e7757af05fda4f3cb56f41008171a151cc223/diesel_cli/src/migrations/mod.rs#L34-L62

pub fn run(Args { config, amount, all }: Args) -> eyre::Result<()> {
    let config = util::load_config(config)?;
    let pool = charted_app::create_db_pool(&config)?;
    let mut conn = pool.get().context("failed to get db connection")?;

    if all {
        info!("reverting all database migrations to a fresh state!");

        charted_database::connection!(@raw conn {
            PostgreSQL(conn) => revert_all(conn, charted_database::migrations::POSTGRESQL_MIGRATIONS);
            SQLite(conn) => revert_all(conn, charted_database::migrations::SQLITE_MIGRATIONS);
        })?;

        return Ok(());
    }

    for _ in 0..amount {
        match charted_database::connection!(@raw conn {
            PostgreSQL(conn) => revert_last(conn, charted_database::migrations::POSTGRESQL_MIGRATIONS);
            SQLite(conn) => revert_last(conn, charted_database::migrations::SQLITE_MIGRATIONS);
        }) {
            Ok(()) => {}
            Err(e) if e.is::<MigrationError>() => {
                match e.downcast_ref::<MigrationError>() {
                    // If `amount` was higher than the migrations that were
                    // reverted, then break out of the loop.
                    Some(MigrationError::NoMigrationRun) => break,
                    _ => return Err(eyre!("failed to revert last migration: {e}")),
                }
            }

            Err(e) => return Err(eyre!("failed to revert last migration: {e}")),
        }
    }

    Ok(())
}

fn revert_all<H: MigrationHarness<DB>, S: MigrationSource<DB>, DB: Backend>(
    harness: &mut H,
    source: S,
) -> eyre::Result<()> {
    harness
        .revert_all_migrations(source)
        .map(|_| ())
        .map_err(|e| eyre!("failed to revert all migrations: {e}"))
}

fn revert_last<H: MigrationHarness<DB>, S: MigrationSource<DB>, DB: Backend>(
    harness: &mut H,
    source: S,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    harness.revert_last_migration(source).map(|_| ())
}
