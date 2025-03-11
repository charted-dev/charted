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

// #[path = "build/generate_path_item.rs"]
// mod generate_path_item;

use serde_json::Value;
use std::fs;
use utoipa::openapi::Paths;

const OPENAPI_DOCUMENT: &str = "../../assets/openapi.json";

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed={}", OPENAPI_DOCUMENT);

    let contents = fs::read_to_string(OPENAPI_DOCUMENT).unwrap();
    let as_value: Value = serde_json::from_str(&contents).unwrap();
    let contents = Value::Object(
        as_value
            .get("paths")
            .expect("`paths` is missing from openapi document")
            .as_object()
            .expect("`paths` was not a object")
            .clone(),
    );

    let Paths { paths, .. } = serde_json::from_value(contents).unwrap();
    eprintln!("{:#?}", paths);
}
