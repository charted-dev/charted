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

use super::BootstrapPhase;
use crate::{Server, SERVER};
use charted_avatars::AvatarsModule;
use charted_common::{is_debug_enabled, Snowflake, COMMIT_HASH, VERSION};
use charted_config::{Config, ConfigExt, SessionBackend};
use charted_database::MIGRATIONS;
use charted_helm_charts::HelmCharts;
use charted_metrics::SingleRegistry;
use charted_redis::RedisClient;
use charted_sessions::SessionManager;
use charted_sessions_local::LocalSessionProvider;
use charted_storage::MultiStorageService;
use eyre::Result;
use sentry::{types::Dsn, ClientOptions};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions,
};
use std::{
    borrow::Cow,
    cell::RefCell,
    str::FromStr,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct StartServerPhase;

#[async_trait::async_trait]
impl BootstrapPhase for StartServerPhase {
    async fn bootstrap(&self, config: &Config) -> Result<()> {
        let guard = if let Ok(Some(dsn)) = config.sentry_dsn() {
            Some(sentry::init(ClientOptions {
                attach_stacktrace: true,
                release: Some(Cow::Owned(format!("v{VERSION}+{COMMIT_HASH}"))),
                debug: is_debug_enabled(),
                dsn: Some(Dsn::from_str(dsn.as_str())?),

                ..Default::default()
            }))
        } else {
            None
        };

        let server = configure_modules(config).await?;
        SERVER.set(server.clone()).unwrap();

        info!("Server is now starting...");
        server.run().await?;

        // drop it once the server is done
        if let Some(guard) = guard {
            drop(guard);
        }

        Ok(())
    }

    fn try_clone(&self) -> Result<Box<dyn BootstrapPhase>> {
        Ok(Box::new(self.clone()))
    }
}

pub(crate) async fn configure_modules(config: &Config) -> Result<Server> {
    let mut now = Instant::now();
    info!("Connecting to PostgreSQL...");

    let pool = PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .connect_with(
            PgConnectOptions::from_str(config.database.to_string().as_str())?
                .log_statements(tracing::log::LevelFilter::Trace)
                .log_slow_statements(tracing::log::LevelFilter::Warn, Duration::from_secs(5)),
        )
        .await?;

    info!(
        took = format!("{:?}", Instant::now().duration_since(now)),
        "Connected to PostgreSQL successfully"
    );

    now = Instant::now();
    {
        let pool = pool.clone();
        let guard = info_span!("database.migrate.run");
        let _entered = guard.enter();

        info!("Running database migrations...");
        MIGRATIONS.run(&pool).await?;

        info!(
            took = format!("{:?}", Instant::now().duration_since(now)),
            "Ran all database migrations!"
        );
    }

    now = Instant::now();
    info!("Connecting to Redis...");
    let redis = RedisClient::new()?;

    info!(
        took = format!("{:?}", Instant::now().duration_since(now)),
        "Connected to Redis successfully, now initializing session manager"
    );

    now = Instant::now();
    let mut sessions = SessionManager::new(
        redis.clone(),
        match config.sessions.backend.clone() {
            SessionBackend::Local => Box::new(LocalSessionProvider::new(redis.clone(), pool.clone())?),
            backend => {
                warn!("Backend {backend:?} is not supported at this time! Using local as a default");
                Box::new(LocalSessionProvider::new(redis.clone(), pool.clone())?)
            }
        },
    );

    sessions.init()?;

    info!(
        took = format!("{:?}", Instant::now().duration_since(now)),
        "Initialized session manager and all remaining sessions, now configuring misc. dependencies..."
    );

    now = Instant::now();

    let storage = MultiStorageService::from(config.storage.clone());
    let snowflake = Snowflake::new(0);
    let registry = SingleRegistry::configure(config.clone(), vec![]);
    let helm_charts = HelmCharts::new(storage.clone());
    helm_charts.init().await?;

    let avatars = AvatarsModule::new(storage.clone());
    avatars.init().await?;

    info!(
        took = format!("{:?}", Instant::now().duration_since(now)),
        "Initialized all misc dependencies!"
    );

    Ok(Server {
        helm_charts,
        snowflake,
        sessions: Arc::new(RwLock::new(sessions)),
        registry,
        avatars,
        storage,
        config: config.clone(),
        redis: RefCell::new(redis),
        pool,
    })
}
