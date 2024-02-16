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

use clap::Parser;
use indicatif::{ProgressState, ProgressStyle};
use std::{
    io::{self, IsTerminal},
    time::Duration,
};
use tracing::{level_filters::LevelFilter, Level};
use tracing_indicatif::IndicatifLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer, Registry};

#[macro_use]
extern crate tracing;

#[macro_use]
extern crate async_trait;

#[macro_use]
extern crate eyre;

pub mod args;
pub mod auth;
pub mod commands;
pub mod config;
pub mod util;

#[derive(Debug, Clone, Parser)]
#[clap(
    bin_name = "helm charted",
    about = "🐻‍❄️📦 Faciliate downloading, pushing, and misc. tools for `charted-server` as a Helm plugin",
    author = "Noelware, LLC.",
    override_usage = "helm charted <COMMAND> [...ARGS]",
    arg_required_else_help = true
)]
pub struct Program {
    /// Sets the global logging level when building the logging system for `helm charted`.
    #[arg(global = true, short = 'l', long = "log-level", env = "CHARTED_HELM_LOG_LEVEL", default_value_t = Level::INFO)]
    pub level: Level,

    /// Disables the use of the progress bars for `helm charted download` and `helm charted push`. This is also disabled if there
    /// is no TTY attached.
    #[arg(global = true, long = "no-progress", env = "CHARTED_HELM_NO_PROGRESS", default_value_t = __check_if_enabled())]
    pub no_progress: bool,

    #[command(subcommand)]
    pub command: commands::Cmd,
}

fn elapsed_subsec(state: &ProgressState, writer: &mut dyn std::fmt::Write) {
    let seconds = state.elapsed().as_secs();
    let sub_seconds = (state.elapsed().as_millis() % 1000) / 100;
    let _ = writer.write_str(&format!("{}.{}s", seconds, sub_seconds));
}

impl Program {
    pub fn init_log(&self) {
        let filter = LevelFilter::from_level(self.level);
        let layer = tracing_subscriber::fmt::layer()
            .with_file(false)
            .with_line_number(false)
            .with_target(true)
            .with_thread_names(true);

        match self.no_progress {
            false => {
                let indicatif: IndicatifLayer<Registry> = IndicatifLayer::new()
                    .with_progress_style(
                        ProgressStyle::with_template(
                            "{color_start}{span_child_prefix} -- {span_name} {wide_msg} {elapsed_subsec}{color_end}",
                        )
                        .unwrap()
                        .with_key("elapsed_subsec", elapsed_subsec)
                        .with_key(
                            "color_start",
                            |state: &ProgressState, writer: &mut dyn std::fmt::Write| {
                                let elapsed = state.elapsed();

                                if elapsed > Duration::from_secs(8) {
                                    // Red
                                    let _ = write!(writer, "\x1b[{}m", 1 + 30);
                                } else if elapsed > Duration::from_secs(4) {
                                    // Yellow
                                    let _ = write!(writer, "\x1b[{}m", 3 + 30);
                                }
                            },
                        )
                        .with_key(
                            "color_end",
                            |state: &ProgressState, writer: &mut dyn std::fmt::Write| {
                                if state.elapsed() > Duration::from_secs(4) {
                                    let _ = write!(writer, "\x1b[0m");
                                }
                            },
                        ),
                    )
                    .with_span_child_prefix_symbol("↳ ")
                    .with_span_child_prefix_indent(" ");

                tracing_subscriber::registry()
                    .with(layer.with_writer(indicatif.get_stderr_writer()).with_filter(filter))
                    .init();
            }

            true => tracing_subscriber::registry().with(layer.with_filter(filter)).init(),
        }
    }
}

fn __check_if_enabled() -> bool {
    let stdout = io::stdout().lock();
    if !stdout.is_terminal() {
        return false;
    }

    true
}

/// Returns the current version of `charted-helm-plugin`.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Returns the version and commit hash of `charted-helm-plugin`.
#[inline]
pub fn version() -> String {
    format!("v{}+{}", VERSION, charted::COMMIT_HASH)
}

#[cfg(test)]
#[test]
fn verify() {
    use clap::CommandFactory;

    Program::command().debug_assert();
}
