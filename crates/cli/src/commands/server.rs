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

use std::path::PathBuf;

/// Runs the API server.
#[derive(Debug, clap::Parser)]
pub struct Args {
    /// Path to a charted `config.toml` configuration file.
    #[arg(long, short = 'c', env = "CHARTED_CONFIG_FILE")]
    config: Option<PathBuf>,

    /// Number of Tokio workers to use.
    ///
    /// By default, this will use the number of avaliable CPU cores on the system
    /// itself.
    #[arg(long, short = 'w', env = "TOKIO_WORKER_THREADS", default_value_t = num_cpus::get())]
    pub workers: usize,
}

pub(crate) async fn run(_: Args) -> eyre::Result<()> {
    Ok(())
}
