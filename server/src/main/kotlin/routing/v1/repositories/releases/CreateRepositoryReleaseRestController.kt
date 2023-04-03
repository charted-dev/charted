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

package org.noelware.charted.server.routing.v1.repositories.releases

import io.github.z4kn4fein.semver.VersionFormatException
import io.github.z4kn4fein.semver.toVersion
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.ktor.server.util.*
import io.swagger.v3.oas.models.PathItem
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.models.flags.ApiKeyScope.Repositories
import org.noelware.charted.models.repositories.RepositoryRelease
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.openapi.toPaths
import org.noelware.charted.modules.postgresql.controllers.get
import org.noelware.charted.modules.postgresql.controllers.repositories.RepositoryDatabaseController
import org.noelware.charted.modules.postgresql.controllers.repositories.releases.CreateRepositoryReleasePayload
import org.noelware.charted.modules.postgresql.controllers.repositories.releases.RepositoryReleaseDatabaseController
import org.noelware.charted.modules.postgresql.ktor.RepositoryAttributeKey
import org.noelware.charted.server.extensions.addAuthenticationResponses
import org.noelware.charted.server.extensions.putAndRemove
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.plugins.sessions.preconditions.canAccessRepository
import org.noelware.charted.server.plugins.sessions.preconditions.canEditMetadata
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController

class CreateRepositoryReleaseRestController(
    private val controller: RepositoryReleaseDatabaseController,
    private val repositoriesController: RepositoryDatabaseController
): RestController("/repositories/{id}/releases", HttpMethod.Put) {
    override val apiVersion: APIVersion = APIVersion.V1
    override fun Route.init() {
        install(Sessions) {
            this += Repositories.Releases.Create

            condition(::canAccessRepository)
            condition { call -> canEditMetadata(call, repositoriesController) }
        }
    }

    override suspend fun call(call: ApplicationCall) {
        val id = call.parameters.getOrFail<Long>("id")
        val repo = repositoriesController.getEntityOrNull(id)!!
        val payload: CreateRepositoryReleasePayload = call.receive()

        try {
            payload.tag.toVersion(true)
        } catch (e: VersionFormatException) {
            return call.respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "INVALID_SEMVER",
                    "Version provided '${payload.tag}' was not a valid SemVer value",
                ),
            )
        }

        return call.attributes.putAndRemove(RepositoryAttributeKey, repo) {
            val release = controller.create(call, payload)
            call.respond(HttpStatusCode.Created, ApiResponse.ok(release))
        }
    }

    override fun toPathDsl(): PathItem = toPaths("/repositories/{id}/releases") {
        put {
            description = "Creates a repository release"
            requestBody {
                description = "Payload for creating a repository release"
                contentType(ContentType.Application.Json) {
                    schema(
                        CreateRepositoryReleasePayload(
                            updateText = "# 0.0.1-beta\nSome updates!",
                            "0.0.1-beta",
                        ),
                    )
                }
            }

            addAuthenticationResponses()
            response(HttpStatusCode.Created) {
                description = "Release resource was created successfully"
                contentType(ContentType.Application.Json) {
                    schema<ApiResponse.Ok<RepositoryRelease>>()
                }
            }
        }
    }
}
