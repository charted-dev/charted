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

use charted_common::cli::Execute;
use clap::CommandFactory;
use clap_complete::{generate, Generator, Shell};
use eyre::Result;
use std::{
    io::{self, Write},
    path::PathBuf,
};

/// Evaluates shell completions for the `charted` binary.
#[derive(Debug, Clone, clap::Parser)]
pub struct Completions {
    /// shell to print out completions for
    shell: Shell,
}

impl Completions {
    fn write_to<G: Generator, W: Write>(generator: G, mut writer: W) {
        let mut cmd = Completions::command();
        generate(generator, &mut cmd, "charted", &mut writer);
    }
}

impl Execute for Completions {
    fn execute(&self) -> Result<()> {
        Completions::write_to(self.shell, io::stdout());
        Ok(())
    }
}
