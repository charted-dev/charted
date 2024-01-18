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
    env::{set_var, var},
    fs,
    path::PathBuf,
    time::SystemTime,
};
use tonic_build::compile_protos;
use which::which;

fn main() {
    // if build.rs changes in any way, then re-run it!
    println!("cargo:rerun-if-changed=build.rs");

    // if protos/emails.proto changes in any way, let Cargo know!
    println!("cargo:rerun-if-changed=protos/emails.proto");

    // if migrations/*.sql changes in any way, let Cargo know!
    println!("cargo:rerun-if-changed=migrations");

    let rust_version = rustc_version::version()
        .expect("unable to get 'rustc' version")
        .to_string();

    println!("cargo:rustc-env=CHARTED_RUSTC_VERSION={rust_version}");

    let build_date = {
        let now = SystemTime::now();
        let date: DateTime<Utc> = now.into();

        date.to_rfc3339()
    };

    println!("cargo:rustc-env=CHARTED_BUILD_DATE={build_date}");

    // First, we need to get the Git commit hash. There is ways we can do it:
    //      1. Find a .git/HEAD file, read where we need to find the file, read the file and `&bytes[0..8]`
    //      2. fuck it and ball with `d1cebae` as the dummy hash
    let headfile = PathBuf::from(".git").join("HEAD");
    if let Ok(true) = headfile.try_exists() {
        // we should expect "ref: ref/heads/<branch we want>"
        let contents = fs::read_to_string(&headfile).expect("to read file [.git/HEAD] contents");
        let Some((_, ref_)) = contents.split_once(' ') else {
            panic!("expected 'ref: ref/heads/<branch we want>'; received {}", contents);
        };

        // i know this isn't rusty but what could i do better
        let mut iter = ref_.split('/');
        let _ = iter.next().expect("to pop 'ref/'");
        let _ = iter.next().expect("to pop 'heads/'");
        let branch = iter.next().expect("full branch");
        let contents = fs::read_to_string(PathBuf::from(".git").join("refs/heads").join(branch.trim()))
            .expect("to read file [.git/refs/heads/<branch>] contents");

        println!("cargo:rustc-env=CHARTED_COMMIT_HASH={}", &contents.trim()[0..8]);
    }

    // allow Prost to use a detected `protoc` binary from the host if no `$PROTOC`
    // is defined as a env variable.
    if var("PROTOC").is_err() {
        let protoc = which("protoc").expect("`protoc` binary to be available in $PATH");
        set_var("protoc", protoc);
    }

    compile_protos("./protos/emails.proto")
        .expect("protobuf [./protos/emails.proto] to be compiled by `prost` successfully");
}
