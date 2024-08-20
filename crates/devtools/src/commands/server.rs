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

use super::CommonArgs;
use std::env;

/// Runs the API server as a single command.
#[derive(clap::Parser)]
pub struct Args {
    #[clap(flatten)]
    common: CommonArgs,
}

pub fn run(Args { common }: Args) -> eyre::Result<()> {
    info!("starting API server...");

    let current_dir = env::current_dir()?;
    crate::cargo(common.cargo.as_ref(), "run", |cmd| {
        cmd.arg("--locked");
        cmd.arg("--package").arg("charted");

        if common.release {
            cmd.arg("--release");
        }

        for arg in common.cargo_args.iter() {
            cmd.arg(arg);
        }

        let mut rustflags = common.rustc_flags.clone().unwrap_or_default();
        if !rustflags.is_empty() {
            rustflags.push(" ");
        }

        rustflags.push("--cfg tokio_unstable");

        cmd.env("RUSTFLAGS", rustflags)
            .env("CHARTED_DISTRIBUTION_KIND", "git")
            .env("RUST_BACKTRACE", "1")
            .args(["--", "server"]);

        for path in [current_dir.join("config.hcl"), current_dir.join("config/charted.hcl")] {
            if path.try_exists().unwrap_or(false) {
                cmd.arg("--config").arg(path);
                break;
            }
        }
    })
    .map(|output| {
        warn!("exited with code {}", output.status.code().unwrap_or(-1));
    })
}
