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

use crate::install_eyre_hook;
use azalia::{
    config::TryFromEnv,
    log::{WriteLayer, writers},
    remi::{StorageService, core::StorageService as _},
};
use charted_authz::Authenticator;
use charted_config::{Config, metrics, sessions::Backend, storage};
use charted_core::Distribution;
use charted_server::set_context;
use eyre::bail;
use opentelemetry::{InstrumentationScope, KeyValue, trace::TracerProvider};
use opentelemetry_otlp::SpanExporter;
use opentelemetry_sdk::trace::{SdkTracer, SdkTracerProvider};
use owo_colors::{OwoColorize, Stream::Stdout};
use std::{
    borrow::Cow,
    io::{self, Write},
    path::PathBuf,
    sync::{Arc, atomic::AtomicUsize},
};
use tracing::{info, level_filters::LevelFilter, warn};
use tracing_subscriber::{filter, prelude::*};

/// Runs the API server.
#[derive(Debug, clap::Parser)]
pub struct Args {
    /// Path to a charted `config.toml` configuration file.
    #[arg(long, short = 'c', env = "CHARTED_CONFIG_FILE")]
    config: Option<PathBuf>,

    /// Number of Tokio workers to use.
    ///
    /// By default, this will use the number of avaliable CPU cores on the system
    /// itself.
    #[arg(long, short = 'w', env = "TOKIO_WORKER_THREADS", default_value_t = num_cpus::get())]
    pub workers: usize,
}

pub(crate) async fn run(Args { config, .. }: Args) -> eyre::Result<()> {
    print_banner();

    let config = load_config(config)?;
    install_eyre_hook()?;

    let _guard = sentry::init(sentry::ClientOptions {
        attach_stacktrace: true,
        server_name: Some(Cow::Borrowed("charted-server")),
        release: Some(Cow::Borrowed(charted_core::version())),
        dsn: config.sentry_dsn.clone(),

        ..Default::default()
    });

    init_logger(&config)?;
    info!("Hello world!");

    let pool = charted_database::create_pool(&config.database).await?;
    let storage = match config.storage.clone() {
        storage::Config::Filesystem(fs) => {
            StorageService::Filesystem(azalia::remi::fs::StorageService::with_config(fs))
        }

        storage::Config::Azure(azure) => StorageService::Azure(azalia::remi::azure::StorageService::new(azure)?),
        storage::Config::S3(s3) => StorageService::S3(azalia::remi::s3::StorageService::new(s3)),
    };

    azalia::remi::StorageService::init(&storage).await?;
    charted_helm_charts::init(&storage).await?;

    let authz: Arc<dyn Authenticator> = match config.sessions.backend.clone() {
        Backend::Static(mapping) => Arc::new(charted_authz_static::Backend::new(mapping)),
        Backend::Local => Arc::new(charted_authz_local::Backend::default()),
        b => {
            warn!("using the {} backend is not supported! switching to local backend", b);
            Arc::new(charted_authz_local::Backend::default())
        }
    };

    let prom_handle = match config.metrics.clone() {
        metrics::Config::Disabled => None,
        metrics::Config::OpenTelemetry(config) => {
            charted_metrics::init_opentelemetry(&config)?;
            None
        }

        metrics::Config::Prometheus(config) => Some(charted_metrics::init_prometheus(&config)?),
    };

    #[allow(unused_mut)]
    let mut features = azalia::hashmap!();
    let context = charted_server::Context {
        requests: AtomicUsize::new(0),
        features,
        storage,
        config,
        authz,
        pool,
    };

    set_context(context.clone());
    charted_server::drive(prom_handle.as_ref()).await?;

    warn!("server has been closed, closing resources...");
    context.pool.close().await?;

    warn!("goodbye.");
    Ok(())
}

pub(in crate::commands) fn load_config(config: Option<PathBuf>) -> eyre::Result<Config> {
    config
        .map(|path| Config::load(Some(path)))
        .unwrap_or_else(|| match Config::find_default_location() {
            Ok(Some(path)) => Config::load(Some(path)),
            Err(err) => {
                eprintln!("[charted :: WARN] failed to load configuration files from default locations: {err}");
                Config::try_from_env()
            }

            _ => Config::load::<&str>(None),
        })
}

fn init_logger(config: &Config) -> eyre::Result<()> {
    let tracer = match config.tracing.as_ref().map(get_otel_tracer) {
        Some(Ok(tracer)) => Some(tracer),
        Some(Err(report)) => return Err(report),

        _ => None,
    };

    tracing_subscriber::registry()
        .with(
            if config.logging.json {
                WriteLayer::new_with(io::stdout(), writers::json)
            } else {
                WriteLayer::new_with(io::stdout(), writers::default::Writer::default())
            }
            .with_filter(LevelFilter::from_level(config.logging.level))
            .with_filter(filter::filter_fn(|meta| {
                // disallow from getting logs from `tokio` since it doesn't contain anything
                // useful to us
                !meta.target().starts_with("tokio::")
            })),
        )
        .with(sentry_tracing::layer())
        .with(tracer.map(|tracer| {
            tracing_opentelemetry::layer()
                .with_tracer(tracer)
                .with_filter(LevelFilter::from_level(config.logging.level))
        }))
        .with(tracing_error::ErrorLayer::default())
        .try_init()
        .map_err(Into::into)
}

fn get_otel_tracer(config: &charted_config::tracing::Config) -> eyre::Result<SdkTracer> {
    let mut provider = SdkTracerProvider::builder();
    match config.url.scheme() {
        "http" | "https" => {
            let exporter = SpanExporter::builder().with_http().build()?;
            provider = provider.with_simple_exporter(exporter);
        }

        "grpc" | "grpcs" => {
            let exporter = SpanExporter::builder().with_tonic().build()?;
            provider = provider.with_simple_exporter(exporter);
        }

        scheme => bail!("unknown scheme for OpenTelemetry Collector: {}", scheme),
    }

    let provider = provider.build();
    let mut attrs = config
        .labels
        .iter()
        .map(|(key, value)| KeyValue::new(key.clone(), value.clone()))
        .collect::<Vec<_>>();

    attrs.push(KeyValue::new("service.name", "charted-server"));
    attrs.push(KeyValue::new("service.vendor", "Noelware, LLC."));
    attrs.push(KeyValue::new("charted.version", charted_core::version()));

    Ok(provider.tracer_with_scope(
        InstrumentationScope::builder("charted-server")
            .with_version(charted_core::version())
            .with_attributes(attrs)
            .build(),
    ))
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
        "» Booting up {} {}, compiled with Rust {} on {distribution}",
        "charted-server".if_supports_color(Stdout, |x| x.bold()),
        charted_core::version().if_supports_color(Stdout, |x| x.bold()),
        charted_core::RUSTC_VERSION.if_supports_color(Stdout, |x| x.bold())
    );
}
