/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022 Noelware <team@noelware.org>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *    http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.core.sessions

import dev.floofy.utils.koin.inject
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import io.ktor.util.*
import kotlinx.serialization.json.buildJsonArray
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put

val SessionKey = AttributeKey<Session>("Session")
val SessionPlugin = createRouteScopedPlugin("ChartedSessionsPlugin") {
    val sessionManager by inject<SessionManager>()

    onCall { call ->
        val auth = call.request.headers[HttpHeaders.Authorization]

        if (auth != null) {
            // Check if the authorization header is correct
            val (prefix, token) = auth.split(":", limit = 2)
            if (prefix != "Bearer") {
                call.respond(
                    HttpStatusCode.BadRequest,
                    buildJsonObject {
                        put("success", false)
                        put(
                            "errors",
                            buildJsonArray {
                                add(
                                    buildJsonObject {
                                        put("code", "INVALID_AUTH_PREFIX")
                                        put("message", "The prefix of the Authorization header must be `Bearer`")
                                    }
                                )
                            }
                        )
                    }
                )

                return@onCall
            }

            val session = sessionManager.getSession(token)
            if (session == null) {
                call.respond(
                    HttpStatusCode.BadRequest,
                    buildJsonObject {
                        put("success", false)
                        put(
                            "errors",
                            buildJsonArray {
                                add(
                                    buildJsonObject {
                                        put("code", "INVALID_AUTH_PREFIX")
                                        put("message", "JWT doesn't currently reside with a session. If the refresh token is still active, you can get a new access token by hitting `POST /users/@me/tokens/refresh`")
                                    }
                                )
                            }
                        )
                    }
                )

                return@onCall
            }

            call.attributes.put(SessionKey, session)
        }
    }
}
