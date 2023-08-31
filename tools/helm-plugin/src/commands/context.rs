// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

use charted_common::cli::AsyncExecute;
use eyre::Result;

mod delete;
mod list;
mod switch;

/// Subcommands for `charted helm context`.
#[derive(Debug, Clone, clap::Subcommand)]
pub enum Context {
    /// Lists all the contexts available.
    List(list::List),

    /// Easily switch to a different context.
    Switch(switch::Switch),

    /// Deletes a context.
    Delete(delete::Delete),
}

#[async_trait]
impl AsyncExecute for Context {
    async fn execute(&self) -> Result<()> {
        match self {
            Context::List(list) => list.execute().await,
            Context::Delete(del) => del.execute().await,
            Context::Switch(sw) => sw.execute().await,
        }
    }
}
