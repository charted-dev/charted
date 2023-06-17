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
import kotlinx.datetime.LocalDateTime
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.models.flags.ApiKeyScope
import org.noelware.charted.models.organizations.Organization
import org.noelware.charted.models.users.User
import org.noelware.charted.modules.helm.charts.HelmChartModule
import org.noelware.charted.modules.openapi.kotlin.dsl.accepted
import org.noelware.charted.modules.openapi.kotlin.dsl.json
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.postgresql.controllers.organizations.CreateOrganizationPayload
import org.noelware.charted.modules.postgresql.controllers.organizations.OrganizationDatabaseController
import org.noelware.charted.modules.search.SearchModule
import org.noelware.charted.server.extensions.addAuthenticationResponses
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController
import org.noelware.charted.server.routing.openapi.ResourceDescription
import org.noelware.charted.server.routing.openapi.describeResource

class CreateOrganizationRestController(
    private val organizations: OrganizationDatabaseController,
    private val search: SearchModule? = null,
    private val charts: HelmChartModule? = null
): RestController("/organizations", HttpMethod.Put) {
    override val apiVersion: APIVersion = APIVersion.V1
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

    companion object: ResourceDescription by describeResource("/organizations", {
        description = "Allows creating an organization."

        put {
            description = "Creates an organization resource with the specified parameters."

            requestBody {
                description = "Payload for creating an organization"

                json {
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
            accepted {
                description = "Returns the created organization resource."

                json {
                    schema(
                        Organization(
                            true,
                            null,
                            null,
                            "Noelware, LLC.",
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
    })
}
