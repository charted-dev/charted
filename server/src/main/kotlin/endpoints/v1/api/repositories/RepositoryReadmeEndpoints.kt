@file:Suppress("unused")

package org.noelware.charted.server.endpoints.v1.api.repositories

import guru.zoroark.tegral.openapi.dsl.RootDsl
import guru.zoroark.tegral.openapi.dsl.schema
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import org.noelware.charted.databases.postgres.asyncTransaction
import org.noelware.charted.databases.postgres.entities.RepositoryEntity
import org.noelware.charted.databases.postgres.models.Repository
import org.noelware.charted.modules.storage.StorageHandler
import org.noelware.charted.server.createKtorContentWithInputStream
import org.noelware.charted.server.openapi.extensions.externalDocsUrl
import org.noelware.charted.server.plugins.PreconditionResult
import org.noelware.charted.server.plugins.SessionsPlugin
import org.noelware.charted.types.responses.ApiError
import org.noelware.charted.types.responses.ApiResponse
import org.noelware.ktor.endpoints.*
import java.io.ByteArrayInputStream

class RepositoryReadmeEndpoints(private val storage: StorageHandler): AbstractEndpoint("/repositories/{id}/readme") {
    init {
        install(HttpMethod.Put, "/repositories/{id}/readme", SessionsPlugin) {
            this += "repo:metadata:update"
            condition { call ->
                val repository = call.getRepositoryEntityById() ?: return@condition PreconditionResult.Failed(ApiError.EMPTY, HttpStatusCode.BadRequest)
                call.repoHasPermission(repository, "metadata:update")
            }
        }

        install(HttpMethod.Delete, "/repositories/{id}/readme", SessionsPlugin) {
            this += "repo:metadata:delete"
            condition { call ->
                val repository = call.getRepositoryEntityById() ?: return@condition PreconditionResult.Failed(ApiError.EMPTY, HttpStatusCode.BadRequest)
                call.repoHasPermission(repository, "metadata:update")
            }
        }
    }

    @Get
    suspend fun get(call: ApplicationCall) {
        val id = call.parameters["id"]!!.toLongOrNull() ?: return call.respond(
            HttpStatusCode.NotAcceptable,
            ApiResponse.err(
                "INVALID_SNOWFLAKE",
                "`id` path parameter must be a valid snowflake",
            ),
        )

        asyncTransaction {
            RepositoryEntity.findById(id)?.let { entity -> Repository.fromEntity(entity) }
        } ?: return call.respond(
            HttpStatusCode.NotFound,
            ApiResponse.err(
                "UNKNOWN_REPOSITORY",
                "Repository with ID [$id] was not found",
            ),
        )

        val readme = storage.open("./repositories/$id/README.md")
            ?: return call.respond(HttpStatusCode.NotFound)

        call.respond(createKtorContentWithInputStream(readme, ContentType.Text.Plain))
    }

    // Would bad people be able to put anything in README.md files? Yes, absolutely
    // but, will it create any vulnerabilities? It could possibly happen!
    //
    // To answer the question "is this a bad idea?" Yes -- but, I do not
    // have a better plan. Please make an issue if you know how to make this
    // better.
    @Put
    suspend fun put(call: ApplicationCall) {
        val id = call.parameters["id"]!!.toLongOrNull() ?: return call.respond(
            HttpStatusCode.NotAcceptable,
            ApiResponse.err(
                "INVALID_SNOWFLAKE",
                "`id` path parameter must be a valid snowflake",
            ),
        )

        val body: String = call.receive()
        storage.upload("./repositories/$id/README.md", ByteArrayInputStream(body.toByteArray()), "text/plain; charset=utf-8")
        call.respond(HttpStatusCode.Accepted, ApiResponse.ok())
    }

    @Delete
    suspend fun delete(call: ApplicationCall) {
        val id = call.parameters["id"]!!.toLongOrNull() ?: return call.respond(
            HttpStatusCode.NotAcceptable,
            ApiResponse.err(
                "INVALID_SNOWFLAKE",
                "`id` path parameter must be a valid snowflake",
            ),
        )

        storage.delete("./repositories/$id/README.md")
        call.respond(HttpStatusCode.Accepted, ApiResponse.ok())
    }

    companion object {
        fun RootDsl.toOpenAPI() {
            "/repositories/{id}/readme" {
                get {
                    externalDocsUrl("repositories", "/{id}/readme")
                    description = "Returns the README of this repository"

                    200 response {
                        "text/plain" content {
                            schema<String>()
                            example = "# Hello, world\n>We do this?"
                        }
                    }

                    404 response {
                        "application/json" content {
                            schema<ApiResponse.Err>()
                        }
                    }
                }

                put {
                    externalDocsUrl("repositories", "/{id}/readme")
                    description = "Create or updates the content of the README"
                    body {
                        "application/json" content {
                            schema<String>()
                            example = "# Hello, world\n>We do this?"
                        }
                    }

                    202 response {
                        "application/json" content {
                            schema<ApiResponse.Ok<Unit>>()
                        }
                    }
                }

                delete {
                    externalDocsUrl("repositories", "/{id}/readme")
                    description = "Deletes the README.md file for this repository"

                    202 response {
                        "application/json" content {
                            schema<ApiResponse.Ok<Unit>>()
                        }
                    }
                }
            }
        }
    }
}
