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

#![allow(unused_imports)]

#[macro_use]
extern crate async_trait;

#[macro_use]
extern crate tracing;

pub mod commands;

use crate::commands::Commands;
use clap::Parser;

pub use commands::execute;

#[derive(Debug, Clone, Parser)]
#[clap(
    bin_name = "charted",
    about = "🐻‍❄️📦 Free, open source, and reliable Helm Chart registry made in Rust",
    author = "Noelware, LLC.",
    override_usage = "charted <COMMAND> [...ARGS]",
    arg_required_else_help = true
)]
pub struct Cli {
    /// Whether if verbose-mode should be enabled on the CLI. This will
    /// not do anything if the 'server' command is invoked.
    #[arg(global = true, short = 'v', long = "verbose")]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}
