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

use crate::name::Name;
use serde::Deserialize;
use utoipa::ToSchema;

super::create_modifying_payload! {
    User;

    /// Request body payload for creating a user.
    #[derive(Debug, Clone, Deserialize, ToSchema)]
    create {
        /// User handle to use to identify yourself.
        pub username: Name,

        /// The password to use when authenticating, this is optional on non-local sessions.
        #[schema(pattern = "^.*(?=.{8,})(?=.*[a-zA-Z])(?=.*\\d)?(?=.*[!#$%&? \"])?.*$")]
        pub password: Option<String>,

        /// Email address to identify this user
        pub email: String,
    }

    /// Request body payload for modifying a user's metadata
    #[derive(Debug, Clone, Deserialize, ToSchema)]
    patch {
        /// Determines if the user avatar should use their Gravatar email.
        #[serde(default)]
        pub prefers_gravatar: Option<bool>,

        /// Updates the email address to fetch the Gravatar email from.
        pub gravatar_email: Option<String>,

        /// Short description about this user.
        pub description: Option<String>,

        /// Updates this user's username.
        pub username: Option<Name>,

        /// Updates this user's password, if the session manager configured allows it.
        #[schema(pattern = "^.*(?=.{8,})(?=.*[a-zA-Z])(?=.*\\d)?(?=.*[!#$%&? \"])?.*$")]
        pub password: Option<String>,

        /// Updates this user's email.
        pub email: Option<String>,

        /// Updates this user's display name.
        pub name: Option<String>,
    }
}
