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

use async_trait::async_trait;
use charted::cli::{AsyncExecute, Execute};

mod down;
mod logs;
mod up;

/// Utilities for starting and tearing down the development Docker compose project
/// that is used to run the server.
#[derive(Debug, Clone, clap::Subcommand)]
pub enum Cmd {
    Down(down::Cmd),
    Logs(logs::Cmd),
    Up(up::Cmd),
}

#[async_trait]
impl AsyncExecute for Cmd {
    async fn execute(&self) -> eyre::Result<()> {
        match self {
            Self::Up(up) => up.execute().await,
            Self::Down(down) => down.execute(),
            Self::Logs(logs) => logs.execute(),
        }
    }
}
