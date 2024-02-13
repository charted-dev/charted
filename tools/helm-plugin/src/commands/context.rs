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

use charted::cli::Execute;
use clap::Subcommand;

mod delete;
mod list;
mod switch;
mod token;

/// Provides all the commands for information about authentication for registry pushes
/// and downloads.
#[derive(Debug, Clone, Subcommand)]
pub enum Cmd {
    Delete(delete::Cmd),
    Switch(switch::Cmd),
    Token(token::Cmd),
    List(list::Cmd),
}

impl Execute for Cmd {
    fn execute(&self) -> eyre::Result<()> {
        match self {
            Cmd::Switch(switch) => switch.execute(),
            Cmd::Delete(del) => del.execute(),
            Cmd::Token(token) => token.execute(),
            Cmd::List(list) => list.execute(),
        }
    }
}
