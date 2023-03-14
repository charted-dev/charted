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

import dev.floofy.utils.koin.inject
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.swagger.v3.oas.models.PathItem
import kotlinx.serialization.Serializable
import org.apache.commons.validator.routines.EmailValidator
import org.jetbrains.exposed.sql.Op
import org.jetbrains.exposed.sql.SqlExpressionBuilder.eq
import org.noelware.charted.ValidationException
import org.noelware.charted.common.extensions.regexp.matchesNameAndIdRegex
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.openapi.toPaths
import org.noelware.charted.modules.postgresql.asyncTransaction
import org.noelware.charted.modules.postgresql.entities.UserEntity
import org.noelware.charted.modules.postgresql.tables.UserTable
import org.noelware.charted.modules.sessions.AbstractSessionManager
import org.noelware.charted.modules.sessions.Session
import org.noelware.charted.server.routing.RestController

/**
 * Payload for logging into charted-server
 * @param username The username to authenticate as. This is mutually exclusive from [email].
 * @param password The password to authenticate as.
 * @param email The email address to authenticate as. Mutually exclusive from [username].
 */
@Serializable
data class UserLoginPayload(
    val username: String? = null,
    val password: String,
    val email: String? = null
) {
    init {
        if (username == null && email == null) {
            throw ValidationException("body", "`username` or `email` is required")
        }

        if (username != null && email != null) {
            throw ValidationException("body", "`username` and `email` are mutually exclusive", "MUTUAL_EXCLUSIVE_PROPERTIES")
        }

        if (username != null && !username.matchesNameAndIdRegex()) {
            throw ValidationException("body.username", "Username can only include letters, symbols, or digits")
        }

        val emailValidator: EmailValidator by inject()
        if (email != null && !emailValidator.isValid(email)) {
            throw ValidationException("body.email", "Email [$email] is not a valid email")
        }
    }
}

class UserLoginRestController(private val sessionManager: AbstractSessionManager): RestController("/users/login", HttpMethod.Post) {
    override suspend fun call(call: ApplicationCall) {
        val body: UserLoginPayload = call.receive()
        val key = when {
            body.username != null -> "username"
            body.email != null -> "email"
            else -> throw AssertionError("unable to reach here")
        }

        val op: Op<Boolean> = when {
            body.username != null -> UserTable.username eq body.username
            body.email != null -> UserTable.email eq body.email
            else -> throw AssertionError("unable to reach here")
        }

        val user = asyncTransaction {
            UserEntity.find { op }.firstOrNull()
        } ?: return call.respond(
            HttpStatusCode.NotFound,
            ApiResponse.err(
                "UNKNOWN_USER",
                "User with $key [${if (key == "username") body.username else body.email}] was not found",
            ),
        )

        val session = sessionManager.doAuthenticate(user, body.password)
        call.respond(HttpStatusCode.Created, ApiResponse.ok(session.toJsonObject(true)))
    }

    override fun toPathDsl(): PathItem = toPaths("/users/login") {
        post {
            description = "Login into charted-server with a username and password"

            requestBody {
                description = "Payload for logging into charted-server"
                contentType(ContentType.Application.Json) {
                    schema<UserLoginPayload>()
                    example = UserLoginPayload(
                        "noel",
                        "somepasswordthatisnotvalid",
                    )
                }
            }

            response(HttpStatusCode.Created) {
                description = "Newly created session that was created"
                contentType(ContentType.Application.Json) {
                    schema<ApiResponse.Ok<Session>>()
                }
            }

            response(HttpStatusCode.NotFound) {
                description = "If the user wasn't found"
                contentType(ContentType.Application.Json) {
                    schema<ApiResponse.Err>()
                    example = ApiResponse.err(
                        "UNKNOWN_USER",
                        "User with username [noel] was not found",
                    )
                }
            }
        }
    }
}
