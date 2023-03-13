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

package org.noelware.charted.modules.postgresql.controllers.users.connections

import io.ktor.server.application.*
import kotlinx.datetime.Clock
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import org.jetbrains.exposed.sql.Column
import org.jetbrains.exposed.sql.SqlExpressionBuilder.eq
import org.jetbrains.exposed.sql.deleteWhere
import org.jetbrains.exposed.sql.update
import org.noelware.charted.ValidationException
import org.noelware.charted.models.users.UserConnections
import org.noelware.charted.modules.postgresql.asyncTransaction
import org.noelware.charted.modules.postgresql.controllers.AbstractController
import org.noelware.charted.modules.postgresql.controllers.getOrNullByProp
import org.noelware.charted.modules.postgresql.entities.UserConnectionsEntity
import org.noelware.charted.modules.postgresql.extensions.fromEntity
import org.noelware.charted.modules.postgresql.tables.UserConnectionsTable
import kotlin.reflect.KProperty0

class UserConnectionsController: AbstractController<UserConnections, Long, PatchUserConnectionsPayload>() {
    override suspend fun <V> getOrNullByProp(prop: KProperty0<Column<V>>, value: V): UserConnections? = asyncTransaction {
        UserConnectionsEntity.find { prop.get() eq value }.firstOrNull()?.let { entity ->
            UserConnections.fromEntity(entity)
        }
    }

    override suspend fun getOrNull(id: Long): UserConnections? = getOrNullByProp(UserConnectionsTable, UserConnectionsTable::id to id)
    override suspend fun delete(id: Long) {
        asyncTransaction {
            UserConnectionsTable.deleteWhere { UserConnectionsTable.id eq id }
        }
    }

    @Suppress("PARAMETER_NAME_CHANGED_ON_OVERRIDE", "MoveLambdaOutsideParentheses") // it doesn't make sense to be `data` since it's only the user ID
    override suspend fun create(call: ApplicationCall, user: Long): UserConnections = asyncTransaction {
        // looks weird when it's new(user) {}, so it's new(user, {})
        UserConnectionsEntity.new(user, {}).let { entity -> UserConnections.fromEntity(entity) }
    }

    override suspend fun update(call: ApplicationCall, id: Long, patched: PatchUserConnectionsPayload) {
        // Check if a Noelware account already exists with the patched ID
        if (patched.noelwareAccountID != null) {
            val existingNLWAccount = getOrNullByProp(UserConnectionsTable::noelwareAccountID to patched.noelwareAccountID)
            if (existingNLWAccount != null) {
                throw ValidationException("body.noelware_account_id", "Account with ID [${patched.noelwareAccountID}] already exists")
            }
        }

        if (!patched.githubAccountID.isNullOrBlank()) {
            val existingGHAccount = getOrNullByProp(UserConnectionsTable::githubAccountID to patched.githubAccountID)
            if (existingGHAccount != null) {
                throw ValidationException("body.noelware_account_id", "GitHub account with ID [${patched.noelwareAccountID}] already exists")
            }
        }

        if (!patched.googleAccountID.isNullOrBlank()) {
            val existingGoogleAccount = getOrNullByProp(UserConnectionsTable::googleAccountID to patched.googleAccountID)
            if (existingGoogleAccount != null) {
                throw ValidationException("body.noelware_account_id", "Google Account with ID [${patched.noelwareAccountID}] already exists")
            }
        }

        if (!patched.appleAccountID.isNullOrBlank()) {
            val existingAPLAccount = getOrNullByProp(UserConnectionsTable::appleAccountID to patched.appleAccountID)
            if (existingAPLAccount != null) {
                throw ValidationException("body.noelware_account_id", "Apple ID with ID [${patched.appleAccountID}] already exists")
            }
        }

        return asyncTransaction {
            UserConnectionsTable.update({ UserConnectionsTable.id eq id }) {
                it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())

                if (patched.noelwareAccountID != null) {
                    it[noelwareAccountID] = patched.noelwareAccountID
                }

                if (patched.githubAccountID != null) {
                    if (patched.githubAccountID.isBlank()) {
                        it[githubAccountID] = null
                    } else {
                        it[githubAccountID] = patched.githubAccountID
                    }
                }

                if (patched.googleAccountID != null) {
                    if (patched.googleAccountID.isBlank()) {
                        it[googleAccountID] = null
                    } else {
                        it[googleAccountID] = patched.googleAccountID
                    }
                }

                if (patched.appleAccountID != null) {
                    if (patched.appleAccountID.isBlank()) {
                        it[appleAccountID] = null
                    } else {
                        it[appleAccountID] = patched.appleAccountID
                    }
                }
            }
        }
    }
}
