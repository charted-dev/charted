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
use charted_common::Snowflake;
use charted_config::Config;
use charted_database::controllers::DatabaseController;
use charted_metrics::SingleRegistry;
use charted_redis::RedisClient;
use charted_sessions::SessionManager;
use charted_storage::MultiStorageService;
use eyre::{Context, Result};
use once_cell::sync::OnceCell;
use sqlx::PgPool;
use std::{any::Any, cell::RefCell, fmt::Debug, sync::Arc};
use tokio::{select, signal, sync::Mutex};

pub(crate) static SERVER: OnceCell<Server> = OnceCell::new();

/// A default implemention of a [`Server`].
#[derive(Clone)]
pub struct Server {
    pub controllers: Vec<Arc<(dyn Any + Send + Sync)>>,
    pub snowflake: RefCell<Snowflake>,
    pub sessions: Arc<Mutex<SessionManager>>,
    pub registry: SingleRegistry,
    pub storage: MultiStorageService,
    pub config: Config,
    pub redis: RefCell<RedisClient>,
    pub pool: PgPool,
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
    pub fn controller<D: DatabaseController + 'static>(&self) -> &D {
        self.controllers
            .iter()
            .find(move |f| f.is::<D>())
            .expect("unable to find any db controller references with specified type")
            .downcast_ref()
            .expect("unable to downcast to &D")
    }

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