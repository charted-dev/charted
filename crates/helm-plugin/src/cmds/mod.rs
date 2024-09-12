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

mod auth;
mod completions;
mod download;
mod init;
mod login;
mod pull;
mod version;

#[derive(Debug, Clone, clap::Subcommand)]
pub enum Cmd {
    Completions(completions::Args),

    #[command(subcommand)]
    Auth(auth::Cmd),
}

impl Cmd {
    pub async fn run(self) -> eyre::Result<()> {
        match self {
            Cmd::Auth(cmd) => cmd.run(),
            Cmd::Completions(args) => completions::run(args),
        }
    }
}
