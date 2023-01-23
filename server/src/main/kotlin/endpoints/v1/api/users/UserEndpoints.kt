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

@file:Suppress("unused")

package org.noelware.charted.server.endpoints.v1.api.users

import dev.floofy.utils.exposed.asyncTransaction
import guru.zoroark.tegral.openapi.dsl.RootDsl
import guru.zoroark.tegral.openapi.dsl.schema
import io.ktor.http.*
import io.ktor.http.content.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import kotlinx.datetime.Clock
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import org.jetbrains.exposed.sql.*
import org.jetbrains.exposed.sql.SqlExpressionBuilder.eq
import org.noelware.charted.ChartedInfo
import org.noelware.charted.ChartedScope
import org.noelware.charted.ValidationException
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.sessions.SessionType
import org.noelware.charted.databases.postgres.entities.RepositoryEntity
import org.noelware.charted.databases.postgres.entities.UserConnectionEntity
import org.noelware.charted.databases.postgres.entities.UserEntity
import org.noelware.charted.databases.postgres.flags.RepositoryFlags
import org.noelware.charted.databases.postgres.models.Repository
import org.noelware.charted.databases.postgres.models.User
import org.noelware.charted.databases.postgres.models.UserConnections
import org.noelware.charted.databases.postgres.tables.OrganizationTable
import org.noelware.charted.databases.postgres.tables.RepositoryTable
import org.noelware.charted.databases.postgres.tables.UserTable
import org.noelware.charted.modules.avatars.AvatarModule
import org.noelware.charted.modules.elasticsearch.ElasticsearchModule
import org.noelware.charted.modules.helm.charts.HelmChartModule
import org.noelware.charted.modules.redis.RedisClient
import org.noelware.charted.modules.sessions.SessionManager
import org.noelware.charted.server.createKtorContentWithByteArray
import org.noelware.charted.server.endpoints.v1.api.CreateRepositoryBody
import org.noelware.charted.server.endpoints.v1.api.NewUserBody
import org.noelware.charted.server.endpoints.v1.api.UpdateUserBody
import org.noelware.charted.server.openapi.extensions.addSessionResponses
import org.noelware.charted.server.plugins.SessionsPlugin
import org.noelware.charted.server.plugins.currentUser
import org.noelware.charted.server.plugins.session
import org.noelware.charted.snowflake.Snowflake
import org.noelware.charted.types.responses.ApiResponse
import org.noelware.ktor.endpoints.*
import org.springframework.security.crypto.argon2.Argon2PasswordEncoder

class UserEndpoints(
    private val config: Config,
    private val redis: RedisClient,
    private val argon2: Argon2PasswordEncoder,
    private val avatars: AvatarModule,
    private val sessions: SessionManager,
    private val snowflake: Snowflake,
    private val helmCharts: HelmChartModule? = null,
    private val elasticsearch: ElasticsearchModule? = null
): AbstractEndpoint("/users") {
    init {
        install(HttpMethod.Delete, "/users/@me/sessions/{sessionId}", SessionsPlugin) {
            assertSessionOnly = true
        }

        install(HttpMethod.Post, "/users/@me/avatars", SessionsPlugin) {
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

    @Get
    suspend fun main(call: ApplicationCall): Unit = call.respond(HttpStatusCode.OK, ApiResponse.ok(MainUserResponse()))

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

        helmCharts?.createIndexYaml(user.id)
        return call.respond(HttpStatusCode.Created, ApiResponse.ok(user))
    }

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

        sessions.revokeAll(id)
        helmCharts?.destroyIndexYaml(id)

        call.respond(HttpStatusCode.Accepted, ApiResponse.ok())
    }

    @Post("/login")
    suspend fun login(call: ApplicationCall) {
        val body: LoginBody = call.receive()
        val op: Op<Boolean> = if (body.username != null) {
            UserTable.username eq body.username
        } else {
            UserTable.email eq body.email!!
        }

        val user = asyncTransaction(ChartedScope) { UserEntity.find(op).firstOrNull() }
            ?: return call.respond(
                HttpStatusCode.NotFound,
                ApiResponse.err(
                    "UNKNOWN_USER",
                    "Unable to find user with ${if (body.username != null) "username [${body.username}]" else "email [${body.email}]"}",
                ),
            )

        val session = sessions.doAuthenticate(user, body.password)
        return call.respond(HttpStatusCode.OK, ApiResponse.ok(session.toJsonObject(true)))
    }

    @Get("/@me")
    suspend fun getCurrentUser(call: ApplicationCall) {
        call.respond(HttpStatusCode.OK, ApiResponse.ok(call.currentUser!!))
    }

    @Get("/@me/connections")
    suspend fun getCurrentUserConnections(call: ApplicationCall) {
        val connections = asyncTransaction(ChartedScope) {
            UserConnectionEntity.findById(call.currentUser!!.id)!!
        }

        call.respond(HttpStatusCode.OK, ApiResponse.ok(UserConnections.fromEntity(connections)))
    }

    @Post("/@me/avatars")
    suspend fun updateCurrentUserAvatar(call: ApplicationCall) {
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

        avatars.updateUserAvatar(call.currentUser!!, correctPart)
        call.respond(HttpStatusCode.Accepted, ApiResponse.ok())
    }

    @Put("/@me/repositories")
    suspend fun createUserRepository(call: ApplicationCall) {
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
        val flags = RepositoryFlags()
        if (body.private) {
            flags.add("PRIVATE")
        }

        val repository = asyncTransaction(ChartedScope) {
            RepositoryEntity.new(id.value) {
                this.description = body.description
                this.owner = call.currentUser!!.id
                this.flags = flags.bits()
                this.name = body.name
                this.type = body.type
            }.let { entity -> Repository.fromEntity(entity) }
        }

        call.respond(HttpStatusCode.Created, ApiResponse.ok(repository))
    }

    @Delete("/@me/sessions/{sessionId}")
    suspend fun deleteSession(call: ApplicationCall) {
        sessions.revoke(call.session!!)
        call.respond(HttpStatusCode.Accepted, ApiResponse.ok())
    }

    @Get("/{idOrName}")
    suspend fun getUserByNameOrId(call: ApplicationCall) {
        val user = call.getUserByIdOrName() ?: return
        call.respond(HttpStatusCode.OK, ApiResponse.ok(user))
    }

    @Get("/{idOrName}/avatars/current.png")
    suspend fun getUserCurrentAvatar(call: ApplicationCall) {
        val user = call.getUserByIdOrName() ?: return
        val (contentType, bytes) = avatars.retrieveUserAvatar(user, null)!!

        call.respond(createKtorContentWithByteArray(bytes, contentType))
    }

    @Get("/{idOrName}/avatars/{hash}")
    suspend fun getUserAvatarByHash(call: ApplicationCall) {
        val user = call.getUserByIdOrName() ?: return
        val (contentType, bytes) = avatars.retrieveUserAvatar(user, call.parameters["hash"])
            ?: return call.respond(HttpStatusCode.NotFound)

        call.respond(createKtorContentWithByteArray(bytes, contentType))
    }

    companion object {
        /**
         * Transforms the [users endpoints][UserEndpoints] with the necessary data that is applicable
         * for the OpenAPI specification. This is used in the [charted][org.noelware.charted.server.openapi.charted] DSL
         * function.
         */
        fun RootDsl.toOpenAPI() {
            "/users" get {
                description = "Generic entrypoint to the Users REST handler"
                externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/users#GET-/users"

                200 response {
                    "application/json" content {
                        schema<ApiResponse.Ok<MainUserResponse>>()
                    }
                }
            }

            "/users" put {
                description = "Registers a new user if the server accepts registrations"
                externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/users#PUT-/users"

                body {
                    required = true
                    "application/json" content {
                        schema<NewUserBody>()
                    }
                }

                201 response {
                    "application/json" content {
                        schema<ApiResponse.Ok<Unit>>()
                    }
                }

                addSessionResponses(listOf(406))

                406 response {
                    description = """If the payload couldn't be accepted due to:
                    • If the username or email given was already taken,
                    • If the header wasn't formed as base64 encoded 'username:password' (in Basic authentication),
                    • Unknown JWT exception had occurred (in Session authentication),
                    • The request header didn't follow the '[Type] [Token]' scheme
                        • `Type` is "Basic", "ApiKey", or "Bearer"
                        • `Token` is the actual token or base64-encoded of 'username:password' if `Type` is Basic
                    """.trimIndent()

                    "application/json" content {
                        schema<ApiResponse.Err>()
                    }
                }
            }

            "/users" patch {
                description = "Updates any user metadata in the database."
                externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/users#PATCH-/users"

                security("sessionToken")
                security("apiKey")

                body {
                    required = true
                    "application/json" content {
                        schema<UpdateUserBody>()
                    }
                }

                201 response {
                    "application/json" content {
                        schema<ApiResponse.Ok<Unit>>()
                    }
                }

                addSessionResponses(listOf(406))

                406 response {
                    description = """If the payload couldn't be accepted due to:
                    • If the username or email given was already taken,
                    • If the header wasn't formed as base64 encoded 'username:password' (in Basic authentication),
                    • Unknown JWT exception had occurred (in Session authentication),
                    • The request header didn't follow the '[Type] [Token]' scheme
                        • `Type` is "Basic", "ApiKey", or "Bearer"
                        • `Token` is the actual token or base64-encoded of 'username:password' if `Type` is Basic
                    """.trimIndent()

                    "application/json" content {
                        schema<ApiResponse.Err>()
                    }
                }
            }

            "/users" delete {
                description = "Delete the current user off the database"
                externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/users#DELETE-/users"

                security("sessionToken")

                202 response {
                    "application/json" content {
                        schema<ApiResponse.Ok<Unit>>()
                    }
                }

                addSessionResponses()
            }

            "/users/@me" get {
                description = "Returns the current user that is logged in."
                externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/users#GET-/users/@me"

                security("sessionToken")
                security("apiKey")

                200 response {
                    "application/json" content {
                        schema<ApiResponse.Ok<User>>()
                    }
                }

                addSessionResponses()
            }

            "/users/@me/connections" get {
                description = "Returns all the current user's connections that they have explicitly connected with."
                externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/users#GET-/users/@me/connections"

                security("sessionToken")
                security("apiKey")

                200 response {
                    "application/json" content {
                        schema<ApiResponse.Ok<UserConnections>>()
                    }
                }

                addSessionResponses()
            }

            "/users/@me/avatars" post {
                description = """Updates the current logged-in user's avatar. If more parts were used in the request, it will be discarded
                and the first part that it can poll is the one that is used.
                """.trimIndent()

                externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/users#GET-/users/@me/avatars"

                security("sessionToken")
                security("apiKey")

                202 response {
                    description = "If the avatar was successfully updated"
                    "application/json" content {
                        schema<ApiResponse.Ok<Unit>>()
                    }
                }

                addSessionResponses(listOf(400, 406))

                400 response {
                    description = """If this request fails due to:
                    • The `Content-Type` header was not "multipart/form-data",
                    • If there were no form data parts available to consume,
                    • If the session token or API key couldn't be validated,
                    • If the Authorization header was malformed.
                    """.trimIndent()
                    "application/json" content {
                        schema<ApiResponse.Err>()
                    }
                }

                406 response {
                    description = """Whether if the request payload was not accepted due to:
                    • If the form data part was not a FileItem,
                    • If the Authorization header wasn't formed as base64 encoded 'username:password' (in Basic authentication),
                    • Unknown JWT exception had occurred (in Session authentication),
                    • The request header didn't follow the '[Type] [Token]' scheme
                        • `Type` is "Basic", "ApiKey", or "Bearer"
                        • `Token` is the actual token or base64-encoded of 'username:password' if `Type` is Basic
                    """.trimIndent()

                    "application/json" content {
                        schema<ApiResponse.Err>()
                    }
                }
            }

            "/users/@me/sessions/{sessionId}" delete {
                description = "Deletes the current session from Redis permanently"
                externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/users#GET-/users/@me/sessions/{sessionId}"

                "sessionId" pathParameter {
                    description = "The session UUID that should be deleted"
                    schema<String>()
                }
            }

            "/users/{idOrName}" get {
                description = "Finds a user by their ID or username"
                externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/users#GET-/users/{idOrName}"

                "idOrName" pathParameter {
                    description = "The user's ID or name to search for"
                    schema<String>()
                }

                200 response {
                    "application/json" content {
                        schema<ApiResponse.Ok<User>>()
                    }
                }

                404 response {
                    "application/json" content {
                        schema<ApiResponse.Err>()
                    }
                }
            }

            "/users/{idOrName}/avatars/current.png" get {
                description = "Returns the current user's avatar by their gravatar email or self-uploaded image, defaults to Identicons if no avatar was found"
                externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/users#GET-/users/@me/avatars/current.png"

                "idOrName" pathParameter {
                    description = "The user's ID or name to search for"
                    schema<String>()
                }

                200 response {
                    "image/png" content {
                        schema<ByteArray>()
                    }

                    "image/jpg" content {
                        schema<ByteArray>()
                    }

                    "image/jpeg" content {
                        schema<ByteArray>()
                    }

                    "image/gif" content {
                        schema<ByteArray>()
                    }
                }

                404 response {
                    "application/json" content {
                        schema<ApiResponse.Err>()
                    }
                }
            }

            "/users/{idOrName}/avatars/{hash}" get {
                description = "Returns the current user's avatar by their avatar hash, gravatar email, or self-uploaded image, defaults to Identicons if no avatar was found"
                externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/users#GET-/users/@me/sessions/{sessionId}"

                "idOrName" pathParameter {
                    description = "The user's ID or name to search for"
                    schema<String>()
                }

                "hash" pathParameter {
                    description = "Avatar hash to look for"
                    schema<String>()
                }

                200 response {
                    "image/png" content {
                        schema<ByteArray>()
                    }

                    "image/jpg" content {
                        schema<ByteArray>()
                    }

                    "image/jpeg" content {
                        schema<ByteArray>()
                    }

                    "image/gif" content {
                        schema<ByteArray>()
                    }
                }

                404 response {
                    "application/json" content {
                        schema<ApiResponse.Err>()
                    }
                }
            }
        }
    }
}
