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

use charted_config::Config;
use charted_database::MIGRATOR;
use eyre::Context;
use owo_colors::{OwoColorize, Stream};
use sqlx::{migrate::Migrate, postgres::PgConnectOptions, ConnectOptions, Connection};
use std::{collections::HashMap, path::PathBuf, process::exit, str::FromStr, time::Duration};
use tracing::{debug, error, warn};

/// Runs all pending migrations on the database
#[derive(Debug, Clone, clap::Parser)]
pub struct Args {
    /// Runs a specific migration only
    #[arg(long, short = 'v')]
    target_version: Option<i64>,

    /// Does not touch the database at all when running migrations. It'll just print
    /// what migrations will run.
    #[arg(long, short = 'd')]
    dry_run: bool,

    /// location to a relative/absolute path to a configuration file. by default, this will locate
    /// in `./config/charted.yml`/`./config.yml` if found.
    #[arg(long, short = 'c', env = "CHARTED_CONFIG_PATH")]
    config: Option<PathBuf>,
}

pub async fn run(
    Args {
        config,
        dry_run,
        target_version,
    }: Args,
) -> eyre::Result<()> {
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
        exit(1);
    }

    let applied: HashMap<_, _> = conn
        .list_applied_migrations()
        .await?
        .into_iter()
        .map(|m| (m.version, m))
        .collect();

    let latest = applied
        .iter()
        .max_by(|(_, x), (_, y)| x.version.cmp(&y.version))
        .map(|(version, _)| *version)
        .unwrap_or(0);

    if let Some(target) = target_version {
        if target > latest {
            error!("version {target} is newer than latest migration {latest}!");
            exit(1);
        }
    }

    let mut has_applied = false;
    for migration in MIGRATOR.iter() {
        if migration.migration_type.is_down_migration() {
            continue;
        }

        match applied.get(&migration.version) {
            Some(m) if m.checksum != migration.checksum => {
                error!("bailing due to version mismatch in migrations");
                exit(1);
            }

            Some(_) => {
                eprintln!(
                    "⏭️     migration {:<25} ({}): {}   0ns",
                    migration.description,
                    migration.migration_type.label(),
                    "Already applied".if_supports_color(Stream::Stderr, |x| x.fg_rgb::<160, 219, 142>())
                );
            }

            None => {
                let skipped = target_version.map(|x| migration.version > x).unwrap_or(false);
                let elapsed = if dry_run || skipped {
                    Duration::new(0, 0)
                } else {
                    conn.apply(migration)
                        .await
                        .inspect(|_| {
                            has_applied = true;
                        })
                        .context(format!("failed to apply migration {}", migration.description))?
                };

                let status = match (dry_run, skipped) {
                    (true, false) => "Can be applied"
                        .if_supports_color(Stream::Stderr, |x| x.fg_rgb::<129, 216, 208>())
                        .to_string(),

                    (false, true) => "Skipped"
                        .if_supports_color(Stream::Stderr, |x| x.fg_rgb::<236, 33, 81>())
                        .to_string(),

                    _ => "Applied"
                        .if_supports_color(Stream::Stderr, |x| x.fg_rgb::<104, 186, 106>())
                        .to_string(),
                };

                let emoji = match (dry_run, skipped) {
                    (true, false) => "💤",
                    (false, true) => "⏭️",
                    _ => "✨",
                };

                eprintln!(
                    "{emoji}    migration {:<25} ({}): {status}    {elapsed:?}",
                    migration.description,
                    migration.migration_type.label(),
                );
            }
        }
    }

    if !dry_run && !has_applied {
        warn!("no new migrations had been applied!");
    }

    let _ = conn.close().await;

    Ok(())
}
