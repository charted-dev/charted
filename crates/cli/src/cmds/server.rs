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

use charted_common::Snowflake;
use charted_config::{sessions::Backend, storage, Config};
use charted_database::{controllers, make_worker};
use charted_entities::Distribution;
use charted_helm_charts::HelmCharts;
use charted_server::ServerContext;
use eyre::eyre;
use noelware_log::{writers, WriteLayer};
use noelware_remi::StorageService;
use sentry::types::Dsn;
use std::{
    borrow::Cow,
    future::Future,
    io,
    path::PathBuf,
    str::FromStr,
    sync::{atomic::AtomicUsize, Arc, RwLock},
    time::Instant,
};
use tracing::{debug, info, level_filters::LevelFilter};

/// Runs the API server
#[derive(Debug, Clone, clap::Parser)]
pub struct Args {
    /// Location to a path that can be deserialized into the configuration format.
    ///
    /// By default, it'll look in the paths:
    ///
    /// * `./config/charted.toml`
    /// * `./config.toml`
    #[arg(long, short = 'c', env = "CHARTED_CONFIG_FILE")]
    config: Option<PathBuf>,

    /// Amount of workers to use for the Tokio runtime that are used.
    #[arg(long, short = 'w', env = "CHARTED_RUNTIME_WORKERS", default_value_t = num_cpus::get())]
    pub workers: usize,
}

impl Args {
    fn init_logging_system(config: &Config) {
        use tracing_subscriber::prelude::*;

        tracing_subscriber::registry()
            .with(
                match config.logging.json {
                    false => WriteLayer::new_with(io::stdout(), writers::default::Writer::default()),
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
            .with(tracing_error::ErrorLayer::default())
            .init();
    }
}

pub async fn run(Args { config, .. }: Args) -> eyre::Result<()> {
    // 1. Print cute banner :3
    print_banner();

    // 2. Load the configuration file
    let config = match config {
        Some(ref path) => Config::new(Some(path))?,
        None => match Config::find_default_conf_location() {
            Some(path) => Config::new(Some(path))?,
            None => Config::new::<&str>(None)?,
        },
    };

    // 3. Setup Sentry
    let _sentry_guard = sentry::init(sentry::ClientOptions {
        traces_sample_rate: 1.0,
        attach_stacktrace: true,
        server_name: Some(Cow::Borrowed("charted-server")),
        release: Some(Cow::Borrowed(charted_common::version())),
        dsn: match config.sentry_dsn.as_ref().map(|dsn| Dsn::from_str(dsn)) {
            Some(Ok(dsn)) => Some(dsn),
            Some(Err(e)) => return Err(e.into()),
            None => None,
        },

        ..Default::default()
    });

    // 4. Initialize logging system
    Args::init_logging_system(&config);

    let mut now = Instant::now();
    let original = now; // keep a copy so we can see how long initialization takes
    let pool = keep_track_of_async(&mut now, || {
        info!("API server is now initializing! Attempting to create a connection to PostgreSQL 🛰️");
        charted_database::create_pool(&config.database)
    })
    .await?;

    info!(
        took = ?Instant::now().duration_since(now),
        "🛰️   Created a connection and pinged PostgreSQL server, creating a connection to a Redis server"
    );

    let redis = keep_track_of(&mut now, || charted_core::redis::Client::new(&config.redis))?;
    info!(took = ?Instant::now().duration_since(now), "🛰️   Created a connection and pinged Redis server!");

    let storage = keep_track_of(&mut now, || {
        info!("creating data storage for persistent data...");
        let storage = config.storage.clone();
        match storage {
            storage::Config::S3(s3) => noelware_remi::StorageService::S3(remi_s3::StorageService::new(s3)),
            storage::Config::Filesystem(fs) => {
                noelware_remi::StorageService::Filesystem(remi_fs::StorageService::with_config(fs))
            }

            storage::Config::Azure(azure) => {
                noelware_remi::StorageService::Azure(remi_azure::StorageService::new(azure))
            }
        }
    });

    <StorageService as noelware_remi::remi::StorageService>::init(&storage).await?;
    info!(took = ?Instant::now().duration_since(now), "initialized data storage successfully");

    let authz = keep_track_of::<eyre::Result<Arc<dyn charted_authz::Authenticator>>, _>(&mut now, || {
        info!("initializing authz backend");
        match config.sessions.backend {
            Backend::Local => Ok(Arc::new(charted_authz_local::Authenticator::new(pool.clone()))),
            _ => Err(eyre!("other authz backends are not supported in this build")),
        }
    })?;

    info!(took = ?Instant::now().duration_since(now), "initialized authz backend, now initializing other core systems...");
    let charts = HelmCharts::new(storage.clone());
    charts.init().await?;

    let controllers = {
        debug!("init db controller registry");

        let pool = pool.clone();
        let redis = redis.clone();
        let mut registry = charted_database::controllers::Registry::new();

        registry.insert(controllers::users::DbController::new(
            pool.clone(),
            make_worker(redis, &config.database)?,
        ));

        Ok::<_, eyre::Report>(registry)
    }?;

    let metrics = {
        debug!("init metrics registry");

        let config = config.metrics.clone();
        let mut registry: Arc<dyn charted_metrics::Registry> = match (config.enabled, config.prometheus) {
            (true, true) => Arc::new(charted_metrics::prometheus::Prometheus::new(
                charted_metrics::Minimal::default(),
                None,
            )),

            (false, true) => Arc::new(charted_metrics::prometheus::Prometheus::new(
                charted_metrics::Disabled::default(),
                None,
            )),

            (true, false) => Arc::new(charted_metrics::Minimal::default()),
            (false, false) => Arc::new(charted_metrics::Disabled::default()),
        };

        #[cfg(tokio_unstable)]
        let collectors: [Box<dyn charted_metrics::Collector>; 3] = [
            Box::new(::charted_metrics::collectors::ProcessCollector),
            Box::new(::charted_metrics::collectors::TokioCollector),
            Box::new(::charted_server::metrics::Collector),
        ];

        #[cfg(not(tokio_unstable))]
        let collectors: [Box<dyn charted_metrics::Collector>; 2] = [
            Box::new(::charted_metrics::collectors::ProcessCollector),
            Box::new(::charted_server::metrics::Collector),
        ];

        // TODO(@auguwu): once `Arc::get_mut_unchecked` is stablised, use instead
        //                of `Arc::get_mut().unwrap()` as we only have created
        //                the registry and don't plan on creating strong/weak
        //                references at this stage.
        for collector in collectors {
            Arc::get_mut(&mut registry).unwrap().insert(collector);
        }

        registry
    };

    let ctx = ServerContext {
        controllers,
        snowflake: Snowflake::computed(),
        requests: AtomicUsize::new(0),
        metrics,
        storage,
        charts,
        config,
        authz,
        redis: Arc::new(RwLock::new(redis)),
        http: reqwest::ClientBuilder::new().build()?,
        pool,
    };

    charted_server::set_global(ctx.clone());
    info!(took = ?now.duration_since(original), "initialized other core systems!");

    charted_server::start(ctx).await
}

fn keep_track_of<O, F: FnOnce() -> O>(now: &mut Instant, track: F) -> O {
    let result = track();
    *now = Instant::now();

    result
}

async fn keep_track_of_async<O, Fut: Future<Output = O> + Send, F: FnOnce() -> Fut>(now: &mut Instant, track: F) -> O {
    let result = track().await;
    *now = Instant::now();

    result
}

fn print_banner() {
    use owo_colors::{OwoColorize, Stream::Stdout};
    use std::io::Write;

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
        "» Booting up {} v{}, compiled with Rust v{} on {distribution} ({}/{})",
        "charted-server".if_supports_color(Stdout, |x| x.bold()),
        charted_common::version().if_supports_color(Stdout, |x| x.bold()),
        charted_common::RUSTC_VERSION.if_supports_color(Stdout, |x| x.bold()),
        charted_common::os().if_supports_color(Stdout, |x| x.bold()),
        charted_common::architecture().if_supports_color(Stdout, |x| x.bold())
    );
}
