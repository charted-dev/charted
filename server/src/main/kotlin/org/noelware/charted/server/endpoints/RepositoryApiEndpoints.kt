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

@file:Suppress("UNUSED")
package org.noelware.charted.server.endpoints

import dev.floofy.utils.exposed.asyncTransaction
import dev.floofy.utils.kotlin.ifNotNull
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import kotlinx.coroutines.runBlocking
import kotlinx.datetime.LocalDateTime
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toInstant
import kotlinx.serialization.Contextual
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.*
import org.jetbrains.exposed.sql.ResultRow
import org.jetbrains.exposed.sql.SqlExpressionBuilder.eq
import org.jetbrains.exposed.sql.and
import org.jetbrains.exposed.sql.insertAndGetId
import org.jetbrains.exposed.sql.select
import org.noelware.charted.core.ChartedScope
import org.noelware.charted.core.config.Config
import org.noelware.charted.core.sessions.SessionKey
import org.noelware.charted.core.sessions.SessionPlugin
import org.noelware.charted.database.enums.RepoType
import org.noelware.charted.database.enums.asString
import org.noelware.charted.database.tables.Organizations
import org.noelware.charted.database.tables.Repositories
import org.noelware.charted.database.tables.Users
import org.noelware.charted.util.Snowflake
import org.noelware.ktor.body
import org.noelware.ktor.endpoints.*

@kotlinx.serialization.Serializable
data class Repository(
    val description: String? = null,
    val deprecated: Boolean = false,

    @SerialName("created_at")
    val createdAt: LocalDateTime,

    @SerialName("updated_at")
    val updatedAt: LocalDateTime,

    @Contextual
    val owner: Any? = null,
    val flags: Long = 0,
    val icon: String? = null,
    val name: String,
    val type: String = "application",
    val id: Long
) {
    companion object {
        suspend fun fromResultRow(row: ResultRow, loadOwner: Boolean = false): Repository {
            // Load in the owner (if we need to)
            val owner: Any? = if (loadOwner) {
                asyncTransaction(ChartedScope) {
                    val userOrNull = Users.select {
                        Users.id eq row[Repositories.ownerId]
                    }.firstOrNull()

                    if (userOrNull != null)
                        return@asyncTransaction User.fromResultRow(userOrNull)

                    Organizations.select {
                        Organizations.id eq row[Repositories.id]
                    }.firstOrNull().ifNotNull { runBlocking { Organization.fromResultRow(it) } }!!
                }
            } else {
                null
            }

            return Repository(
                row[Repositories.description],
                row[Repositories.deprecated],
                row[Repositories.createdAt],
                row[Repositories.updatedAt],
                owner,
                row[Repositories.flags],
                row[Repositories.iconHash],
                row[Repositories.name],
                row[Repositories.type].asString(),
                row[Repositories.id].value
            )
        }
    }

    fun toJsonObject(): JsonObject = buildJsonObject {
        put("description", description)
        put("deprecated", deprecated)
        put("created_at", createdAt.toInstant(TimeZone.currentSystemDefault()).toString())
        put("updated_at", updatedAt.toInstant(TimeZone.currentSystemDefault()).toString())
        put("owner", if (owner != null) (owner as? User)?.toJsonObject() ?: (owner as Organization).toJsonObject() else JsonNull)
        put("flags", flags)
        put("icon", icon)
        put("name", name)
        put("type", type)
        put("id", id)
    }
}

@kotlinx.serialization.Serializable
data class CreateRepoBody(
    val name: String,
    val description: String? = null,
    val type: RepoType
)

class RepositoryApiEndpoints(private val config: Config): AbstractEndpoint("/repositories") {
    init {
        install(HttpMethod.Put, "/repositories", SessionPlugin)
        install(HttpMethod.Patch, "/repositories/{id}", SessionPlugin)
        install(HttpMethod.Delete, "/repositories/{id}", SessionPlugin)
    }

    @Get
    suspend fun main(call: ApplicationCall) {
        call.respond(
            HttpStatusCode.OK,
            buildJsonObject {
                put("success", true)
                putJsonObject("data") {
                    put("message", "Welcome to the Repositories API!")
                    put("docs_uri", "https://charts.noelware.org/docs/api/repositories")
                }
            }
        )
    }

    @Get("/{id}")
    suspend fun get(call: ApplicationCall) {
        val id = call.parameters["id"]!!
        val loadOwner = call.request.queryParameters["owner"]

        if (id == "null") {
            call.respond(
                HttpStatusCode.NotAcceptable,
                buildJsonObject {
                    put("success", false)
                    put(
                        "errors",
                        buildJsonArray {
                            add(
                                buildJsonObject {
                                    put("code", "INVALID_REPO_ID")
                                    put("message", "Cannot use `null` as a parameter")
                                }
                            )
                        }
                    )
                }
            )

            return
        }

        val repository = asyncTransaction(ChartedScope) {
            Repositories.select {
                if (id.toLongOrNull() != null)
                    Repositories.id eq id.toLong()
                else
                    Repositories.name eq id
            }.firstOrNull()
        }

        if (repository == null) {
            call.respond(
                HttpStatusCode.NotFound,
                buildJsonObject {
                    put("success", false)
                    putJsonArray("errors") {
                        addJsonObject {
                            put("code", "UNKNOWN_REPO")
                            put("message", "Cannot find repository with ID or name $id")
                        }
                    }
                }
            )

            return
        }

        val result = Repository.fromResultRow(repository, loadOwner != null)
        call.respond(
            HttpStatusCode.OK,
            buildJsonObject {
                put("success", true)
                put("data", result.toJsonObject())
            }
        )
    }

    @Put
    suspend fun create(call: ApplicationCall) {
        val session = call.attributes[SessionKey]
        val body by call.body<CreateRepoBody>()

        // Check if the repository name exists on the owner's account
        val repoExists = asyncTransaction(ChartedScope) {
            Repositories.select {
                (Repositories.name eq body.name) and (Repositories.type eq body.type) and (Repositories.ownerId eq session.userId)
            }.firstOrNull()
        }

        if (repoExists != null) {
            call.respond(
                HttpStatusCode.NotAcceptable,
                buildJsonObject {
                    put("success", false)
                    putJsonArray("errors") {
                        addJsonObject {
                            put("code", "REPO_EXISTS")
                            put("message", "Repository with name ${body.name} (type: ${body.type.asString()}) exists on your account/org!")
                        }
                    }
                }
            )

            return
        }

        // Create the repository
        val id = Snowflake.generate()
        val repository = asyncTransaction(ChartedScope) {
            val entityID = Repositories.insertAndGetId {
                it[name] = body.name
                it[type] = body.type
                it[ownerId] = session.userId
                it[description] = body.description
                it[Repositories.id] = id
            }

            Repositories
                .select { Repositories.id eq entityID }
                .limit(1)
                .first()
        }

        call.respond(
            HttpStatusCode.Created,
            buildJsonObject {
                put("success", true)
                put("data", Repository.fromResultRow(repository, true).toJsonObject())
            }
        )
    }

    @Patch("/{id}")
    suspend fun update(call: ApplicationCall) {}

    @Delete("/{id}")
    suspend fun delete(call: ApplicationCall) {}
}
