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

use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use utoipa::{
    openapi::{schema::SchemaType, KnownFormat, ObjectBuilder, OneOfBuilder, RefOr, Schema, SchemaFormat, Type},
    PartialSchema, ToSchema,
};

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

impl Serialize for ApiKeyScope {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.as_bit())
    }
}

impl<'de> Deserialize<'de> for ApiKeyScope {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        let content = <serde::__private::de::Content as serde::Deserialize>::deserialize(deserializer)?;
        let deserializer = serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
        let flags = <ApiKeyScope as crate::bitflags::Bitflags>::flags();

        if let Ok(s) = <String as serde::Deserialize>::deserialize(deserializer) {
            if let Some(value) = flags.get(s.as_str()).copied() {
                // Safety: the implementation of the `bitflags!` macro will ensure
                //         that `value` is a discriminant of `ApiKeyScope`.
                return Ok(unsafe { std::mem::transmute::<u64, ApiKeyScope>(value) });
            }

            return Err(D::Error::custom(format!("unknown value [{s}]")));
        }

        if let Ok(value) = <u64 as serde::Deserialize>::deserialize(deserializer) {
            let max = <ApiKeyScope as crate::bitflags::Bitflags>::max();
            if value >= 1 && value <= max {
                // Safety: the condition above checks if value is greater than once, since
                //         we can't accept `0` as a value and therefore, will not pass the check
                //
                //         And, we check if the `value` is less than or equal to the
                //         possible maximum value from the `bitflags!` impl. Therefore,
                //         it is safe to assume that we can transmute from `u64` -> `ApiKeyScope`.
                return Ok(unsafe { std::mem::transmute::<u64, ApiKeyScope>(value) });
            }

            return Err(D::Error::custom(format!("out of range: [1..{max})")));
        }

        Err(D::Error::custom("invalid path: expected a `string` or `uint64`"))
    }
}

impl PartialSchema for ApiKeyScope {
    fn schema() -> RefOr<Schema> {
        let flags = <ApiKeyScope as crate::bitflags::Bitflags>::flags();
        let max = <ApiKeyScope as crate::bitflags::Bitflags>::max();

        RefOr::T(Schema::OneOf({
            let oneof = OneOfBuilder::new()
                .description(Some(
                    "Representation of a API key scope. A scope determines a permission between an API key",
                ))
                .item(Schema::Object({
                    let object = ObjectBuilder::new()
                        .schema_type(SchemaType::Type(Type::String))
                        .description(Some("A humane name of the scope. This allows to determine the scope without knowing the integer representation of it."))
                        .enum_values(Some(flags.keys().copied().collect::<Vec<_>>()));

                    object.build()
                }))
                .item(Schema::Object({
                    let object = ObjectBuilder::new()
                        .schema_type(SchemaType::Type(Type::Number))
                        .format(Some(SchemaFormat::KnownFormat(KnownFormat::UInt64)))
                        .description(Some("The actual representation of the scope. This is the repsentation the server checks and stores as AND is used when comparing permissions"))
                        .minimum(Some(1))
                        .maximum(Some(max));

                    object.build()
                }));

            oneof.build()
        }))
    }
}

impl ToSchema for ApiKeyScope {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("ApiKeyScope")
    }
}

impl FromIterator<ApiKeyScope> for crate::bitflags::ApiKeyScopes {
    fn from_iter<T: IntoIterator<Item = ApiKeyScope>>(iter: T) -> Self {
        let mut bitfield = ApiKeyScopes::default();
        bitfield.add(iter);

        bitfield
    }
}

#[cfg(test)]
mod tests {
    use super::ApiKeyScope;
    use crate::bitflags::Bitflags;

    #[test]
    fn test_deserialize_str() {
        let a = "\"user:access\"";
        let deserialized = serde_json::from_str::<ApiKeyScope>(a).unwrap();

        assert_eq!(deserialized, ApiKeyScope::UserAccess);

        let b = "true";
        assert!(serde_json::from_str::<ApiKeyScope>(b).is_err());
    }

    #[test]
    fn test_deserialize_u64() {
        {
            let value: ApiKeyScope = serde_json::from_str("1").unwrap();
            assert_eq!(value, ApiKeyScope::UserAccess);
        }

        {
            let value: ApiKeyScope = serde_json::from_str("16384").unwrap();
            assert_eq!(value, ApiKeyScope::RepoReleaseUpdate);
        }

        {
            let value: ApiKeyScope =
                serde_json::from_str::<ApiKeyScope>(&format!("{}", <ApiKeyScope as Bitflags>::max())).unwrap();

            assert_eq!(value, ApiKeyScope::ApiKeyList);
        }

        // error case #1: if we pass in zero
        {
            let e = serde_json::from_str::<ApiKeyScope>("0").unwrap_err();
            let e = e.to_string();

            assert_eq!(e, format!("out of range: [1..{})", <ApiKeyScope as Bitflags>::max()));
        }

        {
            let Err(e) = serde_json::from_str::<ApiKeyScope>(&format!("{}", <ApiKeyScope as Bitflags>::max() + 1))
            else {
                unreachable!()
            };

            let e = e.to_string();
            assert_eq!(e, format!("out of range: [1..{})", <ApiKeyScope as Bitflags>::max()));
        }

        {
            let Err(e) = serde_json::from_str::<ApiKeyScope>("false") else {
                unreachable!()
            };

            let e = e.to_string();
            assert_eq!(e, "invalid path: expected a `string` or `uint64`");
        }
    }
}
