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

use crate::db::MIGRATIONS;
use charted_config::Config;
use cli_table::{format::Justify, Cell, Table};
use owo_colors::{OwoColorize, Stream};
use sqlx::{migrate::Migrate, postgres::PgConnectOptions, ConnectOptions, Connection};
use std::{borrow::Cow, collections::HashMap, path::PathBuf, str::FromStr, time::Duration};

#[derive(Table)]
struct Migration {
    #[table(title = "Name", justify = "Justify::Left")]
    name: String,

    #[table(title = "Status")]
    status: String,

    #[table(title = "Checksum")]
    checksum: String,
}

/// Lists all migrations available
#[derive(Debug, Clone, clap::Parser)]
pub struct Args {
    /// location to a relative/absolute path to a configuration file. by default, this will locate
    /// in `./config/charted.yml`/`./config.yml` if found.
    #[arg(long, short = 'c', env = "CHARTED_CONFIG_PATH")]
    config: Option<PathBuf>,
}

pub async fn run(Args { config }: Args) -> eyre::Result<()> {
    debug!("🛰️   Connecting to PostgreSQL...");

    let config = match config {
        Some(ref path) => Config::new(Some(path)),
        None => match Config::find_default_conf_location() {
            Some(path) => Config::new(Some(path)),
            None => Config::new::<&str>(None),
        },
    }?;

    let mut conn = sqlx::postgres::PgConnection::connect_with(
        &PgConnectOptions::from_str(&config.database.to_string())?
            .application_name("charted-server")
            .log_statements(tracing::log::LevelFilter::Trace)
            .log_slow_statements(tracing::log::LevelFilter::Warn, Duration::from_secs(1)),
    )
    .await?;

    debug!("connected to PostgreSQL successfully! ensuring that `migrations` table exists");
    conn.ensure_migrations_table().await?;

    if let Some(version) = conn.dirty_version().await? {
        error!("migration {version} was previously applied but is missing when resolving migrations!");
    }

    let applied: HashMap<_, _> = conn
        .list_applied_migrations()
        .await?
        .into_iter()
        .map(|m| (m.version, m))
        .collect();

    let mut table = Vec::with_capacity(MIGRATIONS.iter().len());
    let mut has_pending = false;

    for migration in MIGRATIONS.iter() {
        if migration.migration_type.is_down_migration() {
            continue;
        }

        let applied = applied.get(&migration.version);
        let (status, mismatched) = if let Some(applied) = applied {
            if applied.checksum != migration.checksum {
                (
                    "Applied (different checksum)"
                        .if_supports_color(Stream::Stderr, |x| x.fg_rgb::<104, 186, 106>())
                        .to_string(),
                    true,
                )
            } else {
                (
                    "Applied"
                        .if_supports_color(Stream::Stderr, |x| x.fg_rgb::<104, 186, 106>())
                        .to_string(),
                    false,
                )
            }
        } else {
            has_pending = true;

            (
                "Pending"
                    .if_supports_color(Stream::Stderr, |x| x.fg_rgb::<236, 33, 81>())
                    .to_string(),
                false,
            )
        };

        table.push(Migration {
            status,
            checksum: hex::encode(&migration.checksum),
            name: migration.description.to_string(),
        });

        if mismatched {
            warn!(
                migration = %migration.description,
                applied.checksum = %applied.map(|x| Cow::Owned(hex::encode(&x.checksum))).unwrap_or_else(|| Cow::Borrowed("<was not applied?!>")),
                local.checksum = hex::encode(&migration.checksum),
                "applied migration checksum is completely different than the locally checked out one"
            );
        }
    }

    let _ = conn.close().await;
    let _ = cli_table::print_stdout(
        table
            .table()
            .title(["Description ".cell(), "Status".cell(), "Checksum".cell()]),
    );

    if has_pending {
        warn!("you have pending migrations to run! use the `charted migrations run` or enable the `database.run_pending_migrations` configuration key when booting up the API server");
    }

    Ok(())
}
