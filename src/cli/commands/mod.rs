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
mod generate;
mod migrations;
mod openapi;
mod server;
mod version;

use clap::Subcommand;

#[derive(Debug, Clone, Subcommand)]
pub enum Cmd {
    #[command(subcommand)]
    Migrations(migrations::Cmd),
    Completions(completions::Args),
    Generate(generate::Args),

    #[command(name = "openapi")]
    OpenApi(openapi::Args),
    Version(version::Args),
    Server(server::Args),
}

pub async fn execute(cmd: Cmd) -> eyre::Result<()> {
    match cmd {
        Cmd::Server(args) => server::run(args).await,
        Cmd::Completions(args) => {
            completions::run(args);
            Ok(())
        }
        Cmd::Generate(args) => generate::run(args),
        Cmd::OpenApi(args) => openapi::run(args),
        Cmd::Version(args) => {
            version::run(args);
            Ok(())
        }

        Cmd::Migrations(cmd) => migrations::execute(cmd).await,
    }
}
