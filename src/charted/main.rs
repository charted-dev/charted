// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2025 Noelware, LLC. <team@noelware.org>
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

use charted_cli::{
    Program,
    commands::{Subcommand, Tokio, migrate, server, worker},
    install_eyre_hook,
};
use clap::Parser;
use mimalloc::MiMalloc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::runtime::Builder;

#[cfg(not(miri))]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() -> eyre::Result<()> {
    // You can also configure charted-server via system environment
    // variables and can be placed in a `.env` file to load them.
    dotenvy::dotenv().unwrap_or_default();

    let program = Program::parse();
    let runtime = match program.command {
        Subcommand::Server(server::Args {
            tokio: Tokio { workers },
            ..
        }) => {
            let mut builder = Builder::new_multi_thread();
            builder.worker_threads(workers);
            configure_runtime(&mut builder);

            builder.build()?
        }

        Subcommand::Migrate(migrate::Subcommand::Index(ref args)) => {
            let workers = args.tokio.workers;

            program.init_logger();

            let mut builder = Builder::new_multi_thread();
            builder.worker_threads(workers);
            configure_runtime(&mut builder);

            builder.build()?
        }

        Subcommand::Worker(worker::Subcmd::ApiKey(worker::apikey::Args {
            tokio: Tokio { workers },
            ..
        })) |
        Subcommand::Worker(worker::Subcmd::Storage(worker::storage::Args {
            tokio: Tokio { workers },
            ..
        })) => {
            program.init_logger();

            let mut builder = Builder::new_multi_thread();
            builder.worker_threads(workers);
            configure_runtime(&mut builder);

            builder.build()?
        }

        _ => {
            // We defer the installation of this after the configuration
            // loads in `charted server` since, for TOML errors, it is
            // pretty wonky.
            install_eyre_hook()?;

            program.init_logger();

            let mut builder = Builder::new_current_thread();
            builder.worker_threads(1);
            configure_runtime(&mut builder);

            builder.build()?
        }
    };

    runtime.block_on(program.execute())
}

fn configure_runtime(builder: &mut Builder) {
    builder.enable_all().thread_name_fn(thread_name_fn);
}

fn thread_name_fn() -> String {
    static WORKER_ID: AtomicUsize = AtomicUsize::new(0);

    let id = WORKER_ID.fetch_add(1, Ordering::SeqCst);
    format!("charted-worker-pool[#{id}]")
}
