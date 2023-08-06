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

use charted_cli::{commands::Commands, execute, Cli};
use charted_common::{is_debug_enabled, os, COMMIT_HASH, RUSTC_VERSION, VERSION};
use charted_config::var;
use charted_logging::generic::GenericLayer;
use clap::Parser;
use color_eyre::config::HookBuilder;
use eyre::{eyre, Result};
use num_cpus::get as cpus;
use std::env::{set_var, var};
use tokio::runtime::Builder;
use tracing::{metadata::LevelFilter, Level};
use tracing_subscriber::prelude::*;

// When `charted server` is invoked, you can specify a `CHARTED_RUNTIME_WORKERS` environment
// variable for Tokio to use a multi-threaded scheduler.
//
// Otherwise, this will be use Tokio's single thread schdeduler.
fn main() -> Result<()> {
    if os::os_name() == "unknown" {
        return Err(eyre!("charted-server is not supported on any other operating systems. Windows, macOS, or Linux is only supported."));
    }

    if os::architecture() == "unknown" {
        return Err(eyre!(
            "charted-server is not supported on any other architectures. x86_64 and ARM64 is only supported."
        ));
    }

    if is_debug_enabled() && var("RUST_BACKTRACE").is_err() {
        set_var("RUST_BACKTRACE", "full");
    }

    let cli = Cli::parse();

    if matches!(cli.command, Commands::Server(_)) && var!("TOKIO_WORKER_THREADS").is_ok() {
        eprintln!("WARN: Using `TOKIO_WORKER_THREADS` won't do anything. Use `CHARTED_RUNTIME_WORKERS` instead.");
        std::env::remove_var("TOKIO_WORKER_THREADS");
    }

    let runtime = match cli.command {
        Commands::Server(ref server) => {
            let workers = var!("CHARTED_RUNTIME_WORKERS", to: usize, or_else: server.workers.unwrap_or(cpus()));
            color_eyre::install()?;

            Builder::new_multi_thread()
                .worker_threads(workers)
                .thread_name("charted-worker-pool")
                .enable_all()
                .build()
        }

        _ => {
            HookBuilder::new()
                .issue_url("https://github.com/charted-dev/charted/issues/new")
                .add_issue_metadata("version", format!("v{VERSION}+{COMMIT_HASH}"))
                .add_issue_metadata("rustc", RUSTC_VERSION)
                .install()?;

            tracing_subscriber::registry()
                .with(
                    GenericLayer { verbose: cli.verbose }.with_filter(LevelFilter::from_level(match cli.verbose {
                        false => Level::INFO,
                        true => Level::DEBUG,
                    })),
                )
                .init();

            Builder::new_current_thread()
                .worker_threads(1)
                .thread_name("cli-worker-pool")
                .build()
        }
    }?;

    runtime.block_on(async { execute(&cli.command).await })
}
