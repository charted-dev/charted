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

use std::io::{self, IsTerminal};

use clap::Parser;
use tracing::Level;

#[macro_use]
extern crate tracing;

pub mod args;
pub mod auth;
pub mod commands;
pub mod config;

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
    #[arg(global, short = 'l', long = "log-level", env = "CHARTED_HELM_LOG_LEVEL", default_level_t = Level::INFO)]
    pub level: Level,

    /// Disables the use of the progress bars for `helm charted download` and `helm charted push`. This is also disabled if there
    /// is no TTY attached.
    #[arg(global, long = "no-progress", env = "CHARTED_HELM_NO_PROGRESS", default_level_t = __check_if_enabled())]
    pub no_progress: bool,
}

fn __check_if_enabled() -> bool {
    let stdout = io::stdout();
    if !stdout.is_terminal() {
        return true;
    }

    false
}

/// Returns the current version of `charted-helm-plugin`.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Returns the version and commit hash of `charted-helm-plugin`.
#[inline]
pub fn version() -> String {
    format!("v{}+{}", VERSION, charted::COMMIT_HASH)
}
