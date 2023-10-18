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

use clap::value_parser;
use std::{ffi::OsString, path::PathBuf};

#[derive(Debug, Clone, clap::Args)]
pub struct CommonBazelArgs {
    /// Location to a `bazel` binary.
    #[arg(long, env = "BAZEL")]
    pub bazel: Option<PathBuf>,

    /// List of external .bazelrc files to include. If `/dev/null` is passed, then
    /// it will stop looking for entries after the index of `/dev/null`.
    #[arg(long)]
    pub bazelrc: Vec<PathBuf>,
}

#[derive(Debug, Clone, clap::Args)]
pub struct CommonBuildRunArgs {
    /// If the binary being built should be the release binary which
    /// optimizes for size and does other stuff to make binaries tiny
    /// for Rust targets.
    #[arg(long)]
    pub release: bool,

    /// Additional arguments to pass to the built binary.
    #[arg(value_parser = value_parser!(OsString), num_args = 0.., last = true, allow_hyphen_values = true)]
    pub args: Vec<OsString>,

    /// Additional Bazel arguments to append to. This is inserted before `build`/`test`/`run`/...etc
    #[arg(long = "bazel-arg", value_parser = value_parser!(OsString), env = "BAZEL_ARGS")]
    pub bazel_args: Vec<OsString>,

    /// Whether if the binary being invoked should be ran or not. This is determined by:
    ///
    /// * `--release` & `--run`: Runs the binary in release mode.
    /// * `--release`:           Only builds the binary.
    /// * `--run`:               Runs the binary in development mode.
    #[arg(long)]
    pub run: bool,
}
