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

use super::Tokio;
use crate::install_eyre_hook;
use azalia::{
    config::env::TryFromEnv,
    log::{WriteLayer, writers},
};
use charted_config::Config;
use charted_core::{Distribution, ResultExt};
use charted_serverv2::Env;
use eyre::bail;
use opentelemetry::{InstrumentationScope, KeyValue, trace::TracerProvider};
use opentelemetry_otlp::SpanExporter;
use opentelemetry_sdk::trace::{SdkTracer, SdkTracerProvider};
use owo_colors::{OwoColorize, Stream::Stdout};
use std::{
    borrow::Cow,
    io::{self, Write},
    path::PathBuf,
};
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{EnvFilter, filter, prelude::*};

/// Runs the API server.
#[derive(Debug, clap::Parser)]
pub struct Args {
    /// Path to a charted `config.toml` configuration file.
    #[arg(long, short = 'c', env = "CHARTED_CONFIG_FILE")]
    config: Option<PathBuf>,

    #[command(flatten)]
    pub tokio: Tokio,
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

    let env = Env::new(config).await?;
    if let Err(e) = env.drive().await {
        tracing::error!(%e, "failed to run HTTP service");
    }

    env.close().await
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

    let loki_layer = (|| -> Result<Option<tracing_loki::Layer>, tracing_loki::Error> {
        match config.logging.loki {
            Some(ref config) => {
                let mut builder = tracing_loki::builder();
                for (name, label) in config.labels.iter() {
                    builder = builder.label(name.to_owned(), label)?;
                }

                for (name, field) in config.fields.iter() {
                    builder = builder.extra_field(name.to_owned(), field.to_owned())?;
                }

                for (name, header) in config.headers.iter() {
                    builder = builder.http_header(name, header)?;
                }

                let (layer, task) = builder.build_url(config.url.to_owned())?;
                tokio::spawn(task);

                Ok(Some(layer))
            }

            None => Ok(None),
        }
    })()?;

    let filter = (|| -> eyre::Result<Option<EnvFilter>> {
        let Some(filter) = config.logging.filter.clone() else {
            return Ok(None);
        };

        filter.into_env_filter().map(Some).into_report()
    })()?;

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
        .with(loki_layer)
        .with(tracer.map(|tracer| {
            tracing_opentelemetry::layer()
                .with_tracer(tracer)
                .with_filter(LevelFilter::from_level(config.logging.level))
        }))
        .with(tracing_error::ErrorLayer::default())
        .with(filter)
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
