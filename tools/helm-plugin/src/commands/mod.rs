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

use charted_common::lazy;
use clap::Subcommand;
use once_cell::sync::Lazy;
use reqwest::Client;

mod completions;
mod context;
mod download;
mod init;
mod lint;
mod login;
mod logout;
mod push;

pub(crate) static HTTP: Lazy<Client> = lazy!(Client::builder()
    .user_agent(format!(
        "Noelware/charted-helm-plugin (+{}; https://github.com/charted-dev/charted/tree/main/tools/helm-plugin)",
        crate::version()
    ))
    .build()
    .unwrap());

#[derive(Debug, Clone, Subcommand)]
pub enum Cmd {
    Completions(completions::Args),
    Download(download::Cmd),

    #[command(subcommand)]
    Context(context::Cmd),
    Logout(logout::Cmd),
    Login(login::Cmd),
    Push(push::Cmd),
    Init(init::Cmd),
    Lint(lint::Cmd),
}

pub async fn execute(cmd: Cmd) -> eyre::Result<()> {
    match cmd {
        Cmd::Completions(args) => {
            completions::run(args);
            Ok(())
        }

        Cmd::Download(cmd) => download::run(cmd).await,
        Cmd::Context(cmd) => context::run(cmd),
        Cmd::Logout(cmd) => logout::run(cmd),
        Cmd::Login(cmd) => login::run(cmd).await,
        Cmd::Init(cmd) => init::run(cmd),
        Cmd::Push(cmd) => push::run(cmd).await,
        Cmd::Lint(cmd) => lint::run(cmd).await,
    }
}
