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
use eyre::eyre;
use std::{
    env::current_dir,
    path::PathBuf,
    process::{exit, Stdio},
};

/// Views all the logs of every container that was spawned from `./dev docker up`
#[derive(Debug, Clone, clap::Parser)]
pub struct Cmd {
    /// Location to a `docker` binary that exists. This must be an absolute path as all
    /// paths are relative to where the `charted-devtools` binary was executed in.
    #[arg(long, env = "DOCKER")]
    docker: Option<PathBuf>,
}

impl Execute for Cmd {
    fn execute(&self) -> eyre::Result<()> {
        let dir = current_dir()?;
        let docker =
            utils::find_binary(self.docker.clone(), "docker").ok_or_else(|| eyre!("unable to find `docker` binary"))?;

        let compose_project = dir.join(".cache/docker-compose.yml");
        if !compose_project.try_exists()? {
            error!(
                project = %compose_project.display(),
                "unable to locate docker compose project! did you run `./dev docker up`?"
            );

            exit(1);
        }

        utils::cmd(docker, |cmd| {
            cmd.stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .args(["compose", "-f"])
                .arg(&compose_project)
                .arg("logs");
        })
        .map(|_| ())
    }
}
