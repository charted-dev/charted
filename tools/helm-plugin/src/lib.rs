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
use std::io;
use tracing::{level_filters::LevelFilter, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};

#[macro_use]
extern crate tracing;

#[macro_use]
extern crate eyre;

pub mod args;
pub mod auth;
pub mod commands;
pub mod config;
pub mod ops;
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

    #[command(subcommand)]
    pub command: commands::Cmd,
}

impl Program {
    pub fn init_log(&self) {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::fmt::layer()
                    .with_level(true)
                    .with_target(true)
                    .with_thread_names(true)
                    .with_writer(io::stderr)
                    .with_filter(LevelFilter::from_level(self.level)),
            )
            .init();
    }
}

/// Returns the current version of `charted-helm-plugin`.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Returns the version and commit hash of `charted-helm-plugin`.
#[inline]
pub fn version() -> String {
    format!("v{}+{}", VERSION, charted_common::COMMIT_HASH)
}

#[cfg(test)]
#[test]
fn verify() {
    use clap::CommandFactory;

    Program::command().debug_assert();
}
