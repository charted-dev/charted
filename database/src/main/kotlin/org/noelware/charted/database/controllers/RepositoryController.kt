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
import kotlinx.serialization.json.*
import org.jetbrains.exposed.sql.SqlExpressionBuilder.eq
import org.jetbrains.exposed.sql.and
import org.noelware.charted.common.ChartedScope
import org.noelware.charted.common.Snowflake
import org.noelware.charted.common.data.helm.RepoType
import org.noelware.charted.common.exceptions.StringOverflowException
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
    }
}

@kotlinx.serialization.Serializable
data class UpdateRepositoryBody(
    val description: String? = null,
    val deprecated: Boolean = false,
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
}
