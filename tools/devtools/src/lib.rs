// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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
use commands::Command;
use std::{ffi::OsString, path::PathBuf};
use tracing::Level;

#[macro_use]
extern crate tracing;

mod commands;
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
    #[arg(global = true, short = 'l', long = "log-level", env = "DEVTOOLS_LOG_LEVEL")]
    pub level: Option<Level>,

    #[command(subcommand)]
    pub cmd: Command,
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

    /// Whether or not if we should use `cargo run` instead of `cargo build`. If both `--release`
    /// and `--run` are specified, it will run the Release binary instead of the Debug one.
    #[arg(long)]
    pub run: bool,
}
