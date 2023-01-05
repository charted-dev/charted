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

package org.noelware.charted.modules.docker.registry.authorization

import kotlinx.serialization.Serializable
import org.noelware.charted.common.Bitfield

/** Represents a [Bitfield] of the available authorization scopes a user can access */
class RegistryAuthorizationScopes(bits: Long = 0L): Bitfield(
    bits,
    mapOf(
        "push" to (1L shl 0),
        "pull" to (1L shl 1),
    ),
)

/**
 * Represents an authentication token to provide authorization towards pushing
 * and pulling repositories.
 *
 * @param userID The user's ID that this authorization token belongs to
 * @param scopes The different scopes that the token has available, use the [hasPush] or [hasPull]
 *               extensions to determine if the token is allowed to push OCI charts or not
 * @param token  The JWT token that was generated when `docker login` was used.
 */
@Serializable
data class RegistryAuthorizationToken(
    val userID: Long,
    val scopes: Long = 0,
    val token: String
)

private val RegistryAuthorizationToken.scopeBits: RegistryAuthorizationScopes
    get() = RegistryAuthorizationScopes(scopes)

/** Checks if this token has permission to push to OCI charts it has access towards or not. */
val RegistryAuthorizationToken.hasPush: Boolean
    get() = scopeBits.has(1L shl 0)

/** Checks if this token has permission to pull OCI charts of private repositories or not */
val RegistryAuthorizationToken.hasPull: Boolean
    get() = scopeBits.has(1L shl 1)
