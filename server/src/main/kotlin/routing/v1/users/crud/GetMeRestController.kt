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
import io.ktor.server.response.*
import io.ktor.server.routing.*
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.models.flags.ApiKeyScope
import org.noelware.charted.models.users.User
import org.noelware.charted.modules.openapi.kotlin.dsl.*
import org.noelware.charted.modules.postgresql.controllers.get
import org.noelware.charted.modules.postgresql.controllers.users.UserDatabaseController
import org.noelware.charted.server.extensions.currentUser
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController
import org.noelware.charted.server.routing.openapi.ResourceDescription
import org.noelware.charted.server.routing.openapi.describeResource
import kotlin.reflect.typeOf

class GetMeRestController(private val controller: UserDatabaseController): RestController("/users/@me") {
    override val apiVersion: APIVersion = APIVersion.V1
    override fun Route.init() {
        install(Sessions) {
            this += ApiKeyScope.User.Access
        }
    }

    override suspend fun call(call: ApplicationCall) {
        call.respond(HttpStatusCode.OK, ApiResponse.ok(controller.get(call.currentUser!!.id)))
    }

    companion object: ResourceDescription by describeResource("/users/@me", {
        description = "REST controller to retrieve the current authenticated user's metadata."
        get {
            description = "REST controller to retrieve the current authenticated user's metadata."
            ok {
                description = "User resource that was located"
                json {
                    schema(typeOf<ApiResponse.Ok<User>>())
                }
            }

            unauthorized {
                description = "If the session token couldn't be authorized successfully"
                json {
                    schema<ApiResponse.Err>()
                }
            }

            forbidden {
                description = "Whether if the `Authorization` header is not present or the body was not a proper session token"
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
