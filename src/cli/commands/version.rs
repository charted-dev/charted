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

use charted_common::{architecture, os, BUILD_DATE, COMMIT_HASH, VERSION};
use charted_entities::Distribution;
use serde_json::json;

/// Returns the version information of this binary.
#[derive(Debug, Clone, clap::Parser)]
pub struct Args {
    /// represent the version information as JSON.
    #[arg(short = 'j', long)]
    json: bool,
}

pub fn run(Args { json }: Args) {
    let distribution = Distribution::detect();
    if json {
        let info = json!({
            "version": VERSION,
            "commit_hash": COMMIT_HASH,
            "build_date": BUILD_DATE,
            "distribution": distribution
        });

        eprintln!("{}", serde_json::to_string(&info).unwrap());
        return;
    }

    eprintln!(
        "🐻‍❄️📦 charted-server v{} ({}/{}) on {}",
        crate::version(),
        os(),
        architecture(),
        distribution
    );
}
