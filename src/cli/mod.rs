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

pub mod commands;

use tracing::{level_filters::LevelFilter, Level};
use tracing_subscriber::{prelude::*, Layer};

/// Represents a generic synchronous execution pipeline for a CLI command.
pub trait Execute: Sized {
    /// Executes the command.
    fn execute(&self) -> eyre::Result<()>;
}

/// Like [`Execute`], but for asynchronous commands.
#[async_trait]
pub trait AsyncExecute: Sized {
    /// Asynchronously execute the command.
    async fn execute(&self) -> eyre::Result<()>;
}

/// Represents the CLI program
#[derive(Debug, Clone, clap::Parser)]
#[clap(
    bin_name = "charted",
    about = "🐻‍❄️📦 Free, open source, and reliable Helm Chart registry made in Rust",
    author = "Noelware, LLC.",
    override_usage = "charted <COMMAND> [...ARGS]",
    arg_required_else_help = true
)]
pub struct Program {
    /// Configures the log level for all CLI-based commands. This will not configure the API server's
    /// log level when you run `charted server`.
    #[arg(global = true, short = 'l', long = "log-level", default_value_t = Level::INFO)]
    pub level: Level,

    #[command(subcommand)]
    pub command: commands::Cmd,
}

impl Program {
    /// Initializes a global tracing subscriber for all CLI-based commands.
    pub fn init_log(&self) {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::fmt::layer()
                    .with_level(true)
                    .with_target(true)
                    .with_filter(LevelFilter::from_level(self.level)),
            )
            .init();
    }
}
