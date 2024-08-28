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

//! The `charted-devtools` library is *unstable* and shouldn't be used outside
//! of charted itself.

#[macro_use]
extern crate tracing;

pub mod commands;

use azalia::log::{writers::default::Writer, WriteLayer};
use commands::Cmd;
use eyre::{eyre, Context};
use std::{
    ffi::OsStr,
    io,
    path::{Path, PathBuf},
    process::{exit, Command, Output, Stdio},
};
use tracing::{level_filters::LevelFilter, Level};

#[derive(clap::Parser)]
pub struct Program {
    /// Level to apply when configuring the CLI logger. If the range for the configured
    /// log level is `DEBUG` or `TRACE`, then file paths will be emitted in the logs where
    /// it was invoked.
    #[arg(global = true, short = 'l', long = "log-level", env = "DEVTOOLS_LOG_LEVEL", default_value_t = Level::INFO)]
    pub level: Level,

    #[command(subcommand)]
    pub command: Cmd,
}

impl Program {
    pub fn init_logging(&self) {
        use tracing_subscriber::prelude::*;

        let writer = Writer {
            print_module: self.level >= Level::DEBUG,
            print_thread: false,

            ..Default::default()
        };

        tracing_subscriber::registry()
            .with(WriteLayer::new_with(io::stdout(), writer).with_filter(LevelFilter::from_level(self.level)))
            .init();
    }
}

/// Finds a binary with a specific path, or finds it under the `$PATH` variable
/// with a given `bin`.
///
/// ## Example
/// ```
/// # use charted_devtools::find_binary;
/// #
/// let bin = find_binary::<&str>(None, "rustc");
/// assert!(bin.is_some());
/// ```
pub fn find_binary<P: AsRef<Path>>(path: Option<P>, bin: &str) -> Option<PathBuf> {
    if let Some(p) = path {
        return Some(p.as_ref().to_path_buf());
    }

    which::which(bin).ok()
}

/// Creates a [`Command`] instance and returns the output of that command. The `builder` parameter
/// is used to customise the output itself or add additional arguments, environment variables, etc.
///
/// ## Example
/// ```
/// # use charted_devtools::execute;
/// #
/// let cmd = execute("ls", |_| {});
/// assert!(cmd.is_ok());
/// ```
pub fn execute<C: AsRef<OsStr>, F: FnOnce(&mut Command)>(command: C, builder: F) -> eyre::Result<Output> {
    let name = command.as_ref();
    let mut cmd = Command::new(name);
    builder(&mut cmd);

    // By default, `stdin` is closed; `stdout`/`stderr` are piped so you can access the output
    // of the command.
    cmd.stdin(Stdio::null());

    let args = cmd.get_args().map(|x| x.to_string_lossy()).collect::<Vec<_>>();
    info!("$ {} {}", name.to_string_lossy(), args.join(" "));

    let output = cmd
        .output()
        .with_context(|| format!("failed to run command '{}'", name.to_string_lossy()))?;

    if output.status.success() {
        return Ok(output);
    }

    let code = output.status.code().unwrap_or(-1);
    if output.stdout.is_empty() && output.stderr.is_empty() {
        exit(code);
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    error!("-- command has failed with code {code} --");
    if !stdout.is_empty() {
        error!("~! STDOUT !~\n{stdout}");
    }

    error!("~! STDERR !~\n{stderr}");

    Err(eyre!("failed to run command"))
}

pub fn cargo<P: AsRef<Path>, F: FnOnce(&mut Command)>(
    cargo: Option<P>,
    subcommand: impl AsRef<OsStr>,
    builder: F,
) -> eyre::Result<Output> {
    let cargo = find_binary(cargo, "cargo").ok_or_else(|| eyre!("failed to find `cargo` binary"))?;
    execute(cargo, |cmd| {
        cmd.arg(subcommand.as_ref());
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());

        builder(cmd);
    })
}
