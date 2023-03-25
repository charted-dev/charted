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

package org.noelware.charted.modules.postgresql.controllers.apikeys

import com.fasterxml.jackson.annotation.JsonProperty
import dev.floofy.utils.kotlin.every
import io.swagger.v3.oas.annotations.media.Schema
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.StringOverflowException
import org.noelware.charted.ValidationException
import org.noelware.charted.common.TimeSpan
import org.noelware.charted.common.extensions.regexp.matchesNameAndIdRegex
import org.noelware.charted.models.flags.SCOPES
import kotlin.time.Duration.Companion.seconds

/**
 * Represents a resource for creating API keys. A user is only allocated ~15 API keys that can be created,
 * updated, or destroyed.
 *
 * @param description The key's description
 * @param expiresIn   [TimeSpan] of when the API key will expire
 * @param scopes      List of [API Key scopes](https://charts.noelware.org/docs/server/current/api/api-keys#reference)
 * @param name        The name of the API key
 */
@Schema(description = "Represents a resource for creating API keys")
@Serializable
data class CreateApiKeyPayload(
    val description: String? = null,

    @JsonProperty("expires_in")
    @SerialName("expires_in")
    val expiresIn: TimeSpan? = null,
    val scopes: List<String> = listOf(),
    val name: String
) {
    init {
        if (description != null && description.length > 140) {
            throw StringOverflowException("body.description", 140, description.length)
        }

        if (expiresIn != null) {
            if (expiresIn.value < 30.seconds.inWholeMilliseconds) {
                throw ValidationException("body.expires_in", "Expiration time can't be under 30 seconds")
            }

            if (scopes.isNotEmpty()) {
                val isAddAll = scopes.size == 1 && scopes.first() == "*"
                if (!isAddAll && !scopes.every { SCOPES.containsKey(it) }) {
                    val invalidScopes = scopes.filter { !SCOPES.containsKey(it) }
                    throw ValidationException("body.scopes", "Invalid scopes: [${invalidScopes.joinToString(", ")}]")
                }
            }
        }

        if (name.isBlank()) {
            throw ValidationException("body.name", "API key name can't be blank")
        }

        if (!name.matchesNameAndIdRegex()) {
            throw ValidationException("body.name", "API key name can only contain letters, digits, dashes, or underscores.")
        }

        if (name.length > 64) {
            throw StringOverflowException("body.name", 64, name.length)
        }
    }
}
