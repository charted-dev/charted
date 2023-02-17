/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

@file:Suppress("UNUSED")

package org.noelware.charted.server.endpoints.v1.api.repositories

import dev.floofy.utils.kotlin.ifNotNull
import guru.zoroark.tegral.openapi.dsl.RootDsl
import guru.zoroark.tegral.openapi.dsl.schema
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import org.jetbrains.exposed.sql.*
import org.noelware.charted.databases.postgres.asyncTransaction
import org.noelware.charted.databases.postgres.entities.OrganizationEntity
import org.noelware.charted.databases.postgres.entities.RepositoryEntity
import org.noelware.charted.databases.postgres.entities.UserEntity
import org.noelware.charted.databases.postgres.models.Repository
import org.noelware.charted.databases.postgres.tables.OrganizationTable
import org.noelware.charted.databases.postgres.tables.RepositoryTable
import org.noelware.charted.databases.postgres.tables.UserTable
import org.noelware.charted.extensions.regexp.toNameRegex
import org.noelware.charted.server.openapi.extensions.externalDocsUrl
import org.noelware.charted.server.plugins.isFailure
import org.noelware.charted.types.responses.ApiResponse
import org.noelware.ktor.endpoints.Get

class RepositoriesEndpoints: AbstractRepositoryEndpoint("/repositories") {
    @Get
    suspend fun main(call: ApplicationCall): Unit = call.respond(HttpStatusCode.OK, ApiResponse.ok(MainRepositoryResponse()))

    @Get("/{id}")
    suspend fun getRepository(call: ApplicationCall) {
        val repo = getRepositoryById(call) ?: return
        call.respond(HttpStatusCode.OK, ApiResponse.ok(repo))
    }

    @Get("/{owner}/{name}")
    suspend fun getRepoByOwnerAndName(call: ApplicationCall) {
        val owner = call.parameters["owner"]!!
        val name = call.parameters["name"]!!
        if (!owner.toNameRegex().matches()) {
            return call.respond(
                HttpStatusCode.NotAcceptable,
                ApiResponse.err(
                    "INVALID_USER_PARAMETER",
                    "The `owner` parameter only accepts usernames, organization names, or snowflakes",
                ),
            )
        }

        if (!name.toNameRegex(true, 24).matches()) {
            return call.respond(
                HttpStatusCode.NotAcceptable,
                ApiResponse.err(
                    "INVALID_USER_PARAMETER",
                    "The `name` parameter only accepts repository names or snowflakes",
                ),
            )
        }

        // Check if a user owns this repository
        var isUserRepo = false
        val ownerID = asyncTransaction {
            val user = UserEntity.find {
                when {
                    owner.toLongOrNull() != null -> UserTable.id eq owner.toLong()
                    else -> UserTable.username eq owner
                }
            }.firstOrNull()

            if (user != null) {
                isUserRepo = true
                return@asyncTransaction user.id.value
            }

            OrganizationEntity.find {
                when {
                    owner.toLongOrNull() != null -> OrganizationTable.id eq owner.toLong()
                    else -> OrganizationTable.name eq owner
                }
            }.firstOrNull()?.ifNotNull { id.value }
        } ?: return call.respond(
            HttpStatusCode.NotFound,
            ApiResponse.err(
                "UNKNOWN_ENTITY",
                "User or organization with ID or username [$owner] was not found",
            ),
        )

        val repo = asyncTransaction {
            RepositoryEntity.find {
                (RepositoryTable.owner eq ownerID) and when {
                    name.toLongOrNull() != null -> RepositoryTable.id eq name.toLong()
                    else -> RepositoryTable.name eq name
                }
            }.firstOrNull()
        } ?: return call.respond(
            HttpStatusCode.NotFound,
            ApiResponse.err(
                "UNKNOWN_REPOSITORY",
                "Repository with ID or name [$name] was not found from owner [$owner]",
            ),
        )

        if (isUserRepo && canAccessRepository(call, repo).isFailure()) {
            return call.respond(
                HttpStatusCode.Forbidden,
                ApiResponse.err(
                    "UNABLE_TO_ACCESS_REPOSITORY",
                    "You are unable to access this repository",
                ),
            )
        }

        call.respond(HttpStatusCode.OK, ApiResponse.ok(Repository.fromEntity(repo)))
    }

    companion object {
        /**
         * Transforms the [RepositoriesEndpoints] with the necessary data that is applicable
         * for the OpenAPI specification. This is used in the [charted][org.noelware.charted.server.openapi.charted] DSL
         * function.
         */
        fun RootDsl.toOpenAPI() {
            "/repositories" get {
                description = "Generic entrypoint route for the Repositories API"
                externalDocsUrl("repositories", "GET-/")

                200 response {
                    "application/json" content {
                        schema<ApiResponse.Ok<MainRepositoryResponse>>()
                    }
                }
            }

            "/repositories/{id}" get {
                description = "Returns a repository entity by the snowflake, or a 404 if not found."
                externalDocsUrl("repositories", "GET-/{id}")

                200 response {
                    "application/json" content {
                        schema<ApiResponse.Ok<Repository>>()
                    }
                }

                404 response {
                    "application/json" content {
                        schema<ApiResponse.Err>()
                    }
                }
            }

            "/repositories/{owner}/{name}" get {
                description = "Returns a repository entity by the owner's name or snowflake, and the repository name, or a 404 if not found."
                externalDocsUrl("repositories", "GET-/{owner}/{name}")

                200 response {
                    "application/json" content {
                        schema<ApiResponse.Ok<Repository>>()
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
