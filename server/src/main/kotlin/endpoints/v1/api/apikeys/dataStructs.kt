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

package org.noelware.charted.server.endpoints.v1.api.apikeys

import dev.floofy.utils.kotlin.every
import kotlinx.datetime.LocalDateTime
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.ChartedInfo
import org.noelware.charted.StringOverflowException
import org.noelware.charted.ValidationException
import org.noelware.charted.databases.postgres.flags.SCOPES
import org.noelware.charted.extensions.regexp.toNameRegex
import org.noelware.charted.serializers.ByteSizeValueSerializer
import kotlin.time.Duration.Companion.seconds

/**
 * Represents a resource for creating API keys.
 * @param description The key's description
 * @param expiresIn   [local date time][LocalDateTime] of when the API key will expire
 * @param scopes      List of [API Key scopes](https://charts.noelware.org/docs/server/current/api/api-keys#reference)
 * @param name        The name of the API key
 */
@Serializable
data class CreateApiKeyBody(
    val description: String? = null,

    @Serializable(with = ByteSizeValueSerializer::class)
    @SerialName("expires_in")
    val expiresIn: Long? = null,
    val scopes: List<String> = listOf(),
    val name: String
) {
    init {
        if (description != null && description.length > 140) {
            throw StringOverflowException("body.description", 140, description.length)
        }

        if (expiresIn != null && expiresIn < 30.seconds.inWholeMilliseconds) {
            throw ValidationException("body.expires_in", "Expiration time can't be under 30 seconds")
        }

        if (scopes.isNotEmpty()) {
            val isAddAll = scopes.size == 1 && scopes.first() == "*"
            if (!isAddAll && !scopes.every { SCOPES.containsKey(it) }) {
                val invalidScopes = scopes.filter { !SCOPES.containsKey(it) }
                throw ValidationException("body.scopes", "Invalid scopes: [${invalidScopes.joinToString(", ")}]")
            }
        }

        if (name.isBlank()) {
            throw ValidationException("body.name", "API key name can't be blank")
        }

        if (!name.toNameRegex(length = 64).matches()) {
            throw ValidationException("body.name", "API key name can only contain letters, digits, dashes, or underscores.")
        }

        if (name.length > 64) {
            throw StringOverflowException("body.name", 64)
        }
    }
}

@Serializable
data class PatchApiKeyScopes(
    val add: List<String> = listOf(),
    val remove: List<String> = listOf()
) {
    init {
        if (add.isNotEmpty()) {
            val isAddAll = add.size == 1 && add.first() == "*"
            if (!isAddAll && !add.every { SCOPES.containsKey(it) }) {
                val invalidScopes = add.filter { !SCOPES.containsKey(it) }
                throw ValidationException("body.scopes", "Invalid scopes: [${invalidScopes.joinToString(", ")}]")
            }
        }

        if (remove.isNotEmpty()) {
            val isAddAll = remove.size == 1 && remove.first() == "*"
            if (!isAddAll && !remove.every { SCOPES.containsKey(it) }) {
                val invalidScopes = remove.filter { !SCOPES.containsKey(it) }
                throw ValidationException("body.scopes", "Invalid scopes: [${invalidScopes.joinToString(", ")}]")
            }
        }
    }
}

@Serializable
data class PatchApiKey(
    val description: String? = null,
    val scopes: PatchApiKeyScopes? = null,
    val name: String? = null
) {
    init {
        if (description != null && description.length > 140) {
            throw StringOverflowException("body.description", 140, description.length)
        }

        if (name != null) {
            if (!name.toNameRegex(length = 64).matches()) {
                throw ValidationException("body.name", "API key name can only contain letters, digits, dashes, or underscores.")
            }

            if (name.length > 64) {
                throw StringOverflowException("body.name", 64)
            }
        }
    }
}

@Serializable
data class MainApiKeysResponse(
    val message: String = "Welcome to the API Keys endpoint!",

    @SerialName("docs_url")
    val docsUrl: String = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/apikeys"
)
