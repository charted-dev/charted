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

pub mod commands;

use azalia::log::{writers::default::Writer, WriteLayer};
use color_eyre::config::HookBuilder;
use commands::Subcommand;
use std::{future::Future, io};
use tracing::{level_filters::LevelFilter, Level};

#[derive(Debug, clap::Parser)]
#[clap(
    bin_name = "charted",
    about = "🐻‍❄️📦 Free, open source, and reliable Helm Chart registry made in Rust",
    author = "Noelware, LLC. <team@noelware.org>",
    override_usage = "charted <COMMAND> [ARGS]",
    arg_required_else_help = true,
    disable_version_flag = true
)]
pub struct Program {
    /// Configures the log level for all CLI commands.
    ///
    /// This will not configure the log level for the `server` subcommand.
    #[arg(
        global = true,
        short = 'l',
        long = "log-level",
        default_value_t = Level::INFO,
        env = "CHARTED_LOG_LEVEL"
    )]
    pub level: Level,

    #[command(subcommand)]
    pub command: Subcommand,
}

impl Program {
    #[doc(hidden)]
    pub fn init_logger(&self) {
        use tracing_subscriber::prelude::*;

        tracing_subscriber::registry()
            .with(
                WriteLayer::new_with(
                    io::stderr(),
                    Writer {
                        print_module: false,
                        print_thread: false,

                        ..Default::default()
                    },
                )
                .with_filter(LevelFilter::from_level(self.level)),
            )
            .init();
    }

    /// Runs the subcommand that was selected by the consumer.
    pub fn execute(self) -> impl Future<Output = eyre::Result<()>> {
        commands::execute(self.command)
    }
}

pub fn install_eyre_hook() -> eyre::Result<()> {
    HookBuilder::new()
        .issue_url(concat!(env!("CARGO_PKG_REPOSITORY"), "/issues/new?labels=rust"))
        .capture_span_trace_by_default(true)
        .add_issue_metadata("version", charted_core::version())
        .add_issue_metadata("rustc", charted_core::RUSTC_VERSION)
        .install()
}

#[cfg(test)]
#[test]
fn verify() {
    use clap::CommandFactory;

    Program::command().debug_assert();
}
