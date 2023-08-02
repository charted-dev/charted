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

mod cli;
mod docker;
mod generate;
mod helm_plugin;
mod server;
mod services;
mod web;

use charted_common::cli::*;
use clap::Subcommand;
use cli::*;
use docker::*;
use eyre::Result;
use generate::*;
use helm_plugin::*;
use server::*;
use web::*;

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    HelmPlugin(HelmPlugin),

    #[command(subcommand)]
    Generate(Generate),
    Server(Server),

    #[command(subcommand)]
    Docker(Docker),
    Cli(Cli),
    Web(Web),
}

impl Commands {
    pub fn execute(self) -> Result<()> {
        match self {
            Commands::HelmPlugin(plugin) => plugin.execute(),
            Commands::Generate(generate) => generate.execute(),
            Commands::Server(server) => server.execute(),
            Commands::Docker(docker) => docker.execute(),
            Commands::Cli(cli) => cli.execute(),
            Commands::Web(web) => web.execute(),
        }
    }
}
