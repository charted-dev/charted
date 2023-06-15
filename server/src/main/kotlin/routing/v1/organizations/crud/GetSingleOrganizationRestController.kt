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
import kotlinx.datetime.LocalDateTime
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.models.flags.ApiKeyScope
import org.noelware.charted.models.organizations.Organization
import org.noelware.charted.models.users.User
import org.noelware.charted.modules.openapi.kotlin.dsl.idOrName
import org.noelware.charted.modules.openapi.kotlin.dsl.json
import org.noelware.charted.modules.openapi.kotlin.dsl.ok
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.postgresql.controllers.getEntityByIdOrNameOrNull
import org.noelware.charted.modules.postgresql.controllers.organizations.OrganizationDatabaseController
import org.noelware.charted.modules.postgresql.extensions.fromEntity
import org.noelware.charted.modules.postgresql.tables.OrganizationTable
import org.noelware.charted.server.extensions.currentUser
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController
import org.noelware.charted.server.routing.openapi.ResourceDescription
import org.noelware.charted.server.routing.openapi.describeResource

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

    companion object: ResourceDescription by describeResource("/organizations/{idOrName}", {
        get {
            description = "Grabs an organization resource by its ID or name."
            idOrName()

            ok {
                description = "Returns the organization resource"
                json {
                    schema(
                        Organization(
                            true,
                            "@noelware",
                            null,
                            "\uD83D\uDC3B\u200D❄️ Noelware, LLC.",
                            LocalDateTime.parse("2023-04-08T02:37:53.741502369"),
                            LocalDateTime.parse("2023-04-08T02:37:53.741502369"),
                            null,
                            false,
                            User(
                                true,
                                null,
                                null,
                                null,
                                LocalDateTime.parse("2023-04-08T02:37:53.741502369"),
                                LocalDateTime.parse("2023-04-08T02:37:53.741502369"),
                                "noel",
                                true,
                                "Noel",
                                1,
                            ),
                            "noelware",
                            2,
                        ),
                    )
                }
            }
        }
    })
}
