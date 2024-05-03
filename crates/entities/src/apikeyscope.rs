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

#![allow(deprecated)]

use crate::gen_rbac_enum;
use charted_common::Bitfield;
use utoipa::{
    openapi::{ObjectBuilder, OneOfBuilder, RefOr, Schema, SchemaFormat, SchemaType},
    ToSchema,
};

gen_rbac_enum!(
    /// Represents a RBAC-based enum for locking certain API server endpoints
    /// when using API keys as the main authentication method.
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    #[allow(clippy::enum_clike_unportable_variant)] // we don't provide support for 32bit systems
    #[repr(u64)]
    pub ApiKeyScope {
        // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
        //           User Scopes
        // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
        /// Allows the current authenticated user to access metadata about themselves.
        UserAccess["user:access"]: 1 << 0;

        /// Allows the authenticated user to patch any user metadata.
        UserUpdate["user:update"]: 1 << 1;

        /// Allows to delete the current authenticated user.
        UserDelete["user:delete"]: 1 << 2;

        /// Allows the current authenticated user to access their connections like their
        /// GitHub or GitLab accounts, this is usually for OIDC providers that can query
        /// a user by an identifier that can identify a user into charted-server.
        ///
        /// As of v0.1.0-beta, this scope is not used at all.
        UserConnections["user:connections"]: 1 << 3;

        /// **UNUSED AS OF v0.1.0-beta**
        ///
        /// Allows the current authenticated user to read from their notifications.
        #[allow(unused)]
        UserNotifications["user:notifications"]: 1 << 4;

        /// Allows the current authenticated user to update their user avatar.
        UserAvatarUpdate["user:avatar:update"]: 1 << 5;

        /// Allows the current authenticated user to list their current sessions.
        UserSessionsList["user:sessions:list"]: 1 << 6;

        // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
        //        Repository Scopes
        // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
        /// Allows access through private repositories that the current authenticated user
        /// can access, except repositories that they are a member apart of.
        RepoAccess["repo:access"]: 1 << 7;

        /// Allows the creation of public or private repositories that will be owned
        /// by the current authenticated user.
        RepoCreate["repo:create"]: 1 << 8;

        /// Allows the deletion of public or private repositories that will be owned
        /// by the current authenticated user.
        RepoDelete["repo:delete"]: 1 << 9;

        /// Allows patching a public or private repository that is owned by the current
        /// authenticated user.
        RepoUpdate["repo:update"]: 1 << 10;

        /// **DEPRECATED AS OF v0.1.0-beta**
        ///
        /// This is replaced by the `repo:releases:create` API key scope, please
        /// use that instead.
        #[deprecated(since = "0.1.0-beta", note = "Replaced by ApiKeyScope::RepoReleaseCreate")]
        RepoWrite["repo:write"]: 1 << 11;

        /// Allows patching a repository icon that the current authenticated user owns.
        RepoIconUpdate["repo:icon:update"]: 1 << 12;

        /// Allows the creation of creating repository releases that the current authenticated
        /// user owns.
        RepoReleaseCreate["repo:releases:create"]: 1 << 13;

        /// Allows patching repository releases that the current authenticated user owns.
        RepoReleaseUpdate["repo:releases:update"]: 1 << 14;

        /// Allows the deletion of repository releases that the current authenticated user owns.
        RepoReleaseDelete["repo:releases:delete"]: 1 << 15;

        /// Allows viewing all repository members.
        RepoMembersList["repo:members:list"]: 1 << 16;

        /// Allows patching repository member metadata.
        RepoMemberUpdate["repo:members:update"]: 1 << 17;

        /// Allows kicking repository members off the repository.
        RepoMemberKick["repo:members:kick"]: 1 << 18;

        /// Allows viewing all repository member invites. This scope is only used if the
        /// [`charted-emails`](https://github.com/charted-dev/emails) gRPC server is running
        /// and configured via the [`config.emails_grpc_endpoint`][emails_grpc_endpoint]
        /// configuration key.
        ///
        /// [emails_grpc_endpoint]: https://charts.noelware.org/docs/server/latest/self-hosting/configuration#emails_grpc_endpoint
        RepoMemberInviteAccess["repo:members:invites:access"]: 1 << 19;

        /// **DEPRECTEAD AS OF v0.1.0-beta**: This is unused.
        #[deprecated(since = "0.1.0-beta", note = "This is unused.")]
        RepoMemberInviteUpdate["repo:members:invites:update"]: 1 << 20;
        RepoMemberInviteDelete["repo:members:invites:delete"]: 1 << 21;
        RepoWebhookList["repo:webhooks:list"]: 1 << 22;
        RepoWebhookCreate["repo:webhooks:create"]: 1 << 23;
        RepoWebhookUpdate["repo:webhooks:update"]: 1 << 24;
        RepoWebhookDelete["repo:webhooks:delete"]: 1 << 25;
        RepoWebhookEventAccess["repo:webhooks:events:access"]: 1 << 26;
        RepoWebhookEventDelete["repo:webhooks:events:delete"]: 1 << 27;

        // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
        //        API Key Scopes
        // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
        ApiKeyView["apikeys:view"]: 1 << 28;
        ApiKeyList["apikeys:list"]: 1u64 << 52u64;
        ApiKeyCreate["apikeys:create"]: 1 << 29;
        ApiKeyDelete["apikeys:delete"]: 1 << 30;
        ApiKeyUpdate["apikeys:update"]: 1 << 31;

        // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
        //      Organization Scopes
        // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
        OrgAccess["org:access"]: 1u64 << 32u64;
        OrgCreate["org:create"]: 1u64 << 33u64;
        OrgUpdate["org:update"]: 1u64 << 34u64;
        OrgDelete["org:delete"]: 1u64 << 35u64;
        OrgMemberInvites["org:members:invites"]: 1u64 << 36u64;
        OrgMemberList["org:members:list"]: 1u64 << 37u64;
        OrgMemberKick["org:members:kick"]: 1u64 << 38u64;
        OrgMemberUpdate["org:members:update"]: 1u64 << 39u64;
        OrgWebhookList["org:webhooks:list"]: 1u64 << 40u64;
        OrgWebhookCreate["org:webhooks:create"]: 1u64 << 41u64;
        OrgWebhookUpdate["org:webhooks:update"]: 1u64 << 42u64;
        OrgWebhookDelete["org:webhooks:delete"]: 1u64 << 43u64;
        OrgWebhookEventList["org:webhooks:events:list"]: 1u64 << 44u64;
        OrgWebhookEventDelete["org:webhooks:events:delete"]: 1u64 << 45u64;

        // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
        //    Administration Scopes
        // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
        AdminStats["admin:stats"]: 1u64 << 46u64;
        AdminUserCreate["admin:users:create"]: 1u64 << 47u64;
        AdminUserDelete["admin:users:delete"]: 1u64 << 48u64;
        AdminUserUpdate["admin:users:update"]: 1u64 << 49u64;
        AdminOrgDelete["admin:orgs:delete"]: 1u64 << 50u64;
        AdminOrgUpdate["admin:orgs:update"]: 1u64 << 51u64;
    }
);

impl<'s> ToSchema<'s> for ApiKeyScope {
    fn schema() -> (&'s str, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>) {
        let oneof = OneOfBuilder::new()
            .description(Some("Describes a role-based enumeration of what a API key can do. This can be represented as an unsigned 64bit integer or a string that represents what scope it functions as"))
            .item(
                Schema::Object(
                    ObjectBuilder::new()
                        .schema_type(SchemaType::Number)
                        .format(Some(SchemaFormat::Custom(String::from("int64"))))
                        .minimum(Some(1f64))
                        .maximum(Some(ApiKeyScope::max() as f64))
                        .description(Some(format!(
                            "Represents the raw unsigned 64bit integer that this scope represents as far the API server is aware of. Scopes can be in a range of 1..{} and can't exceed or go below over it",
                            ApiKeyScope::max()
                        )))
                        .build()
                )
            )
            .item(
                Schema::Object(
                    ObjectBuilder::new()
                        .schema_type(SchemaType::String)
                        .enum_values(Some(ApiKeyScope::values_str().to_vec()))
                        .description(Some("Represents a named, human-readable version of the scope that is always represented as 'entity:operation', i.e, `apikey:list` will allow the API key to list all API keys"))
                        .build()
                )
            ).build();

        ("ApiKeyScope", RefOr::T(Schema::OneOf(oneof)))
    }
}

/// Represents a [`Bitfield`] for managing [`ApiKeyScope`]s.
#[derive(Debug, Clone)]
pub struct ApiKeyScopes(Bitfield);
impl Default for ApiKeyScopes {
    fn default() -> Self {
        Self::init(0)
    }
}

impl ApiKeyScopes {
    /// Initializes a possible empty [`MemberPermissions`] bitfield with the given bits.
    pub fn init(bits: u64) -> ApiKeyScopes {
        let map = ApiKeyScope::as_map();
        let bitfield = Bitfield::with_flags(map.iter().map(|(k, v)| (*k, (*v) as u64)).collect());

        ApiKeyScopes(bitfield.with_value(bits))
    }

    /// Initialize a [`ApiKeyScope`] bitfield with an iterator of [`ApiKeyScope`]s.
    pub fn with_iter<I: IntoIterator<Item = ApiKeyScope>>(iter: I) -> ApiKeyScopes {
        let mut bitfield = Bitfield::new(
            0,
            ApiKeyScope::as_map()
                .iter()
                .map(|(key, val)| (*key, (*val) as u64))
                .collect(),
        );

        bitfield.add(iter.into_iter().map(|x| x as u64));

        ApiKeyScopes(bitfield)
    }
}

impl std::ops::Deref for ApiKeyScopes {
    type Target = Bitfield;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ApiKeyScopes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
