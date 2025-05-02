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

use charted_serverv2::feature::Metadata;

/// Metadata about this feature.
pub const METADATA: Metadata = Metadata {
    name: "Repository/Organization Members",
    config_key: "members",
    description: env!("CARGO_PKG_DESCRIPTION"),
    authors: &["Noelware, LLC. <team@noelware.org>"],
    since: "0.1.0",
    deprecated: None,
};

#[derive(Debug, Clone)]
pub struct Feature;
impl charted_serverv2::feature::Feature for Feature {
    fn metadata(&self) -> Metadata {
        METADATA
    }
}
