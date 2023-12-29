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
use std::{env::current_dir, path::PathBuf, process::Stdio};

/// Runs or builds the web distribution. This will require [Bun](https://bun.sh) to be installed.
#[derive(Debug, Clone, clap::Parser)]
pub struct Cmd {
    /// whether or not if we should just build the web distribution's production build.
    #[arg(long)]
    build: bool,

    /// Location as an absolute path to a [`bun`](https://bun.sh) binary.
    #[arg(long, env = "BUN")]
    bun: Option<PathBuf>,

    /// whether or not if we should run in development mode (invokes `bun run dev` in `web/`),
    /// or in production mode (invokes `bun run preview`). If `--build` is specified, then it will
    /// only build and not run the web distribution in Vite itself.
    #[arg(long)]
    run: bool,
}

impl Execute for Cmd {
    fn execute(&self) -> eyre::Result<()> {
        let bun =
            utils::find_binary(self.bun.clone(), "bun").ok_or_else(|| eyre!("unable to find the `bun` binary"))?;

        utils::cmd(bun, |cmd| {
            cmd.stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .current_dir(current_dir().unwrap().join("web"));

            match (self.build, self.run) {
                // only build the web distribution
                (true, true) => {
                    cmd.args(["run", "preview"]);
                }

                (false, true) => {
                    cmd.args(["run", "dev"]);
                }

                (true, false) => {
                    cmd.args(["run", "build"]);
                }

                (_, _) => {
                    cmd.args(["run", "dev"]);
                }
            }

            cmd.args(["--", "--clearScreen=false"]);
        })
        .map(|_| ())
    }
}
