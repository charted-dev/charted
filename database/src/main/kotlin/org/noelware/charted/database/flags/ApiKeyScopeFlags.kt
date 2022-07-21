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
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.database.flags

import org.noelware.charted.common.Bitfield

val SCOPE_FLAGS = mapOf(
    "repo:create" to (1L shl 0),
    "repo:delete" to (1L shl 1),
    "repo:update" to (1L shl 2),
    "repo:access" to (1L shl 3),
    "repo:invites" to (1L shl 4),
    "repo:write" to (1L shl 5),
    "notifications" to (1L shl 6),

    // Before you freak out. Yes, we do plan to create a closed source version
    // of charted-server and Pak for Enterprise use-cases. But, it won't be "totally"
    // closed sourced, we plan to let enterprise customers access the Enterprise version
    // on our GitLab version, employees can access it via https://lab.noelware.org/Noelware/charted/enterprise-server.
    "enterprise:access" to (1L shl 7),
    "enterprise:create" to (1L shl 8),
    "enterprise:update" to (1L shl 9),
    "enterprise:delete" to (1L shl 10),
    "enterprise:invites" to (1L shl 11),
    "organizations:create" to (1L shl 12),
    "organizations:delete" to (1L shl 13),
    "organizations:update" to (1L shl 14),
    "organizations:access" to (1L shl 15),
    "organizations:invites" to (1L shl 16),
    "user:update" to (1L shl 17),
    "user:delete" to (1L shl 18),
    "user:connections" to (1L shl 19),
    "user:avatar:update" to (1L shl 20),
    "user:sessions:view" to (1L shl 21),
    "repo:release:create" to (1L shl 22),
    "repo:release:update" to (1L shl 23),
    "repo:release:delete" to (1L shl 24),
    "repo:member:join" to (1L shl 25),
    "repo:member:kick" to (1L shl 26),
    "repo:member:update" to (1L shl 27),
    "org:member:join" to (1L shl 28),
    "org:member:kick" to (1L shl 29),
    "org:member:update" to (1L shl 30),
    "repo:webhooks:list" to (1L shl 31),
    "repo:webhooks:create" to (1L shl 32),
    "repo:webhooks:update" to (1L shl 33),
    "repo:webhooks:delete" to (1L shl 34)
)

class ApiKeyScopeFlags(originalBits: Long = 0): Bitfield(originalBits, SCOPE_FLAGS)
