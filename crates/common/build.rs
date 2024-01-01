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

use chrono::{DateTime, Utc};
use std::{
    ffi::{OsStr, OsString},
    fs,
    panic::catch_unwind,
    process::Command,
    time::{Instant, SystemTime},
};
use which::which;

fn execute<T: AsRef<OsStr>>(command: T, args: &[&str]) -> String {
    let start = Instant::now();
    let cmd = command.as_ref();
    let res = Command::new(cmd).args(args).output().unwrap_or_else(|e| {
        let cmd = command.as_ref();

        panic!(
            "unable to execute command [{} {} (took {:?})]: {}",
            cmd.to_string_lossy(),
            args.join(" "),
            start.elapsed(),
            e.kind(),
        )
    });

    if !res.status.success() {
        panic!(
            "command [{} {}] has failed with exit code {} (took ~{:?})\n\n--- stdout ---\n{}\n\n--- stderr ---\n{}",
            cmd.to_string_lossy(),
            args.join(" "),
            res.status.code().unwrap_or(-1),
            start.elapsed(),
            String::from_utf8_lossy(&res.stdout),
            String::from_utf8_lossy(&res.stderr),
        );
    }

    eprintln!(
        "command [{} {}] has successfully executed in {:?}",
        cmd.to_string_lossy(),
        args.join(" "),
        start.elapsed()
    );

    String::from_utf8(res.stdout).unwrap_or_default()
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

    let git = which("git")
        .map(|os| os.into_os_string())
        .unwrap_or(OsString::from("git")); // let the os find the 'git' command

    let commit_hash = catch_unwind(|| execute(git, &["rev-parse", "--short=8", "HEAD"])).unwrap_or_default();
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
