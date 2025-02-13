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

use charted_config::database::Config;
use charted_core::serde::Duration;
use migrations::Migrator;
use sea_orm::{
    metric::Info, ConnectOptions, DatabaseBackend, DatabaseConnection, SqlxPostgresConnector, SqlxSqliteConnector,
};
use sea_orm_migration::MigratorTrait;
use std::ops::Deref;
use tracing::{info, instrument, trace};

pub mod entities;
pub mod migrations;

#[instrument(name = "charted.database.createDbPool", skip_all)]
pub async fn create_pool(config: &Config) -> eyre::Result<DatabaseConnection> {
    info!("establishing database connection");
    let mut conn = match config {
        Config::PostgreSQL(_) => SqlxPostgresConnector::connect(connect_options_with(config)).await?,
        Config::SQLite(_) => SqlxSqliteConnector::connect(connect_options_with(config)).await?,
    };

    conn.set_metric_callback(metric_callback);
    if config.common().run_migrations {
        info!("now running pending migrations!");

        Migrator::up(&conn, None).await?;
    }

    Ok(conn)
}

fn metric_callback(info: &Info<'_>) {
    let elapsed: Duration = info.elapsed.into();
    let backend = match info.statement.db_backend {
        DatabaseBackend::Sqlite => "sqlite",
        DatabaseBackend::MySql => "mysql",
        DatabaseBackend::Postgres => "postgres",
    };

    trace!(%elapsed, failed = %info.failed, %backend, stmt.sql = info.statement.sql, stmt.values = ?info.statement.values);
}

fn connect_options_with(config: &Config) -> ConnectOptions {
    let common = config.common();

    ConnectOptions::new(config.to_string())
        .max_connections(common.max_connections)
        .acquire_timeout(common.acquire_timeout.deref().into())
        .connect_timeout(common.connect_timeout.deref().into())
        .idle_timeout(common.idle_timeout.deref().into())
        .sqlx_logging_level(tracing::log::LevelFilter::Trace)
        .sqlx_slow_statements_logging_settings(tracing::log::LevelFilter::Warn, std::time::Duration::from_secs(3))
        .to_owned()
}
