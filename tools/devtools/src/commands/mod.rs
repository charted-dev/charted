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

mod ci;
mod cli;
mod docker;
mod helm_plugin;
mod server;

/// List of all available subcommands for `./dev`
#[derive(Debug, Clone, clap::Subcommand)]
pub enum Cmd {
    #[command(subcommand)]
    Docker(docker::Cmd),

    #[command(subcommand)]
    Ci(ci::Cmd),

    HelmPlugin(helm_plugin::Cmd),
    Server(server::Cmd),
    Cli(cli::Cmd),
}

pub async fn run(cmd: Cmd) -> eyre::Result<()> {
    match cmd {
        Cmd::Server(cmd) => server::run(cmd),
        Cmd::HelmPlugin(cmd) => helm_plugin::run(cmd),
        Cmd::Cli(cmd) => cli::run(cmd),
        Cmd::Docker(cmd) => docker::run(cmd).await,
        Cmd::Ci(cmd) => ci::run(cmd),
    }
}
