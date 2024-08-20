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

use crate::name::Name;
use serde::Deserialize;
use utoipa::ToSchema;

super::create_modifying_payload! {
    Organization;

    /// Request body payload for creating a new organization.
    #[derive(Debug, Clone, Deserialize, ToSchema)]
    create {
        /// Short description about this organization. If `description` was set to `null`, then
        /// this will not be updated, if `description` was a empty string, the `description`
        /// will be set to a empty string and will present as "*no description for this organization*"
        /// in Hoshi.
        #[serde(default)]
        pub description: Option<String>,

        /// Display name for this organization.
        #[serde(default)]
        pub display_name: Option<String>,

        /// Whether if the organization is private or not.
        #[serde(default)]
        pub private: bool,

        /// Organization name.
        pub name: Name,
    }

    /// Request body payload for patching an organization's metadata.
    #[derive(Debug, Clone, Deserialize, ToSchema)]
    patch {
        /// Adds or removes a Twitter handle from this organization's metadata.
        ///
        /// * If this is `null`, this will not do any patching
        /// * If this is a empty string, this will act as "removing" it from the metadata
        /// * If the comparsion (`old.twitter_handle == twitter_handle`) is false, then this will update it.
        #[serde(default)]
        pub twitter_handle: Option<String>,

        /// Optional field to update this organization's gravatar email. If this organization doesn't
        /// have an avatar that is used or prefers not to use their previously uploaded
        /// avatars and they set their Gravatar email, their Gravatar will be used.
        #[serde(default)]
        pub gravatar_email: Option<String>,

        /// Display name for this organization.
        #[serde(default)]
        pub display_name: Option<String>,

        /// Whether if the organization is private or not.
        #[serde(default)]
        pub private: Option<bool>,

        /// Organization name to rename to.
        #[serde(default)]
        pub name: Option<Name>,
    }
}
