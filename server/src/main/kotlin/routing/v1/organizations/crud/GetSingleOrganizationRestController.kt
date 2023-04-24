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

package org.noelware.charted.server.routing.v1.organizations.crud

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.ktor.server.util.*
import io.swagger.v3.oas.models.PathItem
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.models.flags.ApiKeyScope
import org.noelware.charted.models.organizations.Organization
import org.noelware.charted.modules.openapi.toPaths
import org.noelware.charted.modules.postgresql.controllers.getEntityByIdOrNameOrNull
import org.noelware.charted.modules.postgresql.controllers.organizations.OrganizationDatabaseController
import org.noelware.charted.modules.postgresql.extensions.fromEntity
import org.noelware.charted.modules.postgresql.tables.OrganizationTable
import org.noelware.charted.server.extensions.currentUser
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController

class GetSingleOrganizationRestController(private val organizations: OrganizationDatabaseController): RestController("/organizations/{idOrName}") {
    override val apiVersion: APIVersion = APIVersion.V1
    override fun Route.init() {
        install(Sessions) {
            this += ApiKeyScope.Organizations.Access
        }
    }

    override suspend fun call(call: ApplicationCall) {
        val idOrName = call.parameters.getOrFail("idOrName")
        val org = organizations.getEntityByIdOrNameOrNull(idOrName, OrganizationTable::name)
            ?: return call.respond(
                HttpStatusCode.NotFound,
                ApiResponse.err(
                    "UNKNOWN_ORGANIZATION",
                    "Organization with ID or name [$idOrName] was not found",
                ),
            )

        if (org.private) {
            if (call.currentUser == null) {
                return call.respond(
                    HttpStatusCode.Unauthorized,
                    ApiResponse.err(
                        "UNAUTHORIZED",
                        "Organization is private, and you do not have permission to access it",
                    ),
                )
            }

            if (!org.members.any { it.account.id.value == call.currentUser!!.id }) {
                return call.respond(
                    HttpStatusCode.Unauthorized,
                    ApiResponse.err(
                        "UNAUTHORIZED",
                        "Organization is private, and you do not have permission to access it",
                    ),
                )
            }
        }

        call.respond(HttpStatusCode.OK, ApiResponse.ok(Organization.fromEntity(org)))
    }

    override fun toPathDsl(): PathItem = toPaths("/organizations/{idOrName}") {
        get {
            description = "Grabs an organization resource by its ID or name."

            pathParameter {
                description = "Snowflake ID or name to retrieve an organization"
                name = "idOrName"
            }
        }
    }
}
