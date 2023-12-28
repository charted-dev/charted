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

use async_trait::async_trait;
use charted_common::cli::{AsyncExecute, Execute};

mod cli;
mod docker;
mod generate;
mod helm_plugin;
mod server;
mod web;

/// List of all available subcommands for `./dev`
#[derive(Debug, Clone, clap::Subcommand)]
pub enum Command {
    #[command(subcommand)]
    Generate(generate::Cmd),

    #[command(subcommand)]
    Docker(docker::Cmd),

    HelmPlugin(helm_plugin::Cmd),
    Server(server::Cmd),
    Cli(cli::Cmd),
    Web(web::Cmd),
}

#[async_trait]
impl AsyncExecute for Command {
    async fn execute(&self) -> eyre::Result<()> {
        match self {
            Self::Generate(generate) => generate.execute(),
            Self::HelmPlugin(helm) => helm.execute(),
            Self::Docker(docker) => docker.execute().await,
            Self::Server(server) => server.execute(),
            Self::Web(web) => web.execute(),
            Self::Cli(cli) => cli.execute(),
        }
    }
}
