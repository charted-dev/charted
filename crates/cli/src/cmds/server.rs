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

use azalia::log::{
    writers::{self, default::Writer},
    WriteLayer,
};
use charted_config::Config;
use charted_core::Distribution;
use owo_colors::{OwoColorize, Stream::Stdout};
use std::{
    borrow::Cow,
    io::{self, Write as _},
    path::PathBuf,
};
use tracing::info;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::prelude::*;

/// Runs the API server.
#[derive(Debug, Clone, clap::Parser)]
pub struct Args {
    /// location to a relative/absolute path to a configuration file. by default, this will locate
    /// in `./config/charted.yml`/`./config.yml` if found.
    #[arg(short = 'c', long, env = "CHARTED_CONFIG_FILE")]
    config: Option<PathBuf>,

    /// amount of workers to spawn for the Tokio runtime. This cannot exceeded
    /// the amount of CPU cores you have.
    #[arg(short = 'w', long, env = "CHARTED_RUNTIME_WORKERS", default_value_t = num_cpus::get())]
    pub workers: usize,
}

pub async fn run(Args { config, .. }: Args) -> eyre::Result<()> {
    print_banner();

    let config =
        config
            .map(|path| Config::new(Some(path)))
            .unwrap_or(match Config::get_default_conf_location_if_any() {
                Ok(Some(path)) => Config::new(Some(path)),
                _ => Config::new::<&str>(None),
            })?;

    let _guard = sentry::init(sentry::ClientOptions {
        attach_stacktrace: true,
        server_name: Some(Cow::Borrowed("charted-server")),
        release: Some(Cow::Borrowed(charted_core::version())),
        dsn: config.sentry_dsn.clone(),

        ..Default::default()
    });

    init_logger(&config);
    info!("initializing systems...");

    let cx = charted_app::Context::new(config).await?;
    charted_server::start(cx).await
}

fn init_logger(config: &Config) {
    tracing_subscriber::registry()
        .with(
            match config.logging.json {
                false => WriteLayer::new_with(io::stdout(), Writer::default()),
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
