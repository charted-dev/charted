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

use std::io;

use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};

use crate::Cli;

use super::Execute;

#[derive(Debug, Parser)]
#[clap(about = "Generates shell completions for any supported shell!")]
pub struct Completion {
    #[clap(help = "The shell to generate completions from")]
    shell: Shell,
}

impl Execute for Completion {
    fn execute(self) -> anyhow::Result<()> {
        generate::<Shell, String>(
            self.shell,
            &mut Cli::command(),
            "charted".into(),
            &mut io::stdout(),
        );

        Ok(())
    }
}
