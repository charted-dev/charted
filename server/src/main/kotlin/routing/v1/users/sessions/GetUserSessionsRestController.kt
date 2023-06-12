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

package org.noelware.charted.server.routing.v1.users.sessions

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.models.flags.ApiKeyScope
import org.noelware.charted.modules.openapi.kotlin.dsl.*
import org.noelware.charted.modules.sessions.AbstractSessionManager
import org.noelware.charted.modules.sessions.Session
import org.noelware.charted.server.extensions.currentUser
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController
import org.noelware.charted.server.routing.openapi.ResourceDescription
import org.noelware.charted.server.routing.openapi.describeResource
import kotlin.reflect.typeOf

class GetUserSessionsRestController(private val sessionManager: AbstractSessionManager): RestController("/users/@me/sessions") {
    override val apiVersion: APIVersion = APIVersion.V1
    override fun Route.init() {
        install(Sessions) {
            this += ApiKeyScope.User.Sessions.List
        }
    }

    override suspend fun call(call: ApplicationCall) {
        val sessions = sessionManager.all(call.currentUser!!.id)
        call.respond(HttpStatusCode.OK, ApiResponse.ok(sessions.map { it.toJsonObject() }))
    }

    companion object: ResourceDescription by describeResource("/users/@me/sessions", {
        description = "REST controller for listing all the sessions by the current authenticated user"
        get {
            description = "Retrieve all the sessions by the current authenticated user"
            ok {
                description = "All the sessions that the API server is managing"
                json {
                    schema(typeOf<ApiResponse.Ok<List<Session>>>(), ApiResponse.ok(listOf<Session>()))
                }
            }

            unauthorized {
                description = "If the session token couldn't be authorized successfully"
                json {
                    schema<ApiResponse.Err>()
                }
            }

            forbidden {
                description = "Whether if the `Authorization` header is not present, or doesn't have the 'users:sessions:list' API key scope."
                json {
                    schema<ApiResponse.Err>()
                }
            }

            notAcceptable {
                description = "Whether if the `Authorization` header was not in an acceptable format"
                json {
                    schema<ApiResponse.Err>()
                }
            }
        }
    })
}
