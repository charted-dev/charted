/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware <team@noelware.org>
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

package org.noelware.charted.server.plugins

import com.auth0.jwt.exceptions.JWTDecodeException
import com.auth0.jwt.exceptions.TokenExpiredException
import dev.floofy.utils.exposed.asyncTransaction
import dev.floofy.utils.koin.inject
import dev.floofy.utils.slf4j.logging
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.util.*
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import org.jetbrains.exposed.sql.transactions.transaction
import org.noelware.charted.ChartedScope
import org.noelware.charted.common.CryptographyUtils
import org.noelware.charted.common.lazy.Lazy
import org.noelware.charted.databases.postgres.entities.ApiKeyEntity
import org.noelware.charted.databases.postgres.entities.UserEntity
import org.noelware.charted.databases.postgres.flags.ApiKeyScopes
import org.noelware.charted.databases.postgres.models.User
import org.noelware.charted.databases.postgres.tables.ApiKeysTable
import org.noelware.charted.modules.sessions.Session
import org.noelware.charted.modules.sessions.SessionManager
import org.noelware.charted.types.responses.ApiError
import org.noelware.charted.types.responses.ApiResponse

val SESSIONS_KEY: AttributeKey<Session> = AttributeKey("Session")
val API_KEY_KEY: AttributeKey<ApiKeyEntity> = AttributeKey("ApiKey")

/**
 * Same as [currentUserEntity] but returns a safe-serializable [User] entity.
 */
val ApplicationCall.currentUser: User?
    // Since it can get expensive on the session side, we do it lazily and fetch it whenever we need it.
    get() = currentUserEntity?.let { entity -> User.fromEntity(entity) }

/**
 * Returns the current [UserEntity] by the [SESSIONS_KEY] or [API_KEY_KEY] which
 * is determined by the [SessionsPlugin].
 */
val ApplicationCall.currentUserEntity: UserEntity?
    get() = Lazy.create {
        if (attributes.contains(SESSIONS_KEY)) {
            val attr: Session = attributes[SESSIONS_KEY]
            return@create transaction { UserEntity.findById(attr.userID) }
        }

        if (attributes.contains(API_KEY_KEY)) {
            val attr: ApiKeyEntity = attributes[API_KEY_KEY]
            return@create attr.owner
        }

        null
    }.get()

/**
 * Represents a pre-condition result.
 */
sealed class PreconditionResult {
    /**
     * Represents the precondition checks have succeeded.
     */
    object Success : PreconditionResult()

    /**
     * Represents a precondition that has failed.
     */
    class Failed(val error: ApiError, val statusCode: HttpStatusCode = HttpStatusCode.PreconditionFailed) : PreconditionResult() {
        constructor(code: String, message: String, detail: Any? = null) : this(ApiError(code, message, detail))

        constructor(
            statusCode: HttpStatusCode,
            code: String,
            message: String,
            detail: Any? = null
        ) : this(ApiError(code, message, detail), statusCode)
    }
}

/**
 * Returns the options for configuring the sessions middleware
 * @param scopes The scopes required for the session, defaults to no scopes being available.
 */
data class SessionOptions(val scopes: ApiKeyScopes = ApiKeyScopes()) {
    /** If the session middleware can allow Api Key usage or not. */
    var assertSessionOnly: Boolean = false

    /**
     * Allows non authorization requests to be passed by. This means that [currentUser] will be null
     * if no authorization header was passed and all checks are bypassed.
     */
    var allowNonAuthorizedRequests: Boolean = false
    private val _conditions: MutableList<suspend (ApplicationCall) -> PreconditionResult> = mutableListOf()

    /**
     * Returns a list of conditions that endpoints can use to do basic checks on a user that was
     * logged in.
     */
    internal val conditions: List<suspend (ApplicationCall) -> PreconditionResult>
        get() = _conditions

    /**
     * Adds a condition to this [SessionOptions] if the endpoint requires some basic checks
     * @param block The function to call.
     */
    fun condition(block: suspend (ApplicationCall) -> PreconditionResult): SessionOptions {
        _conditions.add(block)
        return this
    }

    /**
     * Assigns a required api key scope to the middleware with the specified bitfield.
     *
     * # Example
     * ```kotlin
     * install(SessionsPlugin) {
     *    this += (1L shl 0)
     * }
     * ```
     *
     * @param key bit to assign
     */
    infix operator fun plusAssign(key: Long) {
        scopes.add(key)
    }

    /**
     * Same as [plusAssign(Long)][plusAssign], but uses a bitfield flag instead of the bit
     * itself to be used.
     *
     * # Example
     * ```kotlin
     * install(SessionsPlugin) {
     *    this += "repo.create"
     * }
     * ```
     *
     * @param key bit flag to assign
     */
    infix operator fun plusAssign(key: String) {
        scopes.add(key)
    }
}

val SessionsPlugin = createRouteScopedPlugin("Sessions", ::SessionOptions) {
    val sessions: SessionManager by inject()
    val log by logging("org.noelware.charted.server.plugins.SessionPluginKt")

    onCall { call ->
        log.debug("Checking if the [Authorization] header exists!")

        val auth = call.request.header(HttpHeaders.Authorization)
        if (auth == null) {
            if (pluginConfig.allowNonAuthorizedRequests) {
                return@onCall
            }

            log.warn("Missing [Authorization] header on endpoint [${call.request.httpMethod.value} ${call.request.path()}]")
            return@onCall call.respond(
                HttpStatusCode.Forbidden,
                ApiResponse.err(
                    "MISSING_AUTH_HEADER",
                    "Rest handler requires you to use a proper Authorization header.",
                    buildJsonObject {
                        put("method", call.request.httpMethod.value)
                        put("uri", call.request.path())
                    },
                ),
            )
        }

        val data = auth.split(" ", limit = 2)
        if (data.size != 2) {
            return@onCall call.respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "INVALID_AUTH_HEADER",
                    "Authorization header didn't follow \"Bearer|ApiKey [token]\"",
                    buildJsonObject {
                        put("method", call.request.httpMethod.value)
                        put("uri", call.request.path())
                    },
                ),
            )
        }

        val token = data.last()
        when (val prefix = data.first()) {
            "Bearer" -> {
                try {
                    val session = sessions.fetch(token)
                        ?: return@onCall call.respond(
                            HttpStatusCode.BadRequest,
                            ApiResponse.err(
                                "UNKNOWN_SESSION",
                                "Current session doesn't exist! Are you sure you got a non expired one?",
                                buildJsonObject {
                                    put("method", call.request.httpMethod.value)
                                    put("uri", call.request.path())
                                },
                            ),
                        )

                    call.attributes.put(SESSIONS_KEY, session)

                    // Perform the conditions here since the session token
                    // was found.
                    for (condition in pluginConfig.conditions) {
                        val result = condition(call)
                        if (result is PreconditionResult.Failed) {
                            // If the call was already handled, let's not do anything.
                            if (call.isHandled) return@onCall

                            call.respond(
                                result.statusCode,
                                ApiResponse.err(result.error),
                            )
                        }
                    }
                } catch (e: JWTDecodeException) {
                    call.respond(
                        HttpStatusCode.BadRequest,
                        ApiResponse.err(
                            "JWT_DECODE_ERROR",
                            e.message!!,
                            buildJsonObject {
                                put("method", call.request.httpMethod.value)
                                put("uri", call.request.path())
                            },
                        ),
                    )
                } catch (_: TokenExpiredException) {
                    call.respond(
                        HttpStatusCode.BadRequest,
                        ApiResponse.err(
                            "EXPIRED_TOKEN",
                            "Access or refresh token had expired",
                            buildJsonObject {
                                put("method", call.request.httpMethod.value)
                                put("uri", call.request.path())
                            },
                        ),
                    )
                } catch (e: Throwable) {
                    log.error("Unable to retrieve session from Authorization header:", e)
                    throw e
                }
            }

            "ApiKey" -> {
                val apiKey = asyncTransaction(ChartedScope) {
                    ApiKeyEntity.find { ApiKeysTable.token eq CryptographyUtils.sha256Hex(token) }.firstOrNull()
                } ?: return@onCall call.respond(
                    HttpStatusCode.NotFound,
                    ApiResponse.err(
                        "UNKNOWN_API_KEY",
                        "Specified API key was not found",
                        buildJsonObject {
                            put("method", call.request.httpMethod.value)
                            put("uri", call.request.path())
                        },
                    ),
                )

                if (pluginConfig.assertSessionOnly) {
                    return@onCall call.respond(
                        HttpStatusCode.Forbidden,
                        ApiResponse.err(
                            "SESSION_ONLY_ROUTE",
                            "REST handler only allows session tokens to be used.",
                            buildJsonObject {
                                put("method", call.request.httpMethod.value)
                                put("uri", call.request.path())
                            },
                        ),
                    )
                }

                val bits = ApiKeyScopes(apiKey.scopes)
                for (bit in pluginConfig.scopes.enabledFlags()) {
                    if (!bits.has(bit)) {
                        call.respond(
                            HttpStatusCode.Forbidden,
                            ApiResponse.err(
                                "MISSING_API_SCOPE",
                                "Current API key doesn't have scope [$bit]",
                                buildJsonObject {
                                    put("method", call.request.httpMethod.value)
                                    put("uri", call.request.path())
                                },
                            ),
                        )

                        return@onCall
                    }
                }

                call.attributes.put(API_KEY_KEY, apiKey)

                // Perform the conditions here since the session token
                // was found.
                for (condition in pluginConfig.conditions) {
                    val result = condition(call)
                    if (result is PreconditionResult.Failed) {
                        // If the call was already handled, let's not do anything.
                        if (call.isHandled) return@onCall

                        call.respond(
                            result.statusCode,
                            ApiResponse.err(result.error),
                        )
                    }
                }
            }

            else -> call.respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "UNKNOWN_AUTH_STRATEGY",
                    "The prefix specified [$prefix] was not 'Bearer' or 'ApiKey'",
                    buildJsonObject {
                        put("method", call.request.httpMethod.value)
                        put("uri", call.request.path())
                    },
                ),
            )
        }
    }
}
