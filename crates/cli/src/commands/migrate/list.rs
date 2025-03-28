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

use crate::commands::server::load_config;
use charted_database::migrations::Migrator;
use cli_table::{Cell, Table, format::Justify};
use sea_orm_migration::{MigrationStatus, MigratorTrait};
use std::path::PathBuf;
use tracing::warn;

#[derive(Table)]
struct CliTable {
    #[table(title = "Name", justify = "Justify::Left")]
    name: String,

    #[table(title = "Applied")]
    applied: &'static str,
}

/// List all avaliable migrations.
#[derive(Debug, clap::Parser)]
pub struct Args {
    /// Path to a charted `config.toml` configuration file.
    #[arg(long, short = 'c', env = "CHARTED_CONFIG_FILE")]
    config: Option<PathBuf>,
}

pub async fn run(Args { config }: Args) -> eyre::Result<()> {
    let mut config = load_config(config)?;
    config.database.common_mut().run_migrations = false;

    let pool = charted_database::create_pool(&config.database).await?;
    let migrations = Migrator::get_migration_with_status(&pool).await?;

    let mut has_pending = false;
    let mut cells = Vec::with_capacity(migrations.len());

    for migration in migrations {
        if migration.status() == MigrationStatus::Pending && !has_pending {
            has_pending = true;
        }

        cells.push(CliTable {
            name: migration.name().to_string(),
            applied: if migration.status() == MigrationStatus::Applied {
                "Yes"
            } else {
                "No"
            },
        });
    }

    if has_pending {
        warn!("you have pending migrations! run `charted migrate run` to apply them");
    }

    let _ = cli_table::print_stdout(cells.table().title(["Name".cell(), "Applied".cell()]));

    Ok(())
}
