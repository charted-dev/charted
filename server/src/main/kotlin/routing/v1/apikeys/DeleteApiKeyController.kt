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
import org.jetbrains.exposed.sql.and
import org.noelware.charted.StringOverflowException
import org.noelware.charted.ValidationException
import org.noelware.charted.common.extensions.regexp.matchesNameAndIdRegex
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.models.flags.ApiKeyScope
import org.noelware.charted.modules.openapi.kotlin.dsl.*
import org.noelware.charted.modules.postgresql.controllers.apikeys.ApiKeysDatabaseController
import org.noelware.charted.modules.postgresql.tables.ApiKeyTable
import org.noelware.charted.server.extensions.addAuthenticationResponses
import org.noelware.charted.server.extensions.currentUserEntity
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController
import org.noelware.charted.server.routing.openapi.ResourceDescription
import org.noelware.charted.server.routing.openapi.describeResource

class DeleteApiKeyController(private val controller: ApiKeysDatabaseController): RestController("/apikeys/{idOrName}", HttpMethod.Delete) {
    override val apiVersion: APIVersion = APIVersion.V1
    override fun Route.init() {
        install(Sessions) {
            this += ApiKeyScope.ApiKeys.Delete
        }
    }

    override suspend fun call(call: ApplicationCall) {
        val name = call.parameters.getOrFail("idOrName")
        if (name.toLongOrNull() != null) {
            val id = name.toLong()
            controller.getEntityOrNull {
                (ApiKeyTable.id eq id) and (ApiKeyTable.owner eq call.currentUserEntity!!.id)
            } ?: return call.respond(
                HttpStatusCode.NotFound,
                ApiResponse.err(
                    "API_KEY_NOT_FOUND",
                    "API key with ID '$id' was not found ",
                ),
            )

            controller.delete(id)
            return call.respond(HttpStatusCode.Accepted, ApiResponse.ok())
        }

        if (!name.matchesNameAndIdRegex()) {
            throw ValidationException("body.name", "API key name can only contain letters, digits, dashes, or underscores.")
        }

        if (name.length > 64) {
            throw StringOverflowException("body.name", 64, name.length)
        }

        val apikey = controller.getEntityOrNull {
            (ApiKeyTable.name eq name) and (ApiKeyTable.owner eq call.currentUserEntity!!.id)
        } ?: return call.respond(
            HttpStatusCode.NotFound,
            ApiResponse.err(
                "API_KEY_NOT_FOUND",
                "API key with name '$name' was not found",
            ),
        )

        controller.delete(apikey.id.value)
        call.respond(HttpStatusCode.Accepted, ApiResponse.ok())
    }

    companion object: ResourceDescription by describeResource("/apikeys/{idOrName}", {
        delete {
            description = "Deletes an API key resource off the current authenticated user's account"

            idOrName()
            addAuthenticationResponses()
            accepted {
                description = "API key resource was deleted"
                json {
                    schema(ApiResponse.ok())
                }
            }

            notFound {
                description = "API key resource with name or ID was not found"
                json {
                    schema(
                        ApiResponse.err(
                            "API_KEY_NOT_FOUND",
                            "API key with name 'noel-is-cute' was not found",
                        ),
                    )
                }
            }
        }
    })
}
