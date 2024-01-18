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
mod openapi;
mod server;
mod version;

use super::{AsyncExecute, Execute};
use clap::Subcommand;

#[derive(Debug, Clone, Subcommand)]
pub enum Cmd {
    Completions(completions::Cmd),
    Generate(generate::Cmd),
    Version(version::Cmd),
    Server(server::Cmd),
}

#[async_trait]
impl AsyncExecute for Cmd {
    async fn execute(&self) -> eyre::Result<()> {
        match self {
            Cmd::Server(server) => server.execute().await,
            Cmd::Completions(comp) => comp.execute(),
            Cmd::Generate(gen) => gen.execute(),
            Cmd::Version(ver) => ver.execute(),
        }
    }
}
