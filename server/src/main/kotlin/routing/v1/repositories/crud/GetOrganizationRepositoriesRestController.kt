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

package org.noelware.charted.server.routing.v1.repositories.crud

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.ktor.server.util.*
import io.swagger.v3.oas.models.PathItem
import org.noelware.charted.common.extensions.regexp.matchesNameAndIdRegex
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.models.repositories.Repository
import org.noelware.charted.modules.openapi.NameOrSnowflake
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.openapi.toPaths
import org.noelware.charted.modules.postgresql.controllers.getOrNullByProp
import org.noelware.charted.modules.postgresql.controllers.organizations.OrganizationController
import org.noelware.charted.modules.postgresql.controllers.repositories.RepositoryController
import org.noelware.charted.modules.postgresql.tables.OrganizationTable
import org.noelware.charted.modules.postgresql.tables.RepositoryTable
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.routing.RestController

class GetOrganizationRepositoriesRestController(
    private val controller: RepositoryController,
    private val organizationController: OrganizationController
): RestController("/organizations/{idOrName}/repositories") {
    override fun Route.init() {
        install(Sessions) {
            // We will allow non-authorized requests, but we will not show
            // private repositories an organization has.
            allowNonAuthorizedRequests = true
        }
    }

    override suspend fun call(call: ApplicationCall) {
        val idOrName = call.parameters.getOrFail("idOrName")
        val org = when {
            idOrName.toLongOrNull() != null -> organizationController.getOrNullByProp(OrganizationTable, OrganizationTable::id to idOrName.toLong())
            idOrName.matchesNameAndIdRegex() -> organizationController.getOrNullByProp(OrganizationTable::name to idOrName)
            else -> null
        } ?: return call.respond(
            HttpStatusCode.BadRequest,
            ApiResponse.err(
                "UNKNOWN_ORGANIZATION",
                "Unknown organization found [$idOrName]",
            ),
        )

        call.respond(HttpStatusCode.OK, ApiResponse.ok(controller.all(RepositoryTable::owner to org.id)))
    }

    override fun toPathDsl(): PathItem = toPaths("/organizations/{idOrName}/repositories") {
        get {
            description = "Returns all of an organization's repositories"

            pathParameter {
                name = "idOrName"
                schema<NameOrSnowflake>()
            }

            response(HttpStatusCode.OK) {
                contentType(ContentType.Application.Json) {
                    schema(ApiResponse.ok(listOf<Repository>()))
                }
            }

            response(HttpStatusCode.NotFound) {
                description = "if an organization couldn't be found"
                contentType(ContentType.Application.Json) {
                    schema<ApiResponse.Err>()
                }
            }
        }
    }
}
