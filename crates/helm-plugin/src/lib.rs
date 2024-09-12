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

#![allow(clippy::incompatible_msrv)]

pub(crate) mod args;
pub(crate) mod auth;
pub(crate) mod config;
pub(crate) mod ops;
pub(crate) mod util;

pub mod cmds;

use azalia::log::{writers::default::Writer, WriteLayer};
use std::io;
use tracing::{level_filters::LevelFilter, Level};
use tracing_subscriber::prelude::*;

#[derive(Debug, Clone, clap::Parser)]
#[clap(
    bin_name = "helm charted",
    about = "🐻‍❄️📦 Helm plugin to help aid developing Helm charts with charted",
    author = "Noelware, LLC. <team@noelware.org>",
    override_usage = "helm charted <COMMAND> [...ARGS]",
    arg_required_else_help = true
)]
pub struct Program {
    /// Configures the log level for the logs that are transmitted.
    #[arg(
        global = true,
        short = 'l',
        long = "log-level",
        default_value_t = Level::INFO,
        env = "CHARTED_HELM_LOG_LEVEL"
    )]
    pub level: Level,

    #[command(subcommand)]
    pub cmd: cmds::Cmd,
}

impl Program {
    pub fn init_logger(&self) {
        tracing_subscriber::registry()
            .with(
                WriteLayer::new_with(
                    io::stdout(),
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
}

#[cfg(test)]
#[test]
fn cli() {
    use clap::CommandFactory;

    Program::command().debug_assert();
}
