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
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.database.controllers

import dev.floofy.utils.exposed.asyncTransaction
import dev.floofy.utils.koin.inject
import io.ktor.http.*
import kotlinx.datetime.Clock
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.buildJsonArray
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import org.apache.commons.validator.routines.EmailValidator
import org.jetbrains.exposed.sql.Op
import org.jetbrains.exposed.sql.SqlExpressionBuilder
import org.jetbrains.exposed.sql.SqlExpressionBuilder.eq
import org.jetbrains.exposed.sql.deleteWhere
import org.jetbrains.exposed.sql.update
import org.noelware.charted.common.ChartedScope
import org.noelware.charted.common.Snowflake
import org.noelware.charted.common.exceptions.StringOverflowException
import org.noelware.charted.common.exceptions.StringUnderflowException
import org.noelware.charted.common.exceptions.ValidationException
import org.noelware.charted.database.entities.UserConnectionEntity
import org.noelware.charted.database.entities.UserEntity
import org.noelware.charted.database.models.User
import org.noelware.charted.database.tables.UserTable
import org.springframework.security.crypto.argon2.Argon2PasswordEncoder

@kotlinx.serialization.Serializable
data class NewUserBody(
    val username: String,
    val password: String,
    val email: String
) {
    init {
        if (!UserController.emailValidator.isValid(email)) {
            throw ValidationException("body.email", "Email [$email] was not a valid email.")
        }

        if (username.length > 64) {
            throw StringOverflowException("body.path", 64)
        }

        if (!username.matches("^([A-z]|-|_|\\d{0,9}){0,16}".toRegex())) {
            throw ValidationException("body.username", "Username can only contain letters, digits, dashes, or underscores.")
        }

        if (password.length < 8) {
            throw StringUnderflowException("body.password", password.length, 8)
        }

        if (!password.matches("^.*(?=.{8,})(?=.*[a-zA-Z])(?=.*\\d)?(?=.*[!#\$%&? \"])?.*\$".toRegex())) {
            throw ValidationException("body.password", "Password can only contain letters, digits, and special characters.")
        }
    }
}

@kotlinx.serialization.Serializable
data class UpdateUserBody(
    @SerialName("gravatar_email")
    val gravatarEmail: String? = null,
    val description: String? = null,
    val username: String? = null,
    val password: String? = null,
    val email: String? = null,
    val name: String? = null
) {
    init {
        if (gravatarEmail != null && !UserController.emailValidator.isValid(gravatarEmail)) {
            throw ValidationException("body.gravatar_email", "The gravatar email provided was not a valid email.")
        }

        if (description != null && description.length > 240) {
            throw StringOverflowException("body.description", 240)
        }

        if (email != null && !UserController.emailValidator.isValid(email)) {
            throw ValidationException("body.email", "The email address you used was not a valid one.")
        }

        if (name != null && name.length > 64) {
            throw ValidationException("body.name", "Can't set display name with over 64 characters.")
        }
    }
}

object UserController {
    internal val emailValidator: EmailValidator by inject()
    private val argon2: Argon2PasswordEncoder by inject()

    suspend fun get(id: Long): User? = asyncTransaction(ChartedScope) {
        UserEntity.findById(id)?.let { entity -> User.fromEntity(entity) }
    }

    suspend fun getByUsername(username: String): User? = asyncTransaction(ChartedScope) {
        UserEntity
            .find { UserTable.username eq username }
            .firstOrNull()
            ?.let { entity ->
                User.fromEntity(entity)
            }
    }

    suspend fun getByEmail(email: String): User? = asyncTransaction(ChartedScope) {
        UserEntity
            .find { UserTable.email eq email }
            .firstOrNull()
            ?.let { entity -> User.fromEntity(entity) }
    }

    suspend fun create(body: NewUserBody): Pair<HttpStatusCode, JsonObject> {
        val userByUsername = getByUsername(body.username)
        if (userByUsername != null) {
            return HttpStatusCode.Forbidden to buildJsonObject {
                put("success", false)
                put(
                    "errors",
                    buildJsonArray {
                        add(
                            buildJsonObject {
                                put("code", "USERNAME_ALREADY_TAKEN")
                                put("message", "Username '${body.username}' already exists.")
                            }
                        )
                    }
                )
            }
        }

        val emailUser = getByEmail(body.email)
        if (emailUser != null) {
            return HttpStatusCode.Forbidden to buildJsonObject {
                put("success", false)
                put(
                    "errors",
                    buildJsonArray {
                        add(
                            buildJsonObject {
                                put("code", "EMAIL_ALREADY_TAKEN")
                                put("message", "Email '${body.email}' already exists.")
                            }
                        )
                    }
                )
            }
        }

        val id = Snowflake.generate()
        val pwd = argon2.encode(body.password)
        val user = asyncTransaction(ChartedScope) {
            UserEntity.new(id) {
                createdAt = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                updatedAt = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                username = body.username
                password = pwd
                email = body.email
            }.let { entity -> User.fromEntity(entity) }
        }

        asyncTransaction(ChartedScope) {
            UserConnectionEntity.new(id) {
                createdAt = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                updatedAt = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
            }
        }

        return HttpStatusCode.Created to buildJsonObject {
            put("success", true)
            put("data", user.toJsonObject())
        }
    }

    suspend fun update(id: Long, body: UpdateUserBody) {
        val whereClause: SqlExpressionBuilder.() -> Op<Boolean> = { UserTable.id eq id }
        if (body.gravatarEmail != null) {
            asyncTransaction(ChartedScope) {
                UserTable.update(whereClause) {
                    it[gravatarEmail] = body.gravatarEmail
                    it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                }
            }
        }

        if (body.description != null) {
            asyncTransaction(ChartedScope) {
                UserTable.update(whereClause) {
                    it[description] = body.description.ifEmpty { null }
                    it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                }
            }
        }

        if (body.username != null) {
            val userWithName = getByUsername(body.username)
            if (userWithName != null) {
                throw ValidationException("body.username", "Username [${body.username}] is already taken!")
            }

            asyncTransaction(ChartedScope) {
                UserTable.update(whereClause) {
                    it[username] = body.username
                    it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                }
            }
        }

        if (body.password != null) {
            val newEncodedPassword = argon2.encode(body.password)
            asyncTransaction(ChartedScope) {
                UserTable.update(whereClause) {
                    it[password] = newEncodedPassword
                    it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                }
            }
        }

        if (body.email != null) {
            val userWithEmail = getByEmail(body.email)
            if (userWithEmail != null) {
                throw ValidationException("body.email", "Email [${body.email}] is already taken!")
            }

            asyncTransaction(ChartedScope) {
                UserTable.update(whereClause) {
                    it[email] = body.email
                    it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                }
            }
        }

        if (body.name != null) {
            asyncTransaction(ChartedScope) {
                UserTable.update(whereClause) {
                    it[description] = body.name.ifEmpty { null }
                    it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                }
            }
        }
    }

    suspend fun delete(id: Long): Boolean = asyncTransaction(ChartedScope) {
        UserTable.deleteWhere { UserTable.id eq id }
        true
    }
}
