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

use crate::{ChartType, DateTime, Ulid, Version, name::Name};
use charted_core::bitflags::ApiKeyScopes;
use serde::Serialize;

/// The baseline entity.
///
/// Users can manage and create repositories and organizations, be apart
/// of repository & organizations and much more.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct User {
    /// Determines whether if this user is a verified publisher or not.
    #[cfg_attr(feature = "openapi", schema(read_only))]
    #[serde(default)]
    pub verified_publisher: bool,

    /// Determines whether or not if this user prefers to use their
    /// Gravatar email associated as their profile picture.
    #[serde(default)]
    pub prefers_gravatar: bool,

    /// Valid email address that points to their Gravatar account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gravatar_email: Option<String>,

    /// Short and concise description about this user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Unique hash by the API server to identify their avatar, if they have uploaded one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_hash: Option<String>,

    /// datetime of when this user was created
    #[cfg_attr(feature = "openapi", schema(read_only))]
    pub created_at: DateTime,

    /// datetime of when this user was last updated
    #[cfg_attr(feature = "openapi", schema(read_only))]
    pub updated_at: DateTime,

    /// the user's username
    pub username: Name,

    /// whether if this user is an administrator of this instance
    #[serde(default)]
    #[cfg_attr(feature = "openapi", schema(read_only))]
    pub admin: bool,

    /// the user's display name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// the user's unique identifier
    #[cfg_attr(feature = "openapi", schema(read_only))]
    pub id: Ulid,
}

/// Connections that a [`User`] is connected to.
///
/// This allows OIDC implementations of charted's authz system to lookup
/// a user by a unique identifier so the flow is easier.
///
/// ## Supported Providers
/// - [Noelware](https://account.noelware.org)
/// - [Google](https://google.com)
/// - [GitHub](https://github.com)
/// - [GitLab](https://gitlab.com)
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct UserConnections {
    /// Account ID (formatted as a [`Ulid`]) that is from [Noelware](https://account.noelware.org).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub noelware_account_id: Option<Ulid>,

    /// Account ID that is from [Google](https://google.com)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub google_account_id: Option<String>,

    /// Account ID that is from [GitHub](https://github.com)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub github_account_id: Option<String>,

    /// Account ID that is from [GitLab](https://gitlab.com)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gitlab_account_id: Option<String>,

    /// datetime of when this object was created.
    ///
    /// this should be in range of when the [`User`] was created, but
    /// it is not 100% a guarantee.
    #[cfg_attr(feature = "openapi", schema(read_only))]
    pub created_at: DateTime,

    /// datetime of when this object was last modified.
    #[cfg_attr(feature = "openapi", schema(read_only))]
    pub updated_at: DateTime,

    /// the object's unique identifier
    pub id: Ulid,
}

/// A **Helm** chart that can be associated by a [`User`] or [`Organization`].
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct Repository {
    /// Short and concise description about this repository.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// whether or not if this repository is marked **deprecated**.
    #[serde(default)]
    pub deprecated: bool,

    /// datetime of when this repository was created.
    #[cfg_attr(feature = "openapi", schema(read_only))]
    pub created_at: DateTime,

    /// datetime of when this repository was last modified.
    #[cfg_attr(feature = "openapi", schema(read_only))]
    pub updated_at: DateTime,

    /// unique icon hash for the repository generated by the API server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_hash: Option<String>,

    /// The creator of the repository.
    ///
    /// This field was added to determine if this is a organization
    /// or user repository without having another db enumeration
    /// to manage.
    ///
    /// This can be `null` if this is a user repository as the [`owner`]
    /// field will be set to the user that created the repository. This
    /// is non-null to the organization member that created the repository as
    /// the [`owner`] field will always be the organization.
    ///
    /// [`owner`]: #
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "openapi", schema(read_only))]
    pub creator: Option<Ulid>,

    /// whether or not if this repository is marked **private**.
    #[serde(default)]
    pub private: bool,

    /// the [`User`] account or [`Organization`] that created this repository.
    #[cfg_attr(feature = "openapi", schema(read_only))]
    pub owner: Ulid,

    /// representation of this repository.
    ///
    /// Repositories can also be considered as a library chart and can be
    /// pulled from a user or organization's Helm chart index.
    #[serde(rename = "type")]
    pub type_: ChartType,

    /// the name of this repository.
    pub name: Name,

    /// the repository's unique identifier.
    #[cfg_attr(feature = "openapi", schema(read_only))]
    pub id: Ulid,
}

/// Resource that contains a [`Repository`] release.
///
/// **Releases** are a way to group new releases of Helm charts that can
/// be easily used and fetched from the API server.
///
/// Any [`Repository`] can have a number of releases but release tags
/// cannot clash between one and another. All release tags must comply
/// to the [SemVer v2] format.
///
/// [SemVer v2]: https://semver.org/
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct RepositoryRelease {
    /// The "changelog" or update text of this release.
    ///
    /// This can be formatted into Markdown and applications like
    /// [Hoshi] can render the Markdown into HTML.
    ///
    /// [Hoshi]: https://charts.noelware.org/docs/hoshi/latest
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_text: Option<String>,

    /// [`Repository`] that owns this release.
    #[cfg_attr(feature = "openapi", schema(read_only))]
    pub repository: Ulid,

    /// datetime of when this release was created.
    #[cfg_attr(feature = "openapi", schema(read_only))]
    pub created_at: DateTime,

    /// datetime of when this release was last modified.
    #[cfg_attr(feature = "openapi", schema(read_only))]
    pub updated_at: DateTime,

    /// whether or not if this release was yanked.
    #[serde(default)]
    pub yanked: bool,

    /// the title of this release.
    ///
    /// If no title is provided, then consumers can place their own title. For
    /// [Hoshi], this will render to "Release [`{tag}`]"
    ///
    /// [`{tag}`]: #
    /// [Hoshi]: https://charts.noelware.org/docs/hoshi/latest
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// the release tag.
    #[cfg_attr(feature = "openapi", schema(read_only))]
    pub tag: Version,

    /// the release's unique identifier.
    #[cfg_attr(feature = "openapi", schema(read_only))]
    pub id: Ulid,
}

/// An organization is a shared resource for users to build, test, and push Helm charts.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct Organization {
    /// whether if this organization is a verified publisher
    #[serde(default)]
    pub verified_publisher: bool,

    /// whether if this organization prefers their Gravatar email as their icon
    #[serde(default)]
    pub prefers_gravatar: bool,

    /// valid email address that points to their Gravatar account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gravatar_email: Option<String>,

    /// the organization's display name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// datetime of when this organization was created.
    #[cfg_attr(feature = "openapi", schema(read_only))]
    pub created_at: DateTime,

    /// datetime of when this organization was last modified.
    #[cfg_attr(feature = "openapi", schema(read_only))]
    pub updated_at: DateTime,

    /// a unique hash generated by the API server to the organization's icon.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "openapi", schema(read_only))]
    pub icon_hash: Option<String>,

    /// whether if this organization is marked private
    #[serde(default)]
    pub private: bool,

    /// reference to the owner of this organization
    pub owner: Ulid,

    /// the organization's name
    pub name: Name,

    /// the organization's unique identifier.
    pub id: Ulid,
}

/// Resource for personal-managed API tokens that is created by a [`User`].
///
/// User API keys are useful for command-line tools or scripts that might need
/// to interact with the API server.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct ApiKey {
    /// the api key's display name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// short and concise description about this api key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// datetime of when this api key was created.
    #[cfg_attr(feature = "openapi", schema(read_only))]
    pub created_at: DateTime,

    /// datetime of when this api key was last modified.
    #[cfg_attr(feature = "openapi", schema(read_only))]
    pub updated_at: DateTime,

    /// datetime of when this api key should be deleted from the server
    /// and can no longer be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "openapi", schema(read_only))]
    pub expires_in: Option<DateTime>,

    /// the list of permissions that this api key has as a [bitfield] data structure.
    ///
    /// [bitfield]: https://charts.noelware.org/docs/server/latest/api/reference#bitfield-data-structure
    #[serde(default)]
    pub scopes: i64,

    /// reference to the [`User`] that owns this api key
    pub owner: Ulid,

    /// the name of the api key
    pub name: Name,

    /// the api key's unique identifier.
    pub id: Ulid,
}

impl ApiKey {
    /// Returns a new [`Bitfield`][charted_core::bitflags::Bitfield] of the
    /// avaliable scopes.
    pub fn bitfield(&self) -> ApiKeyScopes {
        ApiKeyScopes::new(self.scopes.try_into().unwrap())
    }
}

/// Resource that represents a [`User`] session.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct Session {
    /// A token that is used to refresh this session via the [`GET
    /// /users/@me/sessions/refresh`] REST endpoint.
    ///
    /// When this session was refreshed, the session is still alive and can still be used
    /// but both the [`refresh_token`] and [`access_token`] fields are different values.
    ///
    /// [`GET /users/@me/sessions/refresh`]: #
    /// [`refresh_token`]: #
    /// [`access_token`]: #
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "openapi", schema(read_only))]
    pub refresh_token: Option<String>,

    /// The token that is used to send API requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "openapi", schema(read_only))]
    pub access_token: Option<String>,

    /// reference to the [`User`] that owns this api key
    pub owner: Ulid,

    /// the api key's unique identifier.
    pub id: Ulid,
}

impl Session {
    /// Sanitizes and removes the `refresh_token` and `access_token` fields.
    ///
    /// Since the `From` implementation from database queries sets the `refresh_token`
    /// and `access_token`, we will need to sanitize the input.
    pub fn sanitize(self) -> Session {
        Session {
            refresh_token: None,
            access_token: None,
            ..self
        }
    }
}

macro_rules! mk_member_struct {
    ($name:ident) => {
        $crate::__private::paste! {
            #[doc = "Resource that correlates to a " $name:lower " member."]
            #[derive(Debug, Clone, Serialize)]
            #[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
            pub struct [<$name Member>] {
                /// the member's display name.
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub display_name: Option<String>,

                /// the permissions that this member has as a [bitfield] data structure.
                ///
                /// [bitfield]: https://charts.noelware.org/docs/server/latest/api/reference#bitfield-data-structure
                #[serde(default)]
                pub permissions: u64,

                /// datetime of when this release was last modified.
                #[cfg_attr(feature = "openapi", schema(read_only))]
                pub updated_at: DateTime,

                #[doc = "datetime of when this member joined this " $name:lower "."]
                #[cfg_attr(feature = "openapi", schema(read_only))]
                pub joined_at: DateTime,

                /// reference to their user account.
                #[cfg_attr(feature = "openapi", schema(read_only))]
                pub account: Ulid,

                /// the member's unique identifier.
                #[cfg_attr(feature = "openapi", schema(read_only))]
                pub id: Ulid,
            }

            impl [<$name Member>] {
                pub fn bitfield(&self) -> ::charted_core::bitflags::MemberPermissions {
                    let perms_as_u64: u64 = self.permissions.try_into().unwrap();
                    ::charted_core::bitflags::MemberPermissions::new(
                        perms_as_u64
                    )
                }
            }
        }
    };
}

mk_member_struct!(Repository);
mk_member_struct!(Organization);
