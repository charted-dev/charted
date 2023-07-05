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

#[macro_use]
extern crate async_trait;

use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[clap(
    about = "🐻‍❄️📦 Free, open source, and reliable Helm Chart registry made in Rust",
    author = "Noelware, LLC.",
    override_usage = "charted <COMMAND> [...ARGS]",
    arg_required_else_help = true
)]
pub struct Cli {
    /// Whether if the CLI should print the current version of the CLI.
    #[arg(short = 'v', long = "version", help = "Prints out the current version of the charted CLI")]
    pub print_version: bool,
}
