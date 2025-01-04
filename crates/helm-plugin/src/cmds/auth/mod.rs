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

mod delete;
mod list;
mod switch;
mod token;

/// Subcommand to perform auth-related actions.
#[derive(Debug, Clone, clap::Subcommand)]
pub enum Cmd {
    /// Switch to a different context when pulling or pushing Helm charts onto a registry.
    Switch(switch::Args),

    /// Lists all the different authentications avaliable
    List(list::Args),
}

impl Cmd {
    pub fn run(self) -> eyre::Result<()> {
        match self {
            Cmd::List(args) => list::run(args),
            Cmd::Switch(args) => switch::run(args),
        }
    }
}
