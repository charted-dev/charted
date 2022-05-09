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

import com.auth0.jwt.exceptions.JWTDecodeException
import dev.floofy.utils.koin.inject
import dev.floofy.utils.slf4j.logging
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
    val log by logging("org.noelware.charted.core.sessions.SessionManagerPluginKt")

    onCall { call ->
        val auth = call.request.headers[HttpHeaders.Authorization]

        if (auth != null) {
            // Check if the authorization header is correct
            val data = auth.split(" ", limit = 2)
            if (data.size != 2) {
                call.respond(
                    HttpStatusCode.BadRequest,
                    buildJsonObject {
                        put("success", false)
                        put(
                            "errors",
                            buildJsonArray {
                                add(
                                    buildJsonObject {
                                        put("code", "INVALID_AUTH_HEADER")
                                        put("message", "Authorization header must be in the style of \"Bearer <token>\"")
                                    }
                                )
                            }
                        )
                    }
                )

                return@onCall
            }

            val prefix = data.first()
            val token = data.last()
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

            try {
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
                                            put(
                                                "message",
                                                "JWT doesn't currently have a session occurring. If the refresh token is still active, you can get a new access token by hitting `POST /users/@me/refresh_token`"
                                            )
                                        }
                                    )
                                }
                            )
                        }
                    )

                    return@onCall
                }

                call.attributes.put(SessionKey, session)
            } catch (e: JWTDecodeException) {
                call.respond(
                    HttpStatusCode.NotAcceptable,
                    buildJsonObject {
                        put("success", false)
                        put(
                            "errors",
                            buildJsonArray {
                                add(
                                    buildJsonObject {
                                        put("code", "JWT_DECODE_ERROR")
                                        put("message", e.message)
                                    }
                                )
                            }
                        )
                    }
                )
            } catch (e: Exception) {
                log.error("Unable to verify JWT token", e)
                throw e
            }
        } else {
            call.respond(
                HttpStatusCode.Forbidden,
                buildJsonObject {
                    put("success", false)
                    put(
                        "errors",
                        buildJsonArray {
                            add(
                                buildJsonObject {
                                    put("code", "MISSING_AUTH_HEADER")
                                    put("message", "This request requires you to have a proper Authorization header.")
                                }
                            )
                        }
                    )
                }
            )
        }
    }
}
