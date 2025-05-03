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

mod apiserver;
mod prometheus;

#[cfg(all(feature = "libsystemd", target_os = "linux"))]
mod systemd;

use crate::{feature, routing};
use axum::extract::FromRef;
use axum_server::Handle;
use charted_authz::Authenticator;
use charted_config::{
    Config,
    sessions::{self, Backend},
};
use charted_core::{serde::Duration, ulid};
use charted_datastore::DataStore;
use sea_orm::DatabaseConnection;
use std::{sync::Arc, time::Instant};

/// Global environment that holds all dependencies.
#[derive(Clone, FromRef)]
pub struct Env {
    pub features: feature::Collection,
    pub config: Config,
    pub authz: Arc<dyn Authenticator>,
    pub ulid: ulid::Generator,
    pub db: DatabaseConnection,
    pub ds: DataStore,
}

impl Env {
    /// Creates a new [`Env`] with the configuration file to initialize
    /// all dependencies.
    pub async fn new(config: Config) -> eyre::Result<Self> {
        let mut now = Instant::now();
        let original = now;

        let pool = charted_database::create_pool(&config.database).await?;
        debug!(
            "database pool: initialized [{}]",
            charted_core::serde::Duration::from(now.elapsed())
        );

        now = Instant::now();

        let ds = DataStore::new(&config.storage).await?;
        debug!("data store: initialized [{}]", Duration::from(now.elapsed()));

        now = Instant::now();

        let authz: Arc<dyn Authenticator> = build_authz_backend(&config.sessions);
        debug!("authenticator: initialized [{}]", Duration::from(now.elapsed()));

        #[allow(unused_mut)]
        let mut features = feature::Collection::new();

        debug!("built environment in {}", Duration::from(original.elapsed()));

        Ok(Self {
            features,
            config,
            authz,
            ulid: ulid::Generator::new(),
            db: pool,
            ds,
        })
    }

    pub async fn close(self) -> eyre::Result<()> {
        warn!("closing dependency resources...");

        self.db.close_by_ref().await?;
        Ok(())
    }

    /// Starts the API server.
    pub async fn drive(&self) -> eyre::Result<()> {
        let router = routing::create_router(self).with_state(self.clone());

        crate::env::apiserver::start(self, router).await

        // futures_util::try_join!(crate::env::apiserver::start(&self, router))
        //     .map(|(..)| ())
        //     .into_report()
    }
}

/*
impl Context {
    /// Starts the API server.
    pub async fn start(&self) -> eyre::Result<()> {
        let config = self.config.clone();
        let router = routing::create_router(self)
            .layer(Extension(self.prometheus_handle.clone()))
            .with_state(self.clone());

        futures_util::try_join!(
            self.start_api_server(router),
            self.start_prometheus_server(&config.metrics)
        )
        .map(|(..)| ())
        .into_report()
    }

    fn init_prometheus_handle(config: &metrics::Config) -> eyre::Result<Option<PrometheusHandle>> {
        match config {
            metrics::Config::Disabled => Ok(None),
            metrics::Config::Prometheus(config) => Ok(Some(charted_metrics::init_prometheus(config)?)),
            metrics::Config::OpenTelemetry(config) => {
                charted_metrics::init_opentelemetry(config)?;
                Ok(None)
            }
        }
    }
}
*/

fn build_authz_backend(config: &sessions::Config) -> Arc<dyn Authenticator> {
    match config.backend {
        Backend::Local => Arc::new(charted_authz_local::Backend::default()),
        Backend::Static(ref users) => Arc::new(charted_authz_static::Backend::new(users.to_owned())),
        Backend::Ldap(_) => {
            warn!("as of this build, the LDAP authenticator is not supported, switching to local backend");
            Arc::new(charted_authz_local::Backend::default())
        }
    }
}

pub(in crate::env) async fn shutdown_signal(handle: Option<Handle>) {
    // Install the CTRL+C handler
    let ctrl_c = async { tokio::signal::ctrl_c().await.expect("failed to install CTRL+C handler") };

    #[cfg(unix)]
    let termination = async {
        use tokio::signal::unix::{SignalKind, signal};

        signal(SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await
    };

    #[cfg(unix)]
    let sigint = async {
        use tokio::signal::unix::{SignalKind, signal};

        signal(SignalKind::interrupt())
            .expect("failed to install SIGINT handler")
            .recv()
            .await
    };

    #[cfg(not(unix))]
    let termination = std::future::pending::<()>();

    #[cfg(not(unix))]
    let sigint = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            warn!("received CTRL+C :3");
        }

        res = termination => {
            warn!("received SIGTERM termination signal :3");
            trace!("result = {res:?}");
        }

        res = sigint => {
            warn!("received SIGINT termination signal :3");
            trace!("result = {res:?}");
        }
    }

    if let Some(handle) = handle {
        handle.graceful_shutdown(Some(Duration::from_secs(10).into()));
    }

    #[cfg(all(target_os = "linux", feature = "libsystemd"))]
    systemd::notify_shutdown();
}
