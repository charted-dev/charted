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
use charted_config::Config;
use eyre::Result;
use std::{io::Write, path::PathBuf, process::exit};
use tokio::{
    fs::{File, OpenOptions},
    io::AsyncWriteExt,
};

/// Writes a new configuration file in the specified `path`. Bails out
/// if the path already exists.
#[derive(Debug, Clone, clap::Parser)]
pub struct GenerateConfig {
    /// Path location to write a configuration file.
    path: PathBuf,
}

#[async_trait]
impl AsyncExecute for GenerateConfig {
    async fn execute(&self) -> Result<()> {
        match self.path.try_exists()? {
            false => {}
            true => {
                error!("FATAL: path {} already exists", self.path.display());
                exit(1);
            }
        }

        info!("writing configuration in [{}]", self.path.display());
        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .append(false)
            .open(self.path.clone())
            .await?;

        let config = Config::default();
        let serialized = serde_yaml::to_string(&config)?;
        file.write_all(serialized.as_ref()).await?;

        info!("successfully configuration in {:?}", self.path);
        Ok(())
    }
}
