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

#![cfg_attr(test, feature(decl_macro))]

/// Accepted content types that are allowed to be sent as a tarball
pub(crate) const ACCEPTABLE_CONTENT_TYPES: &[&str] = &["application/gzip", "application/tar+gzip"];

/// Exempted files that aren't usually in a Helm chart, but they are allowed to be in one.
pub(crate) const EXEMPTED_FILES: &[&str] = &["values.schema.json", "README.md", "LICENSE"];
pub(crate) const ALLOWED_FILES: &[&str] = &["README.md", "LICENSE", "values.yaml", "Chart.yaml", "Chart.lock"];

mod functions;
pub use functions::*;
