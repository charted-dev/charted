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

pub mod apikey;
pub mod storage;

#[derive(Debug, clap::Subcommand)]
pub enum Subcmd {
    /// Starts the **API Key Expiration** background worker
    #[command(name = "apikey")]
    ApiKey(apikey::Args),

    /// Starts the **Data Storage Pruner** background worker
    #[command(name = "storage")]
    Storage(storage::Args),
}

impl Subcmd {
    pub async fn run(self) -> eyre::Result<()> {
        match self {
            Self::ApiKey(_) => Ok(()),
            Self::Storage(_) => Ok(()),
        }
    }
}
