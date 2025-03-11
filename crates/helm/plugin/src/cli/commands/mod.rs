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

mod auth;
mod completions;
mod context;
mod download;
mod init;
mod login;
mod logout;
mod repository;

#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    Completions(completions::Args),

    #[command(hide(true))]
    Download(download::Args),
}

impl Subcommand {
    pub async fn run(self) -> eyre::Result<()> {
        match self {
            Subcommand::Completions(args) => completions::run(args),
            Subcommand::Download(args) => download::run(args).await,
        }
    }
}
