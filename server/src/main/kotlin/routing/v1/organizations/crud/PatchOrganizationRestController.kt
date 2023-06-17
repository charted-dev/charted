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
import io.ktor.server.util.*
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.models.flags.ApiKeyScope
import org.noelware.charted.modules.openapi.kotlin.dsl.accepted
import org.noelware.charted.modules.openapi.kotlin.dsl.json
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.postgresql.controllers.getByIdOrNameOrNull
import org.noelware.charted.modules.postgresql.controllers.organizations.OrganizationDatabaseController
import org.noelware.charted.modules.postgresql.controllers.organizations.PatchOrganizationPayload
import org.noelware.charted.modules.postgresql.ktor.UserEntityAttributeKey
import org.noelware.charted.modules.postgresql.tables.OrganizationTable
import org.noelware.charted.server.extensions.addAuthenticationResponses
import org.noelware.charted.server.extensions.currentUserEntity
import org.noelware.charted.server.extensions.putAndRemove
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.plugins.sessions.preconditions.canAccessOrganization
import org.noelware.charted.server.plugins.sessions.preconditions.canEditMetadata
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController
import org.noelware.charted.server.routing.openapi.ResourceDescription
import org.noelware.charted.server.routing.openapi.describeResource
import org.noelware.charted.modules.openapi.kotlin.dsl.idOrName

class PatchOrganizationRestController(private val organizations: OrganizationDatabaseController): RestController("/organizations/{idOrName}", HttpMethod.Patch) {
    override val apiVersion: APIVersion = APIVersion.V1
    override fun Route.init() {
        install(Sessions) {
            this += ApiKeyScope.Organizations.Update

            condition(::canAccessOrganization)
            condition { call -> canEditMetadata(call, organizations) }
        }
    }

    override suspend fun call(call: ApplicationCall): Unit = call.attributes.putAndRemove(UserEntityAttributeKey, call.currentUserEntity!!) {
        val org = organizations.getByIdOrNameOrNull(call.parameters.getOrFail("idOrName"), OrganizationTable::name)!!
        organizations.update(call, org.id, call.receive())

        call.respond(HttpStatusCode.Accepted)
    }

    companion object: ResourceDescription by describeResource("/organizations/{idOrName}", {
        patch {
            description = "Patch an organization's metadata"

            idOrName()
            requestBody {
                json {
                    schema(
                        PatchOrganizationPayload(
                            null,
                            null,
                            "awau!",
                            true,
                            "heck",
                        ),
                    )
                }
            }

            addAuthenticationResponses()
            accepted {
                json {
                    schema(ApiResponse.ok())
                }
            }
        }
    })
}
