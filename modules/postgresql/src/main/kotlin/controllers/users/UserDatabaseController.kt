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

package org.noelware.charted.modules.postgresql.controllers.users

import io.ktor.server.application.*
import org.noelware.charted.ValidationException
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.models.users.User
import org.noelware.charted.modules.postgresql.asyncTransaction
import org.noelware.charted.modules.postgresql.entities.UserEntity
import org.noelware.charted.modules.postgresql.extensions.fromEntity
import org.noelware.charted.modules.postgresql.tables.UserTable
import org.noelware.charted.snowflake.Snowflake
import org.springframework.security.crypto.argon2.Argon2PasswordEncoder
import kotlinx.datetime.*
import kotlinx.datetime.TimeZone
import org.jetbrains.exposed.sql.*
import org.noelware.charted.configuration.kotlin.dsl.sessions.SessionType
import org.noelware.charted.modules.postgresql.controllers.AbstractDatabaseController

class UserDatabaseController(
    private val argon2: Argon2PasswordEncoder,
    private val config: Config,
    private val snowflake: Snowflake
): AbstractDatabaseController<User, UserEntity, CreateUserPayload, PatchUserPayload>(
    UserTable,
    UserEntity,
    { entity -> User.fromEntity(entity) },
) {
    override suspend fun create(call: ApplicationCall, data: CreateUserPayload): User {
        val userByUsername = getOrNull(UserTable::username to data.username)
        if (userByUsername != null) throw ValidationException("body.username", "Username [${data.username}] already exists")

        val userByEmail = getOrNull(UserTable::email to data.email)
        if (userByEmail != null) throw ValidationException("body.email", "Email [${data.email}] is already taken!")

        val id = snowflake.generate()
        return asyncTransaction {
            UserEntity.new(id.value) {
                if (data.password != null) {
                    password = argon2.encode(data.password)
                }

                createdAt = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                updatedAt = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                username = data.username
                email = data.email
            }.let { entity -> User.fromEntity(entity) }
        }
    }

    override suspend fun update(call: ApplicationCall, id: Long, patched: PatchUserPayload) {
        val sqlSelector: SqlExpressionBuilder.() -> Op<Boolean> = { UserTable.id eq id }
        if (patched.username != null) {
            val userWithUsername = getOrNull(UserTable::username to patched.username)
            if (userWithUsername != null) {
                throw ValidationException("body.username", "Username [${patched.username}] is already taken", "USERNAME_ALREADY_TAKEN")
            }
        }

        if (patched.email != null) {
            val userWithEmail = getOrNull(UserTable::email to patched.email)
            if (userWithEmail != null) {
                throw ValidationException("body.email", "Email [${patched.email}] is already taken", "EMAIL_ALREADY_TAKEN")
            }
        }

        return asyncTransaction {
            UserTable.update(sqlSelector) {
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
    }
}
