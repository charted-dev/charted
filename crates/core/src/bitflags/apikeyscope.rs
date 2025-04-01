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

use super::Bitflags;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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
        UserAccess["user:access"] => 1u64 << 0u64;

        /// Allows the authenticated user to patch any user metadata.
        UserUpdate["user:update"] => 1u64 << 1u64;

        /// Allows to delete the current authenticated user.
        UserDelete["user:delete"] => 1u64 << 2u64;

        /// Allows the current authenticated user to access their connections like their
        /// GitHub or GitLab accounts, this is usually for OIDC providers that can query
        /// a user by an identifier that can identify a user into charted-server.
        ///
        /// As of v0.1.0-beta, this scope is not used at all.
        UserConnections["user:connections"] => 1u64 << 3u64;

        /// Allows the current authenticated user to update their user avatar.
        UserAvatarUpdate["user:avatar:update"] => 1u64 << 4u64;

        /// Allows the current authenticated user to list their current sessions.
        UserSessionsList["user:sessions:list"] => 1u64 << 5u64;

        // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
        //        Repository Scopes
        // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
        /// Allows access through private repositories that the current authenticated user
        /// can access, except repositories that they are a member apart of.
        RepoAccess["repo:access"] => 1u64 << 6u64;

        /// Allows the creation of public or private repositories that will be owned
        /// by the current authenticated user.
        RepoCreate["repo:create"] => 1u64 << 7u64;

        /// Allows the deletion of public or private repositories that will be owned
        /// by the current authenticated user.
        RepoDelete["repo:delete"] => 1u64 << 8u64;

        /// Allows patching a public or private repository that is owned by the current
        /// authenticated user.
        RepoUpdate["repo:update"] => 1u64 << 9u64;

        /// Allows patching a repository icon that the current authenticated user owns.
        RepoIconUpdate["repo:icon:update"] => 1u64 << 10u64;

        /// Allows the creation of creating repository releases that the current authenticated
        /// user owns.
        RepoReleaseCreate["repo:releases:create"] => 1u64 << 11u64;

        /// Allows patching repository releases that the current authenticated user owns.
        RepoReleaseUpdate["repo:releases:update"] => 1u64 << 12u64;

        /// Allows the deletion of repository releases that the current authenticated user owns.
        RepoReleaseDelete["repo:releases:delete"] => 1u64 << 13u64;

        /// Allows viewing all repository members.
        RepoMembersList["repo:members:list"] => 1u64 << 14u64;

        /// Allows patching repository member metadata.
        RepoMemberUpdate["repo:members:update"] => 1u64 << 15u64;

        /// Allows kicking repository members off the repository.
        RepoMemberKick["repo:members:kick"] => 1u64 << 16u64;

        /// Allows viewing all repository member invites.
        ///
        /// This scope is only used if the [`charted-emails`](https://github.com/charted-dev/emails) gRPC server is
        /// running and configured via the [`config.emails_grpc_endpoint`][emails_grpc_endpoint] configuration key.
        ///
        /// [emails_grpc_endpoint] => https://charts.noelware.org/docs/server/latest/self-hosting/configuration#emails_grpc_endpoint
        RepoMemberInviteAccess["repo:members:invites:access"] => 1u64 << 17u64;

        /// Deletes a repository member's invite
        RepoMemberInviteDelete["repo:members:invites:delete"] => 1u64 << 18u64;

        /// Allows a user to view a repository's webhooks.
        ///
        /// This scope is only used via the [HTTP webhooks] feature.
        ///
        /// [HTTP webhooks]: https://charts.noelware.org/docs/server/latest/features/webhooks
        RepoWebhookList["repo:webhooks:list"] => 1u64 << 19u64;

        /// Allows a user to create a HTTP webhook in a repository.
        ///
        /// This scope is only used via the [HTTP webhooks] feature.
        ///
        /// [HTTP webhooks]: https://charts.noelware.org/docs/server/latest/features/webhooks
        RepoWebhookCreate["repo:webhooks:create"] => 1u64 << 20u64;

        /// Allows a user to update a repository webhook's metadata.
        ///
        /// This scope is only used via the [HTTP webhooks] feature.
        ///
        /// [HTTP webhooks]: https://charts.noelware.org/docs/server/latest/features/webhooks
        RepoWebhookUpdate["repo:webhooks:update"] => 1u64 << 21u64;

        /// Allows a user to delete repository webhooks.
        ///
        /// This scope is only used via the [HTTP webhooks] feature.
        ///
        /// [HTTP webhooks]: https://charts.noelware.org/docs/server/latest/features/webhooks
        RepoWebhookDelete["repo:webhooks:delete"] => 1u64 << 22u64;

        /// Allows a user to view a repository webhook's event data.
        ///
        /// This scope is only used via the [HTTP webhooks] feature.
        ///
        /// [HTTP webhooks]: https://charts.noelware.org/docs/server/latest/features/webhooks
        RepoWebhookEventAccess["repo:webhooks:events:access"] => 1u64 << 23u64;

        /// Allows a user to delete a repository webhook's event data.
        ///
        /// This scope is only used via the [HTTP webhooks] feature.
        ///
        /// [HTTP webhooks]: https://charts.noelware.org/docs/server/latest/features/webhooks
        RepoWebhookEventDelete["repo:webhooks:events:delete"] => 1u64 << 24u64;

        // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
        //        API Key Scopes
        // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
        /// Allows a user to view a single API key.
        ApiKeyView["apikeys:view"] => 1u64 << 25u64;

        /// Allows a user to list all API keys.
        ApiKeyList["apikeys:list"] => 1u64 << 26u64;

        /// Allows a user to update a API key.
        ApiKeyCreate["apikeys:create"] => 1u64 << 27u64;

        /// Allows a user to delete a API key.
        ApiKeyDelete["apikeys:delete"] => 1u64 << 28u64;

        /// Allows a user to update a API key's metadata.
        ApiKeyUpdate["apikeys:update"] => 1u64 << 29u64;

        // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
        //      Organization Scopes
        // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
        OrgAccess["org:access"] => 1u64 << 30u64;
        OrgCreate["org:create"] => 1u64 << 31u64;
        OrgUpdate["org:update"] => 1u64 << 32u64;
        OrgDelete["org:delete"] => 1u64 << 33u64;
        OrgMemberInvites["org:members:invites"] => 1u64 << 34u64;
        OrgMemberList["org:members:list"] => 1u64 << 35u64;
        OrgMemberKick["org:members:kick"] => 1u64 << 36u64;
        OrgMemberUpdate["org:members:update"] => 1u64 << 37u64;
        OrgWebhookList["org:webhooks:list"] => 1u64 << 38u64;
        OrgWebhookCreate["org:webhooks:create"] => 1u64 << 39u64;
        OrgWebhookUpdate["org:webhooks:update"] => 1u64 << 40u64;
        OrgWebhookDelete["org:webhooks:delete"] => 1u64 << 41u64;
        OrgWebhookEventList["org:webhooks:events:list"] => 1u64 << 42u64;
        OrgWebhookEventDelete["org:webhooks:events:delete"] => 1u64 << 43u64;

        // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
        //    Administration Scopes
        // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
        AdminStats["admin:stats"] => 1u64 << 44u64;
        AdminUserCreate["admin:users:create"] => 1u64 << 45u64;
        AdminUserDelete["admin:users:delete"] => 1u64 << 46u64;
        AdminUserUpdate["admin:users:update"] => 1u64 << 47u64;
        AdminOrgDelete["admin:orgs:delete"] => 1u64 << 48u64;
        AdminOrgUpdate["admin:orgs:update"] => 1u64 << 49u64;
    }
}

impl Serialize for ApiKeyScope {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ApiKeyScope {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        let flags = ApiKeyScope::flags();

        serde_untagged::UntaggedEnumVisitor::new()
            .expecting("string or uint64")
            .string(|v| {
                if let Some(value) = flags.get(v).copied() {
                    return Ok(unsafe {
                        // SAFETY: the `bitflags!` macro will ensure that
                        // `value` can be transmuted from a `u64` into
                        // a `ApiKeyScope`.
                        std::mem::transmute::<u64, ApiKeyScope>(value)
                    });
                }

                Err(serde_untagged::de::Error::custom(format!(
                    "unknown variant of ApiKeyScope: {v}"
                )))
            })
            .u64(|v| {
                let max = <ApiKeyScope as Bitflags>::max();
                if v >= 1 && v <= max {
                    return Ok(unsafe { std::mem::transmute::<u64, ApiKeyScope>(v) });
                }

                Err(serde_untagged::de::Error::custom(format!("out of range: [1..{max})")))
            })
            .deserialize(deserializer)
    }
}

#[cfg(feature = "openapi")]
const _: () = {
    use utoipa::{
        PartialSchema, ToSchema,
        openapi::{
            KnownFormat, ObjectBuilder, OneOfBuilder, RefOr, Schema, SchemaFormat, Type, schema::SchemaType,
        },
    };

    #[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "openapi")))]
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

    #[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "openapi")))]
    impl ToSchema for ApiKeyScope {}
};

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
            assert_eq!(value, ApiKeyScope::RepoMembersList);
        }

        {
            let value: ApiKeyScope =
                serde_json::from_str::<ApiKeyScope>(&format!("{}", <ApiKeyScope as Bitflags>::max())).unwrap();

            assert_eq!(value, ApiKeyScope::AdminOrgUpdate);
        }

        // error case #1: if we pass in zero
        {
            let e = serde_json::from_str::<ApiKeyScope>("0").unwrap_err();
            let e = e.to_string();

            assert_eq!(
                e,
                format!(
                    "out of range: [1..{}) at line 1 column 1",
                    <ApiKeyScope as Bitflags>::max()
                )
            );
        }

        {
            let Err(e) = serde_json::from_str::<ApiKeyScope>(&format!("{}", <ApiKeyScope as Bitflags>::max() + 1))
            else {
                unreachable!()
            };

            let e = e.to_string();
            assert_eq!(
                e,
                format!(
                    "out of range: [1..{}) at line 1 column 15",
                    <ApiKeyScope as Bitflags>::max()
                )
            );
        }

        {
            let Err(e) = serde_json::from_str::<ApiKeyScope>("false") else {
                unreachable!()
            };

            let e = e.to_string();
            assert_eq!(
                e,
                "invalid type: boolean `false`, expected string or uint64 at line 1 column 5"
            );
        }
    }
}
