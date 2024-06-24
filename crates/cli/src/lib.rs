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

mod cmds;
pub use cmds::{exec, Cmd};

use clap::Parser;
use noelware_log::writers;
use std::io;
use tracing::Level;
use tracing_subscriber::prelude::*;

#[derive(Debug, Clone, Parser)]
#[clap(
    bin_name = "charted",
    about = "🐻‍❄️📦 Free, open source, and reliable Helm Chart registry made in Rust",
    author = "Noelware, LLC. <team@noelware.org>",
    override_usage = "charted <COMMAND> [..ARGS..]",
    arg_required_else_help = true
)]
pub struct Program {
    /// Configures the log level for all CLI commands. This will not configure the log level
    /// for `charted server`.
    #[arg(long, global = true, short = 'l', default_value_t = Level::INFO)]
    pub log_level: Level,

    #[command(subcommand)]
    pub command: cmds::Cmd,
}

impl Program {
    /// Initializes the logger for the program. Do not use this outside
    /// of the library since it'll panic if one was already initialised.
    pub fn init_logging(&self) {
        tracing_subscriber::registry()
            .with(noelware_log::WriteLayer::new_with(
                io::stdout(),
                writers::default::Writer {
                    print_timestamp: true,
                    print_level: true,
                    print_module: false,
                    print_thread: false,
                    emit_spans: true,

                    ..Default::default()
                },
            ))
            .init();
    }
}
