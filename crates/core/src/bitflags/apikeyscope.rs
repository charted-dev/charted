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

pub type ApiKeyScopes = crate::bitflags::Bitfield<ApiKeyScope>;

crate::bitflags! {
    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    #[allow(clippy::enum_clike_unportable_variant)]
    #[repr(u64)]
    pub ApiKeyScope[u64] {
        // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
        //           User Scopes
        // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
        /// Allows the current authenticated user to access metadata about themselves.
        UserAccess["user:access"]: 1u64 << 0u64;

        /// Allows the authenticated user to patch any user metadata.
        UserUpdate["user:update"]: 1u64 << 1u64;

        /// Allows to delete the current authenticated user.
        UserDelete["user:delete"]: 1u64 << 2u64;

        /// Allows the current authenticated user to access their connections like their
        /// GitHub or GitLab accounts, this is usually for OIDC providers that can query
        /// a user by an identifier that can identify a user into charted-server.
        ///
        /// As of v0.1.0-beta, this scope is not used at all.
        UserConnections["user:connections"]: 1u64 << 3u64;

        /// **UNUSED AS OF v0.1.0-beta**
        ///
        /// Allows the current authenticated user to read from their notifications.
        #[allow(unused)]
        UserNotifications["user:notifications"]: 1u64 << 4u64;

        /// Allows the current authenticated user to update their user avatar.
        UserAvatarUpdate["user:avatar:update"]: 1u64 << 5u64;

        /// Allows the current authenticated user to list their current sessions.
        UserSessionsList["user:sessions:list"]: 1u64 << 6u64;

        // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
        //        Repository Scopes
        // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
        /// Allows access through private repositories that the current authenticated user
        /// can access, except repositories that they are a member apart of.
        RepoAccess["repo:access"]: 1u64 << 7u64;

        /// Allows the creation of public or private repositories that will be owned
        /// by the current authenticated user.
        RepoCreate["repo:create"]: 1u64 << 8u64;

        /// Allows the deletion of public or private repositories that will be owned
        /// by the current authenticated user.
        RepoDelete["repo:delete"]: 1u64 << 9u64;

        /// Allows patching a public or private repository that is owned by the current
        /// authenticated user.
        RepoUpdate["repo:update"]: 1u64 << 10u64;

        /// **DEPRECATED AS OF v0.1.0-beta**
        ///
        /// This is replaced by the `repo:releases:create` API key scope, please
        /// use that instead.
        #[deprecated(since = "0.1.0-beta", note = "Replaced by ApiKeyScope::RepoReleaseCreate")]
        RepoWrite["repo:write"]: 1u64 << 11u64;

        /// Allows patching a repository icon that the current authenticated user owns.
        RepoIconUpdate["repo:icon:update"]: 1u64 << 12u64;

        /// Allows the creation of creating repository releases that the current authenticated
        /// user owns.
        RepoReleaseCreate["repo:releases:create"]: 1u64 << 13u64;

        /// Allows patching repository releases that the current authenticated user owns.
        RepoReleaseUpdate["repo:releases:update"]: 1u64 << 14u64;

        /// Allows the deletion of repository releases that the current authenticated user owns.
        RepoReleaseDelete["repo:releases:delete"]: 1u64 << 15u64;

        /// Allows viewing all repository members.
        RepoMembersList["repo:members:list"]: 1u64 << 16u64;

        /// Allows patching repository member metadata.
        RepoMemberUpdate["repo:members:update"]: 1u64 << 17u64;

        /// Allows kicking repository members off the repository.
        RepoMemberKick["repo:members:kick"]: 1u64 << 18u64;

        /// Allows viewing all repository member invites. This scope is only used if the
        /// [`charted-emails`](https://github.com/charted-dev/emails) gRPC server is running
        /// and configured via the [`config.emails_grpc_endpoint`][emails_grpc_endpoint]
        /// configuration key.
        ///
        /// [emails_grpc_endpoint]: https://charts.noelware.org/docs/server/latest/self-hosting/configuration#emails_grpc_endpoint
        RepoMemberInviteAccess["repo:members:invites:access"]: 1u64 << 19u64;

        /// **DEPRECTEAD AS OF v0.1.0-beta**: This is unused.
        #[deprecated(since = "0.1.0-beta", note = "This is unused.")]
        RepoMemberInviteUpdate["repo:members:invites:update"]: 1u64 << 20u64;
        RepoMemberInviteDelete["repo:members:invites:delete"]: 1u64 << 21u64;
        RepoWebhookList["repo:webhooks:list"]: 1u64 << 22u64;
        RepoWebhookCreate["repo:webhooks:create"]: 1u64 << 23u64;
        RepoWebhookUpdate["repo:webhooks:update"]: 1u64 << 24u64;
        RepoWebhookDelete["repo:webhooks:delete"]: 1u64 << 25u64;
        RepoWebhookEventAccess["repo:webhooks:events:access"]: 1u64 << 26u64;
        RepoWebhookEventDelete["repo:webhooks:events:delete"]: 1u64 << 27u64;

        // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
        //        API Key Scopes
        // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
        ApiKeyView["apikeys:view"]: 1u64 << 28u64;
        ApiKeyList["apikeys:list"]: 1u64 << 52u64;
        ApiKeyCreate["apikeys:create"]: 1u64 << 29u64;
        ApiKeyDelete["apikeys:delete"]: 1u64 << 30u64;
        ApiKeyUpdate["apikeys:update"]: 1u64 << 31u64;

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
}
