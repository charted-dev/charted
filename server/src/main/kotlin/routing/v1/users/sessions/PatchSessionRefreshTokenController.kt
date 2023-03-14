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
import io.swagger.v3.oas.models.PathItem
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.openapi.toPaths
import org.noelware.charted.modules.sessions.AbstractSessionManager
import org.noelware.charted.modules.sessions.Session
import org.noelware.charted.server.extensions.addAuthenticationResponses
import org.noelware.charted.server.extensions.session
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.routing.RestController

class PatchSessionRefreshTokenController(
    private val sessionManager: AbstractSessionManager
): RestController("/users/@me/sessions/refresh_token", HttpMethod.Post) {
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

    override fun toPathDsl(): PathItem = toPaths("/users/@me/sessions/refresh_token") {
        post {
            description = "Refreshes the current authenticated session's access token with the refresh token."

            addAuthenticationResponses()
            response(HttpStatusCode.Accepted) {
                contentType(ContentType.Application.Json) {
                    schema<ApiResponse.Ok<Session>>()
                }
            }

            response(HttpStatusCode.BadRequest) {
                description = "If the access token is still too new or if the passed in Authorization header didn't use the refresh token"
                contentType(ContentType.Application.Json) {
                    schema<ApiResponse.Err>()
                }
            }
        }
    }
}
