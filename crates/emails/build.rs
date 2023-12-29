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
    path::PathBuf,
};
use tonic_build::compile_protos;
use which::which;

fn main() {
    // allow cargo to detect changes in build.rs!
    println!("cargo:rerun-if-changed=build.rs");

    // detect changes in protos/emails.proto
    println!("cargo:rerun-if-changed=protos/emails.proto");

    // Allow Prost to use the detected `protoc` binary if there is no PROTOC
    // environment variable.
    if var("PROTOC").is_err() {
        let protoc = which("protoc").expect("missing `protoc` binary!");
        set_var("PROTOC", protoc);
    }

    let file = PathBuf::from("./protos/emails.proto");

    eprintln!("[BUILD] >> compiling proto file {}", file.display());
    compile_protos(file).expect("protobufs to be compiled successfully");
}
