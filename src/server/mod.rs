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

pub use charted_proc_macros::controller;

use crate::Instance;
use argon2::{password_hash::SaltString, Algorithm, Argon2, Params, PasswordHasher, Version};
use axum::Router;
use axum_server::{tls_rustls::RustlsConfig, Handle};
use charted_common::lazy;
use charted_config::server::{self, ssl};
use eyre::Context;
use once_cell::sync::Lazy;
use rand::rngs::OsRng;
use std::time::Duration;

pub mod middleware;
pub mod openapi;
pub mod routing;
pub mod validation;
pub mod version;

/// Static [`Argon2`] instance that is used for the API server.
pub static ARGON2: Lazy<Argon2<'static>> = lazy!(Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::default()));

/// Hashes a password into a Argon2-based password that is safely secure to store.
pub fn hash_password<P: Into<String>>(password: P) -> eyre::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    ARGON2
        .hash_password(password.into().as_ref(), &salt)
        .map(|hash| hash.to_string())
        .inspect_err(|e| {
            error!(error = %e, "unable to compute password");
        })
        .map_err(|e| eyre!(e))
}

/// Starts the API server, this is the main code for the `charted server` command.
pub async fn start(instance: Instance) -> eyre::Result<()> {
    info!("starting API server");

    let config = instance.config.server.clone();
    let router: Router = self::routing::create_router(&instance).with_state(instance);

    match config.ssl {
        Some(ref ssl) => start_with_https(&config, ssl, &router).await,
        None => start_without_https(&config, router).await,
    }
}

async fn start_with_https(config: &server::Config, ssl: &ssl::Config, router: &Router) -> eyre::Result<()> {
    info!("handling all TLS connections!");

    let handle = Handle::new();
    tokio::spawn(shutdown_signal(Some(handle.clone())));

    let addr = config.addr();
    let config = RustlsConfig::from_pem_file(&ssl.cert, &ssl.cert_key).await?;

    info!(address = %addr, "now listening on HTTPS");
    axum_server::bind_rustls(addr, config)
        .handle(handle)
        .serve(router.clone().into_make_service())
        .await
        .context("received unexpected error")
}

async fn start_without_https(config: &server::Config, router: Router) -> eyre::Result<()> {
    let addr = config.addr();
    let listener = tokio::net::TcpListener::bind(addr).await?;

    info!(address = %addr, "listening on HTTP");
    axum::serve(listener, router.into_make_service())
        .with_graceful_shutdown(shutdown_signal(None))
        .await
        .context("received unexpected error")
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
