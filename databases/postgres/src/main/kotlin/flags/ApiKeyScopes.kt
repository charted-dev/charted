/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

package org.noelware.charted.databases.postgres.flags

import dev.floofy.utils.slf4j.logging
import org.noelware.charted.common.Bitfield

// As of v0.4-nightly, the API scopes were changed, so this is empty (for now).
private val deprecatedApiScopes = mapOf<String, Long>()

val SCOPES: Map<String, Long> = mapOf(
    // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
    //           User Scopes
    // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
    "user:access" to (1L shl 0),
    "user:update" to (1L shl 1),
    "user:delete" to (1L shl 2),
    "user:connections" to (1L shl 3),
    "user:notifications" to (1L shl 4),
    "user:avatar:update" to (1L shl 5),
    "user:sessions:list" to (1L shl 6),

    // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
    //        Repository Scopes
    // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
    "repo:access" to (1L shl 7),
    "repo:create" to (1L shl 8),
    "repo:delete" to (1L shl 9),
    "repo:update" to (1L shl 10),
    "repo:write" to (1L shl 11),
    "repo:icons:update" to (1L shl 12),
    "repo:releases:create" to (1L shl 13),
    "repo:releases:update" to (1L shl 14),
    "repo:releases:delete" to (1L shl 15),
    "repo:members:list" to (1L shl 16),
    "repo:members:update" to (1L shl 17),
    "repo:members:kick" to (1L shl 18),
    "repo:members:invites:access" to (1L shl 19),
    "repo:members:invites:create" to (1L shl 20),
    "repo:members:invites:delete" to (1L shl 21),
    "repo:webhooks:list" to (1L shl 22),
    "repo:webhooks:create" to (1L shl 23),
    "repo:webhooks:update" to (1L shl 24),
    "repo:webhooks:delete" to (1L shl 25),
    "repo:webhooks:events:access" to (1L shl 26),
    "repo:webhooks:events:delete" to (1L shl 27),

    // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
    //        API Key Scopes
    // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
    "apikeys:view" to (1L shl 28),
    "apikeys:create" to (1L shl 29),
    "apikeys:delete" to (1L shl 30),
    "apikeys:update" to (1L shl 31),

    // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
    //      Organization Scopes
    // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
    "org:access" to (1L shl 32),
    "org:create" to (1L shl 33),
    "org:update" to (1L shl 34),
    "org:delete" to (1L shl 35),
    "org:members:invites" to (1L shl 36),
    "org:members:list" to (1L shl 37),
    "org:members:kick" to (1L shl 38),
    "org:members:update" to (1L shl 39),
    "org:webhooks:list" to (1L shl 40),
    "org:webhooks:create" to (1L shl 41),
    "org:webhooks:update" to (1L shl 42),
    "org:webhooks:delete" to (1L shl 43),
    "org:webhooks:events:list" to (1L shl 44),
    "org:webhooks:events:delete" to (1L shl 45),

    // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
    //    Administration Scopes
    // +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
    "admin:stats" to (1L shl 46),

    "repo:metadata:update" to (1L shl 47),
    "repo:metadata:delete" to (1L shl 48),
)

class ApiKeyScopes(bits: Long = 0) : Bitfield(bits, SCOPES) {
    private val log by logging<ApiKeyScopes>()

    override fun add(flag: String): Bitfield {
        if (deprecatedApiScopes.containsKey(flag)) {
            log.warn("API key scope [$flag (${deprecatedApiScopes[flag]})] is deprecated and will be removed in a future release")
        }

        return super.add(flag)
    }

    override fun add(vararg bits: Long): Bitfield {
        for (bit in bits) {
            if (deprecatedApiScopes.containsValue(bit)) {
                log.warn("API key scope [$bit] is deprecated and will be removed in a future release")
            }
        }

        return super.add(*bits)
    }
}
