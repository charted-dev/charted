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

use clap::value_parser;
use std::{ffi::OsString, path::PathBuf};

mod cli;
mod helm_plugin;
mod internals;
mod server;

/// Represents all the common arguments that *mostly* are present in all CLI commands
#[derive(clap::Args)]
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

#[derive(clap::Subcommand)]
pub enum Cmd {
    HelmPlugin(helm_plugin::Args),
    Internals(internals::Args),
    Server(server::Args),
    Cli(cli::Args),
}

pub fn run(cmd: Cmd) -> eyre::Result<()> {
    match cmd {
        Cmd::Server(args) => server::run(args),
        Cmd::HelmPlugin(args) => helm_plugin::run(args),
        Cmd::Cli(args) => cli::run(args),
        Cmd::Internals(args) => internals::run(args),
    }
}
