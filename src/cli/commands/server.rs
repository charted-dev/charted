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

use crate::{authz, db, redis, Instance};
use charted_common::Snowflake;
use charted_config::Config;
use charted_entities::Distribution;
use charted_helm_charts as charts;
use noelware_log::{writers, WriteLayer};
use owo_colors::{OwoColorize, Stream::Stdout};
use remi::StorageService;
use sentry::types::Dsn;
use std::{
    borrow::Cow,
    io::{self, Write as _},
    path::PathBuf,
    str::FromStr,
    sync::{atomic::AtomicUsize, Arc},
    time::Instant,
};
use tokio::sync::{Mutex, RwLock};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::prelude::*;

/// Runs the API server.
#[derive(Debug, Clone, clap::Parser)]
pub struct Args {
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

impl Args {
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
            .init();
    }
}

pub async fn run(args: Args) -> eyre::Result<()> {
    let config = match args.config {
        Some(ref path) => Config::new(Some(path)),
        None => match Config::find_default_conf_location() {
            Some(path) => Config::new(Some(path)),
            None => Config::new::<&str>(None),
        },
    }?;

    if args.print {
        eprintln!("{}", toml::to_string(&config).unwrap());
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
    Args::init_log(&config);

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
        charted_config::storage::Config::Filesystem(ref fs) => {
            noelware_remi::StorageService::Filesystem(remi_fs::StorageService::with_config(fs.clone()))
        }

        charted_config::storage::Config::Azure(ref azure) => {
            noelware_remi::StorageService::Azure(remi_azure::StorageService::new(azure.clone()))
        }

        charted_config::storage::Config::S3(ref s3) => {
            noelware_remi::StorageService::S3(remi_s3::StorageService::new(s3.clone()))
        }
    };

    storage.init().await?;
    info!(took = ?Instant::now().duration_since(now), "initialized external storage successfully");
    now = Instant::now();

    info!("initializing authz backend...");
    let authz: Arc<dyn crate::authz::Backend> = match config.sessions.backend {
        charted_config::sessions::Backend::Local => Arc::new(authz::local::Backend::new(pool.clone())),
        charted_config::sessions::Backend::Ldap(ref config) => Arc::new(authz::ldap::Backend::new(config.clone())),
        charted_config::sessions::Backend::Passwordless | charted_config::sessions::Backend::Htpasswd(_) => {
            warn!("other authz backends are not supported at this time, switching to `local` backend instead");
            Arc::new(authz::local::Backend::new(pool.clone()))
        }
    };

    info!(took = ?Instant::now().duration_since(now), "initialized authz backend successfully");

    info!("now initializing sessions manager");
    let mut sessions = crate::sessions::Manager::new(redis.clone());
    sessions.init()?;

    info!(took = ?Instant::now().duration_since(now), "initialized sessions manager successfully");
    now = Instant::now();

    let charts = charts::HelmCharts::new(storage.clone());
    charts.init().await?;

    info!(took = ?Instant::now().duration_since(now), "initialized core chart library");

    let instance = Instance {
        controllers: db::controllers::Controllers::new(&config, &pool, &redis),
        snowflake: Snowflake::computed(),
        requests: AtomicUsize::new(0),
        sessions: Arc::new(Mutex::new(sessions)),
        metrics: crate::metrics::new(&config.metrics),
        storage,
        charts,
        config,
        redis: Arc::new(RwLock::new(redis)),
        authz,
        pool,
    };

    crate::set_instance(instance.clone());
    info!(took = ?Instant::now().duration_since(original), "initialized global instance, starting server...");

    crate::server::start(instance).await?;

    info!("charted system is shutting down...");
    let instance = Instance::get();

    {
        let sessions = instance.sessions.lock().await;
        sessions.destroy();
    }

    info!("Goodbye...");
    Ok(())
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
