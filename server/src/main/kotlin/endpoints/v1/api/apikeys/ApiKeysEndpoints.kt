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

package org.noelware.charted.server.endpoints.v1.api.apikeys

import guru.zoroark.tegral.openapi.dsl.RootDsl
import guru.zoroark.tegral.openapi.dsl.schema
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import kotlinx.datetime.Clock
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import kotlinx.serialization.json.buildJsonObject
import org.jetbrains.exposed.sql.and
import org.noelware.charted.RandomStringGenerator
import org.noelware.charted.common.CryptographyUtils
import org.noelware.charted.databases.postgres.asyncTransaction
import org.noelware.charted.databases.postgres.entities.ApiKeyEntity
import org.noelware.charted.databases.postgres.flags.ApiKeyScopes
import org.noelware.charted.databases.postgres.flags.SCOPES
import org.noelware.charted.databases.postgres.models.ApiKeys
import org.noelware.charted.databases.postgres.tables.ApiKeysTable
import org.noelware.charted.extensions.json.toJsonArray
import org.noelware.charted.modules.apikeys.ApiKeyManager
import org.noelware.charted.server.openapi.extensions.externalDocsUrl
import org.noelware.charted.server.plugins.SessionsPlugin
import org.noelware.charted.server.plugins.currentUserEntity
import org.noelware.charted.snowflake.Snowflake
import org.noelware.charted.types.responses.ApiError
import org.noelware.charted.types.responses.ApiResponse
import org.noelware.ktor.endpoints.*
import kotlin.time.Duration.Companion.milliseconds

class ApiKeysEndpoints(private val apiKeyManager: ApiKeyManager, private val snowflake: Snowflake): AbstractEndpoint("/apikeys") {
    init {
        install(HttpMethod.Delete, "/apikeys/{name}", SessionsPlugin) {
            assertSessionOnly = true
            this += "apikeys:delete"
        }

        install(HttpMethod.Patch, "/apikeys/{name}", SessionsPlugin) {
            this += "apikeys:update"
        }

        install(HttpMethod.Get, "/apikeys/{name}", SessionsPlugin) {
            this += "apikeys:view"
        }

        install(HttpMethod.Put, "/apikeys", SessionsPlugin) {
            this += "apikeys:create"
        }
    }

    @Get
    suspend fun main(call: ApplicationCall): Unit = call.respond(HttpStatusCode.OK, MainApiKeysResponse())

    @Put
    suspend fun createApiKey(call: ApplicationCall) {
        val body: CreateApiKeyBody = call.receive()
        val expiresIn = if (body.expiresIn != null) body.expiresIn.milliseconds else null

        val bitfield = ApiKeyScopes()
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
                                    put("provided", keys.toJsonArray())
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
                this.expiresIn = if (expiresIn != null) Clock.System.now().plus(expiresIn).toLocalDateTime(TimeZone.currentSystemDefault()) else null
                this.token = CryptographyUtils.sha256Hex(token)
                scopes = bitfield.bits()
                owner = call.currentUserEntity!!
                name = body.name
            }
        }.let { entity -> ApiKeys.fromEntity(entity, true) }

        if (expiresIn != null) apiKeyManager.send(apiKey, expiresIn)
        call.respond(HttpStatusCode.Created, ApiResponse.ok(apiKey))
    }

    @Get("/{name}")
    suspend fun getByName(call: ApplicationCall) {
        val apiKey = asyncTransaction {
            ApiKeyEntity.find { (ApiKeysTable.name eq call.parameters["name"]!!) and (ApiKeysTable.owner eq call.currentUserEntity!!.id) }.firstOrNull()?.let { entity -> ApiKeys.fromEntity(entity, false) }
        } ?: return call.respond(
            HttpStatusCode.NotFound,
            ApiResponse.err(
                "UNKNOWN_API_KEY", "API key with name [${call.parameters["name"]}] was not found.",
            ),
        )

        call.respond(HttpStatusCode.OK, ApiResponse.ok(apiKey))
    }

    @Patch("/{name}")
    suspend fun patchApiKey(call: ApplicationCall) {
        val patch: PatchApiKey = call.receive()
    }

    @Delete("/{name}")
    suspend fun deleteApiKey(call: ApplicationCall) {
        // Check if we can find it
        asyncTransaction {
            ApiKeyEntity.find { (ApiKeysTable.name eq call.parameters["name"]!!) and (ApiKeysTable.owner eq call.currentUserEntity!!.id) }.firstOrNull()?.let { entity -> ApiKeys.fromEntity(entity, false) }
        } ?: return call.respond(
            HttpStatusCode.NotFound,
            ApiResponse.err(
                "UNKNOWN_API_KEY", "API key with name [${call.parameters["name"]}] was not found.",
            ),
        )

        // If we can, then delete it
    }

    companion object {
        fun RootDsl.toOpenAPI() {
            "/apikeys" get {
                description = "Generic entrypoint message for this route"
                externalDocsUrl("apikeys", "GET-/apikeys")

                200 response {
                    "application/json" content {
                        schema<ApiResponse.Ok<MainApiKeysResponse>>()
                    }
                }
            }

            "/apikeys" put {
            }

            "/apikeys/{name}" patch {
            }

            "/apikeys/{name}" delete {
            }

            "/apikeys/{name}" get {
            }
        }
    }
}
