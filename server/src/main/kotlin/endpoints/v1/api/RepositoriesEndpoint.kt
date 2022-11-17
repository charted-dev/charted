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

package org.noelware.charted.server.endpoints.v1.api

import dev.floofy.utils.exposed.asyncTransaction
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.jetbrains.exposed.sql.Op
import org.jetbrains.exposed.sql.SqlExpressionBuilder.eq
import org.jetbrains.exposed.sql.and
import org.noelware.charted.ChartedInfo
import org.noelware.charted.ChartedScope
import org.noelware.charted.databases.postgres.entities.RepositoryEntity
import org.noelware.charted.databases.postgres.entities.UserEntity
import org.noelware.charted.databases.postgres.models.Repository
import org.noelware.charted.databases.postgres.tables.RepositoryTable
import org.noelware.charted.databases.postgres.tables.UserTable
import org.noelware.charted.types.responses.ApiResponse
import org.noelware.ktor.endpoints.AbstractEndpoint
import org.noelware.ktor.endpoints.Get

@Serializable
data class MainRepositoryResponse(
    val message: String = "Welcome to the Repositories API!",

    @SerialName("docs_url")
    val docsUrl: String = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/repositories"
)

class RepositoriesEndpoint: AbstractEndpoint("/repositories") {
    @Get
    suspend fun main(call: ApplicationCall) = call.respond(HttpStatusCode.OK, ApiResponse.ok(MainRepositoryResponse()))

    @Get("/{id}")
    suspend fun get(call: ApplicationCall) {
        val id = call.parameters["id"]?.toLongOrNull()
            ?: return call.respond(
                HttpStatusCode.NotFound,
                ApiResponse.err(
                    "INVALID_ID",
                    "You will need to provide a valid repository ID"
                )
            )

        val repository = asyncTransaction(ChartedScope) {
            RepositoryEntity.find { RepositoryTable.id eq id }.firstOrNull()?.let { entity ->
                Repository.fromEntity(entity)
            }
        } ?: return call.respond(
            HttpStatusCode.NotFound,
            ApiResponse.err("UNKNOWN_REPOSITORY", "Unable to find repository by ID [$id]")
        )

        call.respond(HttpStatusCode.OK, ApiResponse.ok(repository))
    }

    @Get("/{name}/{repo}")
    suspend fun getByOwnerAndRepo(call: ApplicationCall) {
        val (name, repoId) = call.parameters["name"]!! to call.parameters["repo"]!!

        val ownerID = asyncTransaction(ChartedScope) {
            UserEntity.find {
                if (name matches "([A-z]|-|_){0,32}".toRegex()) {
                    UserTable.username eq name
                } else {
                    UserTable.id eq name.toLong()
                }
            }.firstOrNull()?.id?.value
        } ?: return call.respond(
            HttpStatusCode.NotFound,
            ApiResponse.err(
                "UNKNOWN_REPO_OWNER",
                "Repository owner with ID or name [$name] doesn't exist"
            )
        )

        val queryRepo: Op<Boolean> = if (repoId matches "([A-z]|-|_){0,32}".toRegex()) {
            RepositoryTable.name eq repoId
        } else {
            RepositoryTable.id eq repoId.toLong()
        }

        val repository = asyncTransaction(ChartedScope) {
            RepositoryEntity.find { queryRepo and (RepositoryTable.owner eq ownerID) }.firstOrNull()?.let { entity ->
                Repository.fromEntity(entity)
            }
        } ?: return call.respond(
            HttpStatusCode.NotFound,
            ApiResponse.err("UNKNOWN_REPOSITORY", "Unable to find repository by ID or name [$repoId]")
        )

        call.respond(HttpStatusCode.OK, ApiResponse.ok(repository))
    }
}
