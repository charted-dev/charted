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

use crate::{helm::ChartType, name::Name, DateTime, Ulid, Version};
use charted_core::bitflags::ApiKeyScopes;
use diesel::prelude::{Insertable, Queryable};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema, Queryable, Insertable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[diesel(table_name = charted_database::schema::postgresql::users)]
#[diesel(table_name = charted_database::schema::sqlite::users)]
pub struct User {
    /// whether or not if this user is considered a verified publisher.
    #[schema(read_only)]
    #[serde(default)]
    pub verified_publisher: bool,

    /// Email address that is the Gravatar email to which we should use the user's avatar.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gravatar_email: Option<String>,

    /// Short description about this user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Unique hash that identifies the user's avatar that they uploaded via the REST API.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_hash: Option<String>,

    /// Date of when this user was created. This uses the host system's local time instead
    /// of UTC.
    #[schema(read_only, value_type = DateTime)]
    pub created_at: DateTime,

    /// Date of when the server has last updated this user's metadata
    #[schema(read_only, value_type = DateTime)]
    pub updated_at: DateTime,

    /// Name of this user that can be identified easier.
    pub username: Name,

    /// Whether if this User is an Administrator of this instance
    #[serde(default)]
    #[schema(read_only)]
    pub admin: bool,

    /// Display name for this user, it should be displayed as '{name} (@{username})' or just '@{username}' if there is no display name
    #[serde(default)]
    pub name: Option<String>,

    /// Unique identifier to locate this user via the REST API.
    pub id: Ulid,

    #[serde(skip)]
    pub password: Option<String>,

    #[serde(skip)]
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema, Queryable, Insertable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[diesel(table_name = charted_database::schema::postgresql::user_connections)]
#[diesel(table_name = charted_database::schema::sqlite::user_connections)]
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

    /// Date of when this entity was created. In most cases, this will be mere milliseconds
    /// or seconds to when a [`User`] is created.
    #[schema(read_only)]
    pub created_at: DateTime,

    /// Last timestamp of when the API server has modified this entity.
    pub updated_at: DateTime,

    /// Unique identifier of this entity.
    #[schema(read_only)]
    pub id: Ulid,
}

#[derive(Debug, Clone, Serialize, ToSchema, Queryable, Insertable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[diesel(table_name = charted_database::schema::postgresql::repositories)]
#[diesel(table_name = charted_database::schema::sqlite::repositories)]
pub struct Repository {
    /// Short description about this user, can be `null` if none was provided.
    #[serde(default)]
    pub description: Option<String>,

    /// whether if this repository is deprecated or not.
    #[serde(default)]
    pub deprecated: bool,

    /// Timestamp of when this entity was created.
    #[schema(read_only)]
    pub created_at: DateTime,

    /// Timestamp of when the API server has last updated this entity.
    pub updated_at: DateTime,

    /// Hash identifier for the repository's icon, if one was uploaded.
    #[serde(default)]
    #[schema(read_only)]
    pub icon_hash: Option<String>,

    /// The "creator" of the repository. This will return `null` if the
    /// owner is already a [`User`], otherwise, this will point to the
    /// user's ID that made the repository under the organization.
    #[serde(default)]
    #[schema(read_only)]
    pub creator: Option<Ulid>,

    /// whether if the repository is private and only its members can view it.
    #[serde(default)]
    pub private: bool,

    /// The owner of the repository. This will return either a [`User`] or [`Organization`]
    /// identifier.
    #[schema(read_only)]
    pub owner: Ulid,

    /// Name of the repository.
    pub name: Name,

    /// What kind of chart this repository is.
    #[serde(rename = "type")]
    pub type_: ChartType,

    /// Unique identifier of this entity.
    #[schema(read_only)]
    pub id: Ulid,
}

/// Represents a resource that contains a release from a [Repository] release. Releases
/// are a way to group releases of new versions of Helm charts that can be easily
/// fetched from the API server.
///
/// Any repository can have an unlimited amount of releases, but tags cannot clash
/// into each other, so the API server will not accept it. Each tag should be
/// a SemVer 2 comformant string, parsing is related to how Cargo evaluates SemVer 2 tags.
#[derive(Debug, Clone, Serialize, ToSchema, Queryable, Insertable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[diesel(table_name = charted_database::schema::postgresql::repository_releases)]
#[diesel(table_name = charted_database::schema::sqlite::repository_releases)]
pub struct RepositoryRelease {
    /// Markdown-formatted string that contains a changelog of this release.
    #[serde(default)]
    pub update_text: Option<String>,

    /// Repository that owns this release
    #[schema(read_only)]
    pub repository: Ulid,

    /// Date of when this release was registered to this instance
    #[schema(read_only)]
    pub created_at: DateTime,

    /// Date of when the server has last updated this repository release
    #[schema(read_only)]
    pub updated_at: DateTime,

    /// SemVer 2 comformant string that represents this tag.
    #[schema(read_only)]
    pub tag: Version,

    /// Unique identifier to locate this repository release resource from the API.
    #[schema(read_only)]
    pub id: Ulid,
}

macro_rules! create_member_struct {
    ($name:ident -> $table:ident) => {
        paste::paste! {
            #[doc = "Resource that correlates to a " $name:lower " member entity."]
            #[derive(Debug, Clone, Serialize, ToSchema, Queryable, Insertable)]
            #[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
            #[diesel(table_name = charted_database::schema::postgresql::$table)]
            #[diesel(table_name = charted_database::schema::sqlite::$table)]
            pub struct [<$name Member>] {
                /// Display name for this member. This should be formatted as '[{display_name}][Member::display_name] (@[{username}][User::username])' if this
                /// is set, otherwise '@[{username}][User::username]' is used.
                pub display_name: Option<String>,

                /// Bitfield value of this member's permissions.
                pub permissions: i64,

                /// Date-time of when this member resource was last updated by the API server.
                #[schema(read_only)]
                pub updated_at: DateTime,

                /// Date-time of when this member resource was created by the API server.
                #[schema(read_only)]
                pub joined_at: DateTime,

                /// [User] resource that this member is.
                #[schema(read_only)]
                pub account: Ulid,

                /// Unique identifier to locate this member with the API
                #[schema(read_only)]
                pub id: Ulid,
            }

            impl [<$name Member>] {
                #[doc = "Creates a [`MemberPermissions`][::charted_core::bitflags::MemberPermissions] for"]
                #[doc = "this " $name:lower " member."]
                pub fn bitfield(&self) -> ::charted_core::bitflags::MemberPermissions {
                    ::charted_core::bitflags::MemberPermissions::new(self.permissions.try_into().expect("cannot convert to u64"))
                }
            }
        }
    };
}

create_member_struct!(Repository -> repository_members);
create_member_struct!(Organization -> organization_members);

#[derive(Debug, Clone, Serialize, ToSchema, Queryable, Insertable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[diesel(table_name = charted_database::schema::postgresql::organizations)]
#[diesel(table_name = charted_database::schema::sqlite::organizations)]
pub struct Organization {
    /// Whether if this Organization is a Verified Publisher or not.
    #[serde(default)]
    #[schema(read_only)]
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
    #[schema(read_only)]
    pub created_at: DateTime,

    /// Date of when the server has last updated this organization
    #[schema(read_only)]
    pub updated_at: DateTime,

    /// Unique hash to locate an organization's icon, this also includes the extension that this icon is, i.e, `png`.
    #[serde(default)]
    pub icon_hash: Option<String>,

    /// Whether this organization is private and only its member can access this resource.
    #[serde(default)]
    pub private: bool,

    /// User ID that owns this organization
    #[schema(read_only)]
    pub owner: Ulid,

    /// The name for this organization.
    #[schema(read_only)]
    pub name: Name,

    /// Unique identifier to locate this organization with the API
    #[schema(read_only)]
    pub id: Ulid,
}

/// A resource for personal-managed API tokens that is created by a User. This is useful
/// for command line tools or scripts that need to interact with charted-server, but
/// the main use-case is for the [Helm plugin](https://charts.noelware.org/docs/helm-plugin/current).
#[derive(Debug, Clone, Serialize, ToSchema, Queryable, Insertable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[diesel(table_name = charted_database::schema::postgresql::api_keys)]
#[diesel(table_name = charted_database::schema::sqlite::api_keys)]
pub struct ApiKey {
    /// Short description about this API key.
    #[serde(default)]
    pub description: Option<String>,

    /// Date of when this API key was created
    #[schema(read_only, value_type = DateTime)]
    pub created_at: DateTime,

    /// Date of when the server has last updated this API key
    #[schema(read_only, value_type = DateTime)]
    pub updated_at: DateTime,

    /// Date-time of when this API token expires in, `null` can be returned
    /// if the token doesn't expire
    #[serde(default)]
    #[schema(read_only, value_type = DateTime)]
    pub expires_in: Option<DateTime>,

    /// The scopes that are attached to this API key resource.
    pub scopes: i64,

    /// The token itself. This is never revealed when querying, but only revealed
    /// when you create the token.
    #[serde(default)]
    #[schema(read_only)]
    pub token: Option<String>,

    /// User resource that owns this API key. This is skipped
    /// when using the API as this is pretty useless.
    #[schema(read_only)]
    pub owner: Ulid,

    /// The name of the API key.
    #[schema(read_only)]
    pub name: Name,

    /// Unique identifer to locate this resource in the API server.
    #[schema(read_only)]
    pub id: Ulid,
}

impl ApiKey {
    /// Returns a new [`Bitfield`], but the API key scopes are filled in
    pub fn bitfield(&self) -> ApiKeyScopes {
        ApiKeyScopes::new(self.scopes.try_into().unwrap())
    }
}
