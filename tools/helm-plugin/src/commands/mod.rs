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

mod context;
mod repo;
mod version;

use charted_common::cli::{AsyncExecute, Execute};
use eyre::Result;

#[derive(Debug, Clone, clap::Subcommand)]
pub enum Commands {
    #[command(subcommand)]
    Context(context::Context),
    Version(version::Version),

    #[command(subcommand)]
    Repo(repo::Repo),
}

#[async_trait]
impl AsyncExecute for Commands {
    async fn execute(&self) -> Result<()> {
        match self {
            Commands::Context(context) => context.execute().await,
            Commands::Version(version) => version.execute(),
            Commands::Repo(repo) => repo.execute().await,
        }
    }
}
