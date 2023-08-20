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

package org.noelware.charted.server.routing.v1.users.repositories

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.ktor.server.util.*
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.models.NameOrSnowflake
import org.noelware.charted.models.flags.ApiKeyScope
import org.noelware.charted.modules.openapi.kotlin.dsl.json
import org.noelware.charted.modules.openapi.kotlin.dsl.notFound
import org.noelware.charted.modules.openapi.kotlin.dsl.ok
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.postgresql.controllers.get
import org.noelware.charted.modules.postgresql.controllers.getByIdOrNameOrNull
import org.noelware.charted.modules.postgresql.controllers.repositories.RepositoryDatabaseController
import org.noelware.charted.modules.postgresql.controllers.users.UserDatabaseController
import org.noelware.charted.modules.postgresql.ktor.OwnerIdAttributeKey
import org.noelware.charted.modules.postgresql.tables.UserTable
import org.noelware.charted.server.extensions.addAuthenticationResponses
import org.noelware.charted.server.extensions.putAndRemove
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.plugins.sessions.preconditions.canEditMetadata
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController
import org.noelware.charted.server.routing.openapi.ResourceDescription
import org.noelware.charted.server.routing.openapi.describeResource

class PatchUserRepositoryRestController(
    private val controller: RepositoryDatabaseController,
    private val usersController: UserDatabaseController
): RestController("/users/{idOrName}/repositories/{id}", HttpMethod.Patch) {
    override val apiVersion: APIVersion = APIVersion.V1
    override fun Route.init() {
        install(Sessions) {
            this += ApiKeyScope.Repositories.Update
            condition { call -> canEditMetadata(call, controller) }
        }
    }

    override suspend fun call(call: ApplicationCall) {
        val idOrName = call.parameters.getOrFail("idOrName")
        val user = usersController.getByIdOrNameOrNull(idOrName, UserTable::username)
            ?: return call.respond(
                HttpStatusCode.NotFound,
                ApiResponse.err(
                    "UNKNOWN_USER",
                    "User with name or snowflake [$idOrName] was not found",
                ),
            )

        val repo = controller.get(call.parameters.getOrFail<Long>("id"))
        return call.attributes.putAndRemove(OwnerIdAttributeKey, user.id) {
            controller.update(call, repo.id, call.receive())
            call.respond(HttpStatusCode.Accepted, ApiResponse.ok())
        }
    }

    companion object: ResourceDescription by describeResource("/users/{idOrName}/repositories/{id}", {
        patch {
            pathParameter {
                name = "idOrName"
                schema<NameOrSnowflake>()
            }

            pathParameter {
                name = "id"
                schema<Long>()
            }

            addAuthenticationResponses()
            ok {
                json {
                    schema(ApiResponse.ok())
                }
            }

            notFound {
                json {
                    schema<ApiResponse.Err>()
                }
            }
        }
    })
}