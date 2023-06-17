/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *    http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.models.flags

import org.noelware.charted.common.Bitfield

private val genericMemberPermissions: Map<String, Long> = mapOf(
    // This member has permission to invite new members into the repository or organization, and
    // they can view all the other invites that are pending
    "member:invite" to (1L shl 0),

    // This member has permission to update any member's permissions
    "member:update" to (1L shl 1),

    // This member has permission to kick any members off the repository
    "member:kick" to (1L shl 2),

    // Whether if this member has permission to update the repository or organization metadata.
    "metadata:update" to (1L shl 3),

    // Whether if this member has permission to create a repository in this organization. As a repository
    // member, this does nothing.
    "repo:create" to (1L shl 4),

    // Whether if this member has permission to delete the repository or not.
    "repo:delete" to (1L shl 5),

    // Whether if this member has permission to create additional webhooks in the given
    // repository or organization.
    "webhooks:create" to (1L shl 6),

    // Whether if this member has permission to update webhooks in the given
    // repository or organization.
    "webhooks:update" to (1L shl 7),

    // Whether if this member has permission to delete webhooks in the given
    // repository or organization.
    "webhooks:delete" to (1L shl 8),

    // Whether if this member has permission to delete any repository/organization metadata (i.e. repo releases)
    "metadata:delete" to (1L shl 9),
)

/**
 * [Bitfield] container for all member permissions.
 */
public class MemberPermissions(bits: Long = 0): Bitfield(bits, genericMemberPermissions)

/**
 * Represents a type-safe permissions container for retrieving a [member permission][MemberPermissions]
 * from the [genericMemberPermissions] map.
 *
 * @param key Key from the [genericMemberPermissions] map.
 */
public sealed class MemberPermission(public val key: String) {
    public val bit: Long
        get() = genericMemberPermissions[key] ?: error("BUG: Key [$key] was not found in permission map.")

    /**
     * This member has permission to invite new members into the repository or organization,
     * and view all other repository or organization invites.
     */
    public object Invite: MemberPermission("member:invite")

    /**
     * This member can update any member's permissions.
     */
    public object Update: MemberPermission("member:update")

    /**
     * This member has the right to kick any member off the repository or organization's
     * member list.
     */
    public object Kick: MemberPermission("member:kick")

    /**
     * Represents a top-level object for all `metadata:` prefixed keys from the [genericMemberPermissions]. This can be anything
     * related to repository or organization metadata.
     */
    public sealed class Metadata {
        /**
         * Whether if this member has permission to update any repository or organization
         * metadata.
         */
        public object Update: MemberPermission("metadata:update")

        /**
         * Whether if this member has permission to delete any repository/organization metadata (i.e. repo releases)
         */
        public object Delete: MemberPermission("metadata:delete")
    }

    /**
     * Represents a top-level object for all `repo:` prefixed keys in the [genericMemberPermissions] map. This
     * is related to repository resources.
     *
     * * For organizations, this is all the repositories that is owned by the organization. In a future release,
     * this can be tied to specific repositories.
     * * For repositories, this is only the selected repository that the member is in.
     */
    public sealed class Repository {
        /**
         * Whether if this member has permission to create a repository in an organization. If this
         * is related to a repository member, this is no-op.
         */
        public object Create: MemberPermission("repo:create")

        /**
         * Whether if this member has permission to delete a repository resource.
         */
        public object Delete: MemberPermission("repo:delete")
    }

    public sealed class Webhooks {
        /**
         * Whether if this member has permission to create repository or organization
         * webhooks.
         */
        public object Create: MemberPermission("webhooks:create")

        /**
         * Whether if this member has permission to delete a repository or organization
         * webhook.
         */
        public object Delete: MemberPermission("webhooks:delete")

        /**
         * Whether if this member has permission to update any repository or organization
         * webhooks.
         */
        public object Update: MemberPermission("webhooks:update")
    }
}
