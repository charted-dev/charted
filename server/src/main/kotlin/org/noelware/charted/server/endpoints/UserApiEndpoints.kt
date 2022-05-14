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

@file:Suppress("UNUSED")
package org.noelware.charted.server.endpoints

import dev.floofy.utils.exposed.asyncTransaction
import dev.floofy.utils.koin.inject
import io.ktor.http.*
import io.ktor.http.content.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import kotlinx.datetime.*
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.*
import org.apache.commons.validator.routines.EmailValidator
import org.jetbrains.exposed.sql.*
import org.noelware.charted.core.ChartedScope
import org.noelware.charted.core.StorageWrapper
import org.noelware.charted.core.config.Config
import org.noelware.charted.core.sessions.SessionKey
import org.noelware.charted.core.sessions.SessionManager
import org.noelware.charted.core.sessions.SessionPlugin
import org.noelware.charted.database.entity.UserConnectionEntity
import org.noelware.charted.database.entity.UserEntity
import org.noelware.charted.database.tables.*
import org.noelware.charted.database.tables.Users.createdAt
import org.noelware.charted.database.tables.Users.description
import org.noelware.charted.database.tables.Users.email
import org.noelware.charted.database.tables.Users.flags
import org.noelware.charted.database.tables.Users.gravatarEmail
import org.noelware.charted.database.tables.Users.name
import org.noelware.charted.database.tables.Users.updatedAt
import org.noelware.charted.database.tables.Users.username
import org.noelware.charted.util.Sha256
import org.noelware.charted.util.Snowflake
import org.noelware.ktor.body
import org.noelware.ktor.endpoints.*
import org.noelware.remi.core.figureContentType
import org.springframework.security.crypto.argon2.Argon2PasswordEncoder
import java.io.ByteArrayInputStream
import java.io.ByteArrayOutputStream
import java.sql.BatchUpdateException

@kotlinx.serialization.Serializable
data class NewUser(
    val username: String,
    val password: String,
    val email: String
)

@kotlinx.serialization.Serializable
data class UpdateUserBody(
    @SerialName("gravatar_email")
    val gravatarEmail: String? = null,
    val description: String? = null,
    val username: String? = null,
    val password: String? = null,
    val email: String? = null,
    val name: String? = null
)

@kotlinx.serialization.Serializable
data class LoginBody(
    val username: String? = null,
    val email: String? = null,
    val password: String
)

@kotlinx.serialization.Serializable
data class User(
    val gravatarEmail: String? = null,
    val description: String? = null,
    val avatar: String? = null,
    val createdAt: LocalDateTime,
    val updatedAt: LocalDateTime,
    val username: String,
    val flags: Long,
    val name: String? = null,
    val id: Long
) {
    fun toJsonObject(): JsonObject = buildJsonObject {
        put("gravatar_email", gravatarEmail)
        put("description", description)
        put(
            "avatar_url",
            avatar?.let {
                JsonPrimitive("https://cdn.noelware.org/charted/avatars/$id/$avatar")
            } ?: JsonNull
        )
        put("created_at", createdAt.toInstant(TimeZone.currentSystemDefault()).toString())
        put("updated_at", updatedAt.toInstant(TimeZone.currentSystemDefault()).toString())
        put("username", username)
        put("flags", flags)
        put("name", name)
        put("id", id)
    }
}

class UserApiEndpoints: AbstractEndpoint("/users") {
    private val validator = EmailValidator.getInstance(true, true)
    private val encoder = Argon2PasswordEncoder()

    init {
        install(HttpMethod.Delete, "/users", SessionPlugin)
        install("/users/@me", SessionPlugin)
        install("/users/@me/avatar", SessionPlugin)
        install("/users/@me/connections", SessionPlugin)
        install("/users/@me/refresh_token", SessionPlugin)
    }

    @Get
    suspend fun main(call: ApplicationCall) {
        call.respond(
            HttpStatusCode.OK,
            buildJsonObject {
                put("success", true)
                put(
                    "data",
                    buildJsonObject {
                        put("message", "Welcome to the Users API!")
                        put("docs", "https://charts.noelware.org/docs/api/users")
                    }
                )
            }
        )
    }

    @Put
    suspend fun createUser(call: ApplicationCall) {
        val body by call.body<NewUser>()
        val config by inject<Config>()

        if (!config.registrations) {
            call.respond(
                HttpStatusCode.Forbidden,
                buildJsonObject {
                    put("success", false)
                    put(
                        "errors",
                        buildJsonArray {
                            add(
                                buildJsonObject {
                                    put("code", "REGISTRATIONS_OFF")
                                    put("message", "This instance is invite only! Please ask an administrator of this instance to give you access.")
                                }
                            )
                        }
                    )
                }
            )

            return
        }

        // Check if the username already exists
        val userByName = asyncTransaction(ChartedScope) {
            UserEntity.find {
                username eq body.username
            }.firstOrNull()
        }

        if (userByName != null) {
            call.respond(
                HttpStatusCode.Forbidden,
                buildJsonObject {
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
            )

            return
        }

        if (!validator.isValid(body.email)) {
            call.respond(
                HttpStatusCode.Forbidden,
                buildJsonObject {
                    put("success", false)
                    put(
                        "errors",
                        buildJsonArray {
                            add(
                                buildJsonObject {
                                    put("code", "INVALID_EMAIL")
                                    put("message", "Email ${body.email} was not a valid email.")
                                }
                            )
                        }
                    )
                }
            )

            return
        }

        // Check if a user with the email already exists
        val userByEmail = asyncTransaction(ChartedScope) {
            UserEntity.find {
                email eq body.email
            }.firstOrNull()
        }

        if (userByEmail != null) {
            call.respond(
                HttpStatusCode.Forbidden,
                buildJsonObject {
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
            )

            return
        }

        val id = Snowflake.generate()
        val pwd = encoder.encode(body.password)
        val user = try {
            asyncTransaction(ChartedScope) {
                val entityID = Users.insertAndGetId {
                    it[username] = body.username
                    it[password] = pwd
                    it[email] = body.email
                    it[Users.id] = id
                }

                Users
                    .select { Users.id eq entityID }
                    .limit(1)
                    .first()
                    .let { row ->
                        User(
                            row[gravatarEmail],
                            row[description],
                            row[Users.avatar],
                            row[createdAt],
                            row[updatedAt],
                            row[username],
                            row[flags],
                            row[name],
                            row[Users.id].value
                        )
                    }
            }
        } catch (e: BatchUpdateException) {
            val nextError = e.nextException
            throw nextError
        }

        // Create the user connections
        asyncTransaction(ChartedScope) {
            UserConnections.insert {
                it[noelwareAccountId] = null
                it[googleAccountId] = null
                it[appleAccountId] = null
                it[createdAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                it[UserConnections.id] = id
            }
        }

        call.respond(
            HttpStatusCode.Created,
            buildJsonObject {
                put("success", true)
                put(
                    "data",
                    user.toJsonObject()
                )
            }
        )
    }

    @Get("/@me")
    suspend fun me(call: ApplicationCall) {
        val session = call.attributes[SessionKey]
        val user = asyncTransaction(ChartedScope) {
            UserEntity.findById(session.userId)!!
        }

        call.respond(
            HttpStatusCode.OK,
            buildJsonObject {
                put("success", true)
                put(
                    "data",
                    buildJsonObject {
                        put("gravatar_email", user.gravatarEmail)
                        put("description", user.description)
                        put(
                            "avatar_url",
                            user.avatar?.let {
                                JsonPrimitive("https://cdn.noelware.org/charted/avatars/${user.id}/${user.avatar}.png")
                            } ?: JsonNull
                        )
                        put("created_at", user.createdAt.toInstant(TimeZone.currentSystemDefault()).toString())
                        put("updated_at", user.updatedAt.toInstant(TimeZone.currentSystemDefault()).toString())
                        put("username", user.username)
                        put("avatar", user.avatar)
                        put("email", user.email)
                        put("flags", user.flags)
                        put("name", user.name)
                    }
                )
            }
        )
    }

    @Get("/@me/connections")
    suspend fun connections(call: ApplicationCall) {
        val session = call.attributes[SessionKey]
        val user = asyncTransaction(ChartedScope) {
            UserConnectionEntity.findById(session.userId)!!
        }

        call.respond(
            HttpStatusCode.OK,
            buildJsonObject {
                put("success", true)
                put(
                    "data",
                    buildJsonObject {
                        put("noelware_account_id", user.noelwareAccountId)
                        put("google_account_id", user.googleAccountId)
                        put("apple_account_id", user.appleAccountId)
                        put("updated_at", user.updatedAt.toInstant(TimeZone.currentSystemDefault()).toString())
                        put("created_at", user.createdAt.toInstant(TimeZone.currentSystemDefault()).toString())
                    }
                )
            }
        )
    }

    @Get("/{id}")
    suspend fun get(call: ApplicationCall) {
        val params = call.parameters["id"] ?: throw IllegalStateException("Received `id` as null (should never happen?)")

        val user = asyncTransaction(ChartedScope) {
            UserEntity.findById(params.toLong())
        }

        if (user == null) {
            call.respond(
                HttpStatusCode.NotFound,
                buildJsonObject {
                    put("success", false)
                    put(
                        "errors",
                        buildJsonArray {
                            add(
                                buildJsonObject {
                                    put("code", "UNKNOWN_USER")
                                    put("message", "Unknown user with ID $params.")
                                }
                            )
                        }
                    )
                }
            )

            return
        }

        call.respond(
            HttpStatusCode.OK,
            buildJsonObject {
                put("success", true)
                put(
                    "data",
                    buildJsonObject {
                        put("gravatar_email", user.gravatarEmail)
                        put("description", user.description)
                        put(
                            "avatar_url",
                            user.avatar?.let {
                                JsonPrimitive("https://cdn.noelware.org/charted/avatars/${user.id}/${user.avatar}.png")
                            } ?: JsonNull
                        )
                        put("created_at", user.createdAt.toInstant(TimeZone.currentSystemDefault()).toString())
                        put("updated_at", user.updatedAt.toInstant(TimeZone.currentSystemDefault()).toString())
                        put("username", user.username)
                        put("avatar", user.avatar)
                        put("flags", user.flags)
                        put("name", user.name)
                    }
                )
            }
        )
    }

    @Patch("/@me")
    suspend fun updateMe(call: ApplicationCall) {
        val body by call.body<UpdateUserBody>()
        val session = call.attributes[SessionKey]

        val errors = mutableListOf<JsonObject>()
        val success = mutableMapOf<String, Boolean>()
        var errored = false

        if (body.gravatarEmail != null) {
            if (!validator.isValid(body.gravatarEmail)) {
                errors.add(
                    buildJsonObject {
                        put("code", "INVALID_EMAIL_ADDRESS")
                        put("message", "Invalid email address to use: '${body.gravatarEmail}'")
                    }
                )

                errored = true
            }

            if (!errored) {
                asyncTransaction(ChartedScope) {
                    Users.update({ Users.id eq session.userId }) {
                        it[gravatarEmail] = gravatarEmail
                        it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                    }
                }

                success["gravatar_email"] = true
            }
        }

        if (body.description != null) {
            if (body.description!!.length > 240) {
                errors.add(
                    buildJsonObject {
                        put("code", "STRING_TOO_LONG")
                        put("message", "The description you provided was over 240 characters.")
                    }
                )

                errored = true
            }

            if (!errored && body.description!!.isEmpty()) {
                asyncTransaction(ChartedScope) {
                    Users.update({ Users.id eq session.userId }) {
                        it[description] = null
                        it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                    }
                }

                success["description"] = true
            } else if (!errored) {
                asyncTransaction(ChartedScope) {
                    Users.update({ Users.id eq session.userId }) {
                        it[description] = body.description
                        it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                    }
                }

                success["description"] = true
            }
        }

        if (body.username != null) {
            val userByName = asyncTransaction(ChartedScope) {
                UserEntity.find {
                    username eq body.username!!
                }.firstOrNull()
            }

            if (userByName != null) {
                errored = true
                errors.add(
                    buildJsonObject {
                        put("code", "USER_ALREADY_EXIST")
                        put("message", "Account with username ${body.username} already exists.")
                    }
                )
            }

            if (!errored) {
                asyncTransaction(ChartedScope) {
                    Users.update({ Users.id eq session.userId }) {
                        it[username] = body.username!!
                        it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                    }
                }

                success["username"] = true
            }
        }

        if (body.password != null) {
            val pass = encoder.encode(body.password)

            asyncTransaction(ChartedScope) {
                Users.update({ Users.id eq session.userId }) {
                    it[password] = pass
                    it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                }
            }

            success["password"] = true
        }

        if (body.email != null) {
            val userByEmail = asyncTransaction(ChartedScope) {
                UserEntity.find {
                    email eq body.email!!
                }.firstOrNull()
            }

            if (!validator.isValid(body.gravatarEmail)) {
                errors.add(
                    buildJsonObject {
                        put("code", "INVALID_EMAIL_ADDRESS")
                        put("message", "Invalid email address to use: '${body.gravatarEmail}'")
                    }
                )

                errored = true
            }

            if (userByEmail != null) {
                errored = true
                errors.add(
                    buildJsonObject {
                        put("code", "USER_ALREADY_EXIST")
                        put("message", "Account with email ${body.email} already exists.")
                    }
                )
            }

            if (!errored) {
                asyncTransaction(ChartedScope) {
                    Users.update({ Users.id eq session.userId }) {
                        it[email] = body.email!!
                        it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                    }
                }

                success["email"] = true
            }
        }

        if (body.name != null) {
            if (body.name!!.length > 69) {
                errors.add(
                    buildJsonObject {
                        put("code", "STRING_TOO_LONG")
                        put("message", "The description you provided was over 69 characters.")
                    }
                )

                errored = true
            }

            if (!errored && body.name!!.isEmpty()) {
                asyncTransaction(ChartedScope) {
                    Users.update({ Users.id eq session.userId }) {
                        it[name] = null
                        it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                    }
                }

                success["name"] = true
            } else if (!errored) {
                asyncTransaction(ChartedScope) {
                    Users.update({ Users.id eq session.userId }) {
                        it[name] = body.name
                        it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                    }
                }

                success["name"] = true
            }
        }

        val statusCode = if (errored) HttpStatusCode.BadRequest else HttpStatusCode.OK
        call.respond(
            statusCode,
            buildJsonObject {
                put("success", errored)

                if (errored) {
                    put(
                        "errors",
                        buildJsonArray {
                            for (error in errors) {
                                add(error)
                            }
                        }
                    )
                } else {
                    put("data", JsonObject(success.mapValues { JsonPrimitive(it.value) }))
                }
            }
        )
    }

    @Delete
    suspend fun deleteCurrent(call: ApplicationCall) {
        val session = call.attributes[SessionKey]
        asyncTransaction {
            // Delete the user
            Users.deleteWhere {
                Users.id eq session.userId
            }

            // Delete all the repositories the user owned
            Repositories.deleteWhere {
                Repositories.ownerId eq session.userId
            }

            Organizations.deleteWhere {
                Organizations.owner eq session.userId
            }
        }

        call.respond(HttpStatusCode.NoContent)
    }

    @Post("/login")
    suspend fun login(call: ApplicationCall) {
        val body by call.body<LoginBody>()
        val (value, key) = if (body.username != null)
            Pair(body.username, username)
        else if (body.email != null)
            Pair(body.email, email)
        else
            Pair(null, null)

        if (value == null && key == null) {
            call.respond(
                HttpStatusCode.BadRequest,
                buildJsonObject {
                    put("success", false)
                    put(
                        "errors",
                        buildJsonArray {
                            add(
                                buildJsonObject {
                                    put("code", "UNKNOWN_KEY_TO_USE")
                                    put("message", "Cannot determine to use username or email to login.")
                                }
                            )
                        }
                    )
                }
            )

            return
        }

        val user = asyncTransaction {
            UserEntity.find {
                key!! eq value!!
            }.firstOrNull()
        }

        if (user == null) {
            call.respond(
                HttpStatusCode.NotFound,
                buildJsonObject {
                    put("success", false)
                    put(
                        "errors",
                        buildJsonArray {
                            add(
                                buildJsonObject {
                                    put("code", "UNKNOWN_USER")
                                    put("message", "Unable to find user with value '$value'")
                                }
                            )
                        }
                    )
                }
            )

            return
        }

        // Check if the password is correct
        if (!encoder.matches(body.password, user.password)) {
            call.respond(
                HttpStatusCode.Forbidden,
                buildJsonObject {
                    put("success", false)
                    put(
                        "errors",
                        buildJsonArray {
                            add(
                                buildJsonObject {
                                    put("code", "INVALID_PASSWORD")
                                    put("message", "Invalid password.")
                                }
                            )
                        }
                    )
                }
            )

            return
        }

        // Get the session manager
        val sessions by inject<SessionManager>()
        val session = sessions.createSession(user.id.toString())

        call.respond(
            HttpStatusCode.Created,
            buildJsonObject {
                put("success", true)
                put(
                    "data",
                    buildJsonObject {
                        put("access_token", session.accessToken)
                        put("refresh_token", session.refreshToken)
                        put("session_id", session.sessionId.toString())
                    }
                )
            }
        )
    }

    @Post("/@me/avatar")
    suspend fun avatar(call: ApplicationCall) {
        val session = call.attributes[SessionKey]
        val body = call.receiveMultipart()
        val parts = body.readAllParts()

        val firstPart = parts.firstOrNull()
        if (firstPart == null) {
            call.respond(
                HttpStatusCode.BadRequest,
                buildJsonObject {
                    put("success", false)
                    put(
                        "errors",
                        buildJsonArray {
                            add(
                                buildJsonObject {
                                    put("code", "BAD_MULTIPART_REQUEST")
                                    put("message", "There can be only file descriptor or there was none.")
                                }
                            )
                        }
                    )
                }
            )

            return
        }

        if (firstPart !is PartData.FileItem) {
            call.respond(
                HttpStatusCode.BadRequest,
                buildJsonObject {
                    put("success", false)
                    put(
                        "errors",
                        buildJsonArray {
                            add(
                                buildJsonObject {
                                    put("code", "BAD_MULTIPART_REQUEST")
                                    put("message", "The multipart object must be a file descriptor.")
                                }
                            )
                        }
                    )
                }
            )

            return
        }

        // Create a SHA256 hash of the file name
        val hash = Sha256.encode(firstPart.originalFileName ?: "file")
        val inputStream = firstPart.streamProvider()

        val baos = ByteArrayOutputStream()
        withContext(Dispatchers.IO) {
            inputStream.transferTo(baos)
        }

        val data = baos.toByteArray()
        val newStream = ByteArrayInputStream(data)

        // We had to clone the input stream, so we can retrieve the content type.
        val storage by inject<StorageWrapper>()

        when (val contentType = storage.trailer.figureContentType(newStream)) {
            ContentType.Image.PNG.toString(), ContentType.Image.GIF.toString(), ContentType.Image.JPEG.toString() -> {
                val ext = when (contentType) {
                    ContentType.Image.PNG.toString() -> "png"
                    ContentType.Image.JPEG.toString() -> "jpg"
                    ContentType.Image.GIF.toString() -> "gif"
                    else -> error("should never happen")
                }

                storage.upload(
                    "./avatars/${session.userId}/$hash.$ext",
                    newStream,
                    contentType
                )

                asyncTransaction {
                    Users.update({ Users.id eq session.userId }) {
                        it[avatar] = "$hash.$ext"
                    }
                }

                call.respond(
                    HttpStatusCode.Created,
                    buildJsonObject {
                        put("success", true)
                    }
                )
            }

            else -> call.respond(
                HttpStatusCode.NotAcceptable,
                buildJsonObject {
                    put("success", false)
                    put(
                        "errors",
                        buildJsonArray {
                            add(
                                buildJsonObject {
                                    put("code", "INVALID_FORMAT")
                                    put("message", "Cannot use content type $contentType.")
                                }
                            )
                        }
                    )
                }
            )
        }
    }

    @Post("/@me/refresh_token")
    suspend fun refreshSessionToken(call: ApplicationCall) {
        val sessions by inject<SessionManager>()
        val session = call.attributes[SessionKey]

        // Check if the access token is not expired.
        val `continue` = sessions.isExpired(session.accessToken)
        if (!`continue`) {
            call.respond(
                HttpStatusCode.NotAcceptable,
                buildJsonObject {
                    put("success", false)
                    put(
                        "errors",
                        buildJsonArray {
                            add(
                                buildJsonObject {
                                    put("code", "NOT_READY_FOR_REFRESH")
                                    put("message", "The access token is too new, cannot refresh token!")
                                }
                            )
                        }
                    )
                }
            )

            return
        }

        val newSession = sessions.refreshSession(session)
        call.respond(
            HttpStatusCode.Created,
            buildJsonObject {
                put("success", true)
                put(
                    "data",
                    buildJsonObject {
                        put("access_token", newSession.accessToken)
                        put("refresh_token", newSession.refreshToken)
                        put("session_id", newSession.sessionId.toString())
                    }
                )
            }
        )
    }
}
