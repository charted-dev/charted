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

package org.noelware.charted.server.endpoints.api

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import kotlinx.serialization.json.*
import org.noelware.charted.apikeys.ApiKeyManager
import org.noelware.charted.common.TimeParser
import org.noelware.charted.common.exceptions.StringOverflowException
import org.noelware.charted.common.exceptions.ValidationException
import org.noelware.charted.common.extensions.every
import org.noelware.charted.database.controllers.ApiKeyController
import org.noelware.charted.database.flags.ApiKeyScopeFlags
import org.noelware.charted.database.flags.SCOPE_FLAGS
import org.noelware.charted.server.plugins.Sessions
import org.noelware.charted.server.plugins.apiKeyKey
import org.noelware.charted.server.session
import org.noelware.ktor.body
import org.noelware.ktor.endpoints.AbstractEndpoint
import org.noelware.ktor.endpoints.Delete
import org.noelware.ktor.endpoints.Get
import org.noelware.ktor.endpoints.Put
import java.time.Duration
import kotlin.time.Duration.Companion.seconds
import kotlin.time.toKotlinDuration

@kotlinx.serialization.Serializable
data class CreateApiKeyBody(
    val expiresIn: String? = null,
    val scopes: List<String> = listOf(),
    val name: String
) {
    init {
        if (expiresIn != null) {
            if (expiresIn.toLongOrNull() == null) {
                val res = try {
                    TimeParser.fromString(expiresIn)
                } catch (e: Exception) {
                    throw ValidationException("body.expires_in", "Unable to parse [$expiresIn] into a valid duration.", e)
                }

                if (res < 30.seconds.inWholeMilliseconds) {
                    throw ValidationException("body.expires_in", "The expiration date cannot be under 30 seconds.")
                }
            }

            if (expiresIn.toLongOrNull() != null && expiresIn.toLong() < 30.seconds.inWholeMilliseconds) {
                throw ValidationException("body.expires_in", "The expiration date cannot be under 30 seconds.")
            }
        }

        // Check if every scope is valid
        if (scopes.isNotEmpty()) {
            val isAddAll = scopes.size == 1 && scopes.first() == "*"
            if (!isAddAll && !scopes.every { SCOPE_FLAGS.containsKey(it) }) {
                val invalidScopes = scopes.filter { !SCOPE_FLAGS.containsKey(it) }
                throw ValidationException("body.scopes", "Invalid scopes: [${invalidScopes.joinToString(", ")}]")
            }

            // Check if we are using Enterprise scopes
            // Since the OSS version doesn't support this!
            if (scopes.any { it.startsWith("enterprise:") }) {
                throw ValidationException("body.scopes", "Can't use Enterprise API key scopes on OSS version of charted-server [https://charts.noelware.org/enterprise]")
            }
        }

        if (name.isEmpty()) {
            throw ValidationException("body.name", "API key name can't be blank")
        }

        if (name.length > 64) {
            throw StringOverflowException("body.name", 64)
        }
    }
}

class ApiKeysEndpoints(private val apikeys: ApiKeyManager): AbstractEndpoint("/apikeys") {
    init {
        install(HttpMethod.Delete, "/apikeys", Sessions)
        install(HttpMethod.Put, "/apikeys", Sessions)
        install("/apikeys/{name}", Sessions)
        install("/apikeys/all", Sessions)
    }

    @Get
    suspend fun main(call: ApplicationCall) {
        call.respond(
            HttpStatusCode.OK,
            buildJsonObject {
                put("success", true)
                put(
                    "data",
                    buildJsonObject {
                        put("message", "Welcome to the API Keys API!")
                        put("docs", "https://charts.noelware.org/docs/api/api-keys")
                    }
                )
            }
        )
    }

    @Get("/all")
    suspend fun all(call: ApplicationCall) {
        val keys = ApiKeyController.getAll(call.session.userID)
        call.respond(
            HttpStatusCode.OK,
            buildJsonObject {
                put("success", true)
                putJsonArray("data") {
                    for (key in keys) {
                        add(key.toJsonObject())
                    }
                }
            }
        )
    }

    @Get("/{name}")
    suspend fun byName(call: ApplicationCall) {
        val name = call.parameters["name"]!!
        val key = ApiKeyController.get(call.session.userID, name)
            ?: return call.respond(
                HttpStatusCode.NotFound,
                buildJsonObject {
                    put("success", false)
                    putJsonArray("errors") {
                        addJsonObject {
                            put("code", "UNKNOWN_API_KEY")
                            put("message", "Unknown API key with name [$name]")
                        }
                    }
                }
            )

        call.respond(
            HttpStatusCode.OK,
            buildJsonObject {
                put("success", true)
                put("data", key.toJsonObject())
            }
        )
    }

    @Put
    suspend fun create(call: ApplicationCall) {
        val body by call.body<CreateApiKeyBody>()
        val expiration = if (body.expiresIn != null) {
            val durationInMs = TimeParser.fromString(body.expiresIn!!)
            Duration.ofMillis(durationInMs).toKotlinDuration()
        } else {
            null
        }

        val bitfield = ApiKeyScopeFlags(0)
        if (body.scopes.size == 1 && body.scopes.first() == "*") {
            bitfield.addAll()
        } else {
            for (key in body.scopes) {
                bitfield.add(key)
            }
        }

        // Remove enterprise stuff since it's not supported in OSS.
        for (key in bitfield.flags.keys.filter { it.startsWith("enterprise:") }) {
            bitfield.remove(bitfield.flags[key]!!)
        }

        val apiKey = apikeys.createApiKey(
            body.name,
            call.session.userID,
            bitfield.bits,
            expiration
        )

        call.respond(
            HttpStatusCode.OK,
            buildJsonObject {
                put("success", false)
                put("data", apiKey.toJsonObject())
            }
        )
    }

    @Delete
    suspend fun delete(call: ApplicationCall) {
        val token = call.attributes.getOrNull(apiKeyKey)
            ?: return call.respond(
                HttpStatusCode.Forbidden,
                buildJsonObject {
                    put("success", false)
                    putJsonArray("errors") {
                        addJsonObject {
                            put("code", "MUST_USE_API_KEY")
                            put("message", "You must supply a API key, not a session token to delete a API key.")
                        }
                    }
                }
            )

        val success = ApiKeyController.delete(token.token!!)
        call.respond(
            if (success) HttpStatusCode.Accepted else HttpStatusCode.Forbidden,
            buildJsonObject {
                put("success", success)
            }
        )
    }
}
