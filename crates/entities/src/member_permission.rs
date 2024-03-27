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

use crate::gen_rbac_enum;
use charted_common::Bitfield;
use utoipa::{
    openapi::{KnownFormat, ObjectBuilder, OneOfBuilder, RefOr, Schema, SchemaFormat, SchemaType},
    ToSchema,
};

gen_rbac_enum!(
    /// Represents a RBAC-based enum that allows to control permissions on a repository or organization member.
    #[derive(Clone, Copy)]
    #[repr(u32)]
    pub MemberPermission {
        /// This member has permission to invite new members into this repository or organization
        /// and can view all other pending invites.
        MemberInvite["member:invite"]: 1 << 0;

        /// This member has the permission to update any other member's permissions
        MemberUpdate["member:update"]: 1 << 1;

        /// This member has the permission to kick other members from the repository or organization
        MemberKick["member:kick"]: 1 << 2;

        /// This member has permission to update any repository or organization metadata
        MetadataUpdate["metadata:update"]: 1 << 3;

        /// > This is only for organization members, this will be nop for repository members
        ///
        /// This member has permission to create repositories in an organization.
        RepoCreate["repo:create"]: 1 << 4;

        /// > This is only for organization members, this will be nop for repository members
        ///
        /// This member has permission to delete repositories in an organization.
        RepoDelete["repo:delete"]: 1 << 5;

        /// This member has permission to create additional repository or organization
        /// webhooks.
        WebhookCreate["webhooks:create"]: 1 << 6;

        /// This member has permission to update repository or organization webhooks.
        WebhookUpdate["webhooks:update"]: 1 << 7;

        /// This member has permission to delete additional repository or organization webhooks.
        WebhookDelete["webhooks:delete"]: 1 << 8;

        /// This member has permission to delete external metadata in an organization
        /// or repository, like repository releases
        MetadataDelete["metadata:delete"]: 1 << 9;
    }
);

impl<'s> ToSchema<'s> for MemberPermission {
    fn schema() -> (&'s str, RefOr<Schema>) {
        ("MemberPermission", RefOr::T(Schema::OneOf(
            OneOfBuilder::new()
                .description(Some("Represents a RBAC-based enum that allows to control permissions on a repository or organization member."))
                .item(
                    Schema::Object(
                        ObjectBuilder::new()
                            .schema_type(SchemaType::Number)
                            .format(Some(SchemaFormat::KnownFormat(KnownFormat::Int32)))
                            .maximum(Some(MemberPermission::max() as f64))
                            .minimum(Some(1f64))
                            .description(Some(format!("Represents a unsigned 32bit integer in range of [1..{}) that the API server can represent as a bit", MemberPermission::max())))
                            .build()
                    )
                )
                .item(
                    Schema::Object(
                        ObjectBuilder::new()
                            .schema_type(SchemaType::String)
                            .enum_values(Some(MemberPermission::values_str().to_vec()))
                            .description(Some("Represents a named, human-readable version of the permission that is always represented as 'entity:operation', i.e, `apikey:list` will allow the API key to list all API keys"))
                            .build()
                    )
                )
                .build()
        )))
    }
}

/// Represents a [`Bitfield`] for managing [`MemberPermission`]s.
#[derive(Debug, Clone)]
pub struct MemberPermissions(Bitfield);
impl Default for MemberPermissions {
    fn default() -> Self {
        Self::init(0)
    }
}

impl MemberPermissions {
    /// Initializes a possible empty [`MemberPermissions`] bitfield with the given bits.
    pub fn init(bits: u64) -> MemberPermissions {
        let map = MemberPermission::as_map();
        let bitfield = Bitfield::with_flags(map.iter().map(|(k, v)| (*k, (*v) as u64)).collect());

        MemberPermissions(bitfield.with_value(bits))
    }
}

impl std::ops::Deref for MemberPermissions {
    type Target = Bitfield;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for MemberPermissions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
