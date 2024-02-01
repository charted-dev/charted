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

use crate::cli::AsyncExecute;
use std::path::PathBuf;

/// Lists all migrations available
#[derive(Debug, Clone, clap::Parser)]
pub struct Cmd {
    /// location to a relative/absolute path to a configuration file. by default, this will locate
    /// in `./config/charted.yml`/`./config.yml` if found.
    #[arg(long, short = 'c', env = "CHARTED_CONFIG_PATH")]
    config: Option<PathBuf>,
}

#[async_trait]
impl AsyncExecute for Cmd {
    async fn execute(&self) -> eyre::Result<()> {
        Ok(())
    }
}
