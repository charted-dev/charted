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

package org.noelware.charted.database.controllers

import dev.floofy.utils.exposed.asyncTransaction
import io.ktor.http.*
import kotlinx.datetime.Clock
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import kotlinx.serialization.json.*
import org.jetbrains.exposed.sql.*
import org.jetbrains.exposed.sql.SqlExpressionBuilder.eq
import org.noelware.charted.common.ChartedScope
import org.noelware.charted.common.Snowflake
import org.noelware.charted.common.data.helm.RepoType
import org.noelware.charted.common.exceptions.StringOverflowException
import org.noelware.charted.common.exceptions.ValidationException
import org.noelware.charted.database.entities.RepositoryEntity
import org.noelware.charted.database.flags.RepositoryFlags
import org.noelware.charted.database.models.Repository
import org.noelware.charted.database.models.bitfield
import org.noelware.charted.database.tables.RepositoryTable

@kotlinx.serialization.Serializable
data class NewRepositoryBody(
    val description: String? = null,
    val private: Boolean = false,
    val name: String,
    val type: RepoType
) {
    init {
        if (description != null && description.length > 240) {
            throw StringOverflowException("body.description", 240)
        }

        if (name.length > 32) {
            throw StringOverflowException("body.name", 32)
        }

        if (!name.matches("^([A-z]|-|_|\\d{0,9}){0,16}".toRegex())) {
            throw ValidationException("body.name", "Repository name can only contain alphabet characters, digits, underscores, and dashes.")
        }
    }
}

@kotlinx.serialization.Serializable
data class UpdateRepositoryBody(
    val description: String? = null,
    val deprecated: Boolean? = null,
    val private: Boolean? = null,
    val name: String? = null,
    val type: RepoType? = null
) {
    init {
        if (description != null && description.length > 240) {
            throw StringOverflowException("body.description", 240)
        }

        if (name != null && name.length > 32) {
            throw StringOverflowException("body.name", 32)
        }
    }
}

object RepositoryController {
    suspend fun getAll(owner: Long, showPrivate: Boolean = false): List<Repository> = asyncTransaction(ChartedScope) {
        RepositoryEntity
            .find { RepositoryTable.owner eq owner }
            .toList()
            .filter { if (showPrivate) true else !(RepositoryFlags(it.flags).has("PRIVATE")) }
            .map { entity -> Repository.fromEntity(entity) }
    }

    suspend fun get(id: Long): Repository? = asyncTransaction(ChartedScope) {
        RepositoryEntity
            .find { RepositoryTable.id eq id }
            .firstOrNull()?.let { entity ->
                val repo = Repository.fromEntity(entity)
                if (repo.bitfield.has("PRIVATE")) return@asyncTransaction null

                repo
            }
    }

    suspend fun create(ownerID: Long, body: NewRepositoryBody): Pair<HttpStatusCode, JsonObject> {
        val repoExists = asyncTransaction(ChartedScope) {
            RepositoryEntity.find {
                (RepositoryTable.owner eq ownerID) and (RepositoryTable.name eq body.name)
            }.firstOrNull()
        }

        if (repoExists != null) {
            return HttpStatusCode.NotAcceptable to buildJsonObject {
                put("success", false)
                putJsonArray("errors") {
                    addJsonObject {
                        put("code", "REPOSITORY_EXISTS")
                        put("message", "Repository with name ${body.name} already exists.")
                    }
                }
            }
        }

        val id = Snowflake.generate()
        val repository = asyncTransaction(ChartedScope) {
            RepositoryEntity.new(id) {
                this.description = body.description
                this.owner = ownerID
                this.flags = if (body.private) RepositoryFlags(0).add("PRIVATE").bits else 0
                this.name = body.name
                this.type = body.type
            }.let { entity -> Repository.fromEntity(entity) }
        }

        return HttpStatusCode.Created to buildJsonObject {
            put("success", true)
            put("data", repository.toJsonObject())
        }
    }

    suspend fun update(id: Long, body: UpdateRepositoryBody) {
        val whereClause: SqlExpressionBuilder.() -> Op<Boolean> = { RepositoryTable.id eq id }
        if (body.deprecated != null) {
            asyncTransaction(ChartedScope) {
                RepositoryTable.update(whereClause) {
                    it[deprecated] = body.deprecated
                    it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                }
            }
        }

        if (body.description != null) {
            asyncTransaction(ChartedScope) {
                RepositoryTable.update(whereClause) {
                    it[description] = body.description.ifEmpty { null }
                    it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                }
            }
        }

        if (body.private != null) {
            val origin = asyncTransaction(ChartedScope) {
                RepositoryEntity.find(whereClause).first().let { Repository.fromEntity(it) }
            }

            asyncTransaction(ChartedScope) {
                RepositoryTable.update(whereClause) {
                    it[flags] = if (body.private) origin.bitfield.add((1L shl 0)).bits else origin.bitfield.remove((1L shl 0)).bits
                    it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                }
            }
        }

        if (body.name != null) {
            asyncTransaction(ChartedScope) {
                RepositoryTable.update(whereClause) {
                    it[name] = body.name
                    it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                }
            }
        }

        if (body.type != null) {
            asyncTransaction(ChartedScope) {
                RepositoryTable.update(whereClause) {
                    it[type] = body.type
                    it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                }
            }
        }
    }

    suspend fun delete(id: Long): Boolean = asyncTransaction(ChartedScope) {
        RepositoryTable.deleteWhere { RepositoryTable.id eq id }
        true
    }
}
