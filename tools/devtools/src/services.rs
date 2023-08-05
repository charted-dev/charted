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

mod emails;
mod search_indexer;

use eyre::Result;
use std::path::PathBuf;

pub use emails::*;
pub use search_indexer::*;

use crate::utils::{build_or_run, BuildCliArgs};

pub trait Service {
    /// Tuple of targets to use. The order should be:
    ///
    /// * release target (when `--release` is passed (and probably `--run`))
    /// * dev target     (when `--run` is specified, but not `--release`)
    fn targets(&self) -> (&'static str, &'static str);

    /// Builds a target and returns the result of it.
    fn build(&self, bazel: PathBuf, args: BuildCliArgs) -> Result<()> {
        build_or_run(bazel, self.targets(), args)
    }

    /// Builds and runs a target that was from the tuple.
    fn run(&self, bazel: PathBuf, args: BuildCliArgs) -> Result<()> {
        build_or_run(bazel, self.targets(), BuildCliArgs { run: true, ..args })
    }
}
