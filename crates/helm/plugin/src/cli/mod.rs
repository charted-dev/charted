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

use azalia::log::writers::default::Writer;
use commands::Subcommand;
use std::io;
use tracing::Level;
use tracing_subscriber::prelude::*;

#[derive(Debug, clap::Parser)]
#[clap(
    bin_name = "charted",
    about = "🐻‍❄️📦 Faciliate Helm operations with charted-server easily",
    author = "Noelware, LLC. <team@noelware.org>",
    override_usage = "helm charted <COMMAND> [...ARGS]",
    arg_required_else_help = true,
    disable_version_flag = true
)]
pub struct Program {
    /// Log level for the log output.
    #[arg(long, short = 'l', env = "CHARTED_LOG_LEVEL", global = true, default_value_t = Level::INFO)]
    pub log_level: Level,

    #[command(subcommand)]
    pub command: Subcommand,
}

impl Program {
    #[doc(hidden)]
    pub fn init_logging(&self) {
        tracing_subscriber::registry()
            .with(azalia::log::WriteLayer::new_with(io::stderr(), Writer {
                print_module: false,
                print_thread: false,

                ..Default::default()
            }))
            .init();
    }
}
