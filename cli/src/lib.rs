// 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
// Copyright 2022 Noelware <team@noelware.org>
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
use log::LevelFilter;

#[macro_use]
extern crate log;

pub mod commands;
pub mod macros;
pub mod server;
pub mod setup_utils;

#[derive(Debug, Parser)]
#[command(name = "charted", about = "Distribute Helm charts safely on the cloud.", author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: Commands,

    #[clap(
        short = 'v',
        long = "verbose",
        help = "Prints more verbose information about what is going on",
        global = true
    )]
    pub verbose: bool,

    #[clap(
        short = 'l',
        long = "level",
        help = "Logging level verbosity.",
        global = true,

        // --verbose/-v will use the "debug" level if used.
        default_value = "info"
    )]
    pub level: Option<LevelFilter>,

    #[clap(
        long = "colors",
        help = "If logs should include colors or not",
        global = true
    )]
    pub colors: Option<bool>,
}
