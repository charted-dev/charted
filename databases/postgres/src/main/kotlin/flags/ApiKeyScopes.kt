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

package org.noelware.charted.databases.postgres.flags

import dev.floofy.utils.slf4j.logging
import org.noelware.charted.common.Bitfield

private val DEPRECATED_SCOPES = mapOf(
    "repo:invites" to (1L shl 13),
    "repo:access" to (1L shl 11)
)

val SCOPES: Map<String, Long> = mapOf(
    // users
    "user:view" to (1L shl 0),
    "user:update" to (1L shl 1),
    "user:delete" to (1L shl 2),
    "user:notifications" to (1L shl 3),
    "user:connections" to (1L shl 4),
    "user:avatar:update" to (1L shl 5),
    "user:sessions:view" to (1L shl 6),

    // repositories
    "repo:view" to (1L shl 7),
    "repo:create" to (1L shl 8),
    "repo:delete" to (1L shl 9),
    "repo:update" to (1L shl 10),
    "repo:access" to (1L shl 11),
    "repo:write" to (1L shl 12),

    // `repo:invites` is deprecated and will be removed in a future release
    // (maybe v0.4-nightly?)
    "repo:invites" to (1L shl 13),
    "repo:release:create" to (1L shl 14),
    "repo:release:update" to (1L shl 15),
    "repo:release:delete" to (1L shl 16),
    "repo:member:invites" to (1L shl 17),
    "repo:member:kick" to (1L shl 18),
    "repo:member:update" to (1L shl 19),
    "repo:webhooks:list" to (1L shl 20),
    "repo:webhooks:create" to (1L shl 21),
    "repo:webhooks:update" to (1L shl 22),
    "repo:webhooks:delete" to (1L shl 23),
    "repo:member:invites:create" to (1L shl 40),
    "repo:member:invites:delete" to (1L shl 41),
    "repo:member:invites:access" to (1L shl 42),
    "repo:member:view" to (1L shl 43),

    // organizations
    "org:view" to (1L shl 24), // view private organizations also, not just public ones
    "org:create" to (1L shl 25),
    "org:update" to (1L shl 26),
    "org:delete" to (1L shl 27),
    "org:invites" to (1L shl 28),
    "org:member:invites" to (1L shl 29),
    "org:member:kick" to (1L shl 30),
    "org:member:update" to (1L shl 31),
    "org:webhooks:list" to (1L shl 32),
    "org:webhooks:create" to (1L shl 33),
    "org:webhooks:update" to (1L shl 34),
    "org:webhooks:delete" to (1L shl 35),

    // api key usage
    "apikeys:view" to (1L shl 36),
    "apikeys:create" to (1L shl 37),
    "apikeys:delete" to (1L shl 38),
    "apikeys:edit:perms" to (1L shl 39)
)

class ApiKeyScopes(bits: Long = 0): Bitfield(bits, SCOPES) {
    private val log by logging<ApiKeyScopes>()

    override fun add(flag: String): Bitfield {
        if (DEPRECATED_SCOPES.containsKey(flag)) {
            log.warn("API key scope [$flag (${DEPRECATED_SCOPES[flag]})] is deprecated and will be removed in a future release")
        }

        return super.add(flag)
    }

    override fun add(vararg bits: Long): Bitfield {
        for (bit in bits) {
            if (DEPRECATED_SCOPES.containsValue(bit)) {
                log.warn("API key scope [$bit] is deprecated and will be removed in a future release")
            }
        }

        return super.add(*bits)
    }
}
