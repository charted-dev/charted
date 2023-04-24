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
import org.noelware.charted.modules.openapi.kotlin.dsl.accepted
import org.noelware.charted.modules.openapi.kotlin.dsl.json
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.openapi.toPaths
import org.noelware.charted.modules.postgresql.controllers.get
import org.noelware.charted.modules.postgresql.controllers.organizations.OrganizationDatabaseController
import org.noelware.charted.server.extensions.addAuthenticationResponses
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.plugins.sessions.preconditions.canAccessOrganization
import org.noelware.charted.server.plugins.sessions.preconditions.canDeleteMetadata
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController

class DeleteOrganizationRestController(private val organizations: OrganizationDatabaseController): RestController("/organizations/{id}", HttpMethod.Delete) {
    override val apiVersion: APIVersion = APIVersion.V1
    override fun Route.init() {
        install(Sessions) {
            this += ApiKeyScope.Organizations.Delete

            condition(::canAccessOrganization)
            condition { call -> canDeleteMetadata(call, organizations) }
        }
    }

    override suspend fun call(call: ApplicationCall) {
        organizations.delete(call.parameters.getOrFail<Long>("id"))
        call.respond(HttpStatusCode.Accepted, ApiResponse.ok())
    }

    override fun toPathDsl(): PathItem = toPaths("/organizations/{id}") {
        delete {
            description = "Deletes an organization resource"

            pathParameter {
                description = "Snowflake ID of the organization resource to delete"
                name = "id"

                schema<Long>()
            }

            addAuthenticationResponses()
            accepted {
                description = "Organization was successfully deleted"
                json {
                    schema(ApiResponse.ok())
                }
            }
        }
    }
}
