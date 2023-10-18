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

use crate::{
    args::{CommonBazelArgs, CommonBuildRunArgs},
    utils::{self, find_bazel},
};
use charted_common::cli::Execute;
use eyre::Result;

#[derive(Debug, Clone, clap::Parser)]
#[clap(about = "Spawns the development server for the web UI")]
pub struct Web {
    #[command(flatten)]
    bazel: CommonBazelArgs,

    #[command(flatten)]
    common: CommonBuildRunArgs,
}

#[async_trait]
impl Execute for Web {
    fn execute(&self) -> Result<()> {
        let bazel = find_bazel(self.bazel.bazel.clone())?;
        let args = utils::BuildCliArgs {
            bazelrc: self.bazel.bazelrc.clone(),
            release: self.common.release,
            args: self.common.args.clone(),
            run: self.common.run,
        };

        match (self.common.release, self.common.run) {
            (true, true) => utils::build_or_run(bazel.clone(), "//web:build", args),
            (false, true) => utils::build_or_run(bazel.clone(), "//web:vite", args),
            _ => utils::build_or_run(bazel.clone(), "//web:build", args),
        }
    }
}
