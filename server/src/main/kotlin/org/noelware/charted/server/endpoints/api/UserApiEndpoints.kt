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
import okhttp3.internal.closeQuietly
import org.apache.commons.validator.routines.EmailValidator
import org.jetbrains.exposed.sql.Op
import org.jetbrains.exposed.sql.SqlExpressionBuilder.eq
import org.jetbrains.exposed.sql.deleteWhere
import org.jetbrains.exposed.sql.update
import org.noelware.charted.common.ChartedScope
import org.noelware.charted.common.IRedisClient
import org.noelware.charted.common.RandomGenerator
import org.noelware.charted.common.SHAUtils
import org.noelware.charted.common.data.helm.ChartIndexYaml
import org.noelware.charted.common.data.responses.Response
import org.noelware.charted.common.exceptions.StringOverflowException
import org.noelware.charted.common.exceptions.StringUnderflowException
import org.noelware.charted.common.exceptions.ValidationException
import org.noelware.charted.configuration.dsl.Config
import org.noelware.charted.configuration.dsl.features.Feature
import org.noelware.charted.core.StorageWrapper
import org.noelware.charted.database.controllers.NewUserBody
import org.noelware.charted.database.controllers.RepositoryController
import org.noelware.charted.database.controllers.UserController
import org.noelware.charted.database.entities.UserConnectionEntity
import org.noelware.charted.database.entities.UserEntity
import org.noelware.charted.database.models.User
import org.noelware.charted.database.models.UserConnections
import org.noelware.charted.database.tables.OrganizationTable
import org.noelware.charted.database.tables.RepositoryTable
import org.noelware.charted.database.tables.UserTable
import org.noelware.charted.server.currentUser
import org.noelware.charted.server.plugins.Sessions
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
import java.util.*

@kotlinx.serialization.Serializable
data class LoginBody(
    val username: String? = null,
    val email: String? = null,
    val password: String
) {
    init {
        if (username == null && email == null) {
            throw ValidationException("body.username|email", "You must use `username` or `email` when logging in.")
        }

        val validator: EmailValidator by inject()
        if (email != null && !validator.isValid(email)) {
            throw ValidationException("body.email", "Invalid email address.")
        }
    }
}

@kotlinx.serialization.Serializable
data class Verify2faBody(val code: String) {
    init {
        if (code.length < 6) {
            throw StringUnderflowException("body.code", code.length, 6)
        }

        if (code.length > 6) {
            throw StringOverflowException("body.code", 6)
        }
    }
}

@kotlinx.serialization.Serializable
private data class UserResponse(
    val message: String,
    val docs: String
)

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
        install(HttpMethod.Delete, "/users/@me/logout", Sessions) {
            assertSessionOnly()
        }

        install(HttpMethod.Get, "/users/@me/sessions", Sessions) {
            addScope("user:sessions:view")
        }

        install(HttpMethod.Delete, "/users/@me/sessions/{sessionId}", Sessions) {
            assertSessionOnly()
        }

//        install(HttpMethod.Get, "/users/@me/2fa/qr", Sessions) {
//            assertSessionOnly()
//        }
//
//        install(HttpMethod.Post, "/users/{id}/2fa/verify", Sessions) {
//            assertSessionOnly()
//        }
//
//        install(HttpMethod.Put, "/users/@me/2fa", Sessions) {
//            assertSessionOnly()
//        }
//
//        install(HttpMethod.Delete, "/users/@me/2fa", Sessions) {
//            assertSessionOnly()
//        }

        install(HttpMethod.Delete, "/users", Sessions) {
            assertSessionOnly()
        }

        install("/users/@me/refresh_token", Sessions) {
            assertSessionOnly()
        }

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
            Response.ok(
                UserResponse(
                    message = "Welcome to the Users API!",
                    docs = "https://charts.noelware.org/docs/server/api/users"
                )
            )
        )
    }

    @Put
    suspend fun create(call: ApplicationCall) {
        val body: NewUserBody by call.body()
        if (!config.registrations) return call.respond(
            HttpStatusCode.Forbidden,
            Response.err("REGISTRATIONS_DISABLED", "This instance has registrations disabled")
        )

        val (status, result) = UserController.create(body)
        if (status != HttpStatusCode.OK) return call.respond(status, result)

        if (!config.isFeatureEnabled(Feature.DOCKER_REGISTRY)) {
            val serialized = yaml.encodeToString(ChartIndexYaml())
            val id = (result as Response.Ok<User>).data!!.id

            if (storage.trailer is FilesystemStorageTrailer) {
                val userFolder = File((storage.trailer as FilesystemStorageTrailer).normalizePath("./metadata/$id"))
                userFolder.mkdirs()
            }

            storage.upload(
                "./metadata/$id/index.yaml",
                ByteArrayInputStream(serialized.toByteArray(Charsets.UTF_8)),
                "application/yaml"
            )
        }

        return call.respond(HttpStatusCode.OK, result)
    }

    @Delete
    suspend fun delete(call: ApplicationCall) {
        UserController.delete(call.currentUser!!.id.toLong())
        sessions.revokeAllSessions(call.currentUser!!.id.toLong())

        // Delete all the user's repositories
        asyncTransaction(ChartedScope) {
            RepositoryTable.deleteWhere { RepositoryTable.owner eq call.currentUser!!.id.toLong() }
        }

        // Delete all the user's organizations
        asyncTransaction(ChartedScope) {
            OrganizationTable.deleteWhere { OrganizationTable.owner eq call.currentUser!!.id.toLong() }
        }

        if (!config.isFeatureEnabled(Feature.DOCKER_REGISTRY)) {
            // Delete their metadata and the tarball releases.
            storage.trailer.delete("./metadata/${call.currentUser!!.id}/index.yaml")
        }

        call.respond(
            HttpStatusCode.Accepted,
            buildJsonObject {
                put("success", true)
            }
        )
    }

    @Get("/@me")
    suspend fun me(call: ApplicationCall) {
        val user = UserController.get(call.currentUser!!.id.toLong())
            ?: return call.respond(
                HttpStatusCode.NotFound,
                Response.err("UNKNOWN_USER", "Can't retrieve this user since their account was deleted.")
            )

        call.respond(
            HttpStatusCode.OK,
            Response.ok(user)
        )
    }

    @Patch("/@me")
    suspend fun update(call: ApplicationCall) {
        UserController.update(call.currentUser!!.id.toLong(), call.receive())
        call.respond(
            HttpStatusCode.OK,
            Response.ok()
        )
    }

    @Get("/@me/connections")
    suspend fun connections(call: ApplicationCall) {
        val connections = asyncTransaction(ChartedScope) {
            UserConnectionEntity.findById(call.currentUser!!.id.toLong())?.let { entity -> UserConnections.fromEntity(entity) }
        } ?: return call.respond(
            HttpStatusCode.NotFound,
            Response.err("UNKNOWN_USER", "Can't retrieve this user since their account was deleted.")
        )

        call.respond(
            HttpStatusCode.OK,
            Response.ok(connections)
        )
    }

    @Get("/@me/avatars/current.png")
    suspend fun myAvatar(call: ApplicationCall) {
        val user = UserController.get(call.currentUser!!.id.toLong())
            ?: return call.respond(
                HttpStatusCode.NotFound,
                Response.err("UNKNOWN_USER", "Can't retrieve this user since their account was deleted.")
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

            val res = httpClient.get("https://avatars.dicebear.com/api/identicon/${call.currentUser!!.id}.svg")
            val body = res.body<ByteArray>()

            call.respond(
                createOutgoingContentWithBytes(
                    body,
                    contentType = ContentType.parse(res.headers[HttpHeaders.ContentType]!!)
                )
            )

            return
        }

        val stream = storage.open("./avatars/${call.currentUser!!.id}/${user.avatarHash}")!!
        val bytes = stream.readBytes()
        when (val contentType = storage.trailer.figureContentType(ByteArrayInputStream(bytes))) {
            ContentType.Image.PNG.toString(), ContentType.Image.GIF.toString(), ContentType.Image.JPEG.toString() -> call.respond(
                createOutgoingContentWithBytes(bytes, contentType = ContentType.parse(contentType))
            )
        }
    }

    @Get("/@me/avatars/{hash}")
    suspend fun getUserAvatarByHash(call: ApplicationCall) {
        val user = UserController.get(call.currentUser!!.id.toLong())
            ?: return call.respond(
                HttpStatusCode.NotFound,
                Response.err("UNKNOWN_USER", "Can't retrieve this user since their account was deleted.")
            )

        val hash = call.parameters["hash"]!!

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

            val res = httpClient.get("https://avatars.dicebear.com/api/identicon/${call.currentUser!!.id}.svg")
            val body = res.body<ByteArray>()

            call.respond(
                createOutgoingContentWithBytes(
                    body,
                    contentType = ContentType.parse(res.headers[HttpHeaders.ContentType]!!)
                )
            )

            return
        }

        val stream = storage.open("./avatars/${call.currentUser!!.id}/$hash")
            ?: return call.respond(HttpStatusCode.NotFound)

        val bytes = stream.readBytes()
        when (val contentType = storage.trailer.figureContentType(ByteArrayInputStream(bytes))) {
            ContentType.Image.PNG.toString(), ContentType.Image.GIF.toString(), ContentType.Image.JPEG.toString() -> {
                stream.closeQuietly()
                call.respond(
                    createOutgoingContentWithBytes(bytes, contentType = ContentType.parse(contentType))
                )
            }
        }
    }

    @Post("/@me/avatar")
    suspend fun uploadAvatar(call: ApplicationCall) {
        val multipart = call.receiveMultipart()
        val parts = multipart.readAllParts()
        val first = parts.firstOrNull() ?: return call.respond(
            HttpStatusCode.BadRequest,
            Response.err("EXCESSIVE_MULTIPART_AMOUNT", "There can be only one multipart in this request.")
        )

        if (first !is PartData.FileItem) {
            return call.respond(
                HttpStatusCode.NotAcceptable,
                Response.err("NOT_FILE_PART", "The multipart object must be a File object.")
            )
        }

        val inputStream = first.streamProvider()
        val baos = ByteArrayOutputStream()
        withContext(Dispatchers.IO) {
            inputStream.transferTo(baos)
        }

        val data = baos.toByteArray()
        baos.closeQuietly()

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

        storage.upload("./avatars/${call.currentUser!!.id.toLong()}/$hash.$ext", ByteArrayInputStream(data), contentType)
        first.dispose()

        asyncTransaction(ChartedScope) {
            UserTable.update({ UserTable.id eq call.currentUser!!.id.toLong() }) {
                it[avatarHash] = "$hash.$ext"
            }
        }

        call.respond(
            HttpStatusCode.Accepted,
            Response.ok()
        )
    }

    @Post("/@me/refresh_token")
    suspend fun refreshToken(call: ApplicationCall) {
        val hasExpired = sessions.isExpired(call.session.accessToken)
        if (!hasExpired) return call.respond(
            HttpStatusCode.NotAcceptable,
            Response.err("ACCESS_TOKEN_STILL_NEW", "Access token hasn't expired yet! Please use your refresh token to expire it.")
        )

        val newSession = sessions.refreshSession(call.session)
        call.respond(
            HttpStatusCode.Created,
            Response.ok(newSession.toJsonObject(true))
        )
    }

    @Delete("/@me/logout")
    suspend fun logout(call: ApplicationCall) {
        sessions.revokeAllSessions(call.currentUser!!.id.toLong())
        call.respond(
            HttpStatusCode.Accepted,
            Response.ok()
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
                serialized.userID == call.currentUser!!.id.toLong()
            }.map {
                json.decodeFromString<Session>(it.value)
            }.map { JsonPrimitive(it.sessionID.toString()) }

        val result = JsonArray(collected)
        call.respond(
            HttpStatusCode.OK,
            Response.ok(result)
        )
    }

    @Delete("/@me/sessions/{sessionId}")
    suspend fun revokeSession(call: ApplicationCall) {
        val sessionId = call.parameters["sessionId"]!!
        val session = sessions.getSessionById(UUID.fromString(sessionId))
            ?: return call.respond(
                HttpStatusCode.NotFound,
                Response.err("UNKNOWN_SESSION", "Unknown session with UUID [$sessionId]")
            )

        // no token 4 u
        call.respond(
            HttpStatusCode.OK,
            Response.ok(session.toJsonObject())
        )
    }

    @Get("/{id}")
    suspend fun user(call: ApplicationCall) {
        val id = call.parameters["id"]!!
        val user = when {
            id.toLongOrNull() != null -> UserController.get(id.toLong())
            id.matches("^([A-z]|-|_|\\d{0,9}){0,32}".toRegex()) -> UserController.getByUsername(id)
            else -> null
        } ?: return call.respond(HttpStatusCode.NotFound, Response.err("UNKNOWN_USER", "Unknown user with ID or username [$id]"))

        call.respond(
            HttpStatusCode.OK,
            Response.ok(user)
        )
    }

    @Get("/{id}/avatar/current.png")
    suspend fun avatar(call: ApplicationCall) {
        val id = call.parameters["id"]!!
        val user = when {
            id.toLongOrNull() != null -> UserController.get(id.toLong())
            id.matches("^([A-z]|-|_|\\d{0,9}){0,32}".toRegex()) -> UserController.getByUsername(id)
            else -> null
        } ?: return call.respond(HttpStatusCode.NotFound, Response.err("UNKNOWN_USER", "Unknown user with ID or username [$id]"))

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

            val res = httpClient.get("https://avatars.dicebear.com/api/identicon/${user.id}.svg")
            val body = res.body<ByteArray>()

            call.respond(
                createOutgoingContentWithBytes(
                    body,
                    contentType = ContentType.parse(res.headers[HttpHeaders.ContentType]!!)
                )
            )

            return
        }

        val stream = storage.open("./avatars/${user.id}/${user.avatarHash}")!!
        val bytes = stream.readBytes()
        when (val contentType = storage.trailer.figureContentType(ByteArrayInputStream(bytes))) {
            ContentType.Image.PNG.toString(), ContentType.Image.GIF.toString(), ContentType.Image.JPEG.toString() -> {
                stream.closeQuietly()
                call.respond(
                    createOutgoingContentWithBytes(bytes, contentType = ContentType.parse(contentType))
                )
            }
        }
    }

    @Get("/{id}/avatars/{hash}")
    suspend fun getUserAvatarHash2(call: ApplicationCall) {
        val id = call.parameters["id"]!!
        val user = when {
            id.toLongOrNull() != null -> UserController.get(id.toLong())
            id.matches("^([A-z]|-|_|\\d{0,9}){0,32}".toRegex()) -> UserController.getByUsername(id)
            else -> null
        } ?: return call.respond(HttpStatusCode.NotFound, Response.err("UNKNOWN_USER", "Unknown user with ID or username [$id]"))

        val hash = call.parameters["hash"]!!

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

            val res = httpClient.get("https://avatars.dicebear.com/api/identicon/${user.id}.svg")
            val body = res.body<ByteArray>()

            call.respond(
                createOutgoingContentWithBytes(
                    body,
                    contentType = ContentType.parse(res.headers[HttpHeaders.ContentType]!!)
                )
            )

            return
        }

        val stream = storage.open("./avatars/${user.id}/$hash")
            ?: return call.respond(HttpStatusCode.NotFound)

        val bytes = stream.readBytes()
        when (val contentType = storage.trailer.figureContentType(ByteArrayInputStream(bytes))) {
            ContentType.Image.PNG.toString(), ContentType.Image.GIF.toString(), ContentType.Image.JPEG.toString() -> {
                stream.closeQuietly()
                call.respond(
                    createOutgoingContentWithBytes(bytes, contentType = ContentType.parse(contentType))
                )
            }
        }
    }

    @Get("/{id}/index.yaml")
    suspend fun indexYaml(call: ApplicationCall) {
        val id = call.parameters["id"]!!
        val user = when {
            id.toLongOrNull() != null -> UserController.get(id.toLong())
            id.matches("^([A-z]|-|_|\\d{0,9}){0,32}".toRegex()) -> UserController.getByUsername(id)
            else -> null
        } ?: return call.respond(HttpStatusCode.NotFound, Response.err("UNKNOWN_USER", "Unknown user with ID or username [$id]"))

        // This should never happen since when a user is created, a blank index.yaml file
        // is created in ./metadata/$uid/index.yaml!
        val result = storage.open("./metadata/${user.id}/index.yaml")!!
        val bytes = result.readBytes()

        call.respondText(
            String(bytes),
            ContentType.parse("text/plain; charset=utf-8"),
            HttpStatusCode.OK
        )
    }

    @Post("/login")
    suspend fun login(call: ApplicationCall) {
        val body: LoginBody by call.body()

        val op: Op<Boolean> = if (body.username != null) { UserTable.username eq body.username!! } else { UserTable.email eq body.email!! }
        val user = asyncTransaction {
            UserEntity.find(op).firstOrNull()
        } ?: return call.respond(
            HttpStatusCode.NotFound,
            Response.err("UNKNOWN_USER", "Unable to find user with ${if (body.username != null) "username" else "email"}.")
        )

        if (!argon2.matches(body.password, user.password)) {
            return call.respond(
                HttpStatusCode.Forbidden,
                Response.err("INVALID_PASSWORD", "Invalid password was provided.")
            )
        }

//        if (User2faController.enabled(user.id.value)) return call.respond(
//            HttpStatusCode.OK,
//            buildJsonObject {
//                put("success", true)
//                putJsonObject("data") {
//                    put("2fa", true)
//                    put("id", user.id.value)
//                }
//            }
//        )

        val session = sessions.createSession(user.id.value)
        call.respond(
            HttpStatusCode.Created,
            Response.ok(session.toJsonObject(true))
        )
    }

    // This endpoint doesn't let you discover their private repositories. API Keys
    // with the `repo:private:view` scope or sessions are allowed to view their private
    // repositories.
    @Get("/{id}/repositories")
    suspend fun repositories(call: ApplicationCall) {
        val id = call.parameters["id"]!!
        val user = when {
            id.toLongOrNull() != null -> UserController.get(id.toLong())
            id.matches("^([A-z]|-|_|\\d{0,9}){0,32}".toRegex()) -> UserController.getByUsername(id)
            else -> null
        } ?: return call.respond(HttpStatusCode.NotFound, Response.err("UNKNOWN_USER", "Unknown user with ID or username [$id]"))

        val repositories = RepositoryController.getAll(user.id.toLong(), false)
        call.respond(
            HttpStatusCode.OK,
            Response.ok(repositories)
        )
    }

    @Get("/@me/repositories")
    suspend fun myRepositories(call: ApplicationCall) {
        val repositories = RepositoryController.getAll(call.currentUser!!.id.toLong(), true)
        call.respond(
            HttpStatusCode.OK,
            Response.ok(repositories)
        )
    }

//    @Get("/@me/2fa/qr")
//    suspend fun qrCode(call: ApplicationCall) {
//        val data = User2faController.qrCode(call.currentUser!!.id)
//            ?: return call.respond(HttpStatusCode.NotFound)
//
//        call.respond(
//            HttpStatusCode.OK,
//            createOutgoingContentWithBytes(
//                data.second,
//                contentType = ContentType.parse(data.first)
//            )
//        )
//    }
//
//    @Post("/{id}/2fa/verify")
//    suspend fun verify2fa(call: ApplicationCall) {
//        val id = call.parameters["id"]!!.toLong()
//        val body by call.body<Verify2faBody>()
//        val success = User2faController.verify(id, body.code)
//        val statusCode = if (success) HttpStatusCode.OK else HttpStatusCode.BadRequest
//
//        if (success) {
//            val session = sessions.createSession(id)
//            return call.respond(
//                HttpStatusCode.Created,
//                buildJsonObject {
//                    put("success", true)
//                    put("data", session.toJsonObject(true))
//                }
//            )
//        }
//
//        call.respond(
//            statusCode,
//            buildJsonObject {
//                put("success", false)
//                putJsonArray("errors") {
//                    addJsonObject {
//                        put("message", "Invalid 2FA code.")
//                        put("code", "INVALID_2FA_CODE")
//                    }
//                }
//            }
//        )
//    }
//
//    @Put("/@me/2fa")
//    suspend fun enable2fa(call: ApplicationCall) {
//        if (User2faController.enabled(call.currentUser!!.id)) {
//            return call.respond(
//                HttpStatusCode.Forbidden,
//                buildJsonObject {
//                    put("success", false)
//                    putJsonArray("errors") {
//                        addJsonObject {
//                            put("message", "2FA is already enabled on this account.")
//                            put("code", "2FA_ENABLED_ALREADY")
//                        }
//                    }
//                }
//            )
//        }
//
//        val (status, result) = User2faController.enable2fa(call.currentUser!!.id)
//        call.respond(
//            status,
//            buildJsonObject {
//                put("success", status.value == 200)
//                put("data", result)
//            }
//        )
//    }
//
//    @Delete("/@me/2fa")
//    suspend fun disable2fa(call: ApplicationCall) {
//        if (!User2faController.enabled(call.currentUser!!.id)) {
//            return call.respond(
//                HttpStatusCode.Forbidden,
//                buildJsonObject {
//                    put("success", false)
//                    putJsonArray("errors") {
//                        addJsonObject {
//                            put("message", "2FA is already disabled on this account.")
//                            put("code", "2FA_ENABLED_ALREADY")
//                        }
//                    }
//                }
//            )
//        }
//
//        User2faController.disable2fa(call.currentUser!!.id)
//        call.respond(
//            HttpStatusCode.OK,
//            buildJsonObject {
//                put("success", true)
//            }
//        )
//    }
}
