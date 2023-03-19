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
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.ktor.server.util.*
import io.swagger.v3.oas.models.PathItem
import org.jetbrains.exposed.sql.and
import org.noelware.charted.common.types.helm.RepoType
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.models.flags.ApiKeyScope.Repositories
import org.noelware.charted.models.organizations.Organization
import org.noelware.charted.models.repositories.Repository
import org.noelware.charted.modules.openapi.NameOrSnowflake
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.openapi.toPaths
import org.noelware.charted.modules.postgresql.asyncTransaction
import org.noelware.charted.modules.postgresql.controllers.repositories.CreateRepositoryPayload
import org.noelware.charted.modules.postgresql.controllers.repositories.RepositoryDatabaseController
import org.noelware.charted.modules.postgresql.entities.OrganizationEntity
import org.noelware.charted.modules.postgresql.extensions.fromEntity
import org.noelware.charted.modules.postgresql.ktor.OwnerIdAttributeKey
import org.noelware.charted.modules.postgresql.tables.OrganizationTable
import org.noelware.charted.server.extensions.addAuthenticationResponses
import org.noelware.charted.server.extensions.currentUserEntity
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.routing.RestController

class CreateOrganizationRepositoryRestController(
    private val controller: RepositoryDatabaseController
): RestController("/organizations/{idOrName}/repositories", HttpMethod.Put) {
    override fun Route.init() {
        install(Sessions) {
            this += Repositories.Create
        }
    }

    override suspend fun call(call: ApplicationCall) {
        val body: CreateRepositoryPayload = call.receive()

        val idOrName = call.parameters.getOrFail("idOrName")
        val org = asyncTransaction {
            OrganizationEntity.find {
                if (idOrName.toLongOrNull() != null) {
                    OrganizationTable.id eq idOrName.toLong()
                } else {
                    (OrganizationTable.name eq idOrName) and (OrganizationTable.owner eq call.currentUserEntity!!.id)
                }
            }.firstOrNull()?.let { entity -> Organization.fromEntity(entity) }
        } ?: return call.respond(
            HttpStatusCode.BadRequest,
            ApiResponse.err(
                "UNKNOWN_ORGANIZATION",
                "Unknown organization: [$idOrName]",
            ),
        )

        call.attributes.put(OwnerIdAttributeKey, org.id)
        call.respond(HttpStatusCode.Created, ApiResponse.ok(controller.create(call, body)))

        call.attributes.remove(OwnerIdAttributeKey)
    }

    override fun toPathDsl(): PathItem = toPaths("/organizations/{idOrName}/repositories") {
        put {
            description = "Creates a repository that is owned by the current authenticated user"

            pathParameter {
                description = "Represents a Name or Snowflake to query the organization as"
                name = "idOrName"

                schema<NameOrSnowflake>()
            }

            requestBody {
                contentType(ContentType.Application.Json) {
                    schema<CreateRepositoryPayload>()
                    example = CreateRepositoryPayload(
                        "helm library to provide common stuff",
                        false,
                        "common",
                        RepoType.LIBRARY,
                    )
                }
            }

            addAuthenticationResponses()
            response(HttpStatusCode.Created) {
                contentType(ContentType.Application.Json) {
                    schema<ApiResponse.Ok<Repository>>()
                }
            }
        }
    }
}
