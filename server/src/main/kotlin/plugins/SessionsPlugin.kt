/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

package org.noelware.charted.server.plugins

import com.auth0.jwt.exceptions.JWTDecodeException
import com.auth0.jwt.exceptions.TokenExpiredException
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
import org.noelware.charted.common.CryptographyUtils
import org.noelware.charted.common.lazy.Lazy
import org.noelware.charted.databases.postgres.asyncTransaction
import org.noelware.charted.databases.postgres.entities.ApiKeyEntity
import org.noelware.charted.databases.postgres.entities.UserEntity
import org.noelware.charted.databases.postgres.flags.ApiKeyScopes
import org.noelware.charted.databases.postgres.models.User
import org.noelware.charted.databases.postgres.tables.ApiKeysTable
import org.noelware.charted.databases.postgres.tables.UserTable
import org.noelware.charted.modules.sessions.Session
import org.noelware.charted.modules.sessions.SessionManager
import org.noelware.charted.types.responses.ApiError
import org.noelware.charted.types.responses.ApiResponse
import java.util.Base64

val SESSIONS_KEY: AttributeKey<Session> = AttributeKey("Session")
val API_KEY_KEY: AttributeKey<ApiKeyEntity> = AttributeKey("ApiKey")
val BASIC_USER_KEY: AttributeKey<UserEntity> = AttributeKey("BasicUser")

/**
 * Returns the current session that is available
 */
val ApplicationCall.session: Session?
    get() = attributes.getOrNull(SESSIONS_KEY)

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
        if (attributes.contains(BASIC_USER_KEY)) {
            return@create attributes[BASIC_USER_KEY]
        }

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

class SessionsPlugin private constructor(private val config: Configuration) {
    private val sessionsManager: SessionManager by inject()
    private val log by logging<SessionsPlugin>()

    class Configuration {
        private val _conditions: MutableList<suspend (call: ApplicationCall) -> PreconditionResult> = mutableListOf()
        internal val scopes: ApiKeyScopes = ApiKeyScopes()

        /** If the session middleware can allow Api Key usage or not. */
        var assertSessionOnly: Boolean = false

        /**
         * Allows non authorization requests to be passed by. This means that [currentUser] will be null
         * if no authorization header was passed and all checks are bypassed, except for precondition
         * checks.
         */
        var allowNonAuthorizedRequests: Boolean = false

        /**
         * Returns a list of conditions that endpoints can use to do basic checks on a user that was
         * logged in.
         */
        internal val conditions: List<suspend (ApplicationCall) -> PreconditionResult>
            get() = _conditions

        /**
         * Adds a condition to this [Configuration] if the endpoint requires some basic checks
         * @param block The function to call.
         */
        fun condition(block: suspend (ApplicationCall) -> PreconditionResult): Configuration {
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
            if (!scopes.available(key)) throw IllegalStateException("API key scope [$key] doesn't exist")
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
            if (!scopes.available(key)) throw IllegalStateException("API key scope [$key] doesn't exist")
            scopes.add(key)
        }
    }

    private fun install(pipeline: ApplicationCallPipeline) {
        pipeline.intercept(ApplicationCallPipeline.Plugins) {
            doAuthorize(call)
        }
    }

    private suspend fun doAuthorize(call: ApplicationCall) {
        log.trace("Checking if request [${call.request.httpMethod.value} ${call.request.path()}] has an [Authorization] header")

        val auth = call.request.header(HttpHeaders.Authorization)
        if (auth == null) {
            log.trace("Request [${call.request.httpMethod.value} ${call.request.path()}] didn't include any authorization!")
            if (config.allowNonAuthorizedRequests) {
                log.trace("--> Endpoint configuration allows non authorized requests, but will be running all preconditions!~")
                runPreconditions(call)

                return
            }

            log.warn("Missing [Authorization] header on request [${call.request.httpMethod.value} ${call.request.path()}]")
            return call.respond(
                HttpStatusCode.Forbidden,
                ApiResponse.err(
                    "MISSING_AUTHORIZATION_HEADER",
                    "Rest handler requires an Authorization header to be present",
                    buildJsonObject {
                        put("method", call.request.httpMethod.value)
                        put("uri", call.request.path())
                    },
                ),
            )
        }

        log.trace("Checking if Authorization header's content is valid!")
        val data = auth.split(" ", limit = 2)
        if (data.size != 2) {
            log.warn("Authorization header didn't follow '[Bearer|ApiKey|Basic] [Token]', throwing early!")
            return call.respond(
                HttpStatusCode.NotAcceptable,
                ApiResponse.err(
                    "INVALID_AUTHORIZATION_HEADER",
                    "Request authorization header given didn't follow '[Bearer|ApiKey|Basic] [Token]'",
                    buildJsonObject {
                        put("method", call.request.httpMethod.value)
                        put("uri", call.request.path())
                    },
                ),
            )
        }

        val (prefix, token) = data
        return when (prefix) {
            "Bearer" -> {
                log.trace("Authorization header prefix for request [${call.request.httpMethod.value} ${call.request.path()}] is [Bearer]")
                doSessionBasedAuth(call, token)
            }

            "ApiKey" -> {
                log.trace("Authorization header prefix for request [${call.request.httpMethod.value} ${call.request.path()}] is [ApiKey]")
                doApiKeyBasedAuth(call, token)
            }

            "Basic" -> {
                log.trace("Authorization header prefix for request [${call.request.httpMethod.value} ${call.request.path()}] is [Basic]")
                doBasicAuth(call, token)
            }

            else -> call.respond(
                HttpStatusCode.NotAcceptable,
                ApiResponse.err(
                    "INVALID_AUTHORIZATION_HEADER_PREFIX",
                    "The given authorization prefix [$prefix] was not 'Bearer', 'ApiKey', or 'Basic'",
                ),
            )
        }
    }

    private suspend fun doSessionBasedAuth(call: ApplicationCall, token: String) {
        try {
            val session = sessionsManager.fetch(token)
                ?: return call.respond(
                    HttpStatusCode.NotFound,
                    ApiResponse.err(
                        "UNKNOWN_SESSION",
                        "Session with token doesn't exist!",
                        buildJsonObject {
                            put("method", call.request.httpMethod.value)
                            put("uri", call.request.path())
                        },
                    ),
                )

            call.attributes.put(SESSIONS_KEY, session)
            return runPreconditions(call)
        } catch (e: JWTDecodeException) {
            call.respond(
                HttpStatusCode.NotAcceptable,
                ApiResponse.err(
                    "JWT_DECODE_EXCEPTION",
                    e.message!!,
                    buildJsonObject {
                        put("method", call.request.httpMethod.value)
                        put("uri", call.request.path())
                    },
                ),
            )
        } catch (ignored: TokenExpiredException) {
            call.respond(
                HttpStatusCode.Unauthorized,
                ApiResponse.err(
                    "EXPIRED_TOKEN",
                    "Access or refresh token had expired!",
                    buildJsonObject {
                        put("method", call.request.httpMethod.value)
                        put("uri", call.request.path())
                    },
                ),
            )
        } catch (e: Throwable) {
            throw e
        }
    }

    private suspend fun doBasicAuth(call: ApplicationCall, token: String) {
        val unhashed = String(Base64.getDecoder().decode(token.toByteArray()))
        val data = unhashed.split(':', limit = 2)
        if (data.size != 2) {
            return call.respond(
                HttpStatusCode.NotAcceptable,
                ApiResponse.err(
                    "INVALID_BASIC_AUTH_CREDENTIALS",
                    "Basic authentication needs to be 'username:password'",
                    buildJsonObject {
                        put("method", call.request.httpMethod.value)
                        put("uri", call.request.path())
                    },
                ),
            )
        }

        val (username, password) = data
        val user = asyncTransaction {
            UserEntity.find { UserTable.username eq username }.firstOrNull()
        } ?: return call.respond(
            HttpStatusCode.NotFound,
            ApiResponse.err(
                "UNKNOWN_USER",
                "User with username [$username] doesn't exist",
                buildJsonObject {
                    put("method", call.request.httpMethod.value)
                    put("uri", call.request.path())
                },
            ),
        )

        if (!sessionsManager.isPasswordValid(user, password)) {
            return call.respond(
                HttpStatusCode.Unauthorized,
                ApiResponse.err(
                    "INVALID_PASSWORD",
                    "Invalid password!",
                    buildJsonObject {
                        put("method", call.request.httpMethod.value)
                        put("uri", call.request.path())
                    },
                ),
            )
        }

        call.attributes.put(BASIC_USER_KEY, user)
        return runPreconditions(call)
    }

    private suspend fun doApiKeyBasedAuth(call: ApplicationCall, token: String) {
        val hashed = CryptographyUtils.sha256Hex(token)
        val apiKey = asyncTransaction {
            ApiKeyEntity.find { ApiKeysTable.token eq hashed }.firstOrNull()
        } ?: return call.respond(
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

        if (config.assertSessionOnly) {
            return call.respond(
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
        for (bit in config.scopes.enabledFlags()) {
            if (!bits.has(bit)) {
                return call.respond(
                    HttpStatusCode.Forbidden,
                    ApiResponse.err(
                        "MISSING_API_KEY_SCOPE",
                        "API key [${apiKey.name}] doesn't have scope [$bit] enabled",
                        buildJsonObject {
                            put("method", call.request.httpMethod.value)
                            put("uri", call.request.path())
                        },
                    ),
                )
            }
        }

        call.attributes.put(API_KEY_KEY, apiKey)
        return runPreconditions(call)
    }

    private suspend fun runPreconditions(call: ApplicationCall) {
        log.trace("Running ${config.conditions.size} preconditions on request [${call.request.httpMethod.value} ${call.request.path()}]")
        for (condition in config.conditions) {
            val result = condition(call)
            if (result is PreconditionResult.Failed) {
                log.trace("--> Precondition has failed [${result.statusCode} (${result.error.code}): ${result.error.message}]")
                if (call.isHandled) {
                    log.trace("--> Precondition was already handled, not doing anything.")
                    return
                }

                call.respond(result.statusCode, ApiResponse.err(result.error))
            }
        }
    }

    companion object: BaseRouteScopedPlugin<Configuration, SessionsPlugin> {
        override val key: AttributeKey<SessionsPlugin> = AttributeKey("Sessions")
        override fun install(pipeline: ApplicationCallPipeline, configure: Configuration.() -> Unit): SessionsPlugin {
            val config = Configuration().apply(configure)
            return SessionsPlugin(config).apply { install(pipeline) }
        }
    }
}
