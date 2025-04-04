// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2025 Noelware, LLC. <team@noelware.org>
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

mod admin;
mod completions;
pub mod migrate;
pub mod server;
pub mod worker;

#[derive(Debug, clap::Args)]
pub struct Tokio {
    /// Number of Tokio workers to use.
    ///
    /// By default, this will use the number of avaliable CPU cores on the system
    /// itself.
    #[arg(long, short = 'w', env = "TOKIO_WORKER_THREADS", default_value_t = num_cpus::get())]
    pub workers: usize,
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    Completions(completions::Args),
    Server(server::Args),

    #[command(subcommand)]
    Worker(worker::Subcmd),

    #[command(subcommand)]
    Admin(admin::Subcommand),

    #[command(subcommand)]
    Migrate(migrate::Subcommand),
}

pub async fn execute(subcmd: Subcommand) -> eyre::Result<()> {
    match subcmd {
        Subcommand::Server(args) => server::run(args).await,
        Subcommand::Worker(subcmd) => subcmd.run().await,
        Subcommand::Migrate(subcmd) => migrate::run(subcmd).await,
        Subcommand::Admin(subcmd) => admin::run(subcmd).await,
        Subcommand::Completions(args) => completions::run(args),
    }
}
