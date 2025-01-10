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

#![feature(never_type, decl_macro, let_chains)]

mod state;
pub use state::*;

mod types;
pub use types::*;

#[macro_use]
pub mod macros;

pub mod extract;
pub mod middleware;
pub mod multipart;
pub mod openapi;
pub mod ops;
pub mod responses;
pub mod routing;

#[cfg(test)]
pub mod test;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    PasswordHasher,
};
use axum::Router;
use axum_server::{tls_rustls::RustlsConfig, Handle};
use charted_core::ARGON2;
use eyre::Context;
use std::time::Duration;
use tracing::{error, info, info_span, warn, Instrument};

pub fn hash_password<P: AsRef<[u8]>>(password: P) -> eyre::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let password = password.as_ref();

    ARGON2
        .hash_password(password, &salt)
        .map(|hash| hash.to_string())
        .inspect_err(|e| {
            error!(error = %e, "failed to compute argon2 password");
        })
        // since `argon2::Error` doesn't implement `std::error::Error`,
        // we implicitlly pass it into the `eyre!` macro, which will create
        // an adhoc error.
        .map_err(|e| eyre::eyre!(e))
}

pub async fn start(cx: charted_app::Context) -> eyre::Result<()> {
    info!("starting API server...");

    #[allow(unused)]
    let features = Vec::new();

    let cx = ServerContext::new(cx, features);
    for feature in &cx.features {
        // Initialize the feature first
        feature
            .init(&cx)
            .instrument(info_span!("charted.server.feature.init"))
            .await?;

        // If it needs to extend the database, then we will
        // allow it to extend it to have its own attributes.
        feature
            .extends_db(&cx.pool)
            .instrument(info_span!("charted.server.feature.extends[database]"))
            .await?;
    }

    // Put a clone of `ServerContext` since we still need to access it.
    set_global(cx.clone());

    let server_config = cx.config.server.clone();
    let router: Router = self::routing::create_router(&cx).with_state(cx);

    match server_config.ssl {
        Some(ref ssl) => start_as_https(&server_config, ssl, router).await,
        None => start_as_http(&server_config, router).await,
    }
}

async fn start_as_https(
    config: &charted_config::server::Config,
    ssl: &charted_config::server::ssl::Config,
    router: Router,
) -> eyre::Result<()> {
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
        .context("failed to run HTTPS service")
}

async fn start_as_http(config: &charted_config::server::Config, router: Router) -> eyre::Result<()> {
    info!("starting HTTP service with TLS disabled");

    let addr = config.addr();
    let listener = tokio::net::TcpListener::bind(addr).await?;

    info!(address = %addr, "listening on HTTP");
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
