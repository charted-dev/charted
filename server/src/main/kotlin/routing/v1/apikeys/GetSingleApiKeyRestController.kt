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

package org.noelware.charted.server.routing.v1.apikeys

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.ktor.server.util.*
import io.swagger.v3.oas.models.PathItem
import org.jetbrains.exposed.sql.and
import org.noelware.charted.common.extensions.regexp.matchesNameAndIdRegex
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.models.ApiKeys
import org.noelware.charted.modules.openapi.NameOrSnowflake
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.openapi.toPaths
import org.noelware.charted.modules.postgresql.controllers.apikeys.ApiKeysDatabaseController
import org.noelware.charted.modules.postgresql.extensions.fromEntity
import org.noelware.charted.modules.postgresql.tables.ApiKeyTable
import org.noelware.charted.server.extensions.addAuthenticationResponses
import org.noelware.charted.server.extensions.currentUserEntity
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.routing.RestController

class GetSingleApiKeyRestController(private val controller: ApiKeysDatabaseController): RestController("/apikeys/{nameOrId}") {
    override fun Route.init() {
        install(Sessions)
    }

    override suspend fun call(call: ApplicationCall) {
        val nameOrId = call.parameters.getOrFail("nameOrId")
        val apikey = when {
            nameOrId.toLongOrNull() != null -> controller.getEntityOrNull {
                (ApiKeyTable.id eq nameOrId.toLong()) and (ApiKeyTable.owner eq call.currentUserEntity!!.id)
            }

            nameOrId.matchesNameAndIdRegex() -> controller.getEntityOrNull {
                (ApiKeyTable.name eq nameOrId) and (ApiKeyTable.owner eq call.currentUserEntity!!.id)
            }

            else -> null
        } ?: return call.respond(
            HttpStatusCode.NotFound,
            ApiResponse.err(
                "API_KEY_NOT_FOUND",
                "API key with name or ID '$nameOrId' was not found",
            ),
        )

        call.respond(HttpStatusCode.OK, ApiResponse.ok(ApiKeys.fromEntity(apikey)))
    }

    override fun toPathDsl(): PathItem = toPaths("/apikeys/{nameOrId}") {
        get {
            description = "Returns a single API key resource owned by the current authenticated user"

            pathParameter {
                description = "Name of the given API key to delete, or a Snowflake value to delete the API key by"
                name = "name"

                schema<NameOrSnowflake>()
            }

            addAuthenticationResponses()
            response(HttpStatusCode.OK) {
                contentType(ContentType.Application.Json) {
                    schema<ApiResponse.Ok<ApiKeys>>()
                }
            }

            response(HttpStatusCode.NotFound) {
                contentType(ContentType.Application.Json) {
                    schema(
                        ApiResponse.err(
                            "API_KEY_NOT_FOUND",
                            "API key with name 'noel-is-cute' was not found",
                        ),
                    )
                }
            }
        }
    }
}
