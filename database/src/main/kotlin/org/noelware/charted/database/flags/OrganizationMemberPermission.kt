/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022 Noelware <team@noelware.org>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *    http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.database.flags

import org.noelware.charted.util.Bitfield

private val FLAGS_MAP = mapOf(
    // If this member is allowed to edit the organization settings.
    "EDIT_SETTINGS" to (1L shl 0),

    // If this member is allowed to view private repositories
    "VIEW_PRIVATE_REPOS" to (1L shl 1),

    // If this member is allowed to moderate other members (like kicking them from the org)
    "MODERATE_MEMBERS" to (1L shl 2),

    // If the member should be overridden from all permissions above.
    "ADMIN" to (1L shl 3)
)

class OrganizationMemberPermission(originalBits: Long): Bitfield(FLAGS_MAP, originalBits)

fun Long.toOrgMemberPermission(): OrganizationMemberPermission = OrganizationMemberPermission(this)
