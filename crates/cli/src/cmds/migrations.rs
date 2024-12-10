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

mod chart;
mod list;
mod revert;
mod run;

/// Allows doing database migrations or Helm chart index migrations.
#[derive(Debug, Clone, clap::Subcommand)]
pub enum Cmd {
    Chart(chart::Args),
    List(list::Args),
    Revert(revert::Args),
    Run(run::Args),
}

impl Cmd {
    pub async fn execute(self) -> eyre::Result<()> {
        match self {
            Cmd::Run(args) => run::run(args),
            Cmd::List(args) => list::run(args),
            Cmd::Chart(args) => chart::run(args).await,
            Cmd::Revert(args) => revert::run(args),
        }
    }
}
