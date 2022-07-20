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

@file:Suppress("UNUSED")

package org.noelware.charted.server.endpoints.api

import com.charleskorn.kaml.Yaml
import dev.floofy.utils.exposed.asyncTransaction
import dev.floofy.utils.koin.inject
import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.request.*
import io.ktor.http.*
import io.ktor.http.content.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.future.await
import kotlinx.coroutines.withContext
import kotlinx.serialization.decodeFromString
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.*
import org.apache.commons.validator.routines.EmailValidator
import org.jetbrains.exposed.sql.update
import org.noelware.charted.common.ChartedScope
import org.noelware.charted.common.IRedisClient
import org.noelware.charted.common.RandomGenerator
import org.noelware.charted.common.SHAUtils
import org.noelware.charted.common.data.Config
import org.noelware.charted.common.data.helm.ChartIndexYaml
import org.noelware.charted.common.exceptions.ValidationException
import org.noelware.charted.core.StorageWrapper
import org.noelware.charted.database.controllers.NewUserBody
import org.noelware.charted.database.controllers.RepositoryController
import org.noelware.charted.database.controllers.UserController
import org.noelware.charted.database.entities.UserConnectionEntity
import org.noelware.charted.database.entities.UserEntity
import org.noelware.charted.database.models.UserConnections
import org.noelware.charted.database.tables.UserTable
import org.noelware.charted.server.apiKeyOrNull
import org.noelware.charted.server.plugins.Sessions
import org.noelware.charted.server.plugins.sessionsKey
import org.noelware.charted.server.session
import org.noelware.charted.server.utils.createOutgoingContentWithBytes
import org.noelware.charted.sessions.Session
import org.noelware.charted.sessions.SessionManager
import org.noelware.ktor.body
import org.noelware.ktor.endpoints.*
import org.noelware.remi.core.figureContentType
import org.noelware.remi.filesystem.FilesystemStorageTrailer
import org.springframework.security.crypto.argon2.Argon2PasswordEncoder
import java.io.ByteArrayInputStream
import java.io.ByteArrayOutputStream
import java.io.File

@kotlinx.serialization.Serializable
data class LoginBody(
    val username: String? = null,
    val email: String? = null,
    val password: String
) {
    init {
        if (username == null && email == null) {
            throw ValidationException("body", "You must use `username` or `email` when logging in.")
        }

        val validator: EmailValidator by inject()
        if (email != null && !validator.isValid(email)) {
            throw ValidationException("body.email", "Invalid email address.")
        }
    }
}

class UserApiEndpoints(
    private val config: Config,
    private val storage: StorageWrapper,
    private val sessions: SessionManager,
    private val redis: IRedisClient,
    private val json: Json,
    private val yaml: Yaml,
    private val httpClient: HttpClient,
    private val argon2: Argon2PasswordEncoder
): AbstractEndpoint("/users") {
    init {
        install(HttpMethod.Delete, "/users/@me/logout", Sessions)
        install(HttpMethod.Get, "/users/@me/sessions", Sessions) {
            addScope("user:sessions:view")
        }

        install(HttpMethod.Delete, "/users", Sessions)
        install("/users/@me/refresh_token", Sessions)
        install("/users/@me/connections", Sessions) {
            addScope("user:connections")
        }

        install("/users/@me/repositories", Sessions) {
            addScope("repo:access")
        }

        install("/users/@me/avatar", Sessions) {
            addScope("user:avatar:update")
        }

        install(HttpMethod.Patch, "/users/@me", Sessions) {
            addScope("user:update")
        }

        install("/users/@me", Sessions)
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
    suspend fun create(call: ApplicationCall) {
        val body: NewUserBody by call.body()
        if (!config.registrations) return call.respond(
            HttpStatusCode.Forbidden,
            buildJsonObject {
                put("success", false)
                putJsonArray("errors") {
                    addJsonObject {
                        put("code", "REGISTRATIONS_OFF")
                        put("message", "This instance is invite only! Please ask an administrator of this instance to give you access.")
                    }
                }
            }
        )

        val (status, result) = UserController.create(body)
        val serialized = yaml.encodeToString(ChartIndexYaml())
        val id = result["data"]!!.jsonObject["id"]!!.jsonPrimitive.content

        if (storage.trailer is FilesystemStorageTrailer) {
            val userFolder = File((storage.trailer as FilesystemStorageTrailer).normalizePath("./metadata/$id"))
            userFolder.mkdirs()
        }

        storage.upload(
            "./metadata/$id/index.yaml",
            ByteArrayInputStream(serialized.toByteArray(Charsets.UTF_8)),
            "application/yaml"
        )

        call.respond(status, result)
    }

    @Delete
    suspend fun delete(call: ApplicationCall) {
        UserController.delete(call.session.userID)
        sessions.revokeAllSessions(call.session.userID)

        call.respond(
            HttpStatusCode.Accepted,
            buildJsonObject {
                put("success", true)
            }
        )
    }

    @Get("/@me")
    suspend fun me(call: ApplicationCall) {
        val user = UserController.get(call.session.userID)
            ?: return call.respond(
                HttpStatusCode.NotFound,
                buildJsonObject {
                    put("success", false)
                    putJsonArray("errors") {
                        addJsonObject {
                            put("code", "UNKNOWN_USER")
                            put("message", "Can't retrieve user if their account was previously deleted!")
                        }
                    }
                }
            )

        call.respond(
            HttpStatusCode.OK,
            buildJsonObject {
                put("success", true)
                put("data", user.toJsonObject())
            }
        )
    }

    @Patch("/@me")
    suspend fun update(call: ApplicationCall) {
        UserController.update(call.session.userID, call.receive())
        call.respond(
            HttpStatusCode.OK,
            buildJsonObject {
                put("success", true)
                putJsonObject("data") {
                    put("acknowledged", true)
                }
            }
        )
    }

    @Get("/@me/connections")
    suspend fun connections(call: ApplicationCall) {
        val connections = asyncTransaction(ChartedScope) {
            UserConnectionEntity.findById(call.session.userID)?.let { entity -> UserConnections.fromEntity(entity) }
        } ?: return call.respond(
            HttpStatusCode.NotFound,
            buildJsonObject {
                put("success", false)
                putJsonArray("errors") {
                    addJsonObject {
                        put("code", "UNKNOWN_USER")
                        put("message", "Can't retrieve user if their account was previously deleted!")
                    }
                }
            }
        )

        call.respond(
            HttpStatusCode.OK,
            buildJsonObject {
                put("success", true)
                put("data", connections.toJsonObject())
            }
        )
    }

    @Get("/@me/avatar")
    suspend fun myAvatar(call: ApplicationCall) {
        val user = UserController.get(call.session.userID)!!

        // We'll determine how to use the avatar from Gravatar (if `user.gravatar_email` is not null)
        // or Dicebar Avatars as a last resort.
        if (user.avatarHash == null) {
            if (user.gravatarEmail != null) {
                val md5Hash = SHAUtils.md5(user.gravatarEmail!!)
                val url = "https://www.gravatar.com/avatar/$md5Hash.png"
                val res = httpClient.get(url)
                val body = res.body<ByteArray>()

                call.respond(
                    createOutgoingContentWithBytes(
                        body,
                        contentType = ContentType.parse(res.headers[HttpHeaders.ContentType]!!)
                    )
                )

                return
            }

            val res = httpClient.get("https://avatars.dicebear.com/api/identicon/${call.session.userID}.svg")
            val body = res.body<ByteArray>()

            call.respond(
                createOutgoingContentWithBytes(
                    body,
                    contentType = ContentType.parse(res.headers[HttpHeaders.ContentType]!!)
                )
            )

            return
        }

        val stream = storage.open("./avatars/${call.session.userID}/${user.avatarHash}")!!
        val bytes = stream.readBytes()
        when (val contentType = storage.trailer.figureContentType(ByteArrayInputStream(bytes))) {
            ContentType.Image.PNG.toString(), ContentType.Image.GIF.toString(), ContentType.Image.JPEG.toString() -> call.respond(
                createOutgoingContentWithBytes(bytes, contentType = ContentType.parse(contentType))
            )
        }
    }

    @Post("/@me/avatar")
    suspend fun uploadAvatar(call: ApplicationCall) {
        val multipart = call.receiveMultipart()
        val parts = multipart.readAllParts()
        val first = parts.firstOrNull() ?: return call.respond(
            HttpStatusCode.BadRequest,
            buildJsonObject {
                put("success", false)
                putJsonArray("errors") {
                    addJsonObject {
                        put("code", "MORE_THAN_ONE_PART_SPECIFIED")
                        put("message", "There can be only one part or there was no parts.")
                    }
                }
            }
        )

        if (first !is PartData.FileItem) {
            return call.respond(
                HttpStatusCode.NotAcceptable,
                buildJsonObject {
                    put("success", false)
                    putJsonArray("errors") {
                        addJsonObject {
                            put("code", "NOT_FILE_PART")
                            put("message", "The multipart item was not a file.")
                        }
                    }
                }
            )
        }

        val inputStream = first.streamProvider()
        val baos = ByteArrayOutputStream()
        withContext(Dispatchers.IO) {
            inputStream.transferTo(baos)
        }

        val data = baos.toByteArray()
        val contentType = storage.trailer.figureContentType(data)
        if (!(listOf("image/png", "image/jpeg", "image/jpg", "image/gif").contains(contentType))) {
            return call.respond(
                HttpStatusCode.NotAcceptable,
                buildJsonObject {
                    put("success", false)
                    putJsonArray("errors") {
                        addJsonObject {
                            put("code", "INVALID_TARBALL")
                            put("message", "File provided was not a image type.")
                        }
                    }
                }
            )
        }

        val hash = RandomGenerator.generate(8)
        val ext = when {
            contentType.startsWith("image/png") -> ".png"
            contentType.startsWith("image/jpg") || contentType.startsWith("image/jpeg") -> ".jpg"
            contentType.startsWith("image/gif") -> ".gif"
            else -> "" // should never happen!
        }

        storage.upload("./avatars/${call.session.userID}/$hash.$ext", ByteArrayInputStream(data), contentType)
        first.dispose()

        asyncTransaction(ChartedScope) {
            UserTable.update({ UserTable.id eq call.session.userID }) {
                it[avatarHash] = "$hash.$ext"
            }
        }

        call.respond(
            HttpStatusCode.Accepted,
            buildJsonObject {
                put("success", true)
            }
        )
    }

    @Post("/@me/refresh_token")
    suspend fun refreshToken(call: ApplicationCall) {
        val hasExpired = sessions.isExpired(call.session.accessToken)
        if (!hasExpired) return call.respond(
            HttpStatusCode.NotAcceptable,
            buildJsonObject {
                put("success", false)
                putJsonArray("errors") {
                    addJsonObject {
                        put("code", "ACCESS_TOKEN_STILL_NEW")
                        put("message", "Access token is still new! Please use your refresh token once it expires.")
                    }
                }
            }
        )

        val newSession = sessions.refreshSession(call.session)
        call.respond(
            HttpStatusCode.Created,
            buildJsonObject {
                put("success", true)
                put("data", newSession.toJsonObject())
            }
        )
    }

    @Delete("/@me/logout")
    suspend fun logout(call: ApplicationCall) {
        sessions.revokeAllSessions(call.session.userID)
        call.respond(
            HttpStatusCode.Accepted,
            buildJsonObject {
                put("success", true)
            }
        )
    }

    @Get("/@me/sessions")
    suspend fun sessions(call: ApplicationCall) {
        // For security reasons, you will not be able to see the access/refresh tokens, only the
        // session UID and the device type (if we ever implement that!)
        val collected = redis
            .commands
            .hgetall("charted:sessions")
            .await()
            .filterValues {
                val serialized = json.decodeFromString<Session>(it)
                serialized.userID == call.session.userID
            }.map {
                json.decodeFromString<Session>(it.value)
            }.map { JsonPrimitive(it.sessionID.toString()) }

        val result = JsonArray(collected)
        call.respond(
            HttpStatusCode.OK,
            buildJsonObject {
                put("success", true)
                put("data", result)
            }
        )
    }

    @Get("/{id}")
    suspend fun user(call: ApplicationCall) {
        val user = UserController.get(call.parameters["id"]!!.toLong())
            ?: return call.respond(
                HttpStatusCode.NotFound,
                buildJsonObject {
                    put("success", false)
                    putJsonArray("errors") {
                        addJsonObject {
                            put("code", "UNKNOWN_USER")
                            put("message", "Unknown user with ID: ${call.parameters["id"]!!.toLong()}")
                        }
                    }
                }
            )

        call.respond(
            HttpStatusCode.OK,
            buildJsonObject {
                put("success", true)
                put("data", user.toJsonObject())
            }
        )
    }

    @Get("/{id}/avatar")
    suspend fun avatar(call: ApplicationCall) {
        val userID = call.parameters["id"]!!
        val user = UserController.get(userID.toLong())
            ?: return call.respond(
                HttpStatusCode.NotFound,
                buildJsonObject {
                    put("success", false)
                    putJsonArray("errors") {
                        addJsonObject {
                            put("code", "UNKNOWN_USER")
                            put("message", "Unknown user by ID $userID")
                        }
                    }
                }
            )

        // We'll determine how to use the avatar from Gravatar (if `user.gravatar_email` is not null)
        // or Dicebar Avatars as a last resort.
        if (user.avatarHash == null) {
            if (user.gravatarEmail != null) {
                val md5Hash = SHAUtils.md5(user.gravatarEmail!!)
                val url = "https://www.gravatar.com/avatar/$md5Hash.png"
                val res = httpClient.get(url)
                val body = res.body<ByteArray>()

                call.respond(
                    createOutgoingContentWithBytes(
                        body,
                        contentType = ContentType.parse(res.headers[HttpHeaders.ContentType]!!)
                    )
                )

                return
            }

            val res = httpClient.get("https://avatars.dicebear.com/api/identicon/$userID.svg")
            val body = res.body<ByteArray>()

            call.respond(
                createOutgoingContentWithBytes(
                    body,
                    contentType = ContentType.parse(res.headers[HttpHeaders.ContentType]!!)
                )
            )

            return
        }

        val stream = storage.open("./avatars/$userID/${user.avatarHash}")!!
        val bytes = stream.readBytes()
        when (val contentType = storage.trailer.figureContentType(ByteArrayInputStream(bytes))) {
            ContentType.Image.PNG.toString(), ContentType.Image.GIF.toString(), ContentType.Image.JPEG.toString() -> call.respond(
                createOutgoingContentWithBytes(bytes, contentType = ContentType.parse(contentType))
            )
        }
    }

    @Get("/{id}/index.yaml")
    suspend fun indexYaml(call: ApplicationCall) {
        val uid = call.parameters["id"]!!.toLong()

        // This should never happen since when a user is created, a blank index.yaml file
        // is created in ./metadata/$uid/index.yaml!
        val result = storage.open("./metadata/$uid/index.yaml")!!
        val bytes = result.readBytes()

        call.respondText(
            String(bytes),
            ContentType.parse("application/yaml; charset=utf-8"),
            HttpStatusCode.OK
        )
    }

    @Post("/login")
    suspend fun login(call: ApplicationCall) {
        val body: LoginBody by call.body()
        val user = asyncTransaction {
            UserEntity.find {
                if (body.username != null) UserTable.username eq body.username!! else UserTable.email eq body.email!!
            }.firstOrNull()
        } ?: return call.respond(
            HttpStatusCode.NotFound,
            buildJsonObject {
                put("success", false)
                putJsonArray("errors") {
                    addJsonObject {
                        put("code", "UNKNOWN_USER")
                        put("message", "Unable to find user from their ${if (body.username != null) "username" else "email"}.")
                    }
                }
            }
        )

        if (!argon2.matches(body.password, user.password)) {
            return call.respond(
                HttpStatusCode.Forbidden,
                buildJsonObject {
                    put("success", false)
                    putJsonArray("errors") {
                        addJsonObject {
                            put("code", "INVALID_PASSWORD")
                            put("message", "Invalid password.")
                        }
                    }
                }
            )
        }

        val session = sessions.createSession(user.id.value)
        call.respond(
            HttpStatusCode.Created,
            buildJsonObject {
                put("success", true)
                put("data", session.toJsonObject(true))
            }
        )
    }

    // This endpoint doesn't let you discover their private repositories. API Keys
    // with the `repo:private:view` scope or sessions are allowed to view their private
    // repositories.
    @Get("/{id}/repositories")
    suspend fun repositories(call: ApplicationCall) {
        val isSessionOrApiKey = call.attributes.getOrNull(sessionsKey) != null || call.apiKeyOrNull != null
        val repositories = RepositoryController.getAll(call.parameters["id"]!!.toLong(), isSessionOrApiKey)

        call.respond(
            HttpStatusCode.OK,
            buildJsonObject {
                put("success", true)
                put("data", JsonArray(repositories.map { it.toJsonObject() }))
            }
        )
    }

    @Get("/@me/repositories")
    suspend fun myRepositories(call: ApplicationCall) {
        val repositories = RepositoryController.getAll(call.session.userID, true)
        call.respond(
            HttpStatusCode.OK,
            buildJsonObject {
                put("success", true)
                put("data", JsonArray(repositories.map { it.toJsonObject() }))
            }
        )
    }
}
