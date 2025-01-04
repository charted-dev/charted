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

use crate::Version;
use serde::Deserialize;
use utoipa::ToSchema;

super::super::create_modifying_payload! {
    RepositoryRelease;

    /// Request body payload for creating a release for a repository.
    #[derive(Debug, Clone, Deserialize, ToSchema)]
    create {
        /// Represents a changelog (that can be Markdown or HTML (it'll remove XSS vulnerabilities)) that will
        /// appear via `/repositories/:id/releases/:version/changelog`.
        ///
        /// > [!NOTE]
        /// > Hoshi will wrap `CHANGELOG.html` to the same styles as when rendering Markdown.
        #[serde(default)]
        pub update_text: Option<String>,

        /// SemVer-based [`Version`] to indicate what version this release is. This is an immutable
        /// tag and can't be patched without conflicting; you can only delete a release by its ID
        /// or version, which will remove this tag and can be freely used.
        pub tag: Version,
    }

    /// Request body payload for updating a repository release's metadata.
    #[derive(Debug, Clone, Deserialize, ToSchema)]
    patch {
        /// Updates the changelog for the repository release that people can view from
        /// the API server or from [Hoshi]
        ///
        /// * If this is `null`, this will not attempt to do anything as this is the
        ///   default state.
        /// * If this is just `""`, then this is considered as a removal and won't be available
        ///   for people to access.
        /// * If this is not an empty string, it will overwrite the previous value.
        ///
        /// [Hoshi]: https://charts.noelware.org/docs/hoshi/latest
        #[serde(default)]
        pub update_text: Option<String>,
    }
}
