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

use ansi_term::Style;
use charted_common::cli::Execute;
use charted_common::{os, version, BUILD_DATE, COMMIT_HASH, RUSTC_VERSION, VERSION};
use chrono::DateTime;
use eyre::Result;
use sysinfo::{System, SystemExt};

#[derive(Debug, Clone, clap::Parser)]
#[command(about = "Prints out the current version, commit hash, and build date of the Helm plugin")]
pub struct Version {
    #[arg(short = 'j', long = "json", help = "output version info in JSON")]
    json: bool,
}

impl Execute for Version {
    fn execute(&self) -> Result<()> {
        debug!("refreshing system information...");
        let mut sys = System::default();
        sys.refresh_all();

        let date = DateTime::parse_from_rfc3339(BUILD_DATE)
            .unwrap()
            .format("%a, %h %d, %Y at %H:%M:%S %Z")
            .to_string();

        let name = sys.name().unwrap();
        let version = version();
        let os_version = sys.os_version().unwrap();
        let os_name = os::os_name();
        let arch = os::architecture();

        if self.json {
            let value = serde_json::json!({
                "version": VERSION,
                "build_date": date,
                "commit_hash": COMMIT_HASH,
                "rust_version": RUSTC_VERSION,
                "github_url": match COMMIT_HASH.is_empty() {
                    true => format!("https://github.com/charted-dev/charted/commit/{COMMIT_HASH}"),
                    false => "https://github.com/charted-dev/charted".to_owned()
                },
                "os": serde_json::json!({
                    "name": name,
                    "arch": arch,
                    "os_name": os_name,
                    "version": os_version
                })
            });

            println!("{}", serde_json::to_string_pretty(&value).unwrap());
        } else {
            let bold = Style::new().bold();
            println!("charted-helm-plugin {} ({date})", bold.paint(format!("v{version}")));

            if os_name == "linux" {
                println!("» {os_name}/{arch} on {name} {os_version} ~ compiled with Rust {RUSTC_VERSION}");
            } else {
                println!("» {os_name}/{arch} {os_version} ~ compiled with Rust {RUSTC_VERSION}");
            }

            if !COMMIT_HASH.is_empty() {
                println!("» GitHub: https://github.com/charted-dev/charted/commit/{COMMIT_HASH}");
            }
        }

        Ok(())
    }
}
