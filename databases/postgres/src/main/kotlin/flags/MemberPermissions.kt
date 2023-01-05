/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware <team@noelware.org>
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

package org.noelware.charted.databases.postgres.flags

import org.noelware.charted.common.Bitfield

private val PERMISSIONS: Map<String, Long> = mapOf(
    // This member has permission to invite new members into the repository
    "member:invite" to (1L shl 0),

    // This member has permission to update any member's permissions
    "member:update" to (1L shl 1),

    // This member has permission to kick any members off the repository
    "member:kick" to (1L shl 2),

    // This member has permission to update the repository (or any repositories if this
    // represents an organization member)
    "repo:update" to (1L shl 3),
)

class MemberPermissions(bits: Long = 0): Bitfield(bits, PERMISSIONS)
