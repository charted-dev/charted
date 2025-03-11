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

use crate::cli::Program;
use clap::CommandFactory;
use clap_complete::Shell;
use std::io;

/// Generates shell completions for Bash, Zsh, Fish, Elvish, and PowerShell.
#[derive(Debug, clap::Parser)]
pub struct Args {
    /// Specifies a specific shell to generate completions from.
    shell: Option<Shell>,
}

pub fn run(Args { shell }: Args) -> eyre::Result<()> {
    let Some(shell) = shell else {
        trace!("figuring out what shell to use based off `$SHELL`");

        let Some(shell) = Shell::from_env() else {
            trace!("...from `$SHELL`, none existed or contained invalid unicode");
            bail!("unable to detect which shell to generate complentions for from the `$SHELL` environment variable")
        };

        return run(Args { shell: Some(shell) });
    };

    let mut cmd = Program::command();
    clap_complete::generate(shell, &mut cmd, "helm charted", &mut io::stdout());

    Ok(())
}
