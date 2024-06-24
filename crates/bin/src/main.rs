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

use charted_cli::Program;
use clap::Parser;
use color_eyre::config::HookBuilder;
use tokio::runtime::Builder;

fn main() -> eyre::Result<()> {
    // Allow to load environment variables from `.env` in the current directory
    // where `charted` is loaded in.
    dotenvy::dotenv().ok();

    // Next is to parse the CLI arguments. If `"charted"` is only passed, we need
    // a subcommand to know what the end user meant; so `clap` will automatically
    // run the `help` command.
    let program = Program::parse();

    // Next, we need to pick a scheduler to use for Tokio. While the `charted server` command allows
    // to run in Tokio's multi-threaded scheduler, all CLI commands (exception: `server`) are ran
    // through the single-threaded scheduler.
    #[allow(clippy::let_unit_value, clippy::match_single_binding)]
    let runtime = match program.command {
        _ => {
            HookBuilder::new()
                .issue_url("https://github.com/charted-dev/charted/issues/new")
                .add_issue_metadata("version", charted_common::version())
                .add_issue_metadata("rustc", charted_common::RUSTC_VERSION)
                .install()?;

            program.init_logging();
            Builder::new_current_thread().worker_threads(1).enable_io().build()?
        }
    };

    // Now, we can use `Runtime#block_on` to spawn a task to run the command.
    runtime.block_on(charted_cli::exec(program.command))
}
