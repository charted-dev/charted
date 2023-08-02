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

use crate::utils::{self, BuildCliArgs};
use charted_common::cli::Execute;
use eyre::Result;
use std::{ffi::OsString, path::PathBuf};

#[derive(Debug, Clone, clap::Parser)]
#[clap(about = "Builds and runs the API server")]
pub struct Server {
    /// Whether if the configuration should be built or not.
    #[arg(long)]
    print_config: bool,

    /// Path to use to locate a configuration file.
    #[arg(short = 'c', long, env = "CHARTED_CONFIG_FILE")]
    config: Option<PathBuf>,

    /// Whether if the release binary should be built instead of the development binary.
    #[arg(long)]
    release: bool,

    /// List of `.bazelrc` files to include. If `/dev/null` is passed, Bazel will
    /// skip looking for other .bazelrc files.
    #[arg(long)]
    bazelrc: Vec<PathBuf>,

    /// Location to a `bazel` binary
    #[arg(long)]
    bazel: Option<PathBuf>,
}

impl Execute for Server {
    fn execute(&self) -> Result<()> {
        let bazel = utils::find_bazel(self.bazel.clone())?;
        let mut args = vec![OsString::from("server")];
        match self.config.clone() {
            Some(path) => {
                args.push("--config".into());
                args.push(path.into());
            }

            None => {
                let workspace = PathBuf::from(utils::info(bazel.clone(), &["workspace"])?.trim());
                let default_location_1 = workspace.join("config.yml");

                if default_location_1.exists() {
                    info!("using provided config in {}", default_location_1.display());

                    args.push("--config".into());
                    args.push(default_location_1.into());
                }

                let default_location_2 = workspace.join("config").join("charted.yaml");
                if default_location_2.exists() {
                    info!("using provided config in {}", default_location_2.display());

                    args.push("--config".into());
                    args.push(default_location_2.into());
                }
            }
        }

        if self.print_config {
            args.push(OsString::from("--print-config"));
        }

        let args = BuildCliArgs {
            release: self.release,
            bazelrc: self.bazelrc.clone(),
            args,
            run: true,
        };

        utils::build_or_run_cli(bazel.clone(), args)
    }
}
