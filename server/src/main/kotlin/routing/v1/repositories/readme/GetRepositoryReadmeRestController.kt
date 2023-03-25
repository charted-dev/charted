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

package org.noelware.charted.server.routing.v1.repositories.readme

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.ktor.server.util.*
import io.swagger.v3.oas.models.PathItem
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.models.flags.ApiKeyScope
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.openapi.toPaths
import org.noelware.charted.modules.postgresql.controllers.repositories.RepositoryDatabaseController
import org.noelware.charted.modules.storage.StorageModule
import org.noelware.charted.server.extensions.currentUser
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController
import org.noelware.charted.server.util.createBodyFromInputStream

class GetRepositoryReadmeRestController(
    private val controller: RepositoryDatabaseController,
    private val storage: StorageModule
): RestController("/repositories/{id}/readme") {
    override val apiVersion: APIVersion = APIVersion.V1
    override fun Route.init() {
        install(Sessions) {
            this += ApiKeyScope.Repositories.Access
            allowNonAuthorizedRequests = true
        }
    }

    override suspend fun call(call: ApplicationCall) {
        val id = call.parameters.getOrFail<Long>("id")
        val repo = controller.getEntityOrNull(id) ?: return call.respond(
            HttpStatusCode.NotFound,
            ApiResponse.err(
                "UNKNOWN_REPOSITORY", "Repository with ID [$id] was not found",
            ),
        )

        if (repo.private) {
            if (call.currentUser == null) {
                return call.respond(
                    HttpStatusCode.Unauthorized,
                    ApiResponse.err(
                        "INVALID_ACCESS",
                        "Repository ${repo.name} is private and you don't have access to it",
                    ),
                )
            }

            if (!repo.members.any { it.account.id.value == call.currentUser!!.id }) {
                return call.respond(
                    HttpStatusCode.Unauthorized,
                    ApiResponse.err(
                        "INVALID_ACCESS",
                        "Repository ${repo.name} is private and you don't have access to it",
                    ),
                )
            }
        }

        val readme = storage.open("./repositories/$id/README") ?: return call.respond(HttpStatusCode.NotFound)
        call.respond(createBodyFromInputStream(readme, ContentType.Text.Plain))
    }

    override fun toPathDsl(): PathItem = toPaths("/repositories/{id}/readme") {
        get {
            description = "Retrieve a repository's README"

            pathParameter {
                description = "Snowflake to query a repository"
                name = "id"

                schema<Long>()
            }

            response(HttpStatusCode.OK) {
                description = "README content in Markdown"
                contentType(ContentType.Text.Plain) {
                    schema("# Some Markdown\n> Hopefully...")
                }
            }

            response(HttpStatusCode.NotFound) {
                description = "If a repository wasn't found or if there is no README"
                contentType(ContentType.Application.Json) {
                    schema<ApiResponse.Err>()
                }
            }
        }
    }
}
