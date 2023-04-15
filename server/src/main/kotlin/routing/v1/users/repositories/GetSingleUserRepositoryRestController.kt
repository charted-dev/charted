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

package org.noelware.charted.server.routing.v1.users.repositories

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.ktor.server.util.*
import io.swagger.v3.oas.models.PathItem
import org.jetbrains.exposed.sql.and
import org.noelware.charted.common.extensions.regexp.matchesRepoNameAndIdRegex
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.models.repositories.Repository
import org.noelware.charted.modules.openapi.NameOrSnowflake
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.openapi.toPaths
import org.noelware.charted.modules.postgresql.controllers.getByIdOrNameOrNull
import org.noelware.charted.modules.postgresql.controllers.repositories.RepositoryDatabaseController
import org.noelware.charted.modules.postgresql.controllers.users.UserDatabaseController
import org.noelware.charted.modules.postgresql.extensions.fromEntity
import org.noelware.charted.modules.postgresql.tables.RepositoryTable
import org.noelware.charted.modules.postgresql.tables.UserTable
import org.noelware.charted.server.extensions.addAuthenticationResponses
import org.noelware.charted.server.extensions.currentUser
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController
import kotlin.reflect.typeOf

class GetSingleUserRepositoryRestController(
    private val controller: RepositoryDatabaseController,
    private val usersController: UserDatabaseController
): RestController("/users/{idOrName}/repositories/{repoIdOrName}") {
    override val apiVersion: APIVersion = APIVersion.V1
    override fun Route.init() {
        install(Sessions) {
            allowNonAuthorizedRequests = true
        }
    }

    override suspend fun call(call: ApplicationCall) {
        val idOrName = call.parameters.getOrFail("idOrName")
        val user = usersController.getByIdOrNameOrNull(idOrName, UserTable::username) ?: return call.respond(
            HttpStatusCode.NotFound,
            ApiResponse.err(
                "UNKNOWN_USER",
                "Unknown user with username or snowflake [$idOrName]",
            ),
        )

        val repoIdOrName = call.parameters.getOrFail("repoIdOrName")
        val repo = when {
            repoIdOrName.toLongOrNull() != null -> controller.getEntityOrNull { (RepositoryTable.owner eq user.id) and (RepositoryTable.id eq repoIdOrName.toLong()) }
            repoIdOrName.matchesRepoNameAndIdRegex() -> controller.getEntityOrNull { (RepositoryTable.owner eq user.id) and (RepositoryTable.name eq repoIdOrName) }
            else -> null
        } ?: return call.respond(
            HttpStatusCode.NotFound,
            ApiResponse.err(
                "UNKNOWN_REPOSITORY",
                "Unknown repository with name or snowflake [$repoIdOrName]",
            ),
        )

        if (repo.private) {
            if (call.currentUser == null) {
                return call.respond(
                    HttpStatusCode.Unauthorized,
                    ApiResponse.err(
                        "INVALID_ACCESS",
                        "Repository ${repo.name} owned by @${user.username} (${user.id}) is private and you don't have access to it",
                    ),
                )
            }

            if (!repo.members.any { it.account.id.value == call.currentUser!!.id }) {
                return call.respond(
                    HttpStatusCode.Unauthorized,
                    ApiResponse.err(
                        "INVALID_ACCESS",
                        "Repository ${repo.name} owned by @${user.username} (${user.id}) is private and you don't have access to it",
                    ),
                )
            }
        }

        call.respond(HttpStatusCode.OK, ApiResponse.ok(Repository.fromEntity(repo)))
    }

    override fun toPathDsl(): PathItem = toPaths("/users/{idOrName}/repositories/{repoIdOrName}") {
        get {
            description = "Fetch a single repository from a user"

            pathParameter {
                description = "Name or Snowflake to query a user"
                name = "idOrName"

                schema<NameOrSnowflake>()
            }

            pathParameter {
                description = "Name or Snowflake to query a repository"
                name = "repoIdOrName"

                schema<NameOrSnowflake>()
            }

            addAuthenticationResponses()
            response(HttpStatusCode.OK) {
                contentType(ContentType.Application.Json) {
                    schema(typeOf<ApiResponse.Ok<List<Repository>>>(), ApiResponse.ok(listOf<Repository>()))
                }
            }

            response(HttpStatusCode.NotFound) {
                description = "if a user or repository couldn't be found"
                contentType(ContentType.Application.Json) {
                    schema<ApiResponse.Err>()
                }
            }
        }
    }
}
