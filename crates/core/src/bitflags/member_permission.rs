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

pub type MemberPermissions = crate::bitflags::Bitfield<MemberPermission>;

crate::bitflags! {
    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    #[allow(clippy::enum_clike_unportable_variant)]
    #[repr(u64)]
    pub MemberPermission[u64] {
        /// This member has permission to invite new members into this repository or organization
        /// and can view all other pending invites.
        MemberInvite["member:invite"] => 1u64 << 0u64;

        /// This member has the permission to update any other member's permissions
        MemberUpdate["member:update"] => 1u64 << 1u64;

        /// This member has the permission to kick other members from the repository or organization
        MemberKick["member:kick"] => 1u64 << 2u64;

        /// This member has permission to update any repository or organization metadata
        MetadataUpdate["metadata:update"] => 1u64 << 3u64;

        /// > This is only for organization members, this will be nop for repository members
        ///
        /// This member has permission to create repositories in an organization.
        RepoCreate["repo:create"] => 1u64 << 4u64;

        /// > This is only for organization members, this will be nop for repository members
        ///
        /// This member has permission to delete repositories in an organization.
        RepoDelete["repo:delete"] => 1u64 << 5u64;

        /// This member has permission to create additional repository or organization
        /// webhooks.
        WebhookCreate["webhooks:create"] => 1u64 << 6u64;

        /// This member has permission to update repository or organization webhooks.
        WebhookUpdate["webhooks:update"] => 1u64 << 7u64;

        /// This member has permission to delete additional repository or organization webhooks.
        WebhookDelete["webhooks:delete"] => 1u64 << 8u64;

        /// This member has permission to delete external metadata in an organization
        /// or repository, like repository releases
        MetadataDelete["metadata:delete"] => 1u64 << 9u64;
    }
}
