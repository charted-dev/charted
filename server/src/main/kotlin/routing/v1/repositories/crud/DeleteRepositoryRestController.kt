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

package org.noelware.charted.server.routing.v1.repositories.crud

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.ktor.server.util.*
import io.swagger.v3.oas.models.PathItem
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.models.flags.ApiKeyScope.Repositories
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.openapi.toPaths
import org.noelware.charted.modules.postgresql.controllers.repositories.RepositoryController
import org.noelware.charted.server.extensions.addAuthenticationResponses
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.routing.RestController

class DeleteRepositoryRestController(private val controller: RepositoryController): RestController("/repositories/{id}", HttpMethod.Delete) {
    override fun Route.init() {
        install(Sessions) {
            this += Repositories.Delete
        }
    }

    override suspend fun call(call: ApplicationCall) {
        val id = call.parameters.getOrFail("id").toLongOrNull()
            ?: return call.respond(
                HttpStatusCode.UnprocessableEntity,
                ApiResponse.err(
                    "UNABLE_TO_PARSE",
                    "Unable to convert into a Snowflake",
                ),
            )

        controller.delete(id)
        call.respond(HttpStatusCode.Accepted, ApiResponse.ok())
    }

    override fun toPathDsl(): PathItem = toPaths("/repositories/{id}") {
        delete {
            description = "Deletes a repository"

            addAuthenticationResponses()
            response(HttpStatusCode.Accepted) {
                description = "The repository was deleted successfully"
                contentType(ContentType.Application.Json) {
                    schema(ApiResponse.ok())
                }
            }

            response(HttpStatusCode.UnprocessableEntity) {
                description = "If the `id` path parameter couldn't be into a valid Snowflake"
                contentType(ContentType.Application.Json) {
                    schema(
                        ApiResponse.err(
                            "UNABLE_TO_PARSE",
                            "Unable to convert into a Snowflake",
                        ),
                    )
                }
            }
        }
    }
}
