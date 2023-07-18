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

mod generate_config;
mod models;
mod openapi;
mod server;
mod version;

use crate::commands::generate_config::*;
use crate::commands::models::*;
use crate::commands::openapi::*;
use crate::commands::server::*;
use crate::commands::version::*;
use clap::Subcommand;
use eyre::Result;

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    GenerateConfig(GenerateConfig),

    #[command(name = "openapi")]
    OpenAPI(OpenAPI),
    Server(Box<Server>),
    Version(Version),
}

pub async fn execute(command: &Commands) -> Result<()> {
    match command {
        Commands::Server(server) => server.execute().await,
        Commands::Version(version) => version.execute(),
        Commands::OpenAPI(openapi) => openapi.execute(),
        Commands::GenerateConfig(generate) => generate.execute().await,
    }
}
