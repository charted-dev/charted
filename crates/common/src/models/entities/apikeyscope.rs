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

#![allow(deprecated)]

use crate::Bitfield;

macro_rules! gen_apikeyscopes {
    ($(
        $(#[$doc:meta])*
        $key:ident[$s:literal] => $value:expr;
    )*) => {
        /// Represents a single API key scope.
        #[derive(Clone, Copy)]
        #[repr(u64)]
        #[allow(clippy::enum_clike_unportable_variant)] // we don't provide support for 32bit systems
        pub enum ApiKeyScope {
            $(
                $(#[$doc])*
                $key = $value,
            )*
        }

        impl ::core::fmt::Debug for ApiKeyScope {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    $(
                        ApiKeyScope::$key => f.write_fmt(format_args!("ApiKeyScope::{} ({})", stringify!($key), $s)),
                    )*
                }
            }
        }

        impl ::core::fmt::Display for ApiKeyScope {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    $(
                        ApiKeyScope::$key => f.write_str($s),
                    )*
                }
            }
        }

        impl ::core::convert::From<ApiKeyScope> for u64 {
            fn from(scope: ApiKeyScope) -> u64 {
                scope as u64
            }
        }

        impl ApiKeyScope {
            /// Returns the API key scope as its code.
            pub const fn as_str(&self) -> &str {
                match self {
                    $(
                        ApiKeyScope::$key => $s,
                    )*
                }
            }

            /// Returns an allocated [`HashMap`](std::collections::HashMap) of all the
            /// API key scopes together.
            pub fn hashmap<'a>() -> ::std::collections::HashMap<&'a str, u64> {
                let mut h = ::std::collections::HashMap::new();
                $(
                    h.insert($s, $value);
                )*

                h
            }
        }
    };
}

gen_apikeyscopes!(
    // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
    //           User Scopes
    // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
    /// Allows the current authenticated user to access metadata about themselves.
    UserAccess["user:access"] => 1 << 0;

    /// Allows the authenticated user to patch any user metadata.
    UserUpdate["user:update"] => 1 << 1;

    /// Allows to delete the current authenticated user.
    UserDelete["user:delete"] => 1 << 2;

    /// Allows the current authenticated user to access their connections like their
    /// GitHub or GitLab connected user.
    UserConnections["user:connections"] => 1 << 3;

    /// **UNUSED AS OF v0.1.0-beta**
    ///
    /// Allows the current authenticated user to read from their notifications.
    #[allow(unused)]
    UserNotifications["user:notifications"] => 1 << 4;

    /// Allows the current authenticated user to update their user avatar.
    UserAvatarUpdate["user:avatar:update"] => 1 << 5;

    /// Allows the current authenticated user to list their current sessions.
    UserSessionsList["user:sessions:list"] => 1 << 6;

    // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
    //        Repository Scopes
    // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
    /// Allows access through private repositories that the current authenticated user
    /// can access, except repositories that they are a member apart of.
    RepoAccess["repo:access"] => 1 << 7;

    /// Allows the creation of public or private repositories that will be owned
    /// by the current authenticated user.
    RepoCreate["repo:create"] => 1 << 8;

    /// Allows the deletion of public or private repositories that will be owned
    /// by the current authenticated user.
    RepoDelete["repo:delete"] => 1 << 9;

    /// Allows patching a public or private repository that is owned by the current
    /// authenticated user.
    RepoUpdate["repo:update"] => 1 << 10;

    /// **DEPRECATED AS OF v0.1.0-beta**
    ///
    /// This is replaced by the `repo:releases:create` API key scope, please
    /// use that instead.
    #[deprecated(since = "0.1.0-beta", note = "Replaced by ApiKeyScope::RepoReleaseCreate")]
    RepoWrite["repo:write"] => 1 << 11;

    /// Allows patching a repository icon that the current authenticated user owns.
    RepoIconUpdate["repo:icon:update"] => 1 << 12;

    /// Allows the creation of creating repository releases that the current authenticated
    /// user owns.
    RepoReleaseCreate["repo:releases:create"] => 1 << 13;

    /// Allows patching repository releases that the current authenticated user owns.
    RepoReleaseUpdate["repo:releases:update"] => 1 << 14;

    /// Allows the deletion of repository releases that the current authenticated user owns.
    RepoReleaseDelete["repo:releases:delete"] => 1 << 15;

    /// Allows viewing all repository members.
    RepoMembersList["repo:members:list"] => 1 << 16;

    /// Allows patching repository member metadata.
    RepoMemberUpdate["repo:members:update"] => 1 << 17;

    /// Allows kicking repository members off the repository.
    RepoMemberKick["repo:members:kick"] => 1 << 18;

    /// Allows viewing all repository member invites. This scope is only used if the
    /// [`charted-emails`](https://github.com/charted-dev/emails) gRPC server is running
    /// and configured via the [`config.emails_grpc_endpoint`][emails_grpc_endpoint]
    /// configuration key.
    ///
    /// [emails_grpc_endpoint]: https://charts.noelware.org/docs/server/latest/self-hosting/configuration#emails_grpc_endpoint
    RepoMemberInviteAccess["repo:members:invites:access"] => 1 << 19;

    /// **DEPRECTEAD AS OF v0.1.0-beta**: This is unused.
    #[deprecated(since = "0.1.0-beta", note = "This is unused.")]
    RepoMemberInviteUpdate["repo:members:invites:update"] => 1 << 20;
    RepoMemberInviteDelete["repo:members:invites:delete"] => 1 << 21;
    RepoWebhookList["repo:webhooks:list"] => 1 << 22;
    RepoWebhookCreate["repo:webhooks:create"] => 1 << 23;
    RepoWebhookUpdate["repo:webhooks:update"] => 1 << 24;
    RepoWebhookDelete["repo:webhooks:delete"] => 1 << 25;
    RepoWebhookEventAccess["repo:webhooks:events:access"] => 1 << 26;
    RepoWebhookEventDelete["repo:webhooks:events:delete"] => 1 << 27;

    // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
    //        API Key Scopes
    // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
    ApiKeyView["apikeys:view"] => 1 << 28;
    ApiKeyCreate["apikeys:create"] => 1 << 29;
    ApiKeyDelete["apikeys:delete"] => 1 << 30;
    ApiKeyUpdate["apikeys:update"] => 1 << 31;

    // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
    //      Organization Scopes
    // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
    OrgAccess["org:access"] => 1 << 32;
    OrgCreate["org:create"] => 1 << 33;
    OrgUpdate["org:update"] => 1 << 34;
    OrgDelete["org:delete"] => 1 << 35;
    OrgMemberInvites["org:members:invites"] => 1 << 36;
    OrgMemberList["org:members:list"] => 1 << 37;
    OrgMemberKick["org:members:kick"] => 1 << 38;
    OrgMemberUpdate["org:members:update"] => 1 << 39;
    OrgWebhookList["org:webhooks:list"] => 1 << 40;
    OrgWebhookCreate["org:webhooks:create"] => 1 << 41;
    OrgWebhookUpdate["org:webhooks:update"] => 1 << 42;
    OrgWebhookDelete["org:webhooks:delete"] => 1 << 43;
    OrgWebhookEventList["org:webhooks:events:list"] => 1 << 44;
    OrgWebhookEventDelete["org:webhooks:events:delete"] => 1 << 45;

    // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
    //    Administration Scopes
    // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
    AdminStats["admin:stats"] => 1 << 46;
    AdminUserCreate["admin:users:create"] => 1 << 47;
    AdminUserDelete["admin:users:delete"] => 1 << 48;
    AdminUserUpdate["admin:users:update"] => 1 << 49;
    AdminOrgDelete["admin:orgs:delete"] => 1 << 50;
    AdminOrgUpdate["admin:orgs:update"] => 1 << 51;
);

#[derive(Debug, Clone)]
pub struct ApiKeyScopes<'a>(Bitfield<'a>);

impl<'a> Default for ApiKeyScopes<'a> {
    fn default() -> Self {
        ApiKeyScopes(Bitfield::new(0, ApiKeyScope::hashmap()))
    }
}

impl<'a> ApiKeyScopes<'a> {
    pub fn init(bits: u64) -> ApiKeyScopes<'a> {
        ApiKeyScopes(Bitfield::new(bits, ApiKeyScope::hashmap()))
    }
}

impl<'a> std::ops::Deref for ApiKeyScopes<'a> {
    type Target = Bitfield<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> std::ops::DerefMut for ApiKeyScopes<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
