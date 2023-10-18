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

mod completions;
mod gc;
mod generate_config;
mod migrations;
mod openapi;
mod organizations;
mod repositories;
mod search;
mod server;
mod users;
mod version;

use charted_common::cli::*;
use clap::Subcommand;
use eyre::Result;

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    GenerateConfig(generate_config::GenerateConfig),
    Completions(completions::Completions),

    #[command(name = "openapi")]
    OpenAPI(openapi::OpenAPI),
    Version(version::Version),
    Server(server::Server),

    #[command(subcommand)]
    Users(users::Users),
}

#[async_trait]
impl AsyncExecute for Commands {
    async fn execute(&self) -> Result<()> {
        match self {
            Commands::Server(server) => server.execute().await,
            Commands::Users(users) => users.execute().await,
            Commands::Completions(completions) => completions.execute(),
            Commands::Version(version) => version.execute(),
            Commands::OpenAPI(openapi) => openapi.execute(),
            Commands::GenerateConfig(generate) => generate.execute().await,
        }
    }
}
