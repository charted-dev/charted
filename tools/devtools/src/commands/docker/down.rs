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

use crate::utils;
use eyre::eyre;
use std::{
    env::current_dir,
    path::PathBuf,
    process::{exit, Stdio},
};

/// Destroys the containers that was created by `./dev docker up`.
#[derive(Debug, Clone, clap::Parser)]
pub struct Cmd {
    /// Removes containers for services not defined in the Compose project file.
    #[arg(long = "remove-orphans")]
    remove_orphans: bool,

    /// Location to a `docker` binary that exists. This must be an absolute path as all
    /// paths are relative to where the `charted-devtools` binary was executed in.
    #[arg(long, env = "DOCKER")]
    docker: Option<PathBuf>,

    /// Removes all of the volume mounts as well.
    #[arg(long, short = 'v')]
    volumes: bool,
}

pub fn run(command: Cmd) -> eyre::Result<()> {
    let dir = current_dir()?;
    let docker =
        utils::find_binary(command.docker.as_ref(), "docker").ok_or_else(|| eyre!("unable to find `docker` binary"))?;

    let compose_project = dir.join(".cache/docker-compose.yml");
    if !compose_project.try_exists()? {
        error!(
            project = %compose_project.display(),
            "unable to locate docker compose project! did you run `./dev docker up`?"
        );

        exit(1);
    }

    let root = dir.join(".cache");
    utils::cmd(docker, |cmd| {
        cmd.stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .args(["compose", "-f"])
            .arg(&compose_project)
            .arg("down")
            .current_dir(&root);

        if command.remove_orphans {
            cmd.arg("--remove-orphans");
        }

        if command.volumes {
            cmd.arg("-v");
        }
    })
    .map(|_| ())
}
