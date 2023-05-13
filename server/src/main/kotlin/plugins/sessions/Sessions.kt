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

package org.noelware.charted.server.plugins.sessions

import com.auth0.jwt.exceptions.JWTDecodeException
import com.auth0.jwt.exceptions.TokenExpiredException
import org.noelware.charted.server.extensions.currentUser
import dev.floofy.utils.koin.inject
import dev.floofy.utils.slf4j.logging
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.util.*
import io.ktor.util.pipeline.*
import io.sentry.Sentry
import io.sentry.protocol.User
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import org.noelware.charted.common.CryptographyUtils
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.models.flags.ApiKeyScope
import org.noelware.charted.models.flags.ApiKeyScopes
import org.noelware.charted.modules.postgresql.asyncTransaction
import org.noelware.charted.modules.postgresql.entities.ApiKeyEntity
import org.noelware.charted.modules.postgresql.entities.UserEntity
import org.noelware.charted.modules.postgresql.ktor.ApiKeyAttributeKey
import org.noelware.charted.modules.postgresql.ktor.UserEntityAttributeKey
import org.noelware.charted.modules.postgresql.tables.ApiKeyTable
import org.noelware.charted.modules.postgresql.tables.UserTable
import org.noelware.charted.modules.sessions.AbstractSessionManager
import org.noelware.charted.server.extensions.currentUserEntity
import org.noelware.charted.server.extensions.sessionKey
import java.util.*

class Sessions private constructor(private val config: Configuration) {
    private val sessionManager: AbstractSessionManager by inject()
    private val log by logging<Sessions>()

    class Configuration {
        private val _conditions: MutableList<suspend (call: ApplicationCall) -> PreconditionResult> = mutableListOf()
        internal val scopes: ApiKeyScopes = ApiKeyScopes()

        /** If the middleware should validate if the session's refresh token is the same as the one the session is using right now. */
        var requireRefreshToken: Boolean = false

        /** If the session middleware can allow Api Key usage or not. */
        var assertSessionOnly: Boolean = false

        /**
         * Allows non authorization requests to be passed by. This means that [currentUser][ApplicationCall.currentUser] will be null
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
        @Deprecated("Please use the plusAssign(ApiKeyScope) overload instead as this is unsafe")
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
        @Deprecated("Please use the plusAssign(ApiKeyScope) overload instead as this is unsafe")
        infix operator fun plusAssign(key: String) {
            if (!scopes.available(key)) throw IllegalStateException("API key scope [$key] doesn't exist")
            scopes.add(key)
        }

        /**
         * Same as [plusAssign(Long)][plusAssign], but uses the [ApiKeyScope] utility classes instead of the bit
         * itself to be used.
         *
         * # Example
         * ```kotlin
         * install(SessionsPlugin) {
         *    this += ApiKeyScope.Repositories.Access
         * }
         * ```
         *
         * @param key bit flag to assign
         */
        infix operator fun plusAssign(key: ApiKeyScope) {
            if (!scopes.available(key)) throw IllegalStateException("API key scope [${key.key}] doesn't exist")
            scopes.add(key)
        }
    }

    private fun install(pipeline: ApplicationCallPipeline) {
        pipeline.intercept(ApplicationCallPipeline.Plugins) {
            doAuthorize(call)
        }
    }

    private suspend fun doAuthorize(call: ApplicationCall) {
        log.trace("Checking if request has a [Authorization] header")

        val auth = call.request.authorization()
        if (auth == null) {
            if (config.allowNonAuthorizedRequests) {
                log.trace("Configured endpoint allows non authorized requests! Skipping authentication and running all preconditions!")
                runPreconditions(call)

                return
            }

            log.warn("Request is missing an [Authorization] header on request")
            return call.respond(
                HttpStatusCode.Forbidden,
                ApiResponse.err(
                    "MISSING_AUTHORIZATION_HEADER",
                    "Rest handler requires a Authorization header to be present",
                    buildJsonObject {
                        put("method", call.request.httpMethod.value)
                        put("url", call.request.path())
                    },
                ),
            )
        }

        log.trace("Checking if present Authorization header content is valid...")
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
        when (prefix) {
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

            else -> return call.respond(
                HttpStatusCode.NotAcceptable,
                ApiResponse.err(
                    "INVALID_AUTHORIZATION_HEADER_PREFIX",
                    "The given authorization prefix [$prefix] was not 'Bearer', 'ApiKey', or 'Basic'",
                ),
            )
        }

        if (Sentry.isEnabled()) {
            Sentry.setUser(
                User().apply {
                    email = call.currentUserEntity!!.email
                    username = call.currentUserEntity!!.username
                    id = call.currentUserEntity!!.id.value.toString()
                },
            )
        }
    }

    private suspend fun doSessionBasedAuth(call: ApplicationCall, token: String) {
        try {
            val session = sessionManager.fetch(token)
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

            if (config.requireRefreshToken && session.refreshToken != token) {
                return call.respond(
                    HttpStatusCode.BadRequest,
                    ApiResponse.err(
                        "REQUIRED_REFRESH_TOKEN",
                        "Current token provided needs to be the refresh token",
                        buildJsonObject {
                            put("method", call.request.httpMethod.value)
                            put("uri", call.request.path())
                        },
                    ),
                )
            }

            call.attributes.put(sessionKey, session)
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

        if (!sessionManager.isPasswordValid(user, password)) {
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

        call.attributes.put(UserEntityAttributeKey, user)
        return runPreconditions(call)
    }

    private suspend fun doApiKeyBasedAuth(call: ApplicationCall, token: String) {
        val hashed = CryptographyUtils.sha256Hex(token)
        val apiKey = asyncTransaction {
            ApiKeyEntity.find { ApiKeyTable.token eq hashed }.firstOrNull()
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

        call.attributes.put(ApiKeyAttributeKey, apiKey)
        return runPreconditions(call)
    }

    private suspend fun runPreconditions(call: ApplicationCall) {
        log.trace("Running ${config.conditions.size} preconditions on request [${call.request.httpMethod.value} ${call.request.path()}]")
        for (condition in config.conditions) {
            val result = condition(call)
            if (result is PreconditionResult.Failed) {
                log.trace("--> Precondition has failed [${result.status.value} ${result.status.description}]")
                if (call.isHandled) {
                    log.trace("--> Precondition was already handled, not doing anything.")
                    return
                }

                call.respond(result.status, ApiResponse.err(result.errors))
            }
        }
    }

    companion object: BaseRouteScopedPlugin<Configuration, Sessions> {
        override val key: AttributeKey<Sessions> = AttributeKey("Sessions")
        override fun install(pipeline: ApplicationCallPipeline, configure: Configuration.() -> Unit): Sessions = Sessions(
            Configuration().apply(configure),
        ).apply { install(pipeline) }
    }
}
