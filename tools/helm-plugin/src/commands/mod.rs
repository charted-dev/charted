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

use charted::cli::{AsyncExecute, Execute};
use clap::Subcommand;

mod completions;
mod context;
mod download;
mod init;
mod lint;
mod login;
mod logout;
mod push;

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

#[async_trait]
impl AsyncExecute for Cmd {
    async fn execute(&self) -> eyre::Result<()> {
        match self {
            Cmd::Completions(args) => {
                completions::run(args.clone());
                Ok(())
            }

            Cmd::Download(dl) => dl.execute().await,
            Cmd::Context(ctx) => ctx.execute(),
            Cmd::Logout(logout) => logout.execute(),
            Cmd::Login(login) => login.execute(),
            Cmd::Init(init) => init.execute(),
            Cmd::Push(push) => push.execute().await,
            Cmd::Lint(cmd) => lint::execute(cmd).await,
        }
    }
}
