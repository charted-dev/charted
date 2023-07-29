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
use eyre::{Context, Result};
use std::{
    path::PathBuf,
    process::{exit, Command, Stdio},
};
use which::which;

#[derive(Debug, Clone, clap::Parser)]
#[clap(about = "Spawns the development server for the web UI")]
pub struct Web {
    /// Location to a `bazel` binary.
    #[arg(long)]
    bazel: Option<PathBuf>,

    /// List of external .bazelrc files to include. If `/dev/null` is passed, then
    /// it will stop looking for entries after the index of `/dev/null`.
    #[arg(long)]
    bazelrc: Vec<PathBuf>,
}

#[async_trait]
impl AsyncExecute for Web {
    async fn execute(&self) -> Result<()> {
        let bazel = match self.bazel.clone() {
            Some(path) => path,
            None => which("bazel").context("unable to locate 'bazel' binary")?,
        };

        debug!("using bazel binary [{}]", bazel.display());

        // validate bazel, even if ./dev requires the devtool to be
        // cached in $DIR/.cache/dev, the command can accept another
        // bazel installation (was it a good idea? probably not)
        {
            let bazel = bazel.clone();
            let output = Command::new(bazel.clone())
                .arg("--version")
                .output()
                .context(format!("unable to run command [{} --version]", bazel.display()))?;

            if !output.status.success() {
                error!("unable to validate bazel binary");
                error!("why: {} --version failed to execute", bazel.display());
                error!("---- stdout ----");

                let stdout = String::from_utf8(output.stdout).context("unable to convert stdout to utf-8")?;
                error!("{stdout}");
                error!("---- stderr ----");

                let stderr = String::from_utf8(output.stderr).context("unable to convert stderr to utf-8")?;
                error!("{stderr}");
                exit(1);
            }

            let stdout = String::from_utf8(output.stdout).context("unable to convert stdout to utf-8")?;
            let split = stdout.split(' ').collect::<Vec<_>>();
            let Some(product) = split.first() else {
                error!("unable to validate bazel binary");
                error!("why: {} --version stdout was empty", bazel.display());

                exit(1);
            };

            match *product {
                "bazel" | "blaze" => {}
                _ => {
                    error!("unable to validate bazel binary");
                    error!(
                        "why: {} --version uses an invalid product (expected: 'bazel', 'blaze'; received: {product})",
                        bazel.display()
                    );

                    exit(1);
                }
            }

            let Some(version) = split.get(1) else {
                error!("unable to validate bazel binary");
                error!(
                    "why: {} --version stdout was only '{product}', missing version number",
                    bazel.display()
                );

                exit(1);
            };

            info!("using product {product}, version {}", version.trim());
        }

        info!("$ {} run //web:vite -- --clearScreen=false", bazel.display());
        let mut cmd = Command::new(bazel.clone());
        cmd.args(self.bazelrc.as_slice());
        cmd.args(["run", "//web:vite", "--", "--clearScreen=false"]);
        cmd.stdin(Stdio::null());

        let mut child = cmd.spawn()?;
        child.wait()?;

        Ok(())
    }
}
