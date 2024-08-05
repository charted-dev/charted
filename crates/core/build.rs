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

use std::{process::Command, time::SystemTime};

use which::which;

macro_rules! rerun_if_changed {
    ($item:literal) => {
        println!("cargo::rerun-if-changed={}", $item);
    };

    ($item:expr) => {
        println!("cargo::rerun-if-changed={}", $item);
    };
}

macro_rules! rustc_env {
    ($name:literal = $item:expr) => {
        println!("cargo::rustc-env={}={}", $name, $item);
    };
}

macro_rules! warn {
    ($message:literal) => {
        println!("cargo::warning={}", $message);
    };

    ($message:expr) => {
        println!("cargo::warning={}", $message);
    };
}

fn main() {
    // If we make any changes to `build.rs`, re-run it so it can be reflected.
    rerun_if_changed!("build.rs");

    let rustc_version = rustc_version::version()
        .expect("failed to get rustc version")
        .to_string();

    rustc_env!("CHARTED_RUSTC_VERSION" = rustc_version);
    rustc_env!(
        "CHARTED_BUILD_DATE" = {
            let now = SystemTime::now();
            let time: chrono::DateTime<chrono::Utc> = now.into();

            time.to_rfc3339()
        }
    );

    // Some people might want to set this if they're creating a distribution
    // of charted-server that Noelware doesn't support.
    //
    // Values other than `nix` or `git` will result in "from_source".
    match std::env::var("CHARTED_DISTRIBUTION_KIND") {
        Ok(s) => match &*s.to_ascii_lowercase() {
            "nix" => {
                rustc_env!("CHARTED_DISTRIBUTION_KIND" = "nix");
            }

            "git" => {
                rustc_env!("CHARTED_DISTRIBUTION_KIND" = "git");
            }

            s => {
                warn!(format!(
                    "value [{s}] is not a valid one that we support, using \"from_source\" instead"
                ));

                rustc_env!("CHARTED_DISTRIBUTION_KIND" = "from_source");
            }
        },

        Err(std::env::VarError::NotPresent) => {
            warn!(
                "env $CHARTED_DISTRIBUTION_KIND was not set, auto-detection will fail, using \"from_source\" instead"
            );

            rustc_env!("CHARTED_DISTRIBUTION_KIND" = "from_source");
        }

        Err(std::env::VarError::NotUnicode(_)) => panic!("env $CHARTED_DISTRIBUTION_KIND was not valid unicode"),
    }

    match which("git") {
        Ok(git) => {
            let mut cmd = Command::new(git);
            cmd.args(["rev-parse", "--short=8", "HEAD"]);

            let output = cmd.output().expect("`git rev-parse --short=8 HEAD` to succeed");
            let stdout = String::from_utf8(output.stdout).expect("invalid utf-8");
            rustc_env!("CHARTED_COMMIT_HASH" = stdout);
        }

        Err(which::Error::CannotFindBinaryPath) => {
            warn!("`git` was not found -- using `d1cebae` as hash instead");
            rustc_env!("CHARTED_COMMIT_HASH" = "d1cebae");
        }

        Err(e) => {
            panic!("received error [{e}] when trying to find `git` binary");
        }
    }
}
