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

use clap::Parser;
use commands::Commands;

#[macro_use]
extern crate async_trait;

#[macro_use]
extern crate tracing;

mod args;
pub mod auth;
pub mod commands;
pub mod config;
pub mod util;

pub use args::*;

#[derive(Debug, Clone, Parser)]
#[clap(
    bin_name = "helm charted",
    about = "🐻‍❄️📦 Helm plugin to help you push your Helm charts into charted-server easily",
    author = "Noelware, LLC.",
    override_usage = "helm charted <COMMAND> [...ARGS]",
    arg_required_else_help = true
)]
pub struct Cli {
    /// Whether if more verbose logging should be printed or not.
    #[arg(long, global = true, short = 'v')]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}
