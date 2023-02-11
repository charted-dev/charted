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

@file:Suppress("unused")

package org.noelware.charted.server.endpoints.v1.api

import dev.floofy.utils.kotlin.every
import guru.zoroark.tegral.openapi.dsl.RootDsl
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import kotlinx.datetime.Clock
import kotlinx.datetime.LocalDateTime
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonArray
import kotlinx.serialization.json.JsonPrimitive
import kotlinx.serialization.json.buildJsonObject
import org.jetbrains.exposed.sql.SqlExpressionBuilder.eq
import org.jetbrains.exposed.sql.deleteWhere
import org.noelware.charted.*
import org.noelware.charted.common.CryptographyUtils
import org.noelware.charted.databases.postgres.asyncTransaction
import org.noelware.charted.databases.postgres.entities.ApiKeyEntity
import org.noelware.charted.databases.postgres.flags.ApiKeyScopes
import org.noelware.charted.databases.postgres.flags.SCOPES
import org.noelware.charted.databases.postgres.models.ApiKeys
import org.noelware.charted.databases.postgres.tables.ApiKeysTable
import org.noelware.charted.extensions.regexp.toNameRegex
import org.noelware.charted.modules.apikeys.ApiKeyManager
import org.noelware.charted.serializers.TimeSpanValueSerializer
import org.noelware.charted.server.plugins.API_KEY_KEY
import org.noelware.charted.server.plugins.SessionsPlugin
import org.noelware.charted.server.plugins.currentUser
import org.noelware.charted.server.plugins.currentUserEntity
import org.noelware.charted.snowflake.Snowflake
import org.noelware.charted.types.responses.ApiError
import org.noelware.charted.types.responses.ApiResponse
import org.noelware.ktor.body
import org.noelware.ktor.endpoints.AbstractEndpoint
import org.noelware.ktor.endpoints.Delete
import org.noelware.ktor.endpoints.Get
import org.noelware.ktor.endpoints.Put
import kotlin.time.Duration.Companion.milliseconds
import kotlin.time.Duration.Companion.seconds

/**
 * Represents a resource for creating API keys. A user is only allocated ~15 API keys that can be created,
 * updated, or destroyed.
 *
 * @param description The key's description
 * @param expiresIn   [local date time][LocalDateTime] of when the API key will expire
 * @param scopes      List of [API Key scopes](https://charts.noelware.org/docs/server/current/api/api-keys#reference)
 * @param name        The name of the API key
 */
@Serializable
data class CreateApiKeyBody(
    val description: String? = null,

    @Serializable(with = TimeSpanValueSerializer::class)
    @SerialName("expires_in")
    val expiresIn: Long? = null,
    val scopes: List<String> = listOf(),
    val name: String
) {
    init {
        if (description != null && description.length > 140) {
            throw StringOverflowException("body.description", 140, description.length)
        }

        if (expiresIn != null) {
            if (expiresIn < 30.seconds.inWholeMilliseconds) {
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

            if (!(name matches "^([A-z]|-|_|\\d{0,9}){0,32}".toRegex())) {
                throw ValidationException("body.name", "API key name can only contain letters, digits, dashes, or underscores.")
            }

            if (name.length > 64) {
                throw StringOverflowException("body.name", 64)
            }
        }
    }
}

@Serializable
data class ApiKeysResponse(
    val message: String = "Welcome to the API Keys endpoint!",

    @SerialName("docs_url")
    val docsUrl: String = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/apikeys"
)

class ApiKeysEndpoint(
    private val apikeys: ApiKeyManager,
    private val snowflake: Snowflake
) : AbstractEndpoint("/apikeys") {
    init {
        install(HttpMethod.Delete, "/apikeys/{id}", SessionsPlugin) {
            this += "apikeys:delete"
        }

//        install(HttpMethod.Patch, "/apikeys/{perm}", SessionsPlugin) {
//            this += "apikeys:edit:perms"
//        }
//
//        install(HttpMethod.Patch, "/apikeys", SessionsPlugin) {
//            this += "apikeys:update"
//        }

        install(HttpMethod.Put, "/apikeys", SessionsPlugin) {
            this += "apikeys:view"
        }

        install(HttpMethod.Get, "/apikeys/{name}", SessionsPlugin) {
            this += "apikeys:view"
        }

        install(HttpMethod.Get, "/apikeys/all", SessionsPlugin) {
            this += "apikeys:view"
        }
    }

    @Get
    suspend fun main(call: ApplicationCall) = call.respond(HttpStatusCode.OK, ApiResponse.ok(ApiKeysResponse()))

    @Get("/all")
    suspend fun all(call: ApplicationCall) {
        val keys = asyncTransaction {
            ApiKeyEntity.find { ApiKeysTable.owner eq call.currentUser!!.id }.toList().map { entity ->
                ApiKeys.fromEntity(entity)
            }
        }

        call.respond(HttpStatusCode.OK, ApiResponse.ok(keys))
    }

    @Get("/{name}")
    suspend fun byName(call: ApplicationCall) {
        val name = call.parameters["name"]!!
        if (!name.toNameRegex(true).matches()) {
            throw ValidationException("param.name", "API key name can only contain letters, digits, dashes, or underscores.")
        }

        val key = asyncTransaction {
            ApiKeyEntity.find { ApiKeysTable.owner eq call.currentUser!!.id }.firstOrNull()?.let { entity ->
                ApiKeys.fromEntity(entity)
            }
        }

        call.respond(HttpStatusCode.OK, ApiResponse.ok(key))
    }

    @Put
    suspend fun create(call: ApplicationCall) {
        val body: CreateApiKeyBody = call.receive()
        val expiresIn = if (body.expiresIn != null) {
            body.expiresIn.milliseconds
        } else {
            null
        }

        val bitfield = ApiKeyScopes(0)
        if (body.scopes.size == 1 && body.scopes.first() == "*") {
            bitfield.addAll()
        } else {
            val keys = mutableListOf<String>()
            for (key in body.scopes) {
                if (!SCOPES.containsKey(key)) {
                    keys.add(key)
                } else {
                    bitfield.add(key)
                }
            }

            if (keys.size > 0) {
                return call.respond(
                    HttpStatusCode.NotAcceptable,
                    ApiResponse.err(
                        keys.map { key ->
                            ApiError(
                                "UNKNOWN_BITFIELD_FLAG", "Bitfield flag [$key] doesn't exist in as a API key scope",
                                buildJsonObject {
                                    put("provided", JsonArray(keys.toList().map { JsonPrimitive(it) }))
                                },
                            )
                        },
                    ),
                )
            }
        }

        val token = RandomStringGenerator.generate(32)
        val id = snowflake.generate()
        val apiKey = asyncTransaction {
            ApiKeyEntity.new(id.value) {
                this.expiresIn = if (expiresIn != null) {
                    Clock.System.now().plus(expiresIn)
                        .toLocalDateTime(TimeZone.currentSystemDefault())
                } else {
                    null
                }

                // We store it as a hash so people can't indirectly get the token
                // from the database.
                this.token = CryptographyUtils.sha256Hex(token)
                description = body.description
                scopes = bitfield.bits()
                owner = call.currentUserEntity!!
                name = body.name
            }.let { entity -> ApiKeys.fromEntity(entity, true, token) }
        }

        if (expiresIn != null) {
            apikeys.send(apiKey, expiresIn)
        }

        call.respond(HttpStatusCode.Created, ApiResponse.ok(apiKey))
    }

    @Delete("/{id}")
    suspend fun delete(call: ApplicationCall) {
        val token = call.attributes.getOrNull(API_KEY_KEY)
            ?: return call.respond(
                HttpStatusCode.Forbidden,
                ApiResponse.err("MUST_USE_API_KEY", "You need to supply a API key, not a session token to delete it."),
            )

        asyncTransaction {
            ApiKeysTable.deleteWhere { ApiKeysTable.id eq token.id }
        }

        call.respond(
            HttpStatusCode.Accepted,
            ApiResponse.ok(),
        )
    }

    companion object {
        fun RootDsl.toOpenAPI() {
        }
    }
}
