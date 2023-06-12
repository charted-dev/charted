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

package org.noelware.charted.server.routing.v1.users.crud

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.sessions.SessionType
import org.noelware.charted.models.users.User
import org.noelware.charted.modules.helm.charts.HelmChartModule
import org.noelware.charted.modules.openapi.kotlin.dsl.*
import org.noelware.charted.modules.postgresql.controllers.users.CreateUserPayload
import org.noelware.charted.modules.postgresql.controllers.users.UserDatabaseController
import org.noelware.charted.modules.postgresql.controllers.users.connections.UserConnectionsDatabaseController
import org.noelware.charted.modules.search.SearchModule
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController
import org.noelware.charted.server.routing.openapi.ResourceDescription
import org.noelware.charted.server.routing.openapi.describeResource
import kotlin.reflect.typeOf

class CreateUserRestController(
    private val config: Config,
    private val charts: HelmChartModule? = null,
    private val search: SearchModule? = null,
    private val controller: UserDatabaseController,
    private val connectionsController: UserConnectionsDatabaseController
): RestController("/users", HttpMethod.Put) {
    override val apiVersion: APIVersion = APIVersion.V1
    override suspend fun call(call: ApplicationCall) {
        if (!config.registrations) {
            return call.respond(
                HttpStatusCode.Forbidden,
                ApiResponse.err(
                    "REGISTRATIONS_DISABLED", "This instance has registrations disabled",
                ),
            )
        }

        val payload: CreateUserPayload = call.receive()
        if (config.sessions.type == SessionType.Local && payload.password == null) {
            return call.respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "MISSING_PASSWORD_FIELD",
                    "Missing the `password` field to create this resource",
                ),
            )
        }

        if (config.sessions.type != SessionType.Local && payload.password != null) {
            return call.respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "UNAVAILABLE_BODY_PARAMETER",
                    "`password` is meant to be used with the local session manager, this is not required for the session manager this instance is using.",
                ),
            )
        }

        val user = controller.create(call, payload)
        connectionsController.create(call, user.id)

        charts?.createIndexYaml(user.id)
        search?.indexUser(user)

        call.respond(HttpStatusCode.Created, ApiResponse.ok(user))
    }

    companion object: ResourceDescription by describeResource("/users", {
        put {
            description = "REST controller to create a new user into this instance. This can fail if the server is invite-only or if registrations are disabled."
            requestBody {
                description = "Payload object to create this user"
                required = true

                json {
                    schema<CreateUserPayload>()
                }
            }

            created {
                description = "User resource was successfully created"
                json {
                    schema(typeOf<ApiResponse.Ok<User>>())
                }
            }

            badRequest {
                description = "If the session manager configured for this server is local, then this will indicate a password was not available"
                json {
                    schema(typeOf<ApiResponse.Ok<User>>())
                }
            }

            forbidden {
                description = "If the server doesn't allow new users to be created"
                json {
                    schema<ApiResponse.Err>()
                }
            }
        }
    })
}
