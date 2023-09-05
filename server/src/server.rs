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

use crate::routing::create_router;
use axum::extract::FromRef;
use charted_avatars::AvatarsModule;
use charted_common::Snowflake;
use charted_config::Config;
use charted_database::controller::DbControllerRegistry;
use charted_helm_charts::HelmCharts;
use charted_metrics::SingleRegistry;
use charted_redis::RedisClient;
use charted_sessions::SessionManager;
use charted_storage::MultiStorageService;
use eyre::{Context, Result};
use once_cell::sync::OnceCell;
use sqlx::PgPool;
use std::{
    cell::RefCell,
    fmt::Debug,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use tokio::{select, signal, sync::RwLock};

pub(crate) static SERVER: OnceCell<Server> = OnceCell::new();

/// A default implemention of a [`Server`].
pub struct Server {
    pub controllers: DbControllerRegistry,
    pub helm_charts: HelmCharts,
    pub snowflake: Snowflake,
    pub sessions: Arc<RwLock<SessionManager>>,
    pub registry: SingleRegistry,
    pub requests: AtomicUsize,
    pub avatars: AvatarsModule,
    pub storage: MultiStorageService,
    pub config: Config,
    pub redis: RefCell<RedisClient>,
    pub pool: PgPool,
}

impl Clone for Server {
    fn clone(&self) -> Server {
        Server {
            controllers: self.controllers.clone(),
            helm_charts: self.helm_charts.clone(),
            snowflake: self.snowflake.clone(),
            sessions: self.sessions.clone(),
            registry: self.registry.clone(),
            requests: AtomicUsize::new(self.requests.load(Ordering::SeqCst)),
            avatars: self.avatars.clone(),
            storage: self.storage.clone(),
            config: self.config.clone(),
            redis: self.redis.clone(),
            pool: self.pool.clone(),
        }
    }
}

impl FromRef<()> for Server {
    fn from_ref(_: &()) -> Self {
        SERVER.get().expect("unable to grab SERVER instance").clone()
    }
}

unsafe impl Send for Server {}
unsafe impl Sync for Server {}
impl Debug for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Server").finish()
    }
}

impl Server {
    pub async fn run(&self) -> Result<()> {
        let addr = self.config.server.addr();
        info!(%addr, "now listening on");

        let router = create_router(self.clone()).with_state(self.clone());
        axum::Server::bind(&addr)
            .serve(router.into_make_service())
            .with_graceful_shutdown(shutdown())
            .await
            .context("server failed to serve")
    }
}

// #[allow(dead_code)]
// const INDEX_HTML: &str = "index.html";

// async fn static_handler(_uri: Uri) -> impl IntoResponse {
//     /* TODO: this */
// }

async fn shutdown() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("unable to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("unable to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    select! {
        _ = ctrl_c => {}
        _ = terminate => {}
    }

    warn!("received signal, terminating API server");
}
