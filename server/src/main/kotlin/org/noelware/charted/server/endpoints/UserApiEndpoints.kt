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

package org.noelware.charted.server.endpoints

import dev.floofy.utils.exposed.asyncTransaction
import dev.floofy.utils.koin.inject
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import kotlinx.serialization.json.*
import org.apache.commons.validator.routines.EmailValidator
import org.noelware.charted.core.ChartedScope
import org.noelware.charted.core.config.Config
import org.noelware.charted.core.sessions.SessionKey
import org.noelware.charted.core.sessions.SessionPlugin
import org.noelware.charted.database.Users
import org.noelware.charted.database.entity.UserEntity
import org.noelware.ktor.body
import org.noelware.ktor.endpoints.*

@kotlinx.serialization.Serializable
data class NewUser(
    val username: String,
    val password: String,
    val email: String
)

class UserApiEndpoints: AbstractEndpoint("/users") {
    init {
        install("/@me", SessionPlugin)
        install("/@me/refresh_token", SessionPlugin)
    }

    @Get
    suspend fun main(call: ApplicationCall) {
        call.respond(HttpStatusCode.OK, buildJsonObject {
            put("success", true)
            put("data", buildJsonObject {
                put("message", "Welcome to the Users API!")
                put("docs", "https://charts.noelware.org/docs/api/users")
            })
        })
    }

    @Post
    suspend fun createUser(call: ApplicationCall) {
        val body by call.body<NewUser>()
        val config by inject<Config>()

        if (!config.registrations) {
            call.respond(HttpStatusCode.Forbidden, buildJsonObject {
                put("success", false)
                put("errors", buildJsonArray {
                    add(buildJsonObject {
                        put("code", "REGISTRATIONS_OFF")
                        put("message", "This instance is invite only! Please ask an administrator of this instance to give you access.")
                    })
                })
            })

            return
        }

        // Check if the username already exists
        val userByName = asyncTransaction(ChartedScope) {
            UserEntity.find {
                Users.username eq body.username
            }.firstOrNull()
        }

        if (userByName != null) {
            call.respond(HttpStatusCode.Forbidden, buildJsonObject {
                put("success", false)
                put("errors", buildJsonArray {
                    add(buildJsonObject {
                        put("code", "USERNAME_ALREADY_TAKEN")
                        put("message", "Username '${body.username}' already exists.")
                    })
                })
            })

            return
        }

        val validator = EmailValidator.getInstance(true, true)
        if (!validator.isValid(body.email)) {
            call.respond(HttpStatusCode.Forbidden, buildJsonObject {
                put("success", false)
                put("errors", buildJsonArray {
                    add(buildJsonObject {
                        put("code", "INVALID_EMAIL")
                        put("message", "Email ${body.email} was not a valid email.")
                    })
                })
            })

            return
        }

        // Check if a user with the email already exists
        val userByEmail = asyncTransaction(ChartedScope) {
            UserEntity.find {
                Users.email eq body.email
            }.firstOrNull()
        }

        if (userByEmail != null) {
            call.respond(HttpStatusCode.Forbidden, buildJsonObject {
                put("success", false)
                put("errors", buildJsonArray {
                    add(buildJsonObject {
                        put("code", "EMAIL_ALREADY_TAKEN")
                        put("message", "Email '${body.email}' already exists.")
                    })
                })
            })

            return
        }
    }

    /*
	hash, err := util.GeneratePassword(password)
	if err != nil {
		logrus.Errorf("Unable to generate Argon2 password: %s", err)
		return result.Err(500, "INTERNAL_SERVER_ERROR", "Unknown password algorithm error had occurred.")
	}

	// Generate a user ID
	id := internal.GlobalContainer.Snowflake.Generate().String()

	user, err := internal.GlobalContainer.Database.Users.CreateOne(
		db.Users.Username.Set(username),
		db.Users.Password.Set(hash),
		db.Users.Email.Set(email),
		db.Users.ID.Set(id),
	).Exec(context.TODO())

	if err != nil {
		logrus.Errorf("Unable to create user in database: %s", err)
		return result.Err(500, "INTERNAL_SERVER_ERROR", "Unable to create user, try again later.")
	}

	// Create the user connections
	connId := internal.GlobalContainer.Snowflake.Generate().String() //nolint
	_, err = internal.GlobalContainer.Database.UserConnections.CreateOne(
		db.UserConnections.Owner.Link(db.Users.ID.Equals(user.ID)),
		db.UserConnections.ID.Set(connId),
	).Exec(context.TODO())

	if err != nil {
		logrus.Errorf("Unable to create user connections: %s", err)
		return result.Err(500, "INTERNAL_SERVER_ERROR", "Unable to create user connections row, try again later.")
	}

	return result.OkWithStatus(201, dbtypes.FromUserDbModel(user))
}
     */

    @Get("/@me")
    suspend fun me(call: ApplicationCall) {
        val session = call.attributes[SessionKey]
        val user = asyncTransaction(ChartedScope) {
            UserEntity.findById(session.userId)!!
        }

        call.respond(HttpStatusCode.OK, buildJsonObject {
            put("success", true)
            put("data", buildJsonObject {
                put("gravatar_email", user.gravatarEmail)
                put("description", user.description)
                put("avatar_url", user.avatar?.let {
                    JsonPrimitive("https://cdn.noelware.org/charted/avatars/${user.id}/${user.avatar}.png")
                } ?: JsonNull)
                put("created_at", user.createdAt)
                put("updated_at", user.updatedAt)
                put("username", user.username)
                put("avatar", user.avatar)
                put("email", user.email)
                put("flags", user.flags)
                put("name", user.name)
            })
        })
    }

    @Get("/{id}")
    suspend fun get(call: ApplicationCall) {
        val params = call.parameters["id"] ?: throw IllegalStateException("Received `id` as null (should never happen?)")

        val user = asyncTransaction(ChartedScope) {
            UserEntity.findById(params.toLong())
        }

        if (user == null) {
            call.respond(HttpStatusCode.NotFound, buildJsonObject {
                put("success", false)
                put("errors", buildJsonArray {
                    add(buildJsonObject {
                        put("code", "UNKNOWN_USER")
                        put("message", "Unknown user with ID $params.")
                    })
                })
            })

            return
        }

        call.respond(HttpStatusCode.OK, buildJsonObject {
            put("success", true)
            put("data", buildJsonObject {
                put("gravatar_email", user.gravatarEmail)
                put("description", user.description)
                put("avatar_url", user.avatar?.let {
                    JsonPrimitive("https://cdn.noelware.org/charted/avatars/${user.id}/${user.avatar}.png")
                } ?: JsonNull)
                put("created_at", user.createdAt)
                put("updated_at", user.updatedAt)
                put("username", user.username)
                put("avatar", user.avatar)
                put("flags", user.flags)
                put("name", user.name)
            })
        })
    }

    @Patch("/@me")
    suspend fun updateMe(call: ApplicationCall) {}

    @Delete("/@me")
    suspend fun deleteCurrent(call: ApplicationCall) {}

    @Post("/login")
    suspend fun login(call: ApplicationCall) {}

    @Post("/@me/refresh_token")
    suspend fun refreshSessionToken(call: ApplicationCall) {}
}
