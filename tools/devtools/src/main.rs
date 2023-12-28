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

use charted_common::cli::AsyncExecute;
use charted_devtools::Program;
use charted_logging::generic::GenericLayer;
use clap::Parser;
use eyre::Result;
use tracing::{level_filters::LevelFilter, Level};
use tracing_subscriber::{layer::SubscriberExt, registry, Layer};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let program = Program::parse();
    let level = program.level.unwrap_or(Level::INFO);
    let verbose = match program.verbose {
        true => true,
        false => level >= Level::DEBUG,
    };

    let filter = match program.verbose {
        true => LevelFilter::from_level(Level::DEBUG),
        false => LevelFilter::from_level(level),
    };

    let registry = registry().with(GenericLayer { verbose }.with_filter(filter));
    tracing::subscriber::set_global_default(registry).unwrap();

    program.cmd.execute().await
}
