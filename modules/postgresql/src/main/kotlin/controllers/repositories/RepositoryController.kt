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

package org.noelware.charted.modules.postgresql.controllers.repositories

import io.ktor.server.application.*
import kotlinx.datetime.*
import org.jetbrains.exposed.sql.*
import org.jetbrains.exposed.sql.SqlExpressionBuilder.eq
import org.noelware.charted.ValidationException
import org.noelware.charted.models.repositories.Repository
import org.noelware.charted.modules.postgresql.asyncTransaction
import org.noelware.charted.modules.postgresql.controllers.AbstractController
import org.noelware.charted.modules.postgresql.controllers.getOrNullByProp
import org.noelware.charted.modules.postgresql.entities.RepositoryEntity
import org.noelware.charted.modules.postgresql.extensions.fromEntity
import org.noelware.charted.modules.postgresql.ktor.ownerId
import org.noelware.charted.modules.postgresql.tables.RepositoryTable
import org.noelware.charted.snowflake.Snowflake
import kotlin.reflect.KProperty0

class RepositoryController(private val snowflake: Snowflake): AbstractController<Repository, CreateRepositoryPayload, PatchRepositoryPayload>() {
    override suspend fun <V> getOrNullByProp(prop: KProperty0<Column<V>>, value: V): Repository? = asyncTransaction {
        RepositoryEntity.find { prop.get() eq value }.firstOrNull()?.let { entity ->
            Repository.fromEntity(entity)
        }
    }

    override suspend fun getOrNull(id: Long): Repository? = getOrNullByProp(RepositoryTable, RepositoryTable::id to id)
    override suspend fun create(call: ApplicationCall, data: CreateRepositoryPayload): Repository {
        val ownerId = call.ownerId ?: error("BUG: Missing owner id to create a repository!")
        val id = snowflake.generate()

        return asyncTransaction {
            RepositoryEntity.new(id.value) {
                description = data.description
                private = data.private
                owner = ownerId
                name = data.name
                type = data.type
            }.let { entity -> Repository.fromEntity(entity) }
        }
    }

    override suspend fun delete(id: Long) {
        asyncTransaction {
            RepositoryTable.deleteWhere { RepositoryTable.id eq id }
        }
    }

    override suspend fun update(call: ApplicationCall, id: Long, patched: PatchRepositoryPayload) {
        val ownerId = call.ownerId ?: error("BUG: Missing owner id when updating a repository!")
        val sqlSelector: SqlExpressionBuilder.() -> Op<Boolean> = { RepositoryTable.id eq id }

        if (patched.name != null) {
            val repoWithName = asyncTransaction {
                RepositoryEntity.find {
                    (RepositoryTable.name eq patched.name) and (RepositoryTable.owner eq ownerId)
                }.firstOrNull()
            }

            if (repoWithName != null) {
                throw ValidationException("body.name", "Repository with name [${patched.name}] already exists")
            }
        }

        return asyncTransaction {
            RepositoryTable.update(sqlSelector) {
                it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())

                if (patched.description != null) {
                    it[description] = patched.description
                }

                if (patched.deprecated != null) {
                    it[deprecated] = patched.deprecated
                }

                if (patched.private != null) {
                    it[private] = patched.private
                }

                if (patched.name != null) {
                    it[name] = patched.name
                }

                if (patched.type != null) {
                    it[type] = patched.type
                }
            }
        }
    }
}
