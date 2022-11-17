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

package org.noelware.charted.server.endpoints.v1.api

import dev.floofy.utils.exposed.asyncTransaction
import dev.floofy.utils.koin.inject
import dev.floofy.utils.slf4j.logging
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
import org.noelware.charted.configuration.kotlin.dsl.ServerFeature
import org.noelware.charted.databases.postgres.entities.RepositoryEntity
import org.noelware.charted.databases.postgres.entities.UserConnectionEntity
import org.noelware.charted.databases.postgres.entities.UserEntity
import org.noelware.charted.databases.postgres.models.Repository
import org.noelware.charted.databases.postgres.models.User
import org.noelware.charted.databases.postgres.tables.OrganizationTable
import org.noelware.charted.databases.postgres.tables.RepositoryTable
import org.noelware.charted.databases.postgres.tables.UserTable
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

        if (!(username matches "^([A-z]|-|_|\\d{0,9}){0,32}".toRegex())) {
            throw ValidationException("body.username", "Username can only contain letters, digits, dashes, or underscores.")
        }

        if (!(password matches("^.*(?=.{8,})(?=.*[a-zA-Z])(?=.*\\d)?(?=.*[!#\$%&? \"])?.*\$".toRegex()))) {
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

        if (!(name matches "^([A-z]|-|_|\\d{0,9}){0,24}".toRegex())) {
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
    private val charts: HelmChartModule
): AbstractEndpoint("/users") {
    private val log by logging<UsersEndpoint>()

    init {
        install(HttpMethod.Post, "/users/@me/avatar", SessionsPlugin) {
            this += "user:avatar:update"
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

        install(HttpMethod.Put, "/users/repositories", SessionsPlugin) {
            this += "repo:create"
        }
    }

    @Get
    suspend fun main(call: ApplicationCall) = call.respond(HttpStatusCode.OK, ApiResponse.ok(MainUserResponse()))

    @Put
    suspend fun create(call: ApplicationCall) {
        if (!config.registrations) {
            return call.respond(HttpStatusCode.Forbidden, ApiResponse.err("REGISTRATIONS_DISABLED", "This instance has registrations disabled."))
        }

        val body: NewUserBody by call.body()

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

        val user = asyncTransaction(ChartedScope) {
            UserEntity.new(Snowflake.generate()) {
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

        if (!config.features.contains(ServerFeature.DOCKER_REGISTRY)) {
            log.info("New registered user [@${user.username}], creating index.yaml entry!")
            charts.createIndexYaml("user", user.id)
        }

        return call.respond(HttpStatusCode.Created, ApiResponse.ok(user))
    }

    @Patch
    suspend fun patch(call: ApplicationCall) {
        val patched: UpdateUserBody by call.body()
        val whereClause: SqlExpressionBuilder.() -> Op<Boolean> = { UserTable.id eq call.currentUser!!.id }

        // Do some post checkup before actually patching data
        if (patched.username != null) {
            val userWithUsername = asyncTransaction(ChartedScope) {
                UserEntity.find { UserTable.username eq patched.username!! }.firstOrNull()
            }

            if (userWithUsername != null) {
                throw ValidationException("body.username", "Username [${patched.username}] is already taken")
            }
        }

        if (patched.email != null) {
            val userWithEmail = asyncTransaction(ChartedScope) {
                UserEntity.find { UserTable.email eq patched.email!! }.firstOrNull()
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
                    it[username] = patched.username!!
                }

                if (patched.password != null) {
                    it[password] = argon2.encode(patched.password!!)
                }

                if (patched.email != null) {
                    it[email] = patched.email!!
                }

                if (patched.name != null) {
                    it[name] = patched.name
                }
            }
        }

        return call.respond(HttpStatusCode.Accepted, ApiResponse.ok())
    }

    @Delete
    suspend fun del(call: ApplicationCall) {
        val id = call.currentUser!!.id

        // Delete all the repositories and organizations owned by this user
        asyncTransaction(ChartedScope) {
            RepositoryTable.deleteWhere { RepositoryTable.owner eq id }
        }

        asyncTransaction(ChartedScope) {
            OrganizationTable.deleteWhere { OrganizationTable.owner eq id }
        }

        // Delete the user and their sessions
        asyncTransaction(ChartedScope) {
            UserTable.deleteWhere { UserTable.id eq id }
        }

        sessions.revokeAll(id)
        if (!config.features.contains(ServerFeature.DOCKER_REGISTRY)) {
            storage.trailer.delete("./users/$id/index.yaml")
        }

        call.respond(HttpStatusCode.Accepted, ApiResponse.ok())
    }

    @Get("/{idOrName}")
    suspend fun fetch(call: ApplicationCall) {
        val idOrName = call.parameters["idOrName"] ?: return call.respond(HttpStatusCode.NotFound, ApiResponse.err("MISSING_PARAMETER", "Missing [idOrName] path parameter"))
        val user = when {
            idOrName.toLongOrNull() != null -> asyncTransaction(ChartedScope) {
                UserEntity.find { UserTable.id eq idOrName.toLong() }.firstOrNull()?.let { entity ->
                    User.fromEntity(entity)
                }
            }

            idOrName matches "^([A-z]|-|_|\\d{0,9}){0,32}".toRegex() -> asyncTransaction(ChartedScope) {
                UserEntity.find { UserTable.username eq idOrName }.firstOrNull()?.let { entity ->
                    User.fromEntity(entity)
                }
            }

            else -> return call.respond(HttpStatusCode.BadRequest, ApiResponse.err("UNKNOWN_ENTITY", "Unable to determine if [idOrName] provided is by ID or name, provided [$idOrName]"))
        } ?: return call.respond(HttpStatusCode.NotFound, ApiResponse.err("UNKNOWN_USER", "User with ID or name [$idOrName] was not found"))

        call.respond(HttpStatusCode.OK, ApiResponse.ok(user))
    }

    @Get("/{idOrName}/avatars/current")
    suspend fun getUserAvatar(call: ApplicationCall) {
        val idOrName = call.parameters["idOrName"] ?: return call.respond(HttpStatusCode.NotFound, ApiResponse.err("MISSING_PARAMETER", "Missing [idOrName] path parameter"))
        val user = when {
            idOrName.toLongOrNull() != null -> asyncTransaction(ChartedScope) {
                UserEntity.find { UserTable.id eq idOrName.toLong() }.firstOrNull()?.let { entity ->
                    User.fromEntity(entity)
                }
            }

            idOrName matches "^([A-z]|-|_|\\d{0,9}){0,32}".toRegex() -> asyncTransaction(ChartedScope) {
                UserEntity.find { UserTable.username eq idOrName }.firstOrNull()?.let { entity ->
                    User.fromEntity(entity)
                }
            }

            else -> return call.respond(HttpStatusCode.BadRequest, ApiResponse.err("UNKNOWN_ENTITY", "Unable to determine if [idOrName] provided is by ID or name, provided [$idOrName]"))
        } ?: return call.respond(HttpStatusCode.NotFound, ApiResponse.err("UNKNOWN_USER", "User with ID or name [$idOrName] was not found"))

        val (contentType, bytes) = AvatarFetchUtil.retrieve(user)!!
        call.respond(createKtorContentWithByteArray(bytes, contentType))
    }

    @Get("/{idOrName}/avatars/{hash}")
    suspend fun getUserAvatarByHash(call: ApplicationCall) {
        val idOrName = call.parameters["idOrName"] ?: return call.respond(HttpStatusCode.NotFound, ApiResponse.err("MISSING_PARAMETER", "Missing [idOrName] path parameter"))
        val user = when {
            idOrName.toLongOrNull() != null -> asyncTransaction(ChartedScope) {
                UserEntity.find { UserTable.id eq idOrName.toLong() }.firstOrNull()?.let { entity ->
                    User.fromEntity(entity)
                }
            }

            idOrName matches "^([A-z]|-|_|\\d{0,9}){0,32}".toRegex() -> asyncTransaction(ChartedScope) {
                UserEntity.find { UserTable.username eq idOrName }.firstOrNull()?.let { entity ->
                    User.fromEntity(entity)
                }
            }

            else -> return call.respond(HttpStatusCode.BadRequest, ApiResponse.err("UNKNOWN_ENTITY", "Unable to determine if [idOrName] provided is by ID or name, provided [$idOrName]"))
        } ?: return call.respond(HttpStatusCode.NotFound, ApiResponse.err("UNKNOWN_USER", "User with ID or name [$idOrName] was not found"))

        val avatar = AvatarFetchUtil.retrieve(user, call.parameters["hash"])
            ?: return call.respond(HttpStatusCode.NotFound)

        call.respond(createKtorContentWithByteArray(avatar.second, avatar.first))
    }

    @Post("/login")
    suspend fun login(call: ApplicationCall) {
        val body: LoginBody by call.body()
        val op: Op<Boolean> = if (body.username != null) {
            UserTable.username eq body.username!!
        } else {
            UserTable.email eq body.email!!
        }

        val user = asyncTransaction(ChartedScope) { UserEntity.find(op).firstOrNull() }
            ?: return call.respond(HttpStatusCode.NotFound, ApiResponse.err("UNKNOWN_USER", "Unable to find user with ${if (body.username != null) "username [${body.username}]" else "email [${body.email}]"}"))

        val session = sessions.doAuthenticate(user, body.password)
        return call.respond(HttpStatusCode.OK, ApiResponse.ok(session.toJsonObject(true)))
    }

    @Get("/@me")
    suspend fun fetchSession(call: ApplicationCall) {
        call.respond(HttpStatusCode.OK, ApiResponse.ok(call.currentUser!!))
    }

    @Post("/@me/avatar")
    suspend fun updateAvatar(call: ApplicationCall) {
        val multipart = call.receiveMultipart()
        val part = multipart.readPart() ?: return call.respond(
            HttpStatusCode.BadRequest,
            ApiResponse.err("EXCESSIVE_MULTIPART_AMOUNT", "There can be only one multipart in this request.")
        )

        if (part !is PartData.FileItem) {
            return call.respond(
                HttpStatusCode.NotAcceptable,
                ApiResponse.err("NOT_FILE_PART", "The multipart object must be a File object.")
            )
        }

        AvatarFetchUtil.update(call.currentUser!!.id, part)
        call.respond(HttpStatusCode.Accepted, ApiResponse.ok())
    }

    @Put("/repositories")
    suspend fun createRepository(call: ApplicationCall) {
        val body: CreateRepositoryBody by call.body()
        val exists = asyncTransaction(ChartedScope) {
            RepositoryEntity.find { (RepositoryTable.owner eq call.currentUser!!.id) and (RepositoryTable.name eq body.name) }
                .firstOrNull()
        }

        if (exists != null) {
            return call.respond(
                HttpStatusCode.NotAcceptable,
                ApiResponse.err(
                    "REPO_EXISTS",
                    "Repository with name [${body.name}] already exists!"
                )
            )
        }

        val repository = asyncTransaction(ChartedScope) {
            RepositoryEntity.new(Snowflake.generate()) {
                this.description = body.description
                this.owner = call.currentUser!!.id
                this.flags = if (body.private) 1 else 0
                this.name = body.name
                this.type = body.type
            }.let { entity -> Repository.fromEntity(entity) }
        }

        call.respond(HttpStatusCode.Created, ApiResponse.ok(repository))
    }
}
