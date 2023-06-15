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
import org.noelware.charted.common.types.helm.RepoType
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.models.flags.ApiKeyScope.Repositories
import org.noelware.charted.models.repositories.Repository
import org.noelware.charted.modules.openapi.kotlin.dsl.created
import org.noelware.charted.modules.openapi.kotlin.dsl.idOrName
import org.noelware.charted.modules.openapi.kotlin.dsl.json
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.postgresql.controllers.getByIdOrNameOrNull
import org.noelware.charted.modules.postgresql.controllers.organizations.OrganizationDatabaseController
import org.noelware.charted.modules.postgresql.controllers.repositories.CreateRepositoryPayload
import org.noelware.charted.modules.postgresql.controllers.repositories.RepositoryDatabaseController
import org.noelware.charted.modules.postgresql.ktor.OwnerIdAttributeKey
import org.noelware.charted.modules.postgresql.tables.OrganizationTable
import org.noelware.charted.modules.search.SearchModule
import org.noelware.charted.server.extensions.putAndRemove
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.plugins.sessions.preconditions.canAccessOrganization
import org.noelware.charted.server.plugins.sessions.preconditions.canEditMetadata
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController
import org.noelware.charted.server.routing.openapi.ResourceDescription
import org.noelware.charted.server.routing.openapi.describeResource

class CreateOrganizationRepositoryRestController(
    private val organizations: OrganizationDatabaseController,
    private val repositories: RepositoryDatabaseController,
    private val search: SearchModule? = null
): RestController("/organizations/{idOrName}/repositories", HttpMethod.Put) {
    override val apiVersion: APIVersion = APIVersion.V1
    override fun Route.init() {
        install(Sessions) {
            this += Repositories.Create

            condition(::canAccessOrganization)
            condition { call -> canEditMetadata(call, organizations) }
        }
    }

    override suspend fun call(call: ApplicationCall) {
        val organization = organizations.getByIdOrNameOrNull(call.parameters.getOrFail("idOrName"), OrganizationTable::name)!!
        return call.attributes.putAndRemove(OwnerIdAttributeKey, organization.id) {
            val repository = repositories.create(call, call.receive())
            search?.indexRepository(repository)

            call.respond(HttpStatusCode.Created, ApiResponse.ok(repository))
        }
    }

    companion object: ResourceDescription by describeResource("/organizations/{idOrName}/repositories", {
        description = "Allows creating a repository underneath an organization."

        put {
            description = "Creates a repository that is owned by an organization."

            idOrName()
            requestBody {
                json {
                    schema(
                        CreateRepositoryPayload(
                            "helm library to provide common stuff",
                            false,
                            "# Hello, world!\n> we do magic stuff here~!",
                            "common",
                            RepoType.LIBRARY,
                        ),
                    )
                }
            }

            created {
                description = "The [Repository] of the newly created repository."

                json {
                    schema<Repository>()
                }
            }
        }
    })
}
