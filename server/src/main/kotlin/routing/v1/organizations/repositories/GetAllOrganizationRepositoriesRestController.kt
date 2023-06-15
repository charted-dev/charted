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
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.models.repositories.Repository
import org.noelware.charted.modules.openapi.kotlin.dsl.idOrName
import org.noelware.charted.modules.openapi.kotlin.dsl.json
import org.noelware.charted.modules.openapi.kotlin.dsl.ok
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.postgresql.controllers.getEntityByIdOrNameOrNull
import org.noelware.charted.modules.postgresql.controllers.organizations.OrganizationDatabaseController
import org.noelware.charted.modules.postgresql.controllers.repositories.RepositoryDatabaseController
import org.noelware.charted.modules.postgresql.tables.OrganizationTable
import org.noelware.charted.modules.postgresql.tables.RepositoryTable
import org.noelware.charted.server.extensions.currentUser
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.plugins.sessions.preconditions.canAccessOrganization
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController
import org.noelware.charted.server.routing.openapi.ResourceDescription
import org.noelware.charted.server.routing.openapi.describeResource

class GetAllOrganizationRepositoriesRestController(
    private val organizations: OrganizationDatabaseController,
    private val controller: RepositoryDatabaseController
): RestController("/organizations/{idOrName}/repositories") {
    override val apiVersion: APIVersion = APIVersion.V1
    override fun Route.init() {
        install(Sessions) {
            allowNonAuthorizedRequests = true

            condition { call -> canAccessOrganization(call, false) }
        }
    }

    override suspend fun call(call: ApplicationCall) {
        val idOrName = call.parameters.getOrFail("idOrName")
        val org = organizations.getEntityByIdOrNameOrNull(idOrName, OrganizationTable::name) ?: return call.respond(
            HttpStatusCode.BadRequest,
            ApiResponse.err(
                "UNKNOWN_USER",
                "Unknown user with username or snowflake [$idOrName]",
            ),
        )

        val repos = controller.all(RepositoryTable::owner to org.id.value)
        if (call.currentUser == null) {
            return call.respond(HttpStatusCode.OK, ApiResponse.ok(repos.filterNot { it.private }))
        }

        if (org.owner.id.value != call.currentUser?.id || !org.members.any { it.account.id.value == call.currentUser!!.id }) {
            return call.respond(HttpStatusCode.OK, ApiResponse.ok(repos.filterNot { it.private }))
        }

        call.respond(HttpStatusCode.OK, ApiResponse.ok(repos))
    }

    companion object: ResourceDescription by describeResource("/organizations/{idOrName}/repositories", {
        description = "Returns all of an organization's repositories."

        get {
            description = "Returns all of an organization's repositories."

            idOrName()
            ok {
                description = "A list of repositories within this organization."
                json {
                    schema<List<Repository>>()
                }
            }
        }
    })
}
