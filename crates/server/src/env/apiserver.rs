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

use super::Env;
use crate::env::shutdown_signal;
#[cfg(all(feature = "libsystemd", target_os = "linux"))]
use crate::env::systemd;
use axum::Router;
use axum_server::{Handle, tls_rustls::RustlsConfig};
use charted_config::server::{self, ssl};
use charted_core::ResultExt;
use tokio::net::TcpListener;

/// Starts the API server.
pub async fn start(env: &Env, router: Router) -> eyre::Result<()> {
    match env.config.server.ssl {
        Some(ref ssl) => start_https(&env.config.server, ssl, router).await,
        None => start_http(&env.config.server, router).await,
    }
}

async fn start_https(config: &server::Config, ssl: &ssl::Config, router: Router) -> eyre::Result<()> {
    info!("starting API server with TLS enabled");

    let handle = Handle::new();
    tokio::spawn(shutdown_signal(Some(handle.clone())));

    let addr = config.to_socket_addr();
    let rustls = RustlsConfig::from_pem_file(&ssl.cert, &ssl.cert_key).await?;

    #[cfg(all(target_os = "linux", feature = "libsystemd"))]
    systemd::notify_ready();

    info!(address = %addr, "binding to address");
    axum_server::bind_rustls(addr, rustls)
        .handle(handle)
        .serve(router.into_make_service())
        .await
        .into_report()
}

async fn start_http(config: &server::Config, router: Router) -> eyre::Result<()> {
    info!("starting HTTP service with TLS disabled");

    let addr = config.to_socket_addr();
    let listener = TcpListener::bind(addr).await?;

    #[cfg(all(target_os = "linux", feature = "libsystemd"))]
    systemd::notify_ready();

    info!(target: "charted_server", address = %addr, "binding to socket address");
    axum::serve(listener, router.into_make_service())
        .with_graceful_shutdown(shutdown_signal(None))
        .await
        .into_report()
}
