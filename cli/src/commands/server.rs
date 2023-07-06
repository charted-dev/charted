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

use clap::Parser;
use dotenv::dotenv;
use eyre::Result;
use std::path::PathBuf;

use super::models::AsyncExecute;

#[derive(Debug, Clone, Parser)]
#[command(about = "Runs the API server in the same process invocation")]
pub struct Server {
    #[arg(
        short = 'c',
        long = "config",
        help = "Configuration file to run the server",
        env = "CHARTED_CONFIG_FILE"
    )]
    config_file: Option<PathBuf>,
}

#[async_trait]
impl AsyncExecute for Server {
    async fn execute(&self) -> Result<()> {
        // fast-load environment variables
        dotenv().unwrap_or_default();

        Ok(())
    }
}
