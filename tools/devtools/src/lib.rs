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

use clap::value_parser;
use commands::Cmd;
use std::{ffi::OsString, path::PathBuf};
use tracing::{level_filters::LevelFilter, Level};
use tracing_subscriber::prelude::*;

#[macro_use]
extern crate tracing;

mod commands;
pub use commands::run;

pub mod utils;

/// CLI program that is used to execute the main binary.
#[derive(Debug, Clone, clap::Parser)]
#[clap(
    bin_name = "./dev",
    about = "Program that helps relieves stress when developing `charted-server`",
    author = "Noelware, LLC. <team@noelware.org>",
    override_usage = "./dev <COMMAND> [...ARGS]",
    arg_required_else_help = true
)]
pub struct Program {
    /// DEPRECATED: This is similar to `--log-level=debug`.
    ///
    /// Whether or not if verbose logs should be emitted, this will enable
    /// all logs in the `DEBUG` range.
    #[arg(global = true, short = 'v', long = "verbose", env = "DEVTOOLS_VERBOSE")]
    pub verbose: bool,

    /// Level to apply when configuring the CLI logger. If the range for the configured
    /// log level is `DEBUG` or `TRACE`, then file paths will be emitted in the logs where
    /// it was invoked.
    #[arg(global = true, short = 'l', long = "log-level", env = "DEVTOOLS_LOG_LEVEL", default_value_t = Level::INFO)]
    pub level: Level,

    #[command(subcommand)]
    pub cmd: Cmd,
}

impl Program {
    /// Initializes a global tracing subscriber for all CLI-based commands.
    pub fn init_log(&self) {
        let level = match self.verbose {
            true => Level::DEBUG,
            false => self.level,
        };

        tracing_subscriber::registry()
            .with(
                tracing_subscriber::fmt::layer()
                    .with_level(true)
                    .with_target(true)
                    .with_file(self.verbose)
                    .with_line_number(self.verbose)
                    .with_filter(LevelFilter::from_level(level)),
            )
            .init();
    }
}

/// Represents all the common arguments that *mostly* are present in all CLI commands
#[derive(Debug, Clone, clap::Args)]
pub struct CommonArgs {
    /// Appends the `--release` flag to Cargo, which will build the specified
    /// project in Release mode and adds other `rustc` flags.
    #[arg(long)]
    pub release: bool,

    /// Other `$RUSTFLAGS` to append when invoking Cargo.
    #[arg(long, env = "RUSTFLAGS")]
    pub rustc_flags: Option<OsString>,

    /// Location as an absolute path to the `cargo` binary.
    #[arg(long, env = "CARGO")]
    pub cargo: Option<PathBuf>,

    /// Append additional arguments to the `cargo` binary. This will removed
    /// already defined arguments that a subcommand might need.
    #[arg(long, env = "CARGO_ARGS")]
    pub cargo_args: Vec<OsString>,

    /// Additional arguments to pass to the built binary.
    #[arg(value_parser = value_parser!(OsString), num_args = 0.., last = true, allow_hyphen_values = true)]
    pub args: Vec<OsString>,
}
