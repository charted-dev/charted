// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

use super::Name;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// Represents the payload for creating a new user.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateUserPayload {
    /// User handle to use to identify yourself.
    #[schema(value_type = Name)]
    pub username: Name,

    /// The password to use when authenticating, this is optional on non-local sessions.
    #[schema(
        value_type = password,
        pattern = "^.*(?=.{8,})(?=.*[a-zA-Z])(?=.*\\d)?(?=.*[!#$%&? \"])?.*$"
    )]
    #[validate(length(min = 8))]
    pub password: Option<String>,

    /// Email address to identify this user
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct PatchUserPayload {
    /// Optional field to update this user's gravatar email. If this user doesn't
    /// have an avatar that is used or prefers not to use their previously uploaded
    /// avatars and they set their Gravatar email, their Gravatar will be used.
    #[validate(email)]
    pub gravatar_email: Option<String>,

    /// Short description about this user. If this field was provided, then the
    /// description will be overwritten. If this field is `null`, then nothing
    /// will happen. If this field is a empty string, then the description
    /// will be wiped.
    #[validate(length(max = 140))]
    pub description: Option<String>,

    /// Updates this user's username.
    #[schema(value_type = Name, nullable)]
    pub username: Option<Name>,

    /// Updates this user's password, if the session manager configured allows it.
    #[schema(
        value_type = password,
        pattern = "^.*(?=.{8,})(?=.*[a-zA-Z])(?=.*\\d)?(?=.*[!#$%&? \"])?.*$"
    )]
    #[validate(length(min = 8))]
    pub password: Option<String>,

    /// Updates this user's email.
    #[validate(email)]
    pub email: Option<String>,

    /// Updates this user's display name.
    #[validate(length(min = 1, max = 64))]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct UserLoginPayload {
    /// Password to authenticate as.
    #[schema(
        value_type = password,
        pattern = "^.*(?=.{8,})(?=.*[a-zA-Z])(?=.*\\d)?(?=.*[!#$%&? \"])?.*$"
    )]
    #[validate(length(min = 8))]
    pub password: String,

    /// Username to authenticate as. This is mutually exclusive with `email`.
    pub username: Option<Name>,

    /// Email to authenticate as. This is mutually exclusive with `username`.
    #[validate(email)]
    pub email: Option<String>,
}
