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
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.swagger.v3.oas.models.PathItem
import kotlinx.datetime.Clock
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.models.flags.ApiKeyScope
import org.noelware.charted.models.organizations.Organization
import org.noelware.charted.models.users.User
import org.noelware.charted.modules.helm.charts.HelmChartModule
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.openapi.toPaths
import org.noelware.charted.modules.postgresql.controllers.organizations.CreateOrganizationPayload
import org.noelware.charted.modules.postgresql.controllers.organizations.OrganizationDatabaseController
import org.noelware.charted.modules.search.SearchModule
import org.noelware.charted.server.extensions.addAuthenticationResponses
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.routing.RestController

class CreateOrganizationRestController(
    private val organizations: OrganizationDatabaseController,
    private val search: SearchModule? = null,
    private val charts: HelmChartModule? = null
): RestController("/organizations", HttpMethod.Put) {
    override fun Route.init() {
        install(Sessions) {
            this += ApiKeyScope.Organizations.Create
        }
    }

    override suspend fun call(call: ApplicationCall) {
        val org = organizations.create(call, call.receive())

        charts?.createIndexYaml(org.id)
        search?.indexOrganization(org)

        call.respond(HttpStatusCode.Created, ApiResponse.ok(org))
    }

    override fun toPathDsl(): PathItem = toPaths("/organizations") {
        put {
            description = "Creates an organization resource"

            requestBody {
                description = "Payload for creating an organization"
                contentType(ContentType.Application.Json) {
                    schema(
                        CreateOrganizationPayload(
                            "Noelware, LLC.",
                            false,
                            "noelware",
                        ),
                    )
                }
            }

            addAuthenticationResponses()
            response(HttpStatusCode.Accepted) {
                description = "Created organization resource"
                contentType(ContentType.Application.Json) {
                    schema(
                        Organization(
                            true,
                            null,
                            null,
                            "Noelware, LLC.",
                            Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault()),
                            Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault()),
                            null,
                            false,
                            User(
                                true,
                                null,
                                null,
                                null,
                                Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault()),
                                Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault()),
                                "noel",
                                false,
                                "Noel",
                                1,
                            ),
                            "noelware",
                            1234,
                        ),
                    )
                }
            }
        }
    }
}
