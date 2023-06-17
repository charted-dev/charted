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
import org.noelware.charted.common.types.helm.RepoType
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.models.flags.ApiKeyScope.Repositories
import org.noelware.charted.models.repositories.Repository
import org.noelware.charted.modules.openapi.kotlin.dsl.created
import org.noelware.charted.modules.openapi.kotlin.dsl.json
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.postgresql.controllers.repositories.CreateRepositoryPayload
import org.noelware.charted.modules.postgresql.controllers.repositories.RepositoryDatabaseController
import org.noelware.charted.modules.postgresql.ktor.OwnerIdAttributeKey
import org.noelware.charted.modules.search.SearchModule
import org.noelware.charted.server.extensions.addAuthenticationResponses
import org.noelware.charted.server.extensions.currentUser
import org.noelware.charted.server.extensions.putAndRemove
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController
import org.noelware.charted.server.routing.openapi.ResourceDescription
import org.noelware.charted.server.routing.openapi.describeResource

class CreateUserRepositoryRestController(
    private val controller: RepositoryDatabaseController,
    private val search: SearchModule? = null
): RestController("/users/@me/repositories", HttpMethod.Put) {
    override val apiVersion: APIVersion = APIVersion.V1
    override fun Route.init() {
        install(Sessions) {
            this += Repositories.Create
        }
    }

    override suspend fun call(call: ApplicationCall) {
        call.attributes.putAndRemove(OwnerIdAttributeKey, call.currentUser!!.id) {
            val repo = controller.create(call, call.receive())

            search?.indexRepository(repo)
            call.respond(HttpStatusCode.Created, ApiResponse.ok(repo))
        }
    }

    companion object: ResourceDescription by describeResource("/users/@me/repositories", {
        put {
            description = "Creates a repository that is owned by the current authenticated user"
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

            addAuthenticationResponses()
            created {
                json {
                    schema<ApiResponse.Ok<Repository>>()
                }
            }
        }
    })
}
