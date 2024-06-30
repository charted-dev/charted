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

mod completions;
mod gc;
mod migrations;
mod server;
mod version;

use clap::Subcommand;

#[derive(Debug, Clone, Subcommand)]
pub enum Cmd {
    #[command(subcommand)]
    Migrations(migrations::Cmd),

    Completions(completions::Args),
    Version(version::Args),
    Server(server::Args),
}

pub async fn exec(cmd: Cmd) -> eyre::Result<()> {
    match cmd {
        Cmd::Server(args) => server::run(args).await,
        Cmd::Migrations(subcmd) => migrations::execute(subcmd).await,
        Cmd::Version(args) => version::exec(args),
        Cmd::Completions(args) => {
            completions::run(args);
            Ok(())
        }
    }
}
