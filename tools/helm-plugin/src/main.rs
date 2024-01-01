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

use charted_common::{cli::AsyncExecute, is_debug_enabled, os};
use charted_helm_plugin::Cli;
use charted_logging::generic::GenericLayer;
use clap::Parser;
use eyre::{eyre, Result};
use std::env::{set_var, var};
use tracing::{metadata::LevelFilter, Level};
use tracing_subscriber::{prelude::*, util::SubscriberInitExt};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    match (os::os_name(), os::architecture()) {
        ("unknown", _) => {
            return Err(eyre!("`charted helm` is not supported on any other operating systems. Only Windows, macOS, and Linux is supported."));
        }

        (_, "unknown") => {
            return Err(eyre!(
                "`charted helm` is not supported on any other CPU architectures. Only x86_64 and ARM64 is supported."
            ));
        }

        _ => {}
    }

    if is_debug_enabled() && var("RUST_BACKTRACE").is_err() {
        set_var("RUST_BACKTRACE", "full");
    }

    let cli = Cli::parse();
    tracing_subscriber::registry()
        .with(
            GenericLayer { verbose: cli.verbose }.with_filter(LevelFilter::from_level(match cli.verbose {
                false => Level::INFO,
                true => Level::DEBUG,
            })),
        )
        .init();

    cli.command.execute().await?;
    Ok(())
}
