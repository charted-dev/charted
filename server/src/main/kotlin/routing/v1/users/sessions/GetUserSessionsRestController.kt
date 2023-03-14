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
import org.noelware.charted.models.flags.ApiKeyScope
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.openapi.toPaths
import org.noelware.charted.modules.sessions.AbstractSessionManager
import org.noelware.charted.modules.sessions.Session
import org.noelware.charted.server.extensions.addAuthenticationResponses
import org.noelware.charted.server.extensions.currentUser
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.routing.RestController

class GetUserSessionsRestController(private val sessionManager: AbstractSessionManager): RestController("/users/@me/sessions") {
    override fun Route.init() {
        install(Sessions) {
            this += ApiKeyScope.User.Sessions.List
        }
    }

    override suspend fun call(call: ApplicationCall) {
        val sessions = sessionManager.all(call.currentUser!!.id)
        call.respond(HttpStatusCode.OK, ApiResponse.ok(sessions.map { it.toJsonObject() }))
    }

    override fun toPathDsl(): PathItem = toPaths("/users/@me/sessions") {
        get {
            description = "Lists all the available sessions from the current authenticated user"

            addAuthenticationResponses()
            response(HttpStatusCode.OK) {
                contentType(ContentType.Application.Json) {
                    schema<ApiResponse.Ok<List<Session>>>()
                    example = ApiResponse.ok(listOf<Session>())
                }
            }
        }
    }
}
