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

use charted_cli::{cmds::Cmd, Program};
use clap::Parser;
use color_eyre::config::HookBuilder;
use dotenvy::dotenv;
use mimalloc::MiMalloc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::runtime::Builder;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() -> eyre::Result<()> {
    let _ = dotenv();
    let program = Program::parse();

    let runtime = match program.command {
        Cmd::Server(ref args) => {
            color_eyre::install()?;

            let workers = std::cmp::max(num_cpus::get(), args.workers);
            Builder::new_multi_thread()
                .worker_threads(workers)
                .enable_all()
                .thread_name_fn(thread_name_fn)
                .build()?
        }

        _ => {
            HookBuilder::new()
                .issue_url("https://github.com/charted-dev/charted/issues/new")
                .add_issue_metadata("version", charted_core::version())
                .add_issue_metadata("rustc", charted_core::RUSTC_VERSION)
                .install()?;

            program.init_logger();
            Builder::new_current_thread()
                .worker_threads(1)
                .enable_io()
                .thread_name_fn(thread_name_fn)
                .build()?
        }
    };

    runtime.block_on(program.command.run())
}

fn thread_name_fn() -> String {
    static WORKER_ID: AtomicUsize = AtomicUsize::new(0);

    let id = WORKER_ID.fetch_add(1, Ordering::SeqCst);
    format!("charted-worker-pool[#{id}]")
}
