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

import org.noelware.charted.common.Bitfield

val SCOPE_FLAGS = mapOf(
    "repo:create" to (1 shl 0).toLong(),
    "repo:delete" to (1 shl 1).toLong(),
    "repo:update" to (1 shl 2).toLong(),
    "repo:access" to (1 shl 3).toLong(),
    "repo:invites" to (1 shl 4).toLong(),
    "repo:write" to (1 shl 5).toLong(),
    "notifications" to (1 shl 6).toLong(),

    // Before you freak out. Yes, we do plan to create a closed source version
    // of charted-server and Pak for Enterprise use-cases. But, it won't be "totally"
    // closed sourced, we plan to let enterprise customers access the Enterprise version
    // on our GitLab version, employees can access it via https://lab.noelware.org/Noelware/charted/enterprise-server.
    "enterprise:access" to (1 shl 7).toLong(),
    "enterprise:create" to (1 shl 8).toLong(),
    "enterprise:update" to (1 shl 9).toLong(),
    "enterprise:delete" to (1 shl 10).toLong(),
    "enterprise:invites" to (1 shl 11).toLong(),
    "organizations:create" to (1 shl 12).toLong(),
    "organizations:delete" to (1 shl 13).toLong(),
    "organizations:update" to (1 shl 14).toLong(),
    "organizations:access" to (1 shl 15).toLong(),
    "organizations:invites" to (1 shl 16).toLong(),
    "user:update" to (1 shl 17).toLong(),
    "user:delete" to (1 shl 18).toLong(),
    "user:connections" to (1 shl 19).toLong(),
    "user:avatar:update" to (1 shl 20).toLong(),
    "user:sessions:view" to (1 shl 21).toLong(),
    "repo:release:create" to (1 shl 22).toLong(),
    "repo:release:update" to (1 shl 23).toLong(),
    "repo:release:delete" to (1 shl 24).toLong(),
    "repo:member:join" to (1 shl 25).toLong(),
    "repo:member:kick" to (1 shl 26).toLong(),
    "repo:member:update" to (1 shl 27).toLong(),
    "org:member:join" to (1 shl 28).toLong(),
    "org:member:kick" to (1 shl 29).toLong(),
    "org:member:update" to (1 shl 30).toLong()
)

class ApiKeyScopeFlags(originalBits: Long = 0): Bitfield(originalBits, SCOPE_FLAGS)
