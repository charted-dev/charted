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

use chrono::{DateTime, Utc};
use std::{ffi::OsStr, fs, process::Command, time::SystemTime};

fn execute<T: AsRef<OsStr>>(command: T, args: &[&str]) -> String {
    let res = Command::new(command.as_ref()).args(args).output().unwrap_or_else(|_| {
        panic!(
            "unable to execute command [$ {:?} {}]",
            command.as_ref(),
            args.join(" ")
        )
    });

    String::from_utf8_lossy(&res.stdout).to_string()
}

fn main() {
    println!("cargo:rerun-if-changed=../../.charted-version");
    println!("cargo:rerun-if-changed=build.rs");
    let version =
        fs::read_to_string("../../.charted-version").expect("missing '.charted-version' in root project src?!");

    let version = version
        .split('\n')
        .filter(|f| !f.is_empty())
        .collect::<Vec<_>>()
        .first()
        .expect("missing version to embed?!")
        .trim();

    let commit_hash = execute("git", &["rev-parse", "--short=8", "HEAD"]);
    let build_date = {
        let now = SystemTime::now();
        let date: DateTime<Utc> = now.into();

        date.to_rfc3339()
    };

    let rustc_compiler = rustc_version::version()
        .expect("unable to get 'rustc' version")
        .to_string();

    println!("cargo:rustc-env=CHARTED_RUSTC_VERSION={rustc_compiler}");
    println!("cargo:rustc-env=CHARTED_COMMIT_HASH={}", commit_hash.trim());
    println!("cargo:rustc-env=CHARTED_BUILD_DATE={build_date}");
    println!("cargo:rustc-env=CHARTED_VERSION={version}");
}
