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

use crate::{routing, Context};
use axum::Router;
use axum_server::{tls_rustls::RustlsConfig, Handle};
use charted_config::server;
use eyre::Context as _;
use std::time::Duration;
use tokio::net::TcpListener;
use tracing::{info, warn};

pub async fn drive() -> eyre::Result<()> {
    let context = Context::get();
    let router: Router = routing::create_router(context).with_state(context.to_owned());
    let server_cfg = context.config.server.clone();

    match server_cfg.ssl {
        Some(ref ssl) => start_https(&server_cfg, ssl, router).await,
        None => start_http(&server_cfg, router).await,
    }
}

async fn start_https(config: &server::Config, ssl: &server::ssl::Config, router: Router) -> eyre::Result<()> {
    info!("starting HTTP service with TLS enabled");

    let handle = Handle::new();
    tokio::spawn(shutdown_signal(Some(handle.clone())));

    let addr = config.to_socket_addr();
    let config = RustlsConfig::from_pem_file(&ssl.cert, &ssl.cert_key).await?;

    info!(address = %addr, "binding to socket address");
    axum_server::bind_rustls(addr, config)
        .handle(handle)
        .serve(router.into_make_service())
        .await
        .context("failed to run HTTPS service")
}

async fn start_http(config: &server::Config, router: Router) -> eyre::Result<()> {
    info!("starting HTTP service with TLS disabled");

    let addr = config.to_socket_addr();
    let listener = TcpListener::bind(addr).await?;

    info!(address = %addr, "binding to socket address");
    axum::serve(listener, router.into_make_service())
        .with_graceful_shutdown(shutdown_signal(None))
        .await
        .context("failed to run HTTP service")
}

async fn shutdown_signal(handle: Option<Handle>) {
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.expect("unable to install CTRL+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("unable to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {}
        _ = terminate => {}
    }

    warn!("received terminal signal! shutting down");
    if let Some(handle) = handle {
        handle.graceful_shutdown(Some(Duration::from_secs(10)));
    }
}
