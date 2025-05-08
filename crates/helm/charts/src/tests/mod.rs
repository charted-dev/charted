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

mod sort_versions;
mod upload;

macro_rules! fixture {
    ($path:literal) => {
        ::std::path::PathBuf::from(::std::env!("CARGO_MANIFEST_DIR"))
            .join("__fixtures__")
            .join($path)
    };
}

pub(in crate::tests) use fixture;

pub(in crate::tests) fn docker_tests_disabled() -> bool {
    matches!(std::env::var("DISABLE_DOCKER_TESTS").as_deref(), Ok("1"))
}
