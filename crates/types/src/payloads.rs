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

//! Types that can effictively create or patch a object's metadata. Used by
//! the API server for the `PUT` and `PATCH` REST endpoints.

use crate::{ChartType, DateTime, Version, name::Name};
use charted_core::bitflags::ApiKeyScope;
use serde::Deserialize;

macro_rules! mk_payload_structs {
    (
        $name:ident;

        $(#[$create:meta])*
        create {
            $(
                $(#[$create_field_meta:meta])*
                $create_vis:vis $create_field:ident: $create_ty:ty,
            )*
        }

        $(#[$patch:meta])*
        patch {
            $(
                $(#[$patch_field_meta:meta])*
                $patch_vis:vis $patch_field:ident: $patch_ty:ty,
            )*
        }
    ) => {
        $crate::__private::paste! {
            $(#[$create])*
            pub struct [<Create $name Payload>] {
                $(
                    $(#[$create_field_meta])*
                    $create_vis $create_field: $create_ty,
                )*
            }

            $(#[$patch])*
            pub struct [<Patch $name Payload>] {
                $(
                    $(#[$patch_field_meta])*
                    $patch_vis $patch_field: $patch_ty,
                )*
            }
        }
    };
}

mk_payload_structs! {
    ApiKey;

    /// Payload object for creating a API key.
    #[derive(Debug, Clone, Deserialize)]
    #[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
    create {
        /// the api key's display name.
        #[serde(default)]
        pub display_name: Option<String>,

        /// short and concise description about this api key.
        #[serde(default)]
        pub description: Option<String>,

        /// datetime of when this api key should be deleted from the server
        /// and can no longer be used.
        #[serde(default)]
        #[cfg_attr(feature = "openapi", schema(read_only))]
        pub expires_in: Option<DateTime>,

        /// the list of permissions that this api key has as a [bitfield] data structure.
        ///
        /// [bitfield]: https://charts.noelware.org/docs/server/latest/api/reference#bitfield-data-structure
        #[serde(default)]
        pub scopes: i64,

        /// the name of the api key
        pub name: Name,
    }

    /// Payload object for patching the metadata of a API key.
    #[derive(Debug, Clone, Deserialize)]
    #[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
    patch {
        /// changes the api key's display name.
        ///
        /// - `null` or empty: field will not be updated
        /// - an empty string: field is set to nothing
        /// - string that is different: field will update
        /// - string that is the same: field will not update
        #[serde(default)]
        pub display_name: Option<String>,

        /// changes the api key's display name.
        ///
        /// - `null` or empty: field will not be updated
        /// - an empty string: field is set to nothing
        /// - string that is different: field will update
        /// - string that is the same: field will not update
        #[serde(default)]
        pub description: Option<String>,

        /// changes the permissions of this api key.
        #[serde(default)]
        pub scopes: Option<Vec<ApiKeyScope>>,

        /// changes the api key's name.
        ///
        /// if the name of the api key already conflicts with another
        /// key, then a 409 Conflict HTTP response is sent instead.
        ///
        /// - `null` or empty: field will not be updated
        /// - an empty string: field is set to nothing
        /// - string that is different: field will update
        /// - string that is the same: field will not update
        #[serde(default)]
        pub name: Option<Name>,
    }
}

mk_payload_structs! {
    User;

    #[derive(Debug, Clone, Deserialize)]
    #[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
    create {
        /// User handle to use to identify yourself.
        pub username: Name,

        /// The password to use when authenticating, this is optional on non-local sessions.
        #[cfg_attr(feature = "openapi", schema(pattern = "^.*(?=.{8,})(?=.*[a-zA-Z])(?=.*\\d)?(?=.*[!#$%&? \"])?.*$"))]
        pub password: Option<String>,

        /// Email address to identify this user
        pub email: String,
    }

    #[derive(Debug, Clone, Deserialize)]
    #[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
    patch {
        /// Toggle to use when preferring the Gravatar avatar
        /// over the ones used by the API server locally.
        ///
        /// - `null` or empty: field will not be updated
        #[serde(default)]
        pub prefers_gravatar: Option<bool>,

        /// Changes the Gravatar email address associated
        /// for this user.
        ///
        /// - `null` or empty: field will not be updated
        /// - an empty string: field is set to nothing
        /// - string that is different: field will update
        /// - string that is the same: field will not update
        #[serde(default)]
        pub gravatar_email: Option<String>,

        /// Changes the description about yourself.
        ///
        /// - `null` or empty: field will not be updated
        /// - an empty string: field is set to nothing
        /// - string that is different: field will update
        /// - string that is the same: field will not update
        #[serde(default)]
        pub description: Option<String>,

        /// changes your username.
        ///
        /// if any user has the username already taken, a 409 Conflict
        /// HTTP response is sent.
        ///
        /// - `null` or empty: field will not be updated
        /// - an empty string: field is set to nothing
        /// - string that is different: field will update
        /// - string that is the same: field will not update
        #[serde(default)]
        pub username: Option<Name>,

        /// Updates this user's password, if the session backend is allowed to do so.
        #[serde(default)]
        #[cfg_attr(feature = "openapi", schema(pattern = "^.*(?=.{8,})(?=.*[a-zA-Z])(?=.*\\d)?(?=.*[!#$%&? \"])?.*$"))]
        pub password: Option<String>,

        /// Updates this user's email.
        #[serde(default)]
        pub email: Option<String>,

        /// Updates this user's display name.
        pub name: Option<String>,
    }
}

/// Login representation, fields are mutually exclusive.
#[derive(Debug, Clone, Deserialize, derive_more::Display)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "lowercase")]
pub enum Login {
    /// Logs into the registry via their username ahead-of-time.
    Username(Name),

    /// Logs into the registry via their registered email address.
    Email(String),
}

/// Request body for creating a session.
#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct UserLoginPayload {
    #[serde(flatten)]
    pub login: Login,

    /// password to login as.
    pub password: String,
}

mk_payload_structs! {
    Repository;

    /// Request body for creating a repository.
    #[derive(Debug, Clone, Deserialize)]
    #[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
    create {
        /// a short description about this repository
        #[serde(default)]
        #[cfg_attr(feature = "openapi", schema(maximum = 140))]
        pub description: Option<String>,

        /// whether if this repository is private and only the owner
        /// or the repository members (if the feature flag is enabled)
        /// can view, download, create, update, etc.
        #[serde(default)]
        pub private: bool,

        /// the contents of the **README.md** file of this repository.
        ///
        /// for clients or web uis (like [Hoshi]) might display the contents
        /// for users to see about the chart itself.
        ///
        /// While it can be either HTML or Markdown, the client is responsible for
        /// sanitization. the server does minimal sanitization if the content
        /// is pure HTML.
        ///
        /// [Hoshi]: https://charts.noelware.org/docs/hoshi/latest
        #[serde(default)]
        #[cfg_attr(feature = "openapi", schema(maximum = 16382))] // 16kib should be fine?
        pub readme: Option<String>,

        /// name of this repository.
        pub name: Name,

        /// chart type.
        ///
        /// this is not overridable when a new chart is published.
        #[serde(default, rename = "type")]
        pub ty: ChartType,
    }

    /// Request body for modifying a repository.
    #[derive(Debug, Clone, Deserialize)]
    #[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
    patch {
        /// a short description about this repository
        #[serde(default)]
        #[cfg_attr(feature = "openapi", schema(maximum = 140))]
        pub description: Option<String>,

        /// whether if this repository is private and only the owner
        /// or the repository members (if the feature flag is enabled)
        /// can view, download, create, update, etc.
        #[serde(default)]
        pub private: Option<bool>,

        /// the contents of the **README.md** file of this repository.
        ///
        /// for clients or web uis (like [Hoshi]) might display the contents
        /// for users to see about the chart itself.
        ///
        /// While it can be either HTML or Markdown, the client is responsible for
        /// sanitization. the server does minimal sanitization if the content
        /// is pure HTML.
        ///
        /// [Hoshi]: https://charts.noelware.org/docs/hoshi/latest
        #[serde(default)]
        #[cfg_attr(feature = "openapi", schema(maximum = 16382))] // 16kib should be fine?
        pub readme: Option<String>,

        /// changes the name of this repository if provided.
        pub name: Option<Name>,

        /// chart type.
        ///
        /// this is not overridable when a new chart is published.
        #[serde(default, rename = "type")]
        pub ty: Option<ChartType>,
    }
}

mk_payload_structs! {
    RepositoryRelease;

    #[derive(Debug, Clone, Deserialize)]
    #[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
    create {
        /// changelog of this release, can be rendered as HTML or Markdown.
        ///
        /// for clients or web uis (like [Hoshi]) might display the contents
        /// for a changelog viewer.
        ///
        /// While it can be either HTML or Markdown, the client is responsible for
        /// sanitization. the server does minimal sanitization if the content
        /// is pure HTML.
        ///
        /// [Hoshi]: https://charts.noelware.org/docs/hoshi/latest
        #[serde(default)]
        pub update_text: Option<String>,

        /// title of this release.
        #[serde(default)]
        pub title: Option<String>,

        /// SemVer-based [`Version`] to indicate what version this release is.
        ///
        /// This is an immutable tag and can't be patched without conflicts; you can only delete a
        /// release by its ID or version, which will remove this tag and can be freely used.
        pub tag: Version,
    }

    #[derive(Debug, Clone, Deserialize)]
    #[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
    patch {
        /// updates the changelog of this release, can be rendered as HTML or Markdown.
        #[serde(default)]
        pub update_text: Option<String>,

        /// changes the title of this release.
        #[serde(default)]
        pub title: Option<String>,
    }
}

mk_payload_structs! {
    Organization;

    #[derive(Debug, Clone, Deserialize)]
    #[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
    create {}

    #[derive(Debug, Clone, Deserialize)]
    #[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
    patch {}
}

mk_payload_structs! {
    Member;

    #[derive(Debug, Clone, Deserialize)]
    #[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
    create {}

    #[derive(Debug, Clone, Deserialize)]
    #[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
    patch {}
}
