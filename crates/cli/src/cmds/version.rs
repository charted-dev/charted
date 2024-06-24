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

use clap::Parser;
use eyre::Context;
use std::io::{self, Write as _};

/// Shows version information of the `charted` binary. If `--verbose` appears, it'll show
/// extra information
#[derive(Debug, Clone, Parser)]
pub struct Args {
    /// shows verbose information
    #[arg(long, short = 'v')]
    verbose: bool,
}

pub fn exec(Args { verbose }: Args) -> eyre::Result<()> {
    let mut stdout = io::stdout().lock();
    if verbose {
        writeln!(stdout, "charted v{}", charted_common::version())?;
        writeln!(
            stdout,
            "• Built on Rust {} ({}/{})",
            charted_common::RUSTC_VERSION,
            charted_common::os(),
            charted_common::architecture()
        )?;

        writeln!(stdout, "• Build Timestamp: {}", charted_common::BUILD_DATE)?;

        return Ok(());
    }

    writeln!(
        stdout,
        "charted v{}; built on Rust {} ({}/{})",
        charted_common::version(),
        charted_common::RUSTC_VERSION,
        charted_common::os(),
        charted_common::architecture()
    )
    .context("failed to write to stdout")
}
