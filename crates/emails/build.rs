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

use std::{
    env::{set_var, var},
    fs,
    path::PathBuf,
};
use tonic_build::configure;
use which::which;

fn main() {
    // allow cargo to detect changes in build.rs!
    println!("cargo:rerun-if-changed=build.rs");

    // detect changes in protos/emails.proto
    #[cfg(not(bazel))]
    println!("cargo:rerun-if-changed=protos/emails.proto");

    // Allow Prost to use the detected `protoc` binary if there is no PROTOC
    // environment variable. Bazel users will have this defined as we have a
    // hermetic protoc toolchain.
    if var("PROTOC").is_err() {
        let protoc = which("protoc").expect("missing `protoc` binary!");
        set_var("PROTOC", protoc);
    }

    let files = var("PROTOS")
        .unwrap_or("./protos/emails.proto".into())
        .split(',')
        .map(|f| f.parse::<PathBuf>())
        .filter_map(|s| s.ok())
        .collect::<Vec<_>>();

    for file in files {
        eprintln!("[BUILD] >> compiling proto file [{}]", file.display());
        let (parent, canonicalized) = match cfg!(bazel) {
            false => {
                let canonicalized = fs::canonicalize(file.clone())
                    .unwrap_or_else(|e| panic!("unable to canonicalize file {}: {e}", file.display()));

                let parent = file.parent().unwrap().canonicalize().unwrap();
                eprintln!(
                    "[BUILD] >> PARENT: [{}] | CANONICAL PATH: [{}]",
                    parent.display(),
                    canonicalized.display()
                );

                (parent, canonicalized)
            }

            true => {
                let canonical_path = file.clone();
                let parent = file.parent().unwrap().to_path_buf();

                eprintln!(
                    "[BUILD] >> PARENT: [{}] | CANONICAL PATH: [{}]",
                    parent.display(),
                    canonical_path.display()
                );

                (file.parent().unwrap().to_path_buf(), file.clone())
            }
        };

        let builder = configure()
            .build_client(true)
            .build_server(false)
            .compile_well_known_types(true);

        builder.compile(&[canonicalized], &[parent]).unwrap();
    }
}
