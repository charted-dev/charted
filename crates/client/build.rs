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

// use std::{
//     fs::{self, File},
//     path::PathBuf,
// };

const OPENAPI: &str = "../../assets/openapi.json";

fn main() {
    println!("cargo::rerun-if-changed={OPENAPI}");
    println!("cargo::rerun-if-changed=build.rs");

    // let file = File::open(OPENAPI).unwrap();
    // let spec = serde_json::from_reader(file).unwrap();

    // let tokens = progenitor::Generator::default().generate_tokens(&spec).unwrap();
    // let ast = syn::parse2(tokens).unwrap();
    // let content = prettyplease::unparse(&ast);

    // let out = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "src/generated.rs"));
    // fs::write(out, content).unwrap();
}
