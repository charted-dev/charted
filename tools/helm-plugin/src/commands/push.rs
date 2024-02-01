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

use charted::cli::AsyncExecute;
use clap::Parser;

/// Push one or all Helm charts to a charted-server registry
#[derive(Debug, Clone, Parser)]
pub struct Cmd {
    /// amount of concurrent workers to push Helm charts to (useful to parallelize
    /// multiple uploads into one request). By default, this will be limited to 2 workers but
    /// it can exceed to the amount of CPUs you have.
    #[arg(short = 'c', long, env = "CHARTED_HELM_CONCURRENCY")]
    concurrency: Option<usize>,
}

#[async_trait]
impl AsyncExecute for Cmd {
    async fn execute(&self) -> eyre::Result<()> {
        todo!()
    }
}
