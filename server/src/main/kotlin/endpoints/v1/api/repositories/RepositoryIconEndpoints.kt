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

@file:Suppress("unused")

package org.noelware.charted.server.endpoints.v1.api.repositories

import guru.zoroark.tegral.openapi.dsl.RootDsl
import guru.zoroark.tegral.openapi.dsl.schema
import io.ktor.http.*
import io.ktor.http.content.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import org.noelware.charted.ChartedInfo
import org.noelware.charted.modules.avatars.AvatarFetchUtil
import org.noelware.charted.modules.avatars.AvatarModule
import org.noelware.charted.server.createKtorContentWithByteArray
import org.noelware.charted.server.plugins.PreconditionResult
import org.noelware.charted.server.plugins.SessionsPlugin
import org.noelware.charted.types.responses.ApiError
import org.noelware.charted.types.responses.ApiResponse
import org.noelware.ktor.endpoints.AbstractEndpoint
import org.noelware.ktor.endpoints.Get
import org.noelware.ktor.endpoints.Post
import kotlin.reflect.full.createType

/**
 * Represents the main API entrypoint for the Repository Icons API.
 */
class RepositoryIconEndpoints(private val avatars: AvatarModule): AbstractEndpoint("/repositories/{idOrName}/icons") {
    init {
        install(HttpMethod.Post, SessionsPlugin) {
            this += "repo:icons:update"
            condition { call ->
                val repository = call.getRepositoryEntityByIdOrName() ?: return@condition PreconditionResult.Failed(ApiError.EMPTY)
                call.repoHasPermission(repository, "metadata:update")
            }
        }
    }

    /**
     * Returns the current repository's icon, if any. Otherwise, a 404 will occur.
     * @statusCode 200 The icon that is visible in the request
     * @statusCode 404 If the repository or repo icon was not found.
     */
    @Get("/current.png")
    suspend fun current(call: ApplicationCall) {
        val repository = call.getRepositoryByIdOrName() ?: return
        val (contentType, bytes) = avatars.retrieveRepoIcon(repository, null)
            ?: return call.respond(HttpStatusCode.NotFound)

        call.respond(createKtorContentWithByteArray(bytes, contentType))
    }

    /**
     * Returns a current repository's icon by its hash, if any. Otherwise, a 404 will occur.
     * @statusCode 200 The icon that is visible in the request
     * @statusCode 404 If the repository or repo icon was not found.
     */
    @Get("/{hash}")
    suspend fun hash(call: ApplicationCall) {
        val repository = call.getRepositoryByIdOrName() ?: return
        val (contentType, bytes) = avatars.retrieveRepoIcon(repository, call.parameters["hash"])
            ?: return call.respond(HttpStatusCode.NotFound)

        call.respond(createKtorContentWithByteArray(bytes, contentType))
    }

    /**
     * Updates the repository's icon. If more parts were used in this request, the server will filter
     * through 5 requests (at most), retrieve the first part that matches a file, and discards the rest,
     * or bails if no parts or more than 5 parts were sent.
     *
     * @statusCode 202 If the repository's icon was successfully updated
     * @statusCode 400 If the request was not a `multipart/form-data` request, or if there were no parts available, or if there were
     *                 more 5 parts included in this request.
     */
    @Post
    suspend fun update(call: ApplicationCall) {
        val repository = call.getRepositoryByIdOrName() ?: return
        val multipart = call.receiveMultipart()
        val parts = multipart.readAllParts()

        if (parts.isEmpty()) {
            return call.respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "MISSING_FILE_PART",
                    "The request is missing a file part to be used.",
                ),
            )
        }

        if (parts.size < 5) {
            return call.respond(
                HttpStatusCode.PayloadTooLarge,
                ApiResponse.err(
                    "TOO_MANY_PARTS",
                    "You have sent ${parts.size} multipart requests, but we only allow to filter over 5 requests.",
                ),
            )
        }

        // probably inefficient, but what else?
        var correctPart: PartData.FileItem? = null
        val partsAsQueue = parts.toMutableList()
        while (true) {
            val current = partsAsQueue.removeFirstOrNull() ?: break
            if (current is PartData.FileItem) {
                correctPart = current
                break
            }
        }

        if (correctPart == null) {
            return call.respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "UNKNOWN_FILE_PART",
                    "Couldn't find any multi-parts that was a File",
                ),
            )
        }

        AvatarFetchUtil.updateRepositoryIcon(repository, correctPart)
        call.respond(HttpStatusCode.Accepted, ApiResponse.ok())
    }

    companion object {
        /**
         * Transforms the [RepositoryIconEndpoints] with the necessary data that is applicable
         * for the OpenAPI specification. This is used in the [charted][org.noelware.charted.server.openapi.charted] DSL
         * function.
         */
        fun RootDsl.toOpenAPI() {
            "/repositories/{idOrName}/icons/current.png" get {
                externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/repositories#GET-/:idOrName/icons/current.png"
                description = "Returns the current repository's icon, if any. Otherwise, a 404 will occur."

                "idOrName" pathParameter {
                    description = "The repository's snowflake ID or name"
                    schema<String>()
                }

                200 response {
                    description = "The actual avatar as an image file"
                    "image/png" content {
                        schema(ByteArray::class.createType())
                    }

                    "image/jpg" content {
                        schema(ByteArray::class.createType())
                    }

                    "image/jpeg" content {
                        schema(ByteArray::class.createType())
                    }

                    "image/gif" content {
                        schema(ByteArray::class.createType())
                    }
                }

                404 response {
                    description = "If the icon was not found"
                    "application/json" content {
                        schema<ApiResponse.Err>()
                    }
                }
            }

            "/repositories/{idOrName}/icons/{hash}" get {
                externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/repositories#GET-/:idOrName/icons/:hash"
                description = "Returns a repository's icon by its hash, if any. Otherwise, a 404 will occur."

                "idOrName" pathParameter {
                    description = "The repository's snowflake ID or name"
                    schema<String>()
                }

                "hash" pathParameter {
                    description = "The icon hash as `<key>.<ext>`"
                    schema<String>()
                }

                200 response {
                    description = "The actual icon as an image file"
                    "image/png" content {
                        schema(ByteArray::class.createType())
                    }

                    "image/jpg" content {
                        schema(ByteArray::class.createType())
                    }

                    "image/jpeg" content {
                        schema(ByteArray::class.createType())
                    }

                    "image/gif" content {
                        schema(ByteArray::class.createType())
                    }
                }

                404 response {
                    description = "If the avatar was not found"
                    "application/json" content {
                        schema<ApiResponse.Err>()
                    }
                }
            }

            "/repositories/{idOrName}/icons" post {
                externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/repositories#POST-/:idOrName/icons"
                description = """Updates the repository's icon. If more parts were used in this request, the server will filter
                through 5 requests (at most), retrieve the first part that matches a file, and discards the rest,
                or bails if no parts or more than 5 parts were sent.
                """.trimIndent()

                "idOrName" pathParameter {
                    description = "The repository's snowflake ID or name"
                    schema<String>()
                }

                202 response {
                    description = "If the repository icon update was a success"
                    "application/json" content {
                        schema<ApiResponse.Ok<Unit>>()
                    }
                }

                400 response {
                    description = "If the request was not a `multipart/form-data` request, or if there were no parts available, or if there were more 5 parts included in this request."
                    "application/json" content {
                        schema<ApiResponse.Err>()
                    }
                }
            }
        }
    }
}
