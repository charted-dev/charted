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

//! The `charted_server` crate implements the types, implementation, and Axum extractors of charted's
//! REST API Specification for transmitting Helm charts safely and securely.

pub use charted_proc_macros::controller;

pub mod extract;
pub mod metrics;
pub mod middleware;
pub mod multipart;
pub mod openapi;
pub(crate) mod ops;
pub mod pagination;
pub mod routing;

mod models;
pub use models::*;

mod state;
pub use state::*;

use axum::Router;
use axum_server::{tls_rustls::RustlsConfig, Handle};
use charted_config::server::{ssl, Config};
use eyre::Context;
use std::time::Duration;
use tracing::{info, warn};

/// Starts the HTTP service with a given [`ServerContext`].
pub async fn start(ctx: ServerContext) -> eyre::Result<()> {
    info!("starting HTTP service for API server");

    let config = ctx.config.server.clone();
    let router = routing::create_router(&ctx).with_state(ctx);

    match config.ssl {
        Some(ref ssl) => start_https_server(&config, ssl, router).await,
        None => start_http_server(&config, router).await,
    }
}

async fn start_http_server(config: &Config, router: Router) -> eyre::Result<()> {
    info!("starting HTTP server with TLS disabled");

    let addr = config.addr();
    let listener = tokio::net::TcpListener::bind(addr).await?;

    info!(address = %addr, "now listening on HTTP");
    axum::serve(listener, router.into_make_service())
        .with_graceful_shutdown(shutdown_signal(None))
        .await
        .context("received unexpected error while running HTTP server")
}

async fn start_https_server(config: &Config, ssl: &ssl::Config, router: Router) -> eyre::Result<()> {
    info!("starting HTTP service with TLS enabled");

    let handle = Handle::new();
    tokio::spawn(shutdown_signal(Some(handle.clone())));

    let addr = config.addr();
    let config = RustlsConfig::from_pem_file(&ssl.cert, &ssl.cert_key).await?;

    info!(address = %addr, "now listening on HTTPS");
    axum_server::bind_rustls(addr, config)
        .handle(handle)
        .serve(router.into_make_service())
        .await
        .context("received unexpected error while running HTTPS server")
}

async fn shutdown_signal(handle: Option<Handle>) {
    let ctrl_c = async { tokio::signal::ctrl_c().await.expect("failed to install CTRL+C handler") };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = ::std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {}
        _ = terminate => {}
    }

    warn!("received termination signal! shutting down server");
    if let Some(handle) = handle {
        handle.graceful_shutdown(Some(Duration::from_secs(10)));
    }
}
