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

package org.noelware.charted.server.routing.v1.organizations.repositories

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.ktor.server.util.*
import org.jetbrains.exposed.sql.and
import org.noelware.charted.common.extensions.regexp.matchesRepoNameAndIdRegex
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.models.NameOrSnowflake
import org.noelware.charted.models.repositories.Repository
import org.noelware.charted.modules.openapi.kotlin.dsl.*
import org.noelware.charted.modules.postgresql.controllers.getByIdOrNameOrNull
import org.noelware.charted.modules.postgresql.controllers.organizations.OrganizationDatabaseController
import org.noelware.charted.modules.postgresql.controllers.repositories.RepositoryDatabaseController
import org.noelware.charted.modules.postgresql.extensions.fromEntity
import org.noelware.charted.modules.postgresql.tables.OrganizationTable
import org.noelware.charted.modules.postgresql.tables.RepositoryTable
import org.noelware.charted.server.extensions.addAuthenticationResponses
import org.noelware.charted.server.extensions.currentUser
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.plugins.sessions.preconditions.canAccessOrganization
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController
import org.noelware.charted.server.routing.openapi.ResourceDescription
import org.noelware.charted.server.routing.openapi.describeResource

class GetSingleOrganizationRepositoryRestController(
    private val organizations: OrganizationDatabaseController,
    private val controller: RepositoryDatabaseController
): RestController("/organizations/{idOrName}/repositories/{repoIdOrName}") {
    override val apiVersion: APIVersion = APIVersion.V1
    override fun Route.init() {
        install(Sessions) {
            allowNonAuthorizedRequests = true

            condition { call -> canAccessOrganization(call, false) }
        }
    }

    override suspend fun call(call: ApplicationCall) {
        val idOrName = call.parameters.getOrFail("idOrName")
        val org = organizations.getByIdOrNameOrNull(idOrName, OrganizationTable::name) ?: return call.respond(
            HttpStatusCode.NotFound,
            ApiResponse.err(
                "UNKNOWN_ORGANIZATION",
                "Unknown organization with name or snowflake [$idOrName]",
            ),
        )

        val repoIdOrName = call.parameters.getOrFail("repoIdOrName")
        val repo = when {
            repoIdOrName.toLongOrNull() != null -> controller.getEntityOrNull { (RepositoryTable.owner eq org.id) and (RepositoryTable.id eq repoIdOrName.toLong()) }
            repoIdOrName.matchesRepoNameAndIdRegex() -> controller.getEntityOrNull { (RepositoryTable.owner eq org.id) and (RepositoryTable.name eq repoIdOrName) }
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
                        "Repository ${repo.name} owned by organization ${org.name} (${org.id}) is private and you don't have access to it",
                    ),
                )
            }

            if (!repo.members.any { it.account.id.value == call.currentUser!!.id }) {
                return call.respond(
                    HttpStatusCode.Unauthorized,
                    ApiResponse.err(
                        "INVALID_ACCESS",
                        "Repository ${repo.name} owned by organization ${org.name} (${org.id}) is private and you don't have access to it",
                    ),
                )
            }
        }

        call.respond(HttpStatusCode.OK, ApiResponse.ok(Repository.fromEntity(repo)))
    }

    companion object: ResourceDescription by describeResource("/organizations/{idOrName}/repositories/{repoIdOrName}", {
        description = "Fetches a single repository from an organization."
        get {
            description = "Fetches a single repository from the given organization based on its name or ID."

            idOrName()
            pathParameter {
                description = "The [Name] or [Snowflake] ID of the repository being queried."
                name = "repoIdOrName"

                schema<NameOrSnowflake>()
            }

            addAuthenticationResponses()
            ok {
                description = "The metadata of this [Repository]."
                json {
                    schema<Repository>()
                }
            }

            notFound {
                description = "Organization or repository doesn't exist"
                json {
                    schema<ApiResponse.Err>()
                }
            }
        }
    })
}
