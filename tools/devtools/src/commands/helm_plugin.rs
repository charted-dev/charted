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

use crate::{utils, CommonArgs};
use charted_common::cli::Execute;
use eyre::{eyre, Result};
use std::{ffi::OsString, process::Stdio};

/// Runs the API server, which invokes the `charted server` CLI command.
#[derive(Debug, Clone, clap::Parser)]
pub struct Cmd {
    #[command(flatten)]
    args: CommonArgs,
}

impl Execute for Cmd {
    fn execute(&self) -> Result<()> {
        let cargo = utils::find_binary(self.args.cargo.clone(), "cargo")
            .ok_or_else(|| eyre!("unable to find `cargo` binary"))?;

        utils::cmd(cargo, |cmd| {
            cmd.stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .arg(match (self.args.release, self.args.run) {
                    (true, true) | (false, true) => "run",
                    (false, false) | (true, false) => "build",
                })
                .arg("--locked");

            if self.args.release {
                cmd.arg("--release");
            }

            let mut rustflags = OsString::from("--cfg tokio_unstable");
            let rustc_flags = self.args.rustc_flags.clone().unwrap_or_default();
            if !rustc_flags.is_empty() {
                rustflags.push(" ");
                rustflags.push(rustc_flags);
            }

            cmd.env("RUSTFLAGS", rustflags);
            cmd.args(["--bin", "charted-helm-plugin"]);
            cmd.args(&self.args.cargo_args);

            if self.args.run && !self.args.args.is_empty() {
                cmd.arg("--").args(&self.args.args);
            }
        })
        .map(|_| ())
    }
}
