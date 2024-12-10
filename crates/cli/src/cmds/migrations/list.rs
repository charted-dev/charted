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

use crate::util;
use charted_config::database::Config;
use cli_table::{format::Justify, Cell, Style, Table};
use diesel::{
    backend::Backend,
    migration::{Migration, MigrationSource, MigrationVersion},
    pg::Pg,
    sqlite::Sqlite,
};
use diesel_migrations::MigrationHarness;
use eyre::{eyre, Context};
use std::path::PathBuf;
use tracing::warn;

#[derive(Table)]
struct CliTable {
    #[table(title = "Name", justify = "Justify::Left")]
    name: String,

    #[table(title = "Applied")]
    applied: &'static str,
}

/// Lists all database migrations
#[derive(Debug, Clone, clap::Parser)]
pub struct Args {
    /// location to a relative/absolute path to a configuration file. by default, this will locate
    /// in `./config/charted.hcl`/`./config.hcl` if found.
    #[arg(short = 'c', long, env = "CHARTED_CONFIG_FILE")]
    config: Option<PathBuf>,
}

// Code for this is from diesel's implementation of `diesel migration list`:
// https://github.com/diesel-rs/diesel/blob/2a3e7757af05fda4f3cb56f41008171a151cc223/diesel_cli/src/migrations/mod.rs#L257-L283

pub fn run(Args { config }: Args) -> eyre::Result<()> {
    let mut config = util::load_config(config)?;

    // Don't run pending migrations even if there is some already pending!
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
    let mut conn = pool.get().context("failed to grab db connection")?;
    let applied = charted_database::connection!(@raw conn {
        PostgreSQL(conn) => conn
            .applied_migrations()
            .map_err(|e| eyre!("failed to get migrations from db: {e}"))
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        SQLite(conn) => conn
            .applied_migrations()
            .map_err(|e| eyre!("failed to get migrations from db: {e}"))
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
    });

    match config.database {
        Config::PostgreSQL(_) => {
            let mut migrations =
                MigrationSource::<Pg>::migrations(&charted_database::migrations::POSTGRESQL_MIGRATIONS)
                    .map_err(|e| eyre!("failed to collect migrations: {e}"))?;

            print_migrations(&mut migrations, applied);
        }

        Config::SQLite(_) => {
            let mut migrations =
                MigrationSource::<Sqlite>::migrations(&charted_database::migrations::SQLITE_MIGRATIONS)
                    .map_err(|e| eyre!("failed to collect migrations: {e}"))?;

            print_migrations(&mut migrations, applied);
        }
    }

    Ok(())
}

fn print_migrations<DB: Backend>(migrations: &mut [Box<dyn Migration<DB>>], applied: Vec<MigrationVersion<'static>>) {
    migrations.sort_unstable_by(|a, b| a.name().version().cmp(&b.name().version()));
    let mut has_pending = false;
    let mut cells = Vec::with_capacity(migrations.len());

    for migration in migrations {
        let name = migration.name();
        if !applied.contains(&name.version()) {
            has_pending = true;
        }

        cells.push(CliTable {
            name: name.to_string(),
            applied: if applied.contains(&name.version()) { "Yes" } else { "No" },
        });
    }

    if has_pending {
        warn!("you have pending migrations! please run `charted migrate run` to run them!");
    }

    let _ = cli_table::print_stdout(
        cells
            .table()
            .title(["Name".cell().bold(true), "Applied?".cell().bold(true)]),
    );
}
