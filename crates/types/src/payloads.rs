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

use crate::{DateTime, name::Name};
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

mk_payload_structs! {
    Repository;

    #[derive(Debug, Clone, Deserialize)]
    #[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
    create {}

    #[derive(Debug, Clone, Deserialize)]
    #[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
    patch {}
}

mk_payload_structs! {
    RepositoryRelease;

    #[derive(Debug, Clone, Deserialize)]
    #[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
    create {}

    #[derive(Debug, Clone, Deserialize)]
    #[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
    patch {}
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

// super::create_modifying_payload! {
// Organization;
//
// Request body payload for creating a new organization.
// #[derive(Debug, Clone, Deserialize, ToSchema)]
// create {
// Short description about this organization. If `description` was set to `null`, then
// this will not be updated, if `description` was a empty string, the `description`
// will be set to a empty string and will present as "*no description for this
// organization*" in Hoshi.
// #[serde(default)]
// pub description: Option<String>,
//
// Display name for this organization.
// #[serde(default)]
// pub display_name: Option<String>,
//
// Whether if the organization is private or not.
// #[serde(default)]
// pub private: bool,
//
// Organization name.
// pub name: Name,
// }
//
// Request body payload for patching an organization's metadata.
// #[derive(Debug, Clone, Deserialize, ToSchema)]
// patch {
// Adds or removes a Twitter handle from this organization's metadata.
//
// * If this is `null`, this will not do any patching
// * If this is a empty string, this will act as "removing" it from the metadata
// * If the comparsion (`old.twitter_handle == twitter_handle`) is false, then this will
//   update it.
// #[serde(default)]
// pub twitter_handle: Option<String>,
//
// Optional field to update this organization's gravatar email. If this organization
// doesn't have an avatar that is used or prefers not to use their previously uploaded
// avatars and they set their Gravatar email, their Gravatar will be used.
// #[serde(default)]
// pub gravatar_email: Option<String>,
//
// Display name for this organization.
// #[serde(default)]
// pub display_name: Option<String>,
//
// Whether if the organization is private or not.
// #[serde(default)]
// pub private: Option<bool>,
//
// Organization name to rename to.
// #[serde(default)]
// pub name: Option<Name>,
// }
// }
//
// use crate::{helm::ChartType, name::Name};
// use serde::Deserialize;
// use utoipa::ToSchema;
//
// super::create_modifying_payload! {
// Repository;
//
// Request body payload for creating a repository.
// #[derive(Debug, Clone, Deserialize, ToSchema)]
// create {
// Short description about this repository.
// #[serde(default)]
// pub description: Option<String>,
//
// Whether if this repository is private.
// #[serde(default)]
// pub private: bool,
//
// The contents of the README that will be displayed on the repository. If you're
// using charted's official Helm plugin, new releases can update its README and it'll
// be reflected.
//
// This should be valid Markdown, but XSS cross scripting is impossible as scripts
// in codeblocks or via `<script>` won't be executed.
//
// You can retrieve a repository's README (if it is public or if you have access) with
// the [`GET /cdn`](https://charts.noelware.org/docs/server/latest/api/cdn#GET-{...params}) REST
// endpoint if the instance has the CDN feature enabled. It'll be under
// `/repositories/{id}/README.md`. #[serde(default)]
// pub readme: Option<String>,
//
// #[serde(skip)]
// pub creator: i64,
//
// [`Name`] to attach to this repository.
// pub name: Name,
//
// Type of chart this represents. When serializing to valid Helm objects,
// `operator` will be replaced with `application`.
// #[serde(default, rename = "type")]
// pub r#type: ChartType,
// }
//
// Request body payload for patching a repository's metadata.
// #[derive(Debug, Clone, Deserialize, ToSchema)]
// patch {
// Short description about this repository. If `description` was set to `null`, then
// this will not be updated, if `description` was a empty string, the `description`
// will be set to a empty string and will present as "*no description for this
// repository*" in Hoshi.
// #[serde(default)]
// pub description: Option<String>,
//
// Whether if this repository is private. This cannot be set to the actual value
// that it was previously.
// #[serde(default)]
// pub private: Option<bool>,
//
// The contents of the README that will be displayed on the repository. If you're
// using charted's official Helm plugin, new releases can update its README and it'll
// be reflected.
//
// This should be valid Markdown, but XSS cross scripting is impossible as scripts
// in codeblocks or via `<script>` won't be executed.
//
// You can retrieve a repository's README (if it is public or if you have access) with
// the [`GET /cdn`](https://charts.noelware.org/docs/server/latest/api/cdn#GET-{...params}) REST
// endpoint if the instance has the CDN feature enabled. It'll be under
// `/repositories/{id}/README.md`. #[serde(default)]
// pub readme: Option<String>,
//
// [`Name`] to update towards, this will not update if it is
// the same.
// #[schema(value_type = Name)]
// pub name: Option<Name>,
//
// Type of chart this represents. When serializing to valid Helm objects,
// `operator` will be replaced with `application`.
// #[serde(default, rename = "type")]
// pub r#type: Option<ChartType>,
// }
// }
//
// super::create_modifying_payload! {
// User;
//
// Request body payload for creating a user.
// #[derive(Debug, Clone, Deserialize, ToSchema)]
// create {
// User handle to use to identify yourself.
// pub username: Name,
//
// The password to use when authenticating, this is optional on non-local sessions.
// #[schema(pattern = "^.*(?=.{8,})(?=.*[a-zA-Z])(?=.*\\d)?(?=.*[!#$%&? \"])?.*$")]
// pub password: Option<String>,
//
// Email address to identify this user
// pub email: String,
// }
//
// Request body payload for modifying a user's metadata
// #[derive(Debug, Clone, Deserialize, ToSchema)]
// patch {
// Determines if the user avatar should use their Gravatar email.
// #[serde(default)]
// pub prefers_gravatar: Option<bool>,
//
// Updates the email address to fetch the Gravatar email from.
// pub gravatar_email: Option<String>,
//
// Short description about this user.
// pub description: Option<String>,
//
// Updates this user's username.
// pub username: Option<Name>,
//
// Updates this user's password, if the session manager configured allows it.
// #[schema(pattern = "^.*(?=.{8,})(?=.*[a-zA-Z])(?=.*\\d)?(?=.*[!#$%&? \"])?.*$")]
// pub password: Option<String>,
//
// Updates this user's email.
// pub email: Option<String>,
//
// Updates this user's display name.
// pub name: Option<String>,
// }
// }
//
// super::super::create_modifying_payload! {
// RepositoryRelease;
//
// Request body payload for creating a release for a repository.
// #[derive(Debug, Clone, Deserialize, ToSchema)]
// create {
// Represents a changelog (that can be Markdown or HTML (it'll remove XSS
// vulnerabilities)) that will appear via `/repositories/:id/releases/:version/changelog`.
//
// > [!NOTE]
// > Hoshi will wrap `CHANGELOG.html` to the same styles as when rendering Markdown.
// #[serde(default)]
// pub update_text: Option<String>,
//
// SemVer-based [`Version`] to indicate what version this release is. This is an immutable
// tag and can't be patched without conflicting; you can only delete a release by its ID
// or version, which will remove this tag and can be freely used.
// pub tag: Version,
// }
//
// Request body payload for updating a repository release's metadata.
// #[derive(Debug, Clone, Deserialize, ToSchema)]
// patch {
// Updates the changelog for the repository release that people can view from
// the API server or from [Hoshi]
//
// * If this is `null`, this will not attempt to do anything as this is the default state.
// * If this is just `""`, then this is considered as a removal and won't be available for
//   people to access.
// * If this is not an empty string, it will overwrite the previous value.
//
// [Hoshi]: https://charts.noelware.org/docs/hoshi/latest
// #[serde(default)]
// pub update_text: Option<String>,
// }
// }
