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

pub mod controllers;
pub mod pagination;

use charted_config::{caching::Strategy, db::Config};
use eyre::Context;
use serde::{de::DeserializeOwned, Serialize};
use sqlx::{
    migrate::Migrator,
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions as _, PgPool,
};
use std::{str::FromStr, time::Duration};
use tracing::{info, instrument};

pub static MIGRATOR: Migrator = sqlx::migrate!("../../migrations");

pub async fn create_pool(config: &Config) -> eyre::Result<PgPool> {
    info!("building PostgreSQL pool...");

    let mut connect_opts = PgConnectOptions::from_str(&config.to_string())?
        .options(config.options.iter())
        .application_name("charted-server")
        .log_statements(tracing::log::LevelFilter::Trace)
        .log_slow_statements(tracing::log::LevelFilter::Warn, Duration::from_secs(1));

    if let Some(ref ssl) = config.ssl {
        if let Some(ref cert) = ssl.client_cert {
            connect_opts = connect_opts.ssl_client_cert(cert);
        }

        if let Some(ref key) = ssl.client_key {
            connect_opts = connect_opts.ssl_client_key(key);
        }

        if let Some(ref path) = ssl.root_cert {
            connect_opts = connect_opts.ssl_root_cert(path);
        }
    }

    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .idle_timeout(
            config
                .idle_timeout
                .as_ref()
                .map(charted_common::serde::Duration::to_std_duration),
        )
        .connect_with(connect_opts)
        .await
        .context("failed to create pool for postgres database")?;

    if config.run_migrations {
        run_migrations(&pool).await?;
        return Ok(pool);
    }

    Ok(pool)
}

#[instrument(name = "charted.db.migrations.run", skip_all)]
pub async fn run_migrations(pool: &PgPool) -> eyre::Result<()> {
    info!("running database migrations!");
    MIGRATOR
        .run(pool)
        .await
        .inspect(|_| {
            info!("ran all migrations successfully");
        })
        .inspect_err(|e| {
            sentry::capture_error(e);
        })
        .context("failed to run migrations")
}

pub fn make_worker<T: Serialize + DeserializeOwned + Send + Sync>(
    redis: charted_core::redis::Client,
    config: &Config,
) -> eyre::Result<Box<dyn charted_cache::CacheWorker<T>>> {
    match config.caching.strategy {
        Strategy::InMemory => Ok(Box::new(charted_cache_inmemory::CacheWorker::new(&config.caching)?)),
        Strategy::Redis => Ok(Box::new(charted_cache_redis::CacheWorker::new(redis, &config.caching)?)),
    }
}

// #[cfg(test)]
// /// Creates a [`PgPool`] from a testcontainer. Returns a tuple of (container, pool) so it can
// /// be dropped when the test ends.
// pub async fn get_pool() -> eyre::Result<(
//     testcontainers::ContainerAsync<testcontainers_modules::postgres::Postgres>,
//     PgPool,
// )> {
//     use testcontainers::runners::AsyncRunner;

//     let container = testcontainers_modules::postgres::Postgres::default()
//         .with_db_name("charted")
//         .start()
//         .await?;

//     let config = Config {
//         run_migrations: true,
//         username: Some("postgres".into()),
//         password: Some("postgres".into()),
//         database: "charted".into(),
//         host: container.get_host().await.map(|x| x.to_string())?,
//         port: container.get_host_port_ipv4(5432).await?,

//         ..Default::default()
//     };

//     Ok((container, create_pool(&config).await?))
// }
