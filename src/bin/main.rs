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

use charted::cli::{commands::Cmd, AsyncExecute, Program};
use clap::Parser;
use color_eyre::config::HookBuilder;
use eyre::Result;
use mimalloc::MiMalloc;
use noelware_config::env;
use std::cmp;
use tokio::runtime::Builder;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

// When `charted server` is invoked, you can specify a `CHARTED_RUNTIME_WORKERS` environment
// variable for Tokio to use a multi-threaded scheduler.
//
// Otherwise, this will be use Tokio's single thread schdeduler.
fn main() -> Result<()> {
    // skip errors since it couldn't be found or whatever, i don't really
    // want to care about it
    let _ = dotenvy::dotenv();

    let program = Program::parse();
    if matches!(program.command, Cmd::Server(_)) && env!("TOKIO_RUNTIME_THREADS").is_ok() {
        eprintln!("[charted WARN] using `TOKIO_RUNTIME_THREADS` will not do anything. please use `CHARTED_RUNTIME_WORKERS` instead");
        std::env::remove_var("TOKIO_RUNTIME_THREADS");
    }

    let runtime = match program.command {
        Cmd::Server(ref server) => {
            color_eyre::install()?;

            let workers = cmp::max(num_cpus::get(), server.workers);
            Builder::new_multi_thread()
                .worker_threads(workers)
                .thread_name("charted-worker-pool")
                .enable_all()
                .build()?
        }

        _ => {
            HookBuilder::new()
                .issue_url("https://github.com/charted-dev/charted/issues/new")
                .add_issue_metadata("version", charted::version())
                .add_issue_metadata("rustc", charted::RUSTC_VERSION)
                .install()?;

            program.init_log();
            Builder::new_current_thread().worker_threads(1).enable_io().build()?
        }
    };

    runtime.block_on(async { program.command.execute().await })
}
