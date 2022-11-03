// 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
// Copyright 2022 Noelware <team@noelware.org>
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

pub mod completion;
pub mod generate;
pub mod ping;
pub mod server;

use anyhow::Result;
use async_trait::async_trait;
use clap::Subcommand;

use self::{completion::Completion, generate::Generate, server::Server};

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap(name = "completions")]
    Completion(Completion),

    #[clap(name = "server")]
    Server(Server),

    #[clap(name = "generate")]
    Generate(Generate),
}

/// Trait to execute commands on the fly with the [`handle`] function in this module
pub trait Execute {
    /// Method to execute the command itself. Must return [`Result<()>`][anyhow::Result].
    fn execute(self) -> Result<()>;
}

#[async_trait]
pub trait AsyncExecute {
    async fn execute(self) -> Result<()>;
}

pub async fn execute(command: Commands) -> Result<()> {
    match command {
        Commands::Completion(completion) => completion.execute(),
        Commands::Generate(generate) => generate.execute(),
        Commands::Server(server) => server.execute().await,
    }
}
