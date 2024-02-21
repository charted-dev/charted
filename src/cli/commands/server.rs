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

use crate::{
    auth,
    avatars::AvatarsModule,
    charts,
    cli::AsyncExecute,
    common::{models::Distribution, Snowflake},
    config::Config,
    db, redis, Instance,
};
use axum::{
    extract::Host,
    handler::HandlerWithoutStateExt,
    http::{StatusCode, Uri},
    response::Redirect,
    BoxError, Router,
};
use axum_server::{tls_rustls::RustlsConfig, Handle};
use eyre::Context;
use noelware_log::{writers, WriteLayer};
use owo_colors::{OwoColorize, Stream::Stdout};
use remi::StorageService;
use sentry::types::Dsn;
use std::{
    borrow::Cow,
    future::Future,
    io::{self, Write as _},
    net::SocketAddr,
    path::PathBuf,
    str::FromStr,
    sync::{atomic::AtomicUsize, Arc},
    time::{Duration, Instant},
};
use tokio::sync::{Mutex, RwLock};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::prelude::*;

/// Runs the API server.
#[derive(Debug, Clone, clap::Parser)]
pub struct Cmd {
    /// location to a relative/absolute path to a configuration file. by default, this will locate
    /// in `./config/charted.yml`/`./config.yml` if found.
    #[arg(short = 'c', long, env = "CHARTED_CONFIG_FILE")]
    config: Option<PathBuf>,

    /// whether or not to print the configuration and exit
    #[arg(long)]
    print: bool,

    /// amount of workers to spawn for the Tokio runtime. This cannot exceeded
    /// the amount of CPU cores you have.
    #[arg(short = 'w', long, env = "CHARTED_RUNTIME_WORKERS", default_value_t = num_cpus::get())]
    pub workers: usize,
}

impl Cmd {
    fn init_log(config: &Config) {
        tracing_subscriber::registry()
            .with(
                match config.logging.json {
                    false => WriteLayer::new_with(io::stdout(), writers::default),
                    true => WriteLayer::new_with(io::stdout(), writers::json),
                }
                .with_filter(LevelFilter::from_level(config.logging.level))
                .with_filter(tracing_subscriber::filter::filter_fn(|meta| {
                    // disallow from getting logs from `tokio` since it doesn't contain anything
                    // useful to us
                    !meta.target().starts_with("tokio::")
                })),
            )
            .with(sentry_tracing::layer())
            .with(
                config
                    .logging
                    .logstash_tcp_uri
                    .as_ref()
                    .map(|url| {
                        let stream = std::net::TcpStream::connect(url).unwrap();
                        WriteLayer::new_with(stream, writers::json)
                    })
                    .with_filter(LevelFilter::from_level(config.logging.level))
                    .with_filter(tracing_subscriber::filter::filter_fn(|meta| {
                        // disallow from getting logs from `tokio` since it doesn't contain anything
                        // useful to us
                        !meta.target().starts_with("tokio::")
                    })),
            )
            .init();
    }
}

#[async_trait]
impl AsyncExecute for Cmd {
    async fn execute(&self) -> eyre::Result<()> {
        let config = match self.config {
            Some(ref path) => Config::new(Some(path)),
            None => match Config::find_default_conf_location() {
                Some(path) => Config::new(Some(path)),
                None => Config::new::<&str>(None),
            },
        }?;

        if self.print {
            eprintln!("{}", serde_yaml::to_string(&config).unwrap());
            return Ok(());
        }

        // 1. print banner
        print_banner();

        // 2. setup Sentry client
        let _sentry_guard = sentry::init(sentry::ClientOptions {
            traces_sample_rate: 1.0,
            attach_stacktrace: true,
            server_name: Some(Cow::Borrowed("charted-server")),
            release: Some(Cow::Owned(crate::version())),
            dsn: config
                .sentry_dsn
                .as_ref()
                .map(|dsn| Dsn::from_str(dsn).expect("valid Sentry DSN")),

            ..Default::default()
        });

        // 3. setup logging
        Cmd::init_log(&config);

        let mut now = Instant::now();
        let original = now; // keep a copy of the original so we can keep a difference

        info!("🛰️   Connecting to PostgreSQL...");

        let pool = crate::db::create_pool(&config).await?;

        info!(took = ?Instant::now().duration_since(now), "connected to PostgreSQL successfully!");
        now = Instant::now();

        info!("🛰️   Connecting to Redis...");

        let redis = redis::Client::new(&config.redis)?;
        info!(took = ?Instant::now().duration_since(now), "connected to Redis successfully!");
        now = Instant::now();

        let storage = match config.storage {
            crate::config::storage::Config::Filesystem(ref fs) => {
                noelware_remi::StorageService::Filesystem(remi_fs::StorageService::with_config(fs.clone()))
            }

            crate::config::storage::Config::Azure(ref azure) => {
                noelware_remi::StorageService::Azure(remi_azure::StorageService::new(azure.clone()))
            }

            crate::config::storage::Config::S3(ref s3) => {
                noelware_remi::StorageService::S3(remi_s3::StorageService::new(s3.clone()))
            }
        };

        storage.init().await?;
        info!(took = ?Instant::now().duration_since(now), "initialized external storage successfully");
        now = Instant::now();

        info!("initializing authz backend...");
        let authz: Arc<dyn crate::auth::Backend> = match config.sessions.backend {
            crate::config::sessions::Backend::Local => Arc::new(auth::local::Backend::new(pool.clone())),
            crate::config::sessions::Backend::Ldap(ref config) => Arc::new(auth::ldap::Backend::new(config.clone())),
            crate::config::sessions::Backend::Passwordless | crate::config::sessions::Backend::Htpasswd(_) => {
                warn!("other authz backends are not supported at this time, switching to `local` backend instead");
                Arc::new(auth::local::Backend::new(pool.clone()))
            }
        };

        info!(took = ?Instant::now().duration_since(now), "initialized authz backend successfully");
        let avatars = AvatarsModule::new(storage.clone());
        avatars.init().await?;
        now = Instant::now();

        info!("now initializing sessions manager");
        let mut sessions = crate::sessions::Manager::new(redis.clone());
        sessions.init()?;

        info!(took = ?Instant::now().duration_since(now), "initialized sessions manager successfully");
        now = Instant::now();

        let charts = charts::HelmCharts::new(storage.clone());
        charts.init().await?;

        info!(took = ?Instant::now().duration_since(now), "initialized core chart library");

        let controllers = db::controllers::Controllers {
            organizations: db::controllers::organization::DbController::new(
                crate::caching::choose_strategy(&config.database.caching, &redis),
                pool.clone(),
            ),
            repositories: db::controllers::repository::DbController::new(
                crate::caching::choose_strategy(&config.database.caching, &redis),
                pool.clone(),
            ),
            apikeys: db::controllers::apikeys::DbController::new(pool.clone()),
            users: db::controllers::user::DbController::new(
                crate::caching::choose_strategy(&config.database.caching, &redis),
                pool.clone(),
            ),
        };

        let instance = Instance {
            controllers,
            snowflake: Snowflake::computed(),
            requests: AtomicUsize::new(0),
            sessions: Arc::new(Mutex::new(sessions)),
            avatars,
            metrics: crate::metrics::new(&config),
            storage,
            charts,
            search: None,
            config,
            redis: Arc::new(RwLock::new(redis)),
            authz,
            pool,
        };

        crate::set_instance(instance.clone());
        info!(took = ?Instant::now().duration_since(original), "initialized global instance, starting server...");

        let router: Router = crate::server::routing::create_router(&instance).with_state(instance);
        let instance = Instance::get();

        if let Some(ref cfg) = instance.config.server.ssl {
            info!("server is now using HTTPS support");

            // keep a handle for the TLS server so the shutdown signal can all shutdown
            let handle = axum_server::Handle::new();
            let fut = shutdown_signal(Some(handle.clone()));

            if cfg.allow_redirections {
                info!("...with HTTP redirection on :7015");
                tokio::spawn(redirect_http_to_https(7015, instance.config.server.port, fut));
            }

            let addr = instance.config.server.addr();
            let config = RustlsConfig::from_pem_file(&cfg.cert, &cfg.cert_key).await?;

            info!(address = ?addr, "listening on HTTPS");
            axum_server::bind_rustls(addr, config)
                .handle(handle)
                .serve(router.into_make_service())
                .await
        } else {
            let addr = instance.config.server.addr();
            let listener = tokio::net::TcpListener::bind(addr).await?;
            info!(address = ?addr, "listening on HTTP");

            axum::serve(listener, router.into_make_service())
                .with_graceful_shutdown(shutdown_signal(None))
                .await
        }
        .context("unable to run HTTP service")?;

        info!("charted system is shutting down...");
        {
            let sessions = instance.sessions.lock().await;
            sessions.destroy();
        }

        info!("Goodbye...");
        Ok(())
    }
}

async fn redirect_http_to_https<F: Future<Output = ()> + Send + 'static>(http: u16, https: u16, signal: F) {
    fn make_https(host: String, uri: Uri, (http, https): (u16, u16)) -> Result<Uri, BoxError> {
        let mut parts = uri.into_parts();
        parts.scheme = Some(axum::http::uri::Scheme::HTTPS);

        if parts.path_and_query.is_none() {
            parts.path_and_query = Some("/".parse().unwrap());
        }

        let host = host.replace(&http.to_string(), &https.to_string());
        parts.authority = Some(host.parse()?);

        Ok(Uri::from_parts(parts)?)
    }

    let redirect = move |Host(host): Host, uri: Uri| async move {
        match make_https(host, uri, (http, https)) {
            Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
            Err(e) => {
                error!(error = %e, "failed to convert URI to HTTPS");
                Err(StatusCode::BAD_REQUEST)
            }
        }
    };

    let addr = SocketAddr::from(([127, 0, 0, 1], https));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!(address = %addr, "HTTP -> HTTPS redirection service is listening on");
    axum::serve(listener, redirect.into_make_service())
        .with_graceful_shutdown(signal)
        .await
        .unwrap();
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

fn print_banner() {
    let mut stdout = io::stdout().lock();
    let _ = writeln!(
        stdout,
        "{}",
        "«~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~»"
            .if_supports_color(Stdout, |x| x.fg_rgb::<134, 134, 134>())
    );

    let _ = writeln!(
        stdout,
        "{}        {}                {}           {}                                     {}",
        "«".if_supports_color(Stdout, |x| x.fg_rgb::<134, 134, 134>()),
        "_".if_supports_color(Stdout, |x| x.fg_rgb::<212, 171, 216>()),
        "_".if_supports_color(Stdout, |x| x.fg_rgb::<212, 171, 216>()),
        "_".if_supports_color(Stdout, |x| x.fg_rgb::<212, 171, 216>()),
        "»".if_supports_color(Stdout, |x| x.fg_rgb::<134, 134, 134>())
    );

    let _ = writeln!(
        stdout,
        "{}    {}  {}",
        "«".if_supports_color(Stdout, |x| x.fg_rgb::<134, 134, 134>()),
        "___| |__   __ _ _ __| |_ ___  __| |      ___  ___ _ ____   _____ _ __"
            .if_supports_color(Stdout, |x| x.fg_rgb::<212, 171, 216>()),
        "»".if_supports_color(Stdout, |x| x.fg_rgb::<134, 134, 134>())
    );

    let _ = writeln!(
        stdout,
        "{}   {} {}",
        "«".if_supports_color(Stdout, |x| x.fg_rgb::<134, 134, 134>()),
        "/ __| '_ \\ / _` | '__| __/ _ \\/ _` |_____/ __|/ _ \\ '__\\ \\ / / _ \\ '__|"
            .if_supports_color(Stdout, |x| x.fg_rgb::<212, 171, 216>()),
        "»".if_supports_color(Stdout, |x| x.fg_rgb::<134, 134, 134>())
    );

    let _ = writeln!(
        stdout,
        "{}  {}    {}",
        "«".if_supports_color(Stdout, |x| x.fg_rgb::<134, 134, 134>()),
        "| (__| | | | (_| | |  | ||  __/ (_| |_____\\__ \\  __/ |   \\ V /  __/ |"
            .if_supports_color(Stdout, |x| x.fg_rgb::<212, 171, 216>()),
        "»".if_supports_color(Stdout, |x| x.fg_rgb::<134, 134, 134>())
    );

    let _ = writeln!(
        stdout,
        "{}   {}    {}",
        "«".if_supports_color(Stdout, |x| x.fg_rgb::<134, 134, 134>()),
        "\\___|_| |_|\\__,_|_|   \\__\\___|\\__,_|     |___/\\___|_|    \\_/ \\___|_|"
            .if_supports_color(Stdout, |x| x.fg_rgb::<212, 171, 216>()),
        "»".if_supports_color(Stdout, |x| x.fg_rgb::<134, 134, 134>())
    );

    let _ = writeln!(
        stdout,
        "{}",
        "«~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~»"
            .if_supports_color(Stdout, |x| x.fg_rgb::<134, 134, 134>())
    );

    let _ = writeln!(stdout);
    let distribution = Distribution::detect();

    let _ = writeln!(
        stdout,
        "» Booting up {} v{}, compiled with Rust v{} on {distribution}",
        "charted-server".if_supports_color(Stdout, |x| x.bold()),
        crate::version().if_supports_color(Stdout, |x| x.bold()),
        crate::RUSTC_VERSION.if_supports_color(Stdout, |x| x.bold())
    );
}
