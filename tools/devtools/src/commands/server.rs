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

use crate::{utils, CommonArgs};
use eyre::{eyre, Result};
use itertools::Itertools;
use std::{collections::HashSet, env::current_dir, ffi::OsString, process::Stdio};

/// Runs the API server, which invokes the `charted server` CLI command.
#[derive(Debug, Clone, clap::Parser)]
pub struct Cmd {
    #[command(flatten)]
    args: CommonArgs,
}

pub fn run(command: Cmd) -> Result<()> {
    let cargo = utils::find_binary(command.args.cargo.as_ref(), "cargo")
        .ok_or_else(|| eyre!("unable to find `cargo` binary"))?;

    utils::cmd(cargo, |cmd| {
        cmd.stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .arg("run")
            .arg("--locked");

        if command.args.release {
            cmd.arg("--release");
        }

        for arg in command.args.cargo_args.iter() {
            cmd.arg(arg);
        }

        let mut rustflags: HashSet<OsString> = HashSet::new();
        rustflags.insert(OsString::from("--cfg tokio_unstable"));

        let rustc_flags = command.args.rustc_flags.clone().unwrap_or_default();
        if !rustc_flags.is_empty() {
            rustflags.insert(rustc_flags);
        }

        cmd.env(
            "RUSTFLAGS",
            rustflags.iter().map(|x| x.to_string_lossy().to_string()).join(" "),
        )
        .env("CHARTED_DISTRIBUTION_KIND", "git")
        .env("RUST_BACKTRACE", "1");

        cmd.args(["--", "server"]);

        let root = current_dir().unwrap();
        for path in [root.join("config.toml"), root.join("config/charted.toml")] {
            if path.try_exists().unwrap() {
                cmd.arg("--config").arg(path);
                break;
            }
        }
    })
    .map(|_| ())
}
