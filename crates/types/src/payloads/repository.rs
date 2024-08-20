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

pub mod member;
pub mod release;

use crate::{helm::ChartType, name::Name};
use serde::Deserialize;
use utoipa::ToSchema;

super::create_modifying_payload! {
    Repository;

    /// Request body payload for creating a repository.
    #[derive(Debug, Clone, Deserialize, ToSchema)]
    create {
        /// Short description about this repository.
        #[serde(default)]
        pub description: Option<String>,

        /// Whether if this repository is private.
        #[serde(default)]
        pub private: bool,

        /// The contents of the README that will be displayed on the repository. If you're
        /// using charted's official Helm plugin, new releases can update its README and it'll
        /// be reflected.
        ///
        /// This should be valid Markdown, but XSS cross scripting is impossible as scripts
        /// in codeblocks or via `<script>` won't be executed.
        ///
        /// You can retrieve a repository's README (if it is public or if you have access) with
        /// the [`GET /cdn`](https://charts.noelware.org/docs/server/latest/api/cdn#GET-{...params}) REST
        /// endpoint if the instance has the CDN feature enabled. It'll be under `/repositories/{id}/README.md`.
        #[serde(default)]
        pub readme: Option<String>,

        #[serde(skip)]
        pub creator: i64,

        /// [`Name`] to attach to this repository.
        pub name: Name,

        /// Type of chart this represents. When serializing to valid Helm objects,
        /// `operator` will be replaced with `application`.
        #[serde(default, rename = "type")]
        pub r#type: ChartType,
    }

    /// Request body payload for patching a repository's metadata.
    #[derive(Debug, Clone, Deserialize, ToSchema)]
    patch {
        /// Short description about this repository. If `description` was set to `null`, then
        /// this will not be updated, if `description` was a empty string, the `description`
        /// will be set to a empty string and will present as "*no description for this repository*"
        /// in Hoshi.
        #[serde(default)]
        pub description: Option<String>,

        /// Whether if this repository is private. This cannot be set to the actual value
        /// that it was previously.
        #[serde(default)]
        pub private: Option<bool>,

        /// The contents of the README that will be displayed on the repository. If you're
        /// using charted's official Helm plugin, new releases can update its README and it'll
        /// be reflected.
        ///
        /// This should be valid Markdown, but XSS cross scripting is impossible as scripts
        /// in codeblocks or via `<script>` won't be executed.
        ///
        /// You can retrieve a repository's README (if it is public or if you have access) with
        /// the [`GET /cdn`](https://charts.noelware.org/docs/server/latest/api/cdn#GET-{...params}) REST
        /// endpoint if the instance has the CDN feature enabled. It'll be under `/repositories/{id}/README.md`.
        #[serde(default)]
        pub readme: Option<String>,

        /// [`Name`] to update towards, this will not update if it is
        /// the same.
        #[schema(value_type = Name)]
        pub name: Option<Name>,

        /// Type of chart this represents. When serializing to valid Helm objects,
        /// `operator` will be replaced with `application`.
        #[serde(default, rename = "type")]
        pub r#type: Option<ChartType>,
    }
}
