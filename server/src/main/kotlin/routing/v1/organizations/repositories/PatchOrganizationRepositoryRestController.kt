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
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.ktor.server.util.*
import io.swagger.v3.oas.models.PathItem
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.models.flags.ApiKeyScope
import org.noelware.charted.modules.openapi.kotlin.dsl.idOrName
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.openapi.toPaths
import org.noelware.charted.modules.postgresql.controllers.get
import org.noelware.charted.modules.postgresql.controllers.getByIdOrNameOrNull
import org.noelware.charted.modules.postgresql.controllers.organizations.OrganizationDatabaseController
import org.noelware.charted.modules.postgresql.controllers.repositories.RepositoryDatabaseController
import org.noelware.charted.modules.postgresql.ktor.OwnerIdAttributeKey
import org.noelware.charted.modules.postgresql.tables.UserTable
import org.noelware.charted.server.extensions.addAuthenticationResponses
import org.noelware.charted.server.extensions.putAndRemove
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.plugins.sessions.preconditions.canAccessOrganization
import org.noelware.charted.server.plugins.sessions.preconditions.canAccessRepository
import org.noelware.charted.server.plugins.sessions.preconditions.canEditMetadata
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController

class PatchOrganizationRepositoryRestController(
    private val controller: RepositoryDatabaseController,
    private val organizations: OrganizationDatabaseController
): RestController("/organizations/{idOrName}/repositories/{id}", HttpMethod.Patch) {
    override val apiVersion: APIVersion = APIVersion.V1
    override fun Route.init() {
        install(Sessions) {
            this += ApiKeyScope.Repositories.Update

            condition(::canAccessOrganization)
            condition(::canAccessRepository)
            condition { call -> canEditMetadata(call, controller) }
        }
    }

    override suspend fun call(call: ApplicationCall) {
        val idOrName = call.parameters.getOrFail("idOrName")
        val org = organizations.getByIdOrNameOrNull(idOrName, UserTable::username)
            ?: return call.respond(
                HttpStatusCode.NotFound,
                ApiResponse.err(
                    "UNKNOWN_ORGANIZATION",
                    "Organization with name or snowflake [$idOrName] was not found",
                ),
            )

        val repo = controller.get(call.parameters.getOrFail<Long>("id"))
        return call.attributes.putAndRemove(OwnerIdAttributeKey, org.id) {
            controller.update(call, repo.id, call.receive())
            call.respond(HttpStatusCode.Accepted, ApiResponse.ok())
        }
    }

    override fun toPathDsl(): PathItem = toPaths("/organizations/{idOrName}/repositories/{id}") {
        patch {
            idOrName()
            pathParameter {
                name = "id"
                schema<Long>()
            }

            addAuthenticationResponses()
            response(HttpStatusCode.OK) {
                contentType(ContentType.Application.Json) {
                    schema(ApiResponse.ok())
                }
            }

            response(HttpStatusCode.NotFound) {
                contentType(ContentType.Application.Json) {
                    schema<ApiResponse.Err>()
                }
            }
        }
    }
}
