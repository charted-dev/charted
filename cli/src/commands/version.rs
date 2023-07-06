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

use super::models::Execute;
//use ansi_term::Style;
//use charted_common::{is_debug_enabled, BUILD_DATE, COMMIT_HASH, RUSTC_VERSION, VERSION};
//use chrono::DateTime;
use eyre::Result;

#[derive(Debug, Clone, clap::Parser)]
#[command(about = "Prints out the current version, commit hash, and build date of the CLI")]
pub struct Version;

impl Execute for Version {
    fn execute(&self) -> Result<()> {
        // let date = DateTime::parse_from_rfc3339(BUILD_DATE)
        //     .unwrap()
        //     .format("%a, %h %d, %Y at %H:%M:%S %Z");

        // let bold = Style::new().bold();
        // println!(
        //     "charted-server {} ({date}) - compiled with Rust {RUSTC_VERSION}",
        //     bold.paint(format!("v{VERSION}+{COMMIT_HASH}"))
        // );

        // println!("» GitHub: https://github.com/charted-dev/charted/commit/{COMMIT_HASH}");

        // if is_debug_enabled() {
        //     println!("» Current build is a debug build, possibly defined by you.");
        // }

        Ok(())
    }
}
