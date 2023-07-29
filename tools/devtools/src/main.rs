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

use charted_common::is_debug_enabled;
use charted_devtools::Cli;
use charted_logging::generic::GenericLayer;
use clap::Parser;
use eyre::Result;
use std::env::{set_var, var};
use tracing_subscriber::{prelude::*, registry};

#[tokio::main]
async fn main() -> Result<()> {
    if is_debug_enabled() && var("RUST_BACKTRACE").is_err() {
        set_var("RUST_BACKTRACE", "full");
    }

    color_eyre::install()?;

    let cli = Cli::parse();
    tracing::subscriber::set_global_default(registry().with(GenericLayer { verbose: cli.verbose }))?;

    cli.command.execute().await?;
    Ok(())
}
