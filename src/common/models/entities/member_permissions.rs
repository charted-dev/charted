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

use crate::common::bitfield::Bitfield;

macro_rules! gen_member_permissions {
    ($(
        $(#[$doc:meta])*
        $key:ident[$s:literal] => $value:expr;
    )*) => {
        /// Represents a single member permission.
        #[derive(Clone, Copy, PartialEq)]
        #[allow(clippy::enum_clike_unportable_variant)] // we don't provide support for 32bit systems
        #[repr(u64)]
        pub enum MemberPermission {
            $(
                $(#[$doc])*
                $key = $value,
            )*
        }

        impl ::core::fmt::Debug for MemberPermission {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    $(
                        MemberPermission::$key => f.write_fmt(format_args!("MemberPermission::{} ({})", stringify!($key), $s)),
                    )*
                }
            }
        }

        impl ::core::fmt::Display for MemberPermission {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    $(
                        MemberPermission::$key => f.write_str($s),
                    )*
                }
            }
        }

        impl ::core::convert::From<MemberPermission> for u64 {
            fn from(perm: MemberPermission) -> u64 {
                perm as u64
            }
        }

        impl ::serde::ser::Serialize for MemberPermission {
            fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_u64((*self).into())
            }
        }

        impl<'de> ::serde::de::Deserialize<'de> for MemberPermission {
            fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                struct MemberPermissionVisitor;
                impl<'de> ::serde::de::Visitor<'de> for MemberPermissionVisitor {
                    type Value = MemberPermission;

                    fn expecting(&self, formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        formatter.write_str("string containing the scope id or a u64 of the raw value")
                    }

                    fn visit_u64<E: ::serde::de::Error>(self, value: u64) -> Result<Self::Value, E> {
                        if value >= u64::MAX {
                            return Err(::serde::de::Error::custom(format!("value is greater or equal to u64::MAX")));
                        }

                        let max = MemberPermission::max();
                        if value > max {
                            return Err(::serde::de::Error::custom(format!("value is greater than the max element ({max})")));
                        }

                        let map = MemberPermission::as_map();
                        let element = map.values().find(|x| ((**x) as u64) == value);
                        if element.is_none() {
                            return Err(::serde::de::Error::custom(format!("unable to find value by {value}")));
                        }

                        Ok(*element.unwrap())
                    }

                    fn visit_str<E: ::serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                        let map = MemberPermission::as_map();
                        if !map.contains_key(value) {
                            return Err(::serde::de::Error::custom(format!("scope id [{value}] was not found")));
                        }

                        Ok(*map.get(value).unwrap())
                    }
                }

                deserializer.deserialize_any(MemberPermissionVisitor)
            }
        }

        impl MemberPermission {
            /// Returns the API key scope as its code.
            #[inline]
            pub const fn as_str(&self) -> &str {
                match self {
                    $(
                        MemberPermission::$key => $s,
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
            pub fn as_map<'a>() -> ::std::collections::HashMap<&'a str, MemberPermission> {
                let mut h = ::std::collections::HashMap::new();
                $(
                    h.insert($s, MemberPermission::$key);
                )*

                h
            }
        }
    };
}

gen_member_permissions!(
    /// This member has permission to invite new members into this repository or organization
    /// and can view all other pending invites.
    MemberInvite["member:invite"] => 1 << 0;

    /// This member has the permission to update any other member's permissions
    MemberUpdate["member:update"] => 1 << 1;

    /// This member has the permission to kick other members from the repository or organization
    MemberKick["member:kick"] => 1 << 2;

    /// This member has permission to update any repository or organization metadata
    MetadataUpdate["metadata:update"] => 1 << 3;

    /// > This is only for organization members, this will be nop for repository members
    ///
    /// This member has permission to create repositories in an organization.
    RepoCreate["repo:create"] => 1 << 4;

    /// > This is only for organization members, this will be nop for repository members
    ///
    /// This member has permission to delete repositories in an organization.
    RepoDelete["repo:delete"] => 1 << 5;

    /// This member has permission to create additional repository or organization
    /// webhooks.
    WebhookCreate["webhooks:create"] => 1 << 6;

    /// This member has permission to update repository or organization webhooks.
    WebhookUpdate["webhooks:update"] => 1 << 7;

    /// This member has permission to delete additional repository or organization webhooks.
    WebhookDelete["webhooks:delete"] => 1 << 8;

    /// This member has permission to delete external metadata in an organization
    /// or repository, like repository releases
    MetadataDelete["metadata:delete"] => 1 << 9;
);

/// Represents a [`Bitfield`] for managing [`ApiKeyScope`]s.
#[derive(Debug, Clone)]
pub struct MemberPermissions<'a>(Bitfield<'a>);
impl<'a> Default for MemberPermissions<'a> {
    fn default() -> Self {
        Self::init(0)
    }
}

impl<'a> MemberPermissions<'a> {
    /// Initializes a possible empty [`MemberPermissions`] bitfield with the given bits.
    pub fn init(bits: u64) -> MemberPermissions<'a> {
        let map = MemberPermission::as_map();

        MemberPermissions(Bitfield::new(
            bits,
            map.iter().map(|(k, v)| (*k, (*v) as u64)).collect(),
        ))
    }
}

impl<'a> std::ops::Deref for MemberPermissions<'a> {
    type Target = Bitfield<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> std::ops::DerefMut for MemberPermissions<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
