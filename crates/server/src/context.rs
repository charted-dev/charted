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

use crate::{feature, routing};
use axum::{Extension, Router};
use axum_server::{Handle, tls_rustls::RustlsConfig};
use azalia::remi::StorageService;
use charted_authz::Authenticator;
use charted_config::{
    Config, metrics, server,
    sessions::{self, Backend},
    storage,
};
use charted_core::{ResultExt, ulid};
use metrics_exporter_prometheus::PrometheusHandle;
use sea_orm::DatabaseConnection;
use std::{
    net::SocketAddr,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use tokio::net::TcpListener;

/// Global context that handles all the dependencies needed by API endpoints.
pub struct Context {
    pub(self) prometheus_handle: Option<PrometheusHandle>,

    #[allow(unused)]
    pub(self) monitor: (),

    pub ulid_generator: charted_core::ulid::Generator,
    pub requests: AtomicUsize,
    pub features: feature::Collection,
    pub storage: StorageService,
    pub config: Config,
    pub authz: Arc<dyn Authenticator>,
    pub http: reqwest::Client,
    pub pool: DatabaseConnection,
}

impl Context {
    /// Creates a new [`Context`] object with a given configuration file.
    ///
    /// This will initialize all dependencies required for this [`Context`] object to
    /// serve its purpose.
    pub async fn new(config: Config) -> eyre::Result<Self> {
        let pool = charted_database::create_pool(&config.database).await?;

        debug!("building storage service...");
        let storage = Context::build_storage_service(&config.storage)?;
        azalia::remi::core::StorageService::init(&storage).await?;

        debug!("building authz backend...");
        let authz: Arc<dyn Authenticator> = Context::build_authz_backend(&config.sessions);

        let prometheus_handle = Context::init_prometheus_handle(&config.metrics)?;

        #[allow(unused_mut)]
        let mut features = feature::Collection::new();

        Ok(Context {
            prometheus_handle,
            monitor: (),

            ulid_generator: ulid::Generator::new(),
            requests: AtomicUsize::default(),
            features,
            storage,
            config,
            authz,
            pool,
            http: reqwest::Client::builder()
                .gzip(true)
                .use_rustls_tls()
                .user_agent(format!(
                    "Noelware/charted-server (+https://github.com/charted-dev/charted; {})",
                    charted_core::version()
                ))
                .build()
                .into_report()?,
        })
    }

    /// Closes all avaliable resources.
    pub async fn close(self) -> eyre::Result<()> {
        warn!("closing resources...");

        self.pool.close_by_ref().await?;

        Ok(())
    }

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

    async fn start_api_server(&self, router: Router) -> eyre::Result<()> {
        async fn start_https(config: &server::Config, ssl: &server::ssl::Config, router: Router) -> eyre::Result<()> {
            info!(target: "charted_server", "starting HTTP service with TLS enabled");

            let handle = Handle::new();
            tokio::spawn(shutdown_signal(Some(handle.clone())));

            let addr = config.to_socket_addr();
            let rustls_config = RustlsConfig::from_pem_file(&ssl.cert, &ssl.cert_key).await?;

            #[cfg(all(target_os = "linux", feature = "libsystemd"))]
            systemd_notify_ready();

            info!(target: "charted_server", address = %addr, "binding to socket address");
            axum_server::bind_rustls(addr, rustls_config)
                .handle(handle)
                .serve(router.into_make_service())
                .await
                .into_report()
        }

        async fn start_http(config: &server::Config, router: Router) -> eyre::Result<()> {
            info!(target: "charted_server", "starting HTTP service with TLS disabled");

            let addr = config.to_socket_addr();
            let listener = TcpListener::bind(addr).await?;

            #[cfg(all(target_os = "linux", feature = "libsystemd"))]
            systemd_notify_ready();

            info!(target: "charted_server", address = %addr, "binding to socket address");
            axum::serve(listener, router.into_make_service())
                .with_graceful_shutdown(shutdown_signal(None))
                .await
                .into_report()
        }

        match self.config.server.ssl {
            Some(ref ssl) => start_https(&self.config.server, ssl, router).await,
            None => start_http(&self.config.server, router).await,
        }
    }

    async fn start_prometheus_server(&self, config: &metrics::Config) -> eyre::Result<()> {
        let metrics::Config::Prometheus(config) = config else {
            return Ok(());
        };

        let Some(ref standalone) = config.standalone else {
            return Ok(());
        };

        let addr = format!("{}:{}", standalone.host, standalone.port).parse::<SocketAddr>()?;

        info!(
            target: "charted_server",
            "starting standalone Prometheus HTTP scrape endpoint at address {}",
            addr
        );

        let router = Router::new()
            .route("/", axum::routing::get(routing::v1::prometheus_scrape))
            .layer(Extension(self.prometheus_handle.clone()));

        let listener = TcpListener::bind(addr).await?;
        axum::serve(listener, router.into_make_service())
            .with_graceful_shutdown(shutdown_signal(None))
            .await
            .into_report()
    }

    /// Creates a [`Context`] object for testing purposes.
    pub async fn for_testing<'s, F: FnOnce(&mut Config) + 's>(config_override: F) -> eyre::Result<Self> {
        use charted_config::{
            database, metrics,
            sessions::{self, Backend},
        };
        use url::Url;

        // so that sessions are "consistent" enough between tests
        const JWT_SECRET_KEY: &str =
            "ahashthatshouldbeavalidhashfromopensslbutidontwanttodothatandnooneshouldusethisvaluetobeginwithuwu";

        let mut config = Config {
            jwt_secret_key: String::from(JWT_SECRET_KEY),
            registrations: true,
            single_user: false,
            single_org: false,
            sentry_dsn: None,
            base_url: Some(Url::parse("http://localhost:3651")?),
            logging: Default::default(),
            storage: Default::default(),
            tracing: None,
            metrics: metrics::Config::Disabled,
            server: Default::default(),

            sessions: sessions::Config {
                enable_basic_auth: false,
                backend: Backend::Static(azalia::btreemap! {
                    // echo "noeliscutieuwu" | cargo cli admin authz hash-password --stdin
                    "noel" => "$argon2id$v=19$m=19456,t=2,p=1$gIcVA4mVHgr8ZWkmDrtJlw$sb5ypFAvphFCGrJXy9fRI1Gb/2vGIH1FTzDax458+xY"
                }),
            },

            database: database::Config::SQLite(database::sqlite::Config {
                common: Default::default(),
                path: String::from(":memory:").into(),
            }),
        };

        config_override(&mut config);
        Self::new(config).await
    }

    fn build_storage_service(config: &storage::Config) -> eyre::Result<StorageService> {
        match config {
            storage::Config::Filesystem(fs) => Ok(StorageService::Filesystem(
                azalia::remi::fs::StorageService::with_config(fs.clone()),
            )),

            storage::Config::Azure(azure) => Ok(StorageService::Azure(azalia::remi::azure::StorageService::new(
                azure.to_owned(),
            )?)),

            storage::Config::S3(s3) => Ok(StorageService::S3(azalia::remi::s3::StorageService::new(s3.to_owned()))),
        }
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

    fn build_authz_backend(config: &sessions::Config) -> Arc<dyn Authenticator> {
        match config.backend {
            Backend::Local => Arc::new(charted_authz_local::Backend::default()),
            Backend::Static(ref users) => Arc::new(charted_authz_static::Backend::new(users.to_owned())),
            Backend::Ldap(_) => {
                warn!("ldap backend is not supported right now! switching to local backend");
                Arc::new(charted_authz_local::Backend::default())
            }
        }
    }
}

impl Clone for Context {
    fn clone(&self) -> Self {
        Context {
            prometheus_handle: self.prometheus_handle.clone(),
            monitor: (),

            ulid_generator: self.ulid_generator.clone(),
            requests: AtomicUsize::new(self.requests.load(Ordering::SeqCst)),
            features: self.features.clone(),
            storage: self.storage.clone(),
            config: self.config.clone(),
            authz: self.authz.clone(),
            http: self.http.clone(),
            pool: self.pool.clone(),
        }
    }
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

    #[cfg(all(target_os = "linux", feature = "libsystemd"))]
    systemd_notify_stopping();
}

#[cfg(all(target_os = "linux", feature = "libsystemd"))]
fn systemd_notify_ready() {
    if libsystemd::daemon::booted() {
        if let Err(e) = libsystemd::daemon::notify(false, &[libsystemd::daemon::NotifyState::Ready]) {
            warn!(error = %e, "received error when notifying systemd that we're ready!");
        }
    }
}

#[cfg(all(target_os = "linux", feature = "libsystemd"))]
fn systemd_notify_stopping() {
    if libsystemd::daemon::booted() {
        if let Err(e) = libsystemd::daemon::notify(false, &[libsystemd::daemon::NotifyState::Stopping]) {
            warn!(error = %e, "received error when notifying systemd that we're ready!");
        }
    }
}
