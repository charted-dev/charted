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
    utils::{self, BuildCliArgs},
};
use charted_common::cli::Execute;
use eyre::Result;

#[derive(Debug, Clone, clap::Parser)]
#[clap(about = "Builds the Helm plugin or runs it")]
pub struct HelmPlugin {
    #[command(flatten)]
    common: CommonBuildRunArgs,

    #[command(flatten)]
    bazel: CommonBazelArgs,
}

impl Execute for HelmPlugin {
    fn execute(&self) -> Result<()> {
        let bazel = utils::find_bazel(self.bazel.bazel.clone())?;
        let args = BuildCliArgs {
            release: self.common.release,
            bazelrc: self.bazel.bazelrc.clone(),
            args: self.common.args.clone(),
            run: self.common.run,
        };

        utils::build_or_run(
            bazel.clone(),
            ("//tools/helm-plugin:release_binary", "//tools/helm-plugin:binary"),
            args,
        )
    }
}
