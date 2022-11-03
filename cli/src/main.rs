// 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
// Copyright 2022 Noelware <team@noelware.org>
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

use charted_cli::{commands::execute, setup_utils::setup_logging, try_get_value, Cli};
use clap::Parser;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // It's safe since fern won't die at all... I hope...
    setup_logging(
        cli.level.unwrap_or_else(|| {
            if cli.verbose {
                return log::LevelFilter::Debug;
            }

            log::LevelFilter::Info
        }),
        cli.colors.unwrap_or(true),
    )
    .unwrap();

    try_get_value!(execute(cli.subcommand).await);
}
