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

    String::from_utf8(res.stdout).expect("didn't received UTF-8 encoded output from cmd stdout")
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

    // used for debugging
    eprintln!(concat!("env: ", env!("PATH")));

    let git = which("git")
        .map(|os| os.into_os_string())
        .unwrap_or(OsString::from("git")); // let the os find the 'git' command

    // TODO(@auguwu): NixOS doesn't allow us to use the `git` supplied (it doesn't exist apparently?!):
    // TODO(@auguwu): do we allow CHARTED_GIT_REVISION to be used?
    //
    // env: /nix/store/xdqlrixlspkks50m9b0mpvag65m3pf2w-bash-5.2-p15/bin:/nix/store/y9gr7abwxvzcpg5g73vhnx1fpssr5frr-coreutils-9.3/bin:/nix/store/q56n7lhjw724i7b33qaqra61p7m7c0cd-diffutils-3.10/bin:/nix/store/3ssn79pr531nfyh578r9kwvinp0mvy72-file-5.45/bin:/nix/store/b6izr8wh0p7dyvh3cyg14wq2rn8d31ik-findutils-4.9.0/bin:/nix/store/8kkn44iwdbgqkrj661nr4cjcpmrqqmx8-gawk-5.2.2/bin:/nix/store/xafzciap7acqhfx84dvqkp18bg4lrai3-gnugrep-3.11/bin:/nix/store/c15ama0p8jr4mn0943yjk4rpa2hxk7ml-patch-2.7.6/bin:/nix/store/x23by79p38ll0js1alifmf3y56vqfs49-gnused-4.9/bin:/nix/store/89s3w7b4g78989kpzc7sy4phv0nqfira-gnutar-1.35/bin:/nix/store/2a9na7bp4r3290yqqzg503325dwglxyq-gzip-1.13/bin:/nix/store/pzf6dnxg8gf04xazzjdwarm7s03cbrgz-python3-3.10.12/bin:/nix/store/hjspq68ljkw2pxlki8mh6shi32s67m89-unzip-6.0/bin:/nix/store/9n0384r446blhgla21fpvyr0qnjgjwaw-which-2.21/bin:/nix/store/r77zgzm8a4086678daxvwja7ivcz1d7l-zip-3.0/bin
    // thread 'main' panicked at 'unable to execute command [git rev-parse --short=8 HEAD (took 399.144µs)]: entity not found', crates/common/build.rs:31:9
    let commit_hash = execute(git, &["rev-parse", "--short=8", "HEAD"]);
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
