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
import org.noelware.charted.modules.openapi.kotlin.dsl.*
import org.noelware.charted.modules.sessions.AbstractSessionManager
import org.noelware.charted.modules.sessions.Session
import org.noelware.charted.server.extensions.session
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController
import org.noelware.charted.server.routing.openapi.ResourceDescription
import org.noelware.charted.server.routing.openapi.describeResource

class PatchSessionRefreshTokenController(
    private val sessionManager: AbstractSessionManager
): RestController("/users/@me/sessions/refresh_token", HttpMethod.Post) {
    override val apiVersion: APIVersion = APIVersion.V1
    override fun Route.init() {
        install(Sessions) {
            requireRefreshToken = true
            assertSessionOnly = true
        }
    }

    override suspend fun call(call: ApplicationCall) {
        // First, we need to check if the access token is
        // relatively new. If it is, then we shouldn't refresh
        // it.
        if (!sessionManager.isTokenExpired(call.session!!.accessToken)) {
            return call.respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "ACCESS_TOKEN_TOO_NEW",
                    "Current access token for session [${call.session!!.sessionID}] is relatively too new",
                ),
            )
        }

        val new = sessionManager.refresh(call.session!!)
        call.respond(HttpStatusCode.Accepted, ApiResponse.ok(new.toJsonObject(true)))
    }

    companion object: ResourceDescription by describeResource("/users/@me/sessions", {
        description = "REST controller for refreshing sessions by the refresh token provided when logging in"
        patch {
            description = "Refresh the session and give a new session in return"
            ok {
                description = "Refreshed session"
                json {
                    schema<ApiResponse.Ok<Session>>()
                }
            }

            unauthorized {
                description = "If the session token couldn't be authorized successfully"
                json {
                    schema<ApiResponse.Err>()
                }
            }

            forbidden {
                description = "Whether if the `Authorization` header is not present, the body was not in the format a Session Token would be in, or if the refresh token was not provided as the token it wants."
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