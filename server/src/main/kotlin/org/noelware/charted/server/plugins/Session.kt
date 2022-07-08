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

package org.noelware.charted.server.plugins

import com.auth0.jwt.exceptions.JWTDecodeException
import com.auth0.jwt.exceptions.TokenExpiredException
import dev.floofy.utils.koin.inject
import dev.floofy.utils.slf4j.logging
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import io.ktor.util.*
import kotlinx.serialization.json.addJsonObject
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import kotlinx.serialization.json.putJsonArray
import org.noelware.charted.core.sessions.Session
import org.noelware.charted.core.sessions.SessionManager
import org.noelware.charted.database.controllers.ApiKeyController
import org.noelware.charted.database.flags.ApiKeyScopeFlags
import org.noelware.charted.database.models.ApiKeys
import org.noelware.charted.database.models.bitfield

data class SessionOptions(
    val scopes: ApiKeyScopeFlags = ApiKeyScopeFlags(),
    val isRequired: Boolean = true
) {
    fun addScope(scope: String) {
        if (!scopes.flags.containsKey(scope)) {
            throw IllegalStateException("Unknown scope: '$scope'")
        }

        scopes.add(scopes.flags[scope]!!)
    }
}

val sessionsKey = AttributeKey<Session>("Session")
val apiKeyKey = AttributeKey<ApiKeys>("ApiKey")
val Sessions = createRouteScopedPlugin("ChartedSession", ::SessionOptions) {
    val sessions: SessionManager by inject()
    val log by logging("org.noelware.charted.core.sessions.SessionPluginKt")

    onCall { call ->
        val auth = call.request.headers[HttpHeaders.Authorization]
        if (auth == null) {
            call.respond(
                HttpStatusCode.Forbidden,
                buildJsonObject {
                    put("success", false)
                    putJsonArray("errors") {
                        addJsonObject {
                            put("code", "MISSING_AUTH_HEADER")
                            put("message", "This request requires you to have a proper Authorization header.")
                        }
                    }
                }
            )

            return@onCall
        }

        val data = auth.split(" ", limit = 2)
        if (data.size != 2) {
            call.respond(
                HttpStatusCode.BadRequest,
                buildJsonObject {
                    put("success", false)
                    putJsonArray("errors") {
                        addJsonObject {
                            put("code", "INVALID_AUTH_HEADER")
                            put("message", "Authorization header must be in the style of \"Bearer|ApiKey <token>\"")
                        }
                    }
                }
            )

            return@onCall
        }

        val token = data.last()
        when (val prefix = data.first()) {
            "Bearer" -> {
                try {
                    val session = sessions.retrieve(token)
                    if (session == null) {
                        call.respond(
                            HttpStatusCode.BadRequest,
                            buildJsonObject {
                                put("success", false)
                                putJsonArray("errors") {
                                    addJsonObject {
                                        put("code", "INVALID_AUTH_PREFIX")
                                        put(
                                            "message",
                                            "JWT doesn't currently have a session occurring. If the refresh token is still active, you can get a new access token by hitting `POST /users/@me/refresh_token`"
                                        )
                                    }
                                }
                            }
                        )

                        return@onCall
                    }

                    call.attributes.put(sessionsKey, session)
                } catch (e: JWTDecodeException) {
                    call.respond(
                        HttpStatusCode.BadRequest,
                        buildJsonObject {
                            put("success", false)
                            putJsonArray("errors") {
                                addJsonObject {
                                    put("code", "JWT_DECODE_ERROR")
                                    put("message", e.message)
                                }
                            }
                        }
                    )
                } catch (e: TokenExpiredException) {
                    call.respond(
                        HttpStatusCode.BadRequest,
                        buildJsonObject {
                            put("success", false)
                            putJsonArray("errors") {
                                addJsonObject {
                                    put("code", "TOKEN_EXPIRED")
                                    put("message", "Access or refresh token has been expired!")
                                }
                            }
                        }
                    )
                } catch (e: Throwable) {
                    log.error("Unable to get session:", e)
                    throw e
                }
            }

            "ApiKey" -> {
                val apiKey = ApiKeyController.getByToken(token, true)
                if (apiKey == null) {
                    call.respond(
                        HttpStatusCode.BadRequest,
                        buildJsonObject {
                            put("success", false)
                            putJsonArray("errors") {
                                addJsonObject {
                                    put("code", "UNKNOWN_API_KEY")
                                    put(
                                        "message",
                                        "Unknown API key."
                                    )
                                }
                            }
                        }
                    )

                    return@onCall
                }

                val bitfield = apiKey.bitfield
                for (bit in bitfield.flags.keys) {
                    if (!pluginConfig.scopes.has(bit)) {
                        call.respond(
                            HttpStatusCode.BadRequest,
                            buildJsonObject {
                                put("success", false)
                                putJsonArray("errors") {
                                    addJsonObject {
                                        put("code", "UNKNOWN_API_KEY")
                                        put(
                                            "message",
                                            "API key doesn't have required scope for route: [$bit]"
                                        )
                                    }
                                }
                            }
                        )

                        return@onCall
                    }
                }

                call.attributes.put(apiKeyKey, apiKey)
            }

            else -> {
                call.respond(
                    HttpStatusCode.BadRequest,
                    buildJsonObject {
                        put("success", false)
                        putJsonArray("errors") {
                            addJsonObject {
                                put("code", "INVALID_AUTH_PREFIX")
                                put("message", "The prefix of the Authorization header must be `Bearer` or `ApiKey` (received: $prefix)")
                            }
                        }
                    }
                )
            }
        }
    }
}
