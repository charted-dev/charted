// 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
// Copyright 2022 Noelware <team@noelware.org>
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

use std::path::PathBuf;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use clap::Parser;

use crate::server::ServerProcess;

use super::AsyncExecute;

#[derive(Debug, Parser)]
#[clap(about = "Bootstraps and runs the server! Requires a Java installation.")]
pub struct Server {
    #[clap(short = 'c', long = "config", help = "Configuration file path to use")]
    config: Option<PathBuf>,
}

#[async_trait]
impl AsyncExecute for Server {
    async fn execute(self) -> Result<()> {
        let proc = ServerProcess::create()?;
        match proc.wait().await {
            Ok(e) => {
                let code = e.code().unwrap_or(128);

                info!("Server process has exited with code {}", code);
                Ok(())
            }

            Err(err) => Err(anyhow!("Process has exited with an error [{}]", err)),
        }
    }
}
