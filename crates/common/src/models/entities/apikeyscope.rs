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

use crate::Bitfield;

macro_rules! gen_apikeyscopes {
    ($(
        $(#[$doc:meta])*
        $key:ident[$s:literal] => $value:expr;
    )*) => {
        /// Represents a single API key scope.
        #[derive(Clone, Copy, PartialEq)]
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

        impl ::serde::ser::Serialize for ApiKeyScope {
            fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_u64((*self).into())
            }
        }

        impl<'de> ::serde::de::Deserialize<'de> for ApiKeyScope {
            fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                struct ApiKeyScopeVisitor;
                impl<'de> ::serde::de::Visitor<'de> for ApiKeyScopeVisitor {
                    type Value = ApiKeyScope;

                    fn expecting(&self, formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        formatter.write_str("string containing the scope id or a u64 of the raw value")
                    }

                    fn visit_u64<E: ::serde::de::Error>(self, value: u64) -> Result<Self::Value, E> {
                        if value >= u64::MAX {
                            return Err(::serde::de::Error::custom(format!("value is greater or equal to u64::MAX")));
                        }

                        let max = ApiKeyScope::max();
                        if value > max {
                            return Err(::serde::de::Error::custom(format!("value is greater than the max element ({max})")));
                        }

                        let map = ApiKeyScope::as_map();
                        let element = map.values().find(|x| ((**x) as u64) == value);
                        if element.is_none() {
                            return Err(::serde::de::Error::custom(format!("unable to find value by {value}")));
                        }

                        Ok(*element.unwrap())
                    }

                    fn visit_str<E: ::serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                        let map = ApiKeyScope::as_map();
                        if !map.contains_key(value) {
                            return Err(::serde::de::Error::custom(format!("scope id [{value}] was not found")));
                        }

                        Ok(*map.get(value).unwrap())
                    }
                }

                deserializer.deserialize_any(ApiKeyScopeVisitor)
            }
        }

        impl ApiKeyScope {
            /// Returns the API key scope as its code.
            #[inline]
            pub const fn as_str(&self) -> &str {
                match self {
                    $(
                        ApiKeyScope::$key => $s,
                    )*
                }
            }

            /// Returns the max element available
            #[inline]
            pub fn max() -> u64 {
                let elems = vec![$($value,)*];
                *elems.iter().max().unwrap()
            }

            /// Returns an allocated [`HashMap`](std::collections::HashMap) of all the
            /// API key scopes together.
            #[inline]
            pub fn as_map<'a>() -> ::std::collections::HashMap<&'a str, ApiKeyScope> {
                let mut h = ::std::collections::HashMap::new();
                $(
                    h.insert($s, ApiKeyScope::$key);
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

/// Represents a [`Bitfield`] for managing [`ApiKeyScope`]s.
#[derive(Debug, Clone)]
pub struct ApiKeyScopes<'a>(Bitfield<'a>);
impl<'a> Default for ApiKeyScopes<'a> {
    fn default() -> Self {
        Self::init(0)
    }
}

impl<'a> ApiKeyScopes<'a> {
    /// Initializes a possible empty [`ApiKeyScopes`] bitfield with the given bits.
    pub fn init(bits: u64) -> ApiKeyScopes<'a> {
        let map = ApiKeyScope::as_map();

        ApiKeyScopes(Bitfield::new(
            bits,
            map.iter().map(|(k, v)| (*k, (*v) as u64)).collect(),
        ))
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Weow {
        scope: ApiKeyScope,
    }

    #[test]
    fn apikeyscope_serialize() {
        let scope = Weow {
            scope: ApiKeyScope::UserAccess,
        };

        let serialized = serde_json::to_string(&scope).unwrap();
        assert_eq!("{\"scope\":1}", serialized);
    }

    #[test]
    fn apikeyscope_deserialize_u64() {
        let deserialized: Weow = serde_json::from_str(r#"{"scope":274877906944}"#).unwrap();
        let expected = Weow {
            scope: ApiKeyScope::OrgMemberKick,
        };

        assert_eq!(expected, deserialized);
    }

    #[test]
    #[should_panic(expected = "unable to find value by 7")]
    fn apikeyscope_deserialize_nonexisting_u64() {
        serde_json::from_str::<Weow>(r#"{"scope":7}"#).unwrap();
    }

    #[test]
    #[should_panic(expected = "value is greater or equal to u64::MAX")]
    fn apikeyscope_deserialize_max_u64() {
        let weow = u64::MAX;
        serde_json::from_str::<Weow>(format!("{{\"scope\":{weow}}}").as_str()).unwrap();
    }

    #[test]
    #[should_panic] // we can't do "expected =" as new scopes can be added and it'll be nondeterministic (unlike the rest of should_panic for u64 variants)
    fn apikeyscope_deserialize_max_scope() {
        let max = ApiKeyScope::max() + 1;
        serde_json::from_str::<Weow>(format!("{{\"scope\":{max}}}").as_str()).unwrap();
    }

    #[test]
    fn apikeyscope_deserialize_str() {
        let deserialized: Weow = serde_json::from_str(r#"{"scope":"user:access"}"#).unwrap();
        let expected = Weow {
            scope: ApiKeyScope::UserAccess,
        };

        assert_eq!(expected, deserialized);
    }

    #[test]
    #[should_panic(expected = "scope id [weow fluff] was not found")]
    fn apikeyscope_deserialize_nonexisting_str() {
        serde_json::from_str::<Weow>("{\"scope\":\"weow fluff\"}").unwrap();
    }
}
