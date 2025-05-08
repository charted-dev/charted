// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2025 Noelware, LLC. <team@noelware.org>
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

use serde::Deserialize;
use std::fs::File;
use utoipa::openapi::{Info, Paths};

const OPENAPI_DOCUMENT: &str = "../../assets/openapi.json";

#[derive(Debug, Deserialize)]
struct PartialOpenApi {
    openapi: String,
    info: Info,
    paths: Paths,
}

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed={}", OPENAPI_DOCUMENT);

    let file = File::open(OPENAPI_DOCUMENT).expect("`assets/openapi.json` to be present in src tree");
    serde_json::from_reader::<_, PartialOpenApi>(file).unwrap();
}
