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
import io.swagger.v3.oas.models.PathItem
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.sessions.SessionType
import org.noelware.charted.models.users.User
import org.noelware.charted.modules.helm.charts.HelmChartModule
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.openapi.toPaths
import org.noelware.charted.modules.postgresql.controllers.users.CreateUserPayload
import org.noelware.charted.modules.postgresql.controllers.users.UserDatabaseController
import org.noelware.charted.modules.postgresql.controllers.users.connections.UserConnectionsDatabaseController
import org.noelware.charted.server.routing.RestController

class CreateUserRestController(
    private val config: Config,
    private val charts: HelmChartModule? = null,
    private val controller: UserDatabaseController,
    private val connectionsController: UserConnectionsDatabaseController
): RestController("/users", HttpMethod.Put) {
    override suspend fun call(call: ApplicationCall) {
        if (!config.registrations) {
            return call.respond(
                HttpStatusCode.Forbidden,
                ApiResponse.err(
                    "REGISTRATIONS_DISABLED", "This instance has registrations disabled",
                ),
            )
        }

        // Check if the server's session manager is using the LDAP provider,
        // if so, they will have to manually do it.
        if (config.sessions.type != SessionType.Local) {
            return call.respond(HttpStatusCode.NotImplemented)
        }

        val user = controller.create(call, call.receive())
        connectionsController.create(call, user.id)

        charts?.createIndexYaml(user.id)
        call.respond(HttpStatusCode.Created, ApiResponse.ok(user))
    }

    override fun toPathDsl(): PathItem = toPaths("/users") {
        put {
            description = "Creates a user that can interact with this instance"
            requestBody {
                description = "Payload to create a user"
                required = true

                contentType(ContentType.Application.Json) {
                    schema<CreateUserPayload>()
                }
            }

            response(HttpStatusCode.Created) {
                description = "The created user"
                contentType(ContentType.Application.Json) {
                    schema<ApiResponse.Ok<User>>()
                }
            }

            response(HttpStatusCode.Forbidden) {
                description = "If the server doesn't allow registrations"
                contentType(ContentType.Application.Json) {
                    schema<ApiResponse.Err>()
                }
            }

            response(HttpStatusCode.NotImplemented) {
                description = "If the configured session manager is not the local one"
                contentType(ContentType.Application.Json) {
                    schema<ApiResponse.Err>()
                }
            }
        }
    }
}
