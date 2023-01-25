/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware <team@noelware.org>
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

import dev.floofy.utils.exposed.asyncTransaction
import guru.zoroark.tegral.openapi.dsl.RootDsl
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import kotlinx.datetime.Clock
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import org.jetbrains.exposed.sql.*
import org.jetbrains.exposed.sql.SqlExpressionBuilder.eq
import org.noelware.charted.ChartedScope
import org.noelware.charted.ValidationException
import org.noelware.charted.databases.postgres.entities.RepositoryEntity
import org.noelware.charted.databases.postgres.flags.RepositoryFlags
import org.noelware.charted.databases.postgres.tables.RepositoryTable
import org.noelware.charted.server.endpoints.v1.api.UpdateRepositoryBody
import org.noelware.charted.server.plugins.PreconditionResult
import org.noelware.charted.server.plugins.SessionsPlugin
import org.noelware.charted.server.plugins.currentUser
import org.noelware.charted.types.responses.ApiError
import org.noelware.charted.types.responses.ApiResponse
import org.noelware.ktor.endpoints.AbstractEndpoint
import org.noelware.ktor.endpoints.Delete
import org.noelware.ktor.endpoints.Get
import org.noelware.ktor.endpoints.Patch

class RepositoriesEndpoints: AbstractEndpoint("/repositories") {
    init {
        install(HttpMethod.Delete, "/repositories/{idOrName}", SessionsPlugin) {
            this += "repo:delete"
            condition { call ->
                val repository = call.getRepositoryEntityByIdOrName() ?: return@condition PreconditionResult.Failed(ApiError.EMPTY, HttpStatusCode.BadRequest)
                call.repoHasPermission(repository, "repo:delete")
            }
        }

        install(HttpMethod.Patch, "/repositories/{idOrName}", SessionsPlugin) {
            this += "repo:update"
            condition { call ->
                val repository = call.getRepositoryEntityByIdOrName() ?: return@condition PreconditionResult.Failed(ApiError.EMPTY, HttpStatusCode.BadRequest)
                call.repoHasPermission(repository, "metadata:update")
            }
        }

        install(HttpMethod.Get, "/repositories/{idOrName}", SessionsPlugin) {
            allowNonAuthorizedRequests = true
            this += "repo:view"

            condition { call ->
                val repository = call.getRepositoryEntityByIdOrName() ?: return@condition PreconditionResult.Failed(ApiError.EMPTY, HttpStatusCode.BadRequest)
                call.canAccessRepository(repository)
            }
        }
    }

    @Get
    suspend fun main(call: ApplicationCall): Unit = call.respond(HttpStatusCode.OK, MainRepositoryResponse())

    @Get("/{idOrName}")
    suspend fun getRepository(call: ApplicationCall) {
        val repo = call.getRepositoryByIdOrName() ?: return
        call.respond(HttpStatusCode.OK, ApiResponse.ok(repo))
    }

    @Patch("/{idOrName}")
    suspend fun patchRepository(call: ApplicationCall) {
        val repo = call.getRepositoryByIdOrName() ?: return
        val patched: UpdateRepositoryBody = call.receive()
        val whereClause: SqlExpressionBuilder.() -> Op<Boolean> = { RepositoryTable.id eq repo.id }

        // Do some post checks before patching
        if (patched.name != null) {
            val anyOtherRepo = asyncTransaction(ChartedScope) {
                RepositoryEntity.find {
                    (RepositoryTable.name eq patched.name) and (RepositoryTable.owner eq call.currentUser!!.id)
                }.firstOrNull()
            }

            if (anyOtherRepo != null) {
                throw ValidationException("body.name", "Can't rename repository ${patched.name} since repository already exists on your account")
            }
        }

        val repoFlags = RepositoryFlags()
        if (patched.private == true) {
            repoFlags.add("PRIVATE")
        }

        asyncTransaction(ChartedScope) {
            RepositoryTable.update(whereClause) {
                it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())

                if (patched.description != null) {
                    it[description] = patched.description
                }

                if (patched.deprecated != null) {
                    it[deprecated] = patched.deprecated
                }

                if (patched.private != null) {
                    it[flags] = repoFlags.bits()
                }

                if (patched.name != null) {
                    it[name] = patched.name
                }

                if (patched.type != null) {
                    it[type] = patched.type
                }
            }
        }

        call.respond(HttpStatusCode.Accepted, ApiResponse.ok())
    }

    @Delete("/{idOrName}")
    suspend fun deleteRepository(call: ApplicationCall) {
        val repository = call.getRepositoryByIdOrName() ?: return
        asyncTransaction(ChartedScope) {
            RepositoryTable.deleteWhere { RepositoryTable.id eq repository.id }
        }

        call.respond(HttpStatusCode.Accepted, ApiResponse.ok())
    }

    companion object {
        /**
         * Transforms the [RepositoriesEndpoints] with the necessary data that is applicable
         * for the OpenAPI specification. This is used in the [charted][org.noelware.charted.server.openapi.charted] DSL
         * function.
         */
        fun RootDsl.toOpenAPI() {
            "/repositories" get {
            }
        }
    }
}
