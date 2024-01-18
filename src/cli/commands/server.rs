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

use crate::{cli::AsyncExecute, config::Config};
use std::path::PathBuf;

/// Runs the API server.
#[derive(Debug, Clone, clap::Parser)]
pub struct Cmd {
    /// location to a relative/absolute path to a configuration file. by default, this will locate
    /// in `./config/charted.yml`/`./config.yml` if found.
    #[arg(short = 'c', long, env = "CHARTED_CONFIG_FILE")]
    config: Option<PathBuf>,

    /// whether or not to print the configuration and exit
    #[arg(long)]
    print: bool,

    /// amount of workers to spawn for the Tokio runtime. This cannot exceeded
    /// the amount of CPU cores you have.
    #[arg(short = 'w', long, env = "CHARTED_RUNTIME_WORKERS", default_value_t = num_cpus::get())]
    pub workers: usize,
}

#[async_trait]
impl AsyncExecute for Cmd {
    async fn execute(&self) -> eyre::Result<()> {
        let config = match self.config {
            Some(ref path) => Config::new(Some(path)),
            None => match Config::find_default_conf_location() {
                Some(path) => Config::new(Some(path)),
                None => Config::new::<&str>(None),
            },
        }?;

        if self.print {
            eprintln!("{}", serde_yaml::to_string(&config).unwrap());
            return Ok(());
        }

        Ok(())
    }
}
