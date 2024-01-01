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

use super::{entities::ApiKeyScope, helm::ChartType, Name};
use crate::serde::duration::Duration;
use semver::Version;
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

/// Payload for patching your user metadata.
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

/// Payload to login as a user from the `GET /users/login` endpoint for session-based
/// authentication.
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

/// Payload to create a Repository entity.
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateRepositoryPayload {
    /// Short description about this repository.
    #[serde(default)]
    #[validate(length(max = 140))]
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

    /// [`Name`] to attach to this repository.
    #[schema(value_type = Name)]
    pub name: Name,

    /// Type of chart this represents. When serializing to valid Helm objects,
    /// `operator` will be replaced with `application`.
    #[serde(default, rename = "type")]
    pub r#type: ChartType,
}

/// Payload to patch a repository's metadata.
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema, Validate)]
pub struct PatchRepositoryPayload {
    /// Short description about this repository. If `description` was set to `null`, then
    /// this will not be updated, if `description` was a empty string, the `description`
    /// will be set to a empty string and will present as "*no description for this repository*"
    /// in Hoshi.
    #[serde(default)]
    #[validate(length(max = 140))]
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

#[derive(Debug, Clone, Default, Validate, Serialize, Deserialize)]
pub struct PatchUserConnectionsPayload {
    /// Snowflake ID that was sourced from [Noelware's Accounts System](https://accounts.noelware.org)
    pub noelware_account_id: Option<u64>,

    /// Account ID that was sourced from Google OAuth2
    pub google_account_id: Option<String>,

    /// Account ID that was sourced from GitHub OAuth2. This can differ from
    /// GitHub (https://github.com) and GitHub Enterprise usage.
    pub github_account_id: Option<String>,
}

/// Request body payload for creating a new organization
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateOrganizationPayload {
    /// Short description about this organization. If `description` was set to `null`, then
    /// this will not be updated, if `description` was a empty string, the `description`
    /// will be set to a empty string and will present as "*no description for this organization*"
    /// in Hoshi.
    #[serde(default)]
    #[validate(length(max = 140))]
    pub description: Option<String>,

    /// Display name for this organization.
    #[serde(default)]
    #[validate(length(max = 64))]
    pub display_name: Option<String>,

    /// Whether if the organization is private or not.
    #[serde(default)]
    pub private: bool,

    /// Organization name.
    pub name: Name,
}

/// Request body payload for patching an organization's metadata.
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema, Validate)]
pub struct PatchOrganizationPayload {
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
    #[validate(email)]
    pub gravatar_email: Option<String>,

    /// Display name for this organization.
    #[serde(default)]
    #[validate(length(max = 64))]
    pub display_name: Option<String>,

    /// Whether if the organization is private or not.
    #[serde(default)]
    pub private: Option<bool>,

    /// Organization name to rename to.
    #[serde(default)]
    pub name: Option<Name>,
}

/// Request body payload for creating a API key.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateApiKeyPayload {
    /// Description of the API key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 140))]
    pub description: Option<String>,

    /// Duration of when the API key should expire at
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<Duration>,

    /// the list of scopes to apply to this API key
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<ApiKeyScope>,

    /// key name to use to identify the key
    #[validate]
    pub name: Name,
}

/// Request body payload to patch a API key's metadata.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Validate, ToSchema)]
pub struct PatchApiKeyPayload {
    /// Updates or removes the description of the API key.
    ///
    /// * If this is `null`, this will not do any patching
    /// * If this is a empty string, this will act as "removing" it from the metadata
    /// * If the comparsion (`old.description == this.description`) is false, then this will update it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 140))]
    pub description: Option<String>,

    /// key name to use to identify the key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[validate]
    pub name: Option<Name>,
}

/// Represents the request body for patching a repository release.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Validate, ToSchema)]
pub struct PatchRepositoryReleasePayload {
    /// The body of the release notes to update. It will not attempt to update
    /// if the contents are the same.
    ///
    /// Conditions:
    /// * If `update_text` is null, then nothing will be attempted to be patched
    /// * If `update_text` is a empty string, then the release notes will be removed
    /// * If `update_text` is an non empty string and is different than the ones in the previous cache,
    ///   then it'll be updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 2048))]
    pub update_text: Option<String>,
}

/// Represents the request body to create a Repository Release.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateRepositoryReleasePayload {
    /// The body of the release notes that can be in Markdown.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 2048))]
    pub update_text: Option<String>,

    /// Release tag that can be filtered through.
    pub tag: Version,
}
