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

use crate::Program;
use clap::CommandFactory;
use clap_complete::{Shell, generate};
use eyre::bail;
use std::io;
use tracing::trace;

/// Generates shell completions for a specified shell or from the `$SHELL` environment
/// variable.
#[derive(Debug, clap::Parser)]
pub struct Args {
    /// Specified shell to generate completions from.
    shell: Option<Shell>,
}

pub fn run(Args { shell }: Args) -> eyre::Result<()> {
    let Some(shell) = shell else {
        trace!("figuring out shell based off $SHELL environment variable");

        let Some(shell) = Shell::from_env() else {
            trace!("...it wasn't found or included invalid unicode");
            bail!("cannot detect current shell based off `$SHELL` environment variable");
        };

        // re-run the command with a shell set
        return run(Args { shell: Some(shell) });
    };

    let mut cmd = Program::command();
    generate(shell, &mut cmd, "charted", &mut io::stdout());

    Ok(())
}
