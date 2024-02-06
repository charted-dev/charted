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

mod apikeyscope;
mod member_permissions;

pub use apikeyscope::*;
pub use member_permissions::*;

use crate::common::{
    models::{helm::ChartType, DateTime, Name},
    snowflake::ID,
};
use semver::Version;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{
    openapi::{RefOr, Schema},
    ToSchema,
};

// only used for Utoipa to use the `ID` schema from snowflake.rs
fn snowflake_schema() -> RefOr<Schema> {
    ID::schema().1
}

fn name_schema() -> RefOr<Schema> {
    crate::common::models::Name::schema().1
}

/// Represents an account that can own [repositories][Repository] and [organizations][Organizations]
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema, FromRow)]
pub struct User {
    /// Whether if this User is a Verified Publisher or not.
    #[serde(default)]
    pub verified_publisher: bool,

    /// Valid email address that points to a Gravatar avatar, or `null` if it shouldn't use one as the primary avatar
    #[serde(default)]
    pub gravatar_email: Option<String>,

    /// Short description about this user, can be `null` if none was provided.
    #[serde(default)]
    pub description: Option<String>,

    /// Unique hash to locate a user's avatar, this also includes the extension that this avatar is, i.e, `png`.
    #[serde(default)]
    pub avatar_hash: Option<String>,

    /// Date of when this user was registered to this instance
    pub created_at: DateTime,

    /// Date of when the server has last updated this user
    pub updated_at: DateTime,

    #[serde(skip)]
    pub password: Option<String>,

    /// Unique username that can be used to locate this user with the API
    pub username: Name,

    #[serde(skip)]
    pub email: String,

    /// Whether if this User is an Administrator of this instance
    #[serde(default)]
    pub admin: bool,

    /// Display name for this user, it should be displayed as '{name} (@{username})' or just '@{username}' if there is no display name
    #[serde(default)]
    #[schema(schema_with = name_schema)]
    pub name: Option<String>,

    /// Unique identifier to locate this user with the API
    #[schema(schema_with = snowflake_schema)]
    pub id: i64,
}

/// Represents a collection of a user's connections that can be used
/// to login from different sources (like GitHub OAuth2)
#[derive(Debug, Clone, Default, ToSchema, Serialize, Deserialize, FromRow)]
pub struct UserConnections {
    /// Snowflake ID that was sourced from [Noelware's Accounts System](https://accounts.noelware.org)
    #[serde(default)]
    pub noelware_account_id: Option<i64>,

    /// Account ID that was sourced from Google OAuth2
    #[serde(default)]
    pub google_account_id: Option<String>,

    /// Account ID that was sourced from GitHub OAuth2. This can differ from
    /// GitHub (https://github.com) and GitHub Enterprise usage.
    #[serde(default)]
    pub github_account_id: Option<String>,

    /// Date of when this connection was inserted to the database.
    pub created_at: DateTime,

    /// Date of when the server has last updated this user's connections.
    pub updated_at: DateTime,

    /// Snowflake of the user that owns this connections object.
    #[schema(schema_with = snowflake_schema)]
    pub id: i64,
}

#[derive(Debug, Clone, Default, ToSchema, Serialize, Deserialize, FromRow)]
pub struct Repository {
    /// Short description about this user, can be `null` if none was provided.
    #[serde(default)]
    pub description: Option<String>,

    /// Whether if this repository is deprecated or not
    #[serde(default)]
    pub deprecated: bool,

    /// Date of when this repository was registered to this instance
    pub created_at: DateTime,

    /// Date of when the server has last updated this repository
    pub updated_at: DateTime,

    /// Unique hash to locate a repository's icon, this also includes the extension that this avatar is, i.e, `png`.
    #[serde(default)]
    pub icon_hash: Option<String>,

    /// Whether if this repository is private or not
    #[serde(default)]
    pub private: bool,

    /// Unique identifier that points to a User or Organization resource that owns this repository
    #[schema(schema_with = snowflake_schema)]
    pub owner: i64,

    /// Unique [Name] to locate this repository from the API
    pub name: Name,

    /// The chart type that this repository is
    pub r#type: ChartType,

    /// Unique identifier to locate this repository from the API
    #[schema(schema_with = snowflake_schema)]
    pub id: i64,
}

/// Represents a resource that contains a release from a [Repository] release. Releases
/// are a way to group releases of new versions of Helm charts that can be easily
/// fetched from the API server.
///
/// Any repository can have an unlimited amount of releases, but tags cannot clash
/// into each other, so the API server will not accept it. Each tag should be
/// a SemVer 2 comformant string, parsing is related to how Cargo evaluates SemVer 2 tags.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
pub struct RepositoryRelease {
    /// Whether if this release is a pre-release or not.
    #[serde(default)]
    pub is_prerelease: bool,

    /// Markdown-formatted string that contains a changelog of this release.
    #[serde(default)]
    pub update_text: Option<String>,

    /// Date of when this release was registered to this instance
    pub created_at: DateTime,

    /// Date of when the server has last updated this repository release
    pub updated_at: DateTime,

    /// SemVer 2 comformant string that represents this tag.
    pub tag: Version,

    /// Unique identifier to locate this repository release resource from the API.
    #[schema(schema_with = snowflake_schema)]
    pub id: i64,
}

/// Represents a resource that is correlated to a repository or organization member
/// that can control the repository's metadata.
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema, FromRow)]
pub struct Member {
    /// Display name for this member. This should be formatted as '[{display_name}][Member::display_name] (@[{username}][User::username])' if this
    /// is set, otherwise '@[{username}][User::username]' is used.
    pub display_name: Option<String>,

    /// Bitfield value of this member's permissions.
    pub permissions: u64,

    /// Date-time of when this member resource was last updated by the API server.
    pub updated_at: DateTime,

    /// Date-time of when this member resource was created by the API server.
    pub joined_at: DateTime,

    /// [User] resource that this member is.
    pub user: User,

    /// Unique identifier to locate this member with the API
    #[schema(schema_with = snowflake_schema)]
    pub id: i64,
}

impl Member {
    /// Returns a new [`Bitfield`], but this member's permissions
    /// are filled in for the bitfield.
    ///
    /// ## Example
    /// ```
    /// # use charted::common::models::entities::Member;
    /// #
    /// let member = Member::default();
    /// assert_eq!(member.bitfield().bits(), 0);
    /// ```
    pub fn bitfield<'a>(&self) -> MemberPermissions<'a> {
        MemberPermissions::init(self.permissions)
    }
}

/// Represents a unified entity that can manage and own repositories outside
/// a User. Organizations to the server is used for business-related Helm charts
/// that aren't tied to a specific User.
#[derive(Debug, Clone, ToSchema, Serialize, Deserialize, Default, FromRow)]
pub struct Organization {
    /// Whether if this Organization is a Verified Publisher or not.
    #[serde(default)]
    pub verified_publisher: bool,

    /// Returns the twitter handle for this organization, if populated.
    #[serde(default)]
    pub twitter_handle: Option<String>,

    /// Valid email address that points to a Gravatar avatar, or `null` if it shouldn't use one as the primary avatar
    #[serde(default)]
    pub gravatar_email: Option<String>,

    /// Display name for this organization. It should be formatted as '[{display_name}][Organization::display_name] (@[{name}][Organization::name])'
    /// or '@[{name}][Organization::name]'.
    #[serde(default)]
    pub display_name: Option<String>,

    /// Date of when this organization was registered to this instance
    pub created_at: DateTime,

    /// Date of when the server has last updated this organization
    pub updated_at: DateTime,

    /// Unique hash to locate an organization's icon, this also includes the extension that this icon is, i.e, `png`.
    #[serde(default)]
    pub icon_hash: Option<String>,

    /// Whether this organization is private and only its member can access this resource.
    #[serde(default)]
    pub private: bool,

    /// User ID that owns this organization
    #[schema(schema_with = snowflake_schema)]
    pub owner: i64,

    /// The name for this organization.
    pub name: Name,

    /// Unique identifier to locate this organization with the API
    #[schema(schema_with = snowflake_schema)]
    pub id: i64,
}

/// A resource for personal-managed API tokens that is created by a User. This is useful
/// for command line tools or scripts that need to interact with charted-server, but
/// the main use-case is for the [Helm plugin](https://charts.noelware.org/docs/helm-plugin/current).
#[derive(Debug, Clone, Default, ToSchema, Serialize, Deserialize, FromRow)]
pub struct ApiKey {
    /// Short description about this API key.
    #[serde(default)]
    pub description: Option<String>,

    /// Date-time of when this API token expires in, `null` can be returned
    /// if the token doesn't expire
    #[serde(default)]
    pub expires_in: Option<DateTime>,

    /// The scopes that are attached to this API key resource.
    pub scopes: i64,

    /// The token itself. This is never revealed when querying, but only revealed
    /// when you create the token.
    #[serde(default)]
    pub token: Option<String>,

    /// User resource that owns this API key.
    pub owner: User,

    /// The name of the API key.
    pub name: Name,

    /// Unique identifer to locate this resource in the API server.
    #[schema(schema_with = snowflake_schema)]
    pub id: i64,
}

impl ApiKey {
    /// Returns a new [`Bitfield`], but this member's permissions
    /// are filled in for the bitfield.
    ///
    /// ## Example
    /// ```
    /// # use charted::common::models::entities::ApiKey;
    /// #
    /// let resource = ApiKey::default();
    /// assert_eq!(resource.bitfield().bits(), 0);
    /// ```
    pub fn bitfield<'a>(&self) -> ApiKeyScopes<'a> {
        ApiKeyScopes::init(self.scopes.try_into().unwrap())
    }
}
