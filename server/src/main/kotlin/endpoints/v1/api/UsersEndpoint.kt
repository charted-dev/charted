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

@file:Suppress("UNUSED")

package org.noelware.charted.server.endpoints.v1.api

import dev.floofy.utils.exposed.asyncTransaction
import dev.floofy.utils.koin.inject
import io.ktor.http.*
import io.ktor.http.content.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import kotlinx.datetime.Clock
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.apache.commons.validator.routines.EmailValidator
import org.jetbrains.exposed.sql.*
import org.jetbrains.exposed.sql.SqlExpressionBuilder.eq
import org.noelware.charted.*
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.features.ServerFeature
import org.noelware.charted.configuration.kotlin.dsl.sessions.SessionType
import org.noelware.charted.databases.postgres.entities.RepositoryEntity
import org.noelware.charted.databases.postgres.entities.UserConnectionEntity
import org.noelware.charted.databases.postgres.entities.UserEntity
import org.noelware.charted.databases.postgres.models.Repository
import org.noelware.charted.databases.postgres.models.User
import org.noelware.charted.databases.postgres.models.UserConnections
import org.noelware.charted.databases.postgres.tables.OrganizationTable
import org.noelware.charted.databases.postgres.tables.RepositoryTable
import org.noelware.charted.databases.postgres.tables.UserTable
import org.noelware.charted.extensions.regexp.toNameRegex
import org.noelware.charted.extensions.regexp.toPasswordRegex
import org.noelware.charted.modules.avatars.AvatarFetchUtil
import org.noelware.charted.modules.avatars.AvatarModule
import org.noelware.charted.modules.helm.charts.HelmChartModule
import org.noelware.charted.modules.redis.RedisClient
import org.noelware.charted.modules.sessions.SessionManager
import org.noelware.charted.modules.storage.StorageHandler
import org.noelware.charted.server.createKtorContentWithByteArray
import org.noelware.charted.server.plugins.SessionsPlugin
import org.noelware.charted.server.plugins.currentUser
import org.noelware.charted.types.helm.RepoType
import org.noelware.charted.types.responses.ApiResponse
import org.noelware.ktor.body
import org.noelware.ktor.endpoints.*
import org.springframework.security.crypto.argon2.Argon2PasswordEncoder

@Serializable
data class NewUserBody(
    val username: String,
    val password: String,
    val email: String
) {
    init {
        val validator: EmailValidator by inject()
        if (!validator.isValid(email)) {
            throw ValidationException("body.email", "Email [$email] was not a valid email.")
        }

        if (username.length > 32) {
            throw StringOverflowException("body.username", 32)
        }

        if (!username.toNameRegex().matches()) {
            throw ValidationException("body.username", "Username can only contain letters, digits, dashes, or underscores.")
        }

        if (!password.toPasswordRegex().matches()) {
            throw ValidationException("body.password", "Password can only contain letters, digits, and special characters.")
        }
    }
}

@Serializable
data class LoginBody(
    val username: String? = null,
    val email: String? = null,
    val password: String
) {
    init {
        if (username == null && email == null) {
            throw ValidationException("body.username|email", "You must use `username` or `email` when logging in.")
        }

        if (username != null && email != null) {
            throw ValidationException("body.username|email", "`username` and `email` are mutually exclusive")
        }

        val validator: EmailValidator by inject()
        if (email != null && !validator.isValid(email)) {
            throw ValidationException("body.email", "Invalid email address.")
        }
    }
}

@Serializable
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
        val emailValidator: EmailValidator by inject()
        if (gravatarEmail != null && !emailValidator.isValid(gravatarEmail)) {
            throw ValidationException("body.gravatar_email", "The gravatar email provided was not a valid email.")
        }

        if (description != null && description.length > 240) {
            throw StringOverflowException("body.description", 240)
        }

        if (password != null && !password.toPasswordRegex().matches()) {
            throw ValidationException("body.password", "New user password can only contain letters, digits, and special characters.")
        }

        if (username != null) {
            if (username.length > 32) {
                throw StringOverflowException("body.username", 32)
            }

            if (!username.toNameRegex().matches()) {
                throw ValidationException("body.username", "Username can only contain letters, digits, dashes, or underscores.")
            }
        }

        if (email != null && !emailValidator.isValid(email)) {
            throw ValidationException("body.email", "The email address you used was not a valid one.")
        }

        if (name != null && name.length > 64) {
            throw ValidationException("body.name", "Can't set display name with over 64 characters.")
        }
    }
}

@Serializable
data class MainUserResponse(
    val message: String = "Welcome to the Users API!",

    @SerialName("docs_url")
    val docsUrl: String = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/users"
)

@Serializable
data class CreateRepositoryBody(
    val description: String? = null,
    val private: Boolean = false,
    val name: String,
    val type: RepoType
) {
    init {
        if (description != null && description.length > 240) {
            throw StringOverflowException("body.description", 240)
        }

        if (name.length > 24) {
            throw StringOverflowException("body.name", 32)
        }

        if (!name.toNameRegex(true, 24).matches()) {
            throw ValidationException("body.name", "Repository name can only contain alphabet characters, digits, underscores, and dashes.")
        }
    }
}

class UsersEndpoint(
    private val config: Config,
    private val storage: StorageHandler,
    private val sessions: SessionManager,
    private val redis: RedisClient,
    private val avatars: AvatarModule,
    private val argon2: Argon2PasswordEncoder,
    private val snowflake: org.noelware.charted.snowflake.Snowflake,
    private val charts: HelmChartModule? = null
) : AbstractEndpoint("/users") {
    init {
        install(HttpMethod.Post, "/users/@me/avatar", SessionsPlugin) {
            this += "user:avatar:update"
        }

        install(HttpMethod.Get, "/users/@me/connections", SessionsPlugin) {
            this += "user:connections"
        }

        install(HttpMethod.Get, "/users/@me", SessionsPlugin) {
            this += "user:view"
        }

        install(HttpMethod.Patch, "/users", SessionsPlugin) {
            this += "user:update"
        }

        install(HttpMethod.Delete, "/users", SessionsPlugin) {
            assertSessionOnly = true
        }

        install(HttpMethod.Put, "/users/@me/repositories", SessionsPlugin) {
            this += "repo:create"
        }
    }

    /**
     * Generic entrypoint route for `GET /users`. Nothing too special!
     * @statusCode 200
     */
    @Get
    suspend fun main(call: ApplicationCall): Unit = call.respond(HttpStatusCode.OK, ApiResponse.ok(MainUserResponse()))

    /**
     * Registers a new user in charted-server that can create repositories and organizations, if the server
     * does allow registrations (determined via `config.registrations`).
     *
     * @statusCode 201 The newly registered user
     * @statusCode 403 If the server has registrations disabled
     * @statusCode 406 If any validation exceptions had been thrown (i.e, username/email was taken)
     * @statusCode 500 If any database errors occur
     * @statusCode 503 If the registered sessions manager disallows user creation
     */
    @Put
    suspend fun create(call: ApplicationCall) {
        if (!config.registrations) {
            return call.respond(
                HttpStatusCode.Forbidden,
                ApiResponse.err(
                    "REGISTRATIONS_DISABLED", "This instance has registrations disabled.",
                ),
            )
        }

        // Check if the server's session manager is using the LDAP provider,
        // if so, they will have to manually do it.
        if (config.sessions.type != SessionType.Local) {
            return call.respond(HttpStatusCode.NotImplemented)
        }

        val body: NewUserBody = call.receive()

        // Check if the username already exists in the database since it is unique
        val userByUserName = asyncTransaction(ChartedScope) {
            UserEntity.find { UserTable.username eq body.username }.firstOrNull()
        }

        if (userByUserName != null) {
            throw ValidationException("body.username", "Username [${body.username}] already exists")
        }

        val userByEmail = asyncTransaction(ChartedScope) {
            UserEntity.find { UserTable.email eq body.username }.firstOrNull()
        }

        if (userByEmail != null) {
            throw ValidationException("body.email", "Email [${body.email}] already exists")
        }

        val id = snowflake.generate()
        val user = asyncTransaction(ChartedScope) {
            UserEntity.new(id.value) {
                createdAt = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                updatedAt = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                username = body.username
                password = argon2.encode(body.password)
                email = body.email
            }.let { entity -> User.fromEntity(entity) }
        }

        asyncTransaction(ChartedScope) {
            UserConnectionEntity.new(user.id) {
                createdAt = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                updatedAt = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
            }
        }

        charts?.createIndexYaml(user.id)
        return call.respond(HttpStatusCode.Created, ApiResponse.ok(user))
    }

    /**
     * Update any user metadata in the database. This is useful for changing the user's username
     * if needed or whatnot. This requires the user to be logged in or have the `users:update`
     * API key scope.
     *
     * @statusCode 202 If the database has patched the user's metadata.
     * @statusCode 406 If any [ValidationException] had been thrown
     * @statusCode 500 If any database errors had been thrown
     */
    @Patch
    suspend fun patch(call: ApplicationCall) {
        val patched: UpdateUserBody = call.receive()
        val whereClause: SqlExpressionBuilder.() -> Op<Boolean> = { UserTable.id eq call.currentUser!!.id }

        // Do some post checkup before actually patching data
        if (patched.username != null) {
            val userWithUsername = asyncTransaction(ChartedScope) {
                UserEntity.find { UserTable.username eq patched.username }.firstOrNull()
            }

            if (userWithUsername != null) {
                throw ValidationException("body.username", "Username [${patched.username}] is already taken")
            }
        }

        if (patched.email != null) {
            val userWithEmail = asyncTransaction(ChartedScope) {
                UserEntity.find { UserTable.email eq patched.email }.firstOrNull()
            }

            if (userWithEmail != null) {
                throw ValidationException("body.username", "Username [${patched.email}] is already taken")
            }
        }

        asyncTransaction(ChartedScope) {
            UserTable.update(whereClause) {
                it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())

                if (patched.gravatarEmail != null) {
                    it[gravatarEmail] = patched.gravatarEmail
                }

                if (patched.description != null) {
                    it[description] = patched.description
                }

                if (patched.username != null) {
                    it[username] = patched.username
                }

                if (patched.password != null && config.sessions.type == SessionType.Local) {
                    it[password] = argon2.encode(patched.password)
                }

                if (patched.email != null) {
                    it[email] = patched.email
                }

                if (patched.name != null) {
                    it[name] = patched.name
                }
            }
        }

        return call.respond(HttpStatusCode.Accepted, ApiResponse.ok())
    }

    /**
     * Deletes the logged-in user. This can only work with session tokens to not
     * accidentally delete your user account with an API key, just a security
     * measure~!
     *
     * @statusCode 202 If the user has successfully been deleted from Postgres and Elasticsearch.
     *                 Optionally, if the email service is enabled, it will send out an email of
     *                 all the collected data that was wiped.
     *
     * @statusCode 500 If any database errors had been thrown
     */
    @Delete
    suspend fun delete(call: ApplicationCall) {
        val id = call.currentUser!!.id

        // First, let's collect all the repositories that are owned
        // by this user, so we can delete it from the storage driver.
//        val repositories = asyncTransaction(ChartedScope) {
//            RepositoryEntity.find { RepositoryTable.owner eq id }.toList()
//        }

        // Delete the user, which will delete all of their organizations
        // except their repositories since repositories can be tied to both
        // organization and a user. So, we can do that after.
        asyncTransaction(ChartedScope) {
            UserTable.deleteWhere { UserTable.id eq id }
        }

        asyncTransaction(ChartedScope) {
            OrganizationTable.deleteWhere { owner eq id }
        }

        // Delete all the repositories owned by this user
        asyncTransaction(ChartedScope) {
            RepositoryTable.deleteWhere { owner eq id }
        }

        // As this can take a while and network failures are prone (if not using
        // the filesystem storage driver), deleting all the repository metadata
        // will be pushed to a separate background job
        //
        // ...but for now, we do this the hard way and run this in the
        // same coroutine as this method is being executed from.
        //
        // but in the future and when charted-server supports High Availability,
        // I plan to have this called in a separate worker pool.
        if (!config.features.contains(ServerFeature.DOCKER_REGISTRY)) {
            /* TODO: this */
        }

        sessions.revokeAll(id)
        charts?.destroyIndexYaml(id)

        call.respond(HttpStatusCode.Accepted, ApiResponse.ok())
    }

    /**
     * Login as a user and return a session object that contains the access and refresh
     * token
     *
     * @statusCode 201 The [Session][org.noelware.charted.modules.sessions.Session] object that is created and persisted
     */
    @Post("/login")
    suspend fun login(call: ApplicationCall) {
        val body: LoginBody = call.receive()
        val op: Op<Boolean> = if (body.username != null) {
            UserTable.username eq body.username
        } else {
            UserTable.email eq body.email!!
        }

        val user = asyncTransaction(ChartedScope) { UserEntity.find(op).firstOrNull() }
            ?: return call.respond(HttpStatusCode.NotFound, ApiResponse.err("UNKNOWN_USER", "Unable to find user with ${if (body.username != null) "username [${body.username}]" else "email [${body.email}]"}"))

        val session = sessions.doAuthenticate(user, body.password)
        return call.respond(HttpStatusCode.OK, ApiResponse.ok(session.toJsonObject(true)))
    }

    /**
     * Gets the current logged-in user.
     * @statusCode 200 The current user's details
     */
    @Get("/@me")
    suspend fun getCurrentUser(call: ApplicationCall) {
        val user = call.currentUser!!
        call.respond(HttpStatusCode.OK, ApiResponse.ok(user))
    }

    /**
     * Returns the current logged-in user's avatar.
     * @param 200 The user avatar or any defaults if the user hasn't ever set an avatar.
     */
    @Get("/@me/avatars/current.png")
    suspend fun getCurrentUserAvatar(call: ApplicationCall) {
        val (contentType, bytes) = AvatarFetchUtil.retrieve(call.currentUser!!)!!
        call.respond(createKtorContentWithByteArray(bytes, contentType))
    }

    /**
     * Returns the current logged-in user's avatar with the specified hash (i.e, `123456.png`)
     * @param 200 The user avatar or any defaults if the user hasn't ever set an avatar.
     * @param 404 If the avatar with the given hash wasn't found
     */
    @Get("/@me/avatars/{hash}")
    suspend fun getCurrentUserAvatarFromHash(call: ApplicationCall) {
        val (contentType, bytes) = AvatarFetchUtil.retrieve(call.currentUser!!, call.parameters["hash"])
            ?: return call.respond(HttpStatusCode.NotFound)

        call.respond(createKtorContentWithByteArray(bytes, contentType))
    }

    /**
     * Updates the current logged-in user's avatar. If more parts were used in the request, it will be discarded
     * and the first part that it can poll is the one that is used.
     *
     * @statusCode 202 If the avatar was successfully updated
     * @statusCode 400 If the request was not a `multipart/form-data` request or there were no parts available.
     * @statusCode 406 If the part was not containing a file, it was a form or something else that we don't accept.
     */
    @Post("/@me/avatar")
    suspend fun updateAvatar(call: ApplicationCall) {
        val multipart = call.receiveMultipart()
        val parts = multipart.readAllParts()
        if (parts.isEmpty()) {
            return call.respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "MISSING_FILE_PART",
                    "The request is missing a file part to be used.",
                ),
            )
        }

        // probably inefficient, but what else?
        var correctPart: PartData.FileItem? = null
        val partsAsQueue = parts.toMutableList()
        while (true) {
            val current = partsAsQueue.removeFirstOrNull() ?: break
            if (current is PartData.FileItem) {
                correctPart = current
                break
            }
        }

        if (correctPart == null) {
            return call.respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "UNKNOWN_FILE_PART",
                    "Couldn't find any multi-parts that was a File",
                ),
            )
        }

        AvatarFetchUtil.update(call.currentUser!!.id, correctPart)
        call.respond(HttpStatusCode.Accepted, ApiResponse.ok())
    }

    @Get("/@me/connections")
    suspend fun getUserConnections(call: ApplicationCall) {
        val user = call.currentUser!!
        val connections = asyncTransaction(ChartedScope) {
            UserConnectionEntity.findById(user.id)!!
        }

        call.respond(HttpStatusCode.OK, ApiResponse.ok(UserConnections.fromEntity(connections)))
    }

    @Put("/@me/repositories")
    suspend fun createRepository(call: ApplicationCall) {
        val body: CreateRepositoryBody = call.receive()
        val exists = asyncTransaction(ChartedScope) {
            RepositoryEntity.find { (RepositoryTable.owner eq call.currentUser!!.id) and (RepositoryTable.name eq body.name) }
                .firstOrNull()
        }

        if (exists != null) {
            return call.respond(
                HttpStatusCode.NotAcceptable,
                ApiResponse.err(
                    "REPO_EXISTS",
                    "Repository with name [${body.name}] already exists!",
                ),
            )
        }

        val id = snowflake.generate()
        val repository = asyncTransaction(ChartedScope) {
            RepositoryEntity.new(id.value) {
                this.description = body.description
                this.owner = call.currentUser!!.id
                this.flags = if (body.private) 1 else 0
                this.name = body.name
                this.type = body.type
            }.let { entity -> Repository.fromEntity(entity) }
        }

        call.respond(HttpStatusCode.Created, ApiResponse.ok(repository))
    }

    @Get("/{idOrName}")
    suspend fun fetch(call: ApplicationCall) {
        val user = call.getDynamicUser() ?: return
        call.respond(HttpStatusCode.OK, ApiResponse.ok(user))
    }

    @Get("/{idOrName}/avatars/current.png")
    suspend fun getUserAvatar(call: ApplicationCall) {
        val user = call.getDynamicUser() ?: return
        val (contentType, bytes) = AvatarFetchUtil.retrieve(User.fromEntity(user), null)!!

        call.respond(createKtorContentWithByteArray(bytes, contentType))
    }

    @Get("/{idOrName}/avatars/{hash}")
    suspend fun getUserAvatarHash(call: ApplicationCall) {
        val user = call.getDynamicUser() ?: return
        val (contentType, bytes) = AvatarFetchUtil.retrieve(User.fromEntity(user), call.parameters["hash"])
            ?: return call.respond(HttpStatusCode.NotFound)

        call.respond(createKtorContentWithByteArray(bytes, contentType))
    }

    private suspend fun ApplicationCall.getDynamicUser(): UserEntity? {
        val idOrName = parameters["idOrName"]
            ?: return run {
                respond(
                    HttpStatusCode.NotFound,
                    ApiResponse.err(
                        "MISSING_PARAMETER",
                        "Missing the `idOrName` parameter",
                    ),
                )

                null
            }

        return when {
            idOrName.toLongOrNull() != null -> asyncTransaction(ChartedScope) {
                UserEntity.find { UserTable.id eq idOrName.toLong() }.firstOrNull()
            }

            idOrName.toNameRegex(false).matches() -> asyncTransaction(ChartedScope) {
                UserEntity.find { UserTable.name eq idOrName }.firstOrNull()
            }

            else -> {
                respond(
                    HttpStatusCode.BadRequest,
                    ApiResponse.err(
                        "UNKNOWN_ENTITY",
                        "Unable to determine if [idOrName] provided is by ID or name, provided [$idOrName]",
                    ),
                )

                null
            }
        }
    }
}
