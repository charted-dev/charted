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

use crate::utils;
use charted_common::cli::Execute;
use eyre::Result;
use std::{path::PathBuf, process::exit};

#[derive(Debug, Clone, clap::Parser)]
#[clap(about = "Teardowns the Docker compose project that is used for development purposes")]
pub struct Down {
    /// Location to a `bazel` binary that is used to locate the workspace
    #[arg(long)]
    bazel: Option<PathBuf>,

    /// Location to a `docker` binary that exists on the filesystem.
    #[arg(long)]
    docker: Option<PathBuf>,
}

impl Execute for Down {
    fn execute(&self) -> Result<()> {
        let bazel = utils::find_bazel(self.bazel.clone())?;
        let workspace: PathBuf = utils::info(bazel.clone(), &["workspace"])?.trim().into();
        let docker_compose_file = workspace.join(".cache/docker-compose.yml");
        if !docker_compose_file.exists() {
            error!(
                "Unable to locate Docker Compose project in [{}]. Are you sure that you ran `./dev docker up` before running this command?",
                docker_compose_file.display()
            );

            exit(1);
        }

        let docker = utils::docker::find(self.docker.clone())?;
        utils::docker::exec(
            docker.clone(),
            workspace,
            &[
                "compose",
                "-f",
                docker_compose_file.as_os_str().to_str().unwrap(),
                "down",
            ],
        )?;

        info!("containers has been destroyed!");
        Ok(())
    }
}
