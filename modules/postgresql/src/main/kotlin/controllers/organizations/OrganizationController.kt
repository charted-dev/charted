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

package org.noelware.charted.modules.postgresql.controllers.organizations

import io.ktor.server.application.*
import kotlinx.datetime.Clock
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import org.jetbrains.exposed.sql.Column
import org.jetbrains.exposed.sql.SqlExpressionBuilder.eq
import org.jetbrains.exposed.sql.and
import org.jetbrains.exposed.sql.deleteWhere
import org.jetbrains.exposed.sql.update
import org.noelware.charted.ValidationException
import org.noelware.charted.models.organizations.Organization
import org.noelware.charted.modules.postgresql.asyncTransaction
import org.noelware.charted.modules.postgresql.controllers.AbstractController
import org.noelware.charted.modules.postgresql.controllers.getOrNullByProp
import org.noelware.charted.modules.postgresql.entities.OrganizationEntity
import org.noelware.charted.modules.postgresql.extensions.fromEntity
import org.noelware.charted.modules.postgresql.ktor.UserEntityAttributeKey
import org.noelware.charted.modules.postgresql.tables.OrganizationTable
import org.noelware.charted.snowflake.Snowflake
import kotlin.reflect.KProperty0

class OrganizationController(private val snowflake: Snowflake): AbstractController<Organization, CreateOrganizationPayload, PatchOrganizationPayload>() {
    override suspend fun <V> getOrNullByProp(prop: KProperty0<Column<V>>, value: V): Organization? = asyncTransaction {
        OrganizationEntity.find { prop.get() eq value }.firstOrNull()?.let { entity ->
            Organization.fromEntity(entity)
        }
    }

    override suspend fun getOrNull(id: Long): Organization? = getOrNullByProp(OrganizationTable, OrganizationTable::id to id)
    override suspend fun create(call: ApplicationCall, data: CreateOrganizationPayload): Organization {
        // Since we can't get the extension for ApplicationCall.currentUserEntity (since
        // :server depends on :modules:postgresql, and we can't have circular dependencies),
        //
        // We have a `UserEntityAttributeKey` in modules/postgresql/src/main/kotlin/ktor/AttributeKeys.kt that
        // we can use. This is filled in if a session is available to us.
        val currentUserEntity = call.attributes.getOrNull(UserEntityAttributeKey) ?: throw IllegalStateException("Unable to fetch user")

        // Check if the owner owns an organization by that name
        val hasOrg = asyncTransaction {
            OrganizationEntity.find {
                (OrganizationTable.name eq data.name) and (OrganizationTable.owner eq currentUserEntity.id)
            }.firstOrNull()
        }

        if (hasOrg != null) {
            throw ValidationException("body.name", "Organization by name [${data.name}] is already taken on your account.", "ORG_ALREADY_EXISTS")
        }

        val id = snowflake.generate()
        return asyncTransaction {
            OrganizationEntity.new(id.value) {
                displayName = data.displayName
                private = data.private
                owner = currentUserEntity
                name = data.name
            }.let { entity -> Organization.fromEntity(entity) }
        }
    }

    override suspend fun delete(id: Long) {
        asyncTransaction {
            OrganizationTable.deleteWhere { OrganizationTable.id eq id }
        }
    }

    override suspend fun update(call: ApplicationCall, id: Long, patched: PatchOrganizationPayload) {
        val currentUserEntity = call.attributes.getOrNull(UserEntityAttributeKey) ?: throw IllegalStateException("Unable to fetch user")
        if (patched.name != null) {
            // Check if the owner owns an organization by that name
            val hasOrg = asyncTransaction {
                OrganizationEntity.find {
                    (OrganizationTable.name eq patched.name) and (OrganizationTable.owner eq currentUserEntity.id)
                }.firstOrNull()
            }

            if (hasOrg != null) {
                throw ValidationException("body.name", "Organization by name [${patched.name}] is already taken on your account.", "ORG_ALREADY_EXISTS")
            }
        }

        return asyncTransaction {
            OrganizationTable.update({ OrganizationTable.id eq id }) {
                it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                if (patched.twitterHandle != null) {
                    if (patched.twitterHandle.isBlank()) {
                        it[twitterHandle] = null
                    } else {
                        it[twitterHandle] = patched.twitterHandle
                    }
                }

                if (patched.gravatarEmail != null) {
                    if (patched.gravatarEmail.isBlank()) {
                        it[gravatarEmail] = null
                    } else {
                        it[gravatarEmail] = patched.gravatarEmail
                    }
                }

                if (patched.displayName != null) {
                    if (patched.displayName.isBlank()) {
                        it[displayName] = null
                    } else {
                        it[displayName] = patched.displayName
                    }
                }

                if (patched.private != null) {
                    it[private] = patched.private
                }

                if (patched.name != null) {
                    it[name] = patched.name
                }
            }
        }
    }
}
