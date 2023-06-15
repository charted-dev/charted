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
import io.ktor.http.content.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.ktor.server.util.*
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.features.ExperimentalFeature
import org.noelware.charted.configuration.kotlin.dsl.features.Feature
import org.noelware.charted.models.flags.ApiKeyScope
import org.noelware.charted.modules.helm.charts.HelmChartModule
import org.noelware.charted.modules.openapi.VersionConstraint
import org.noelware.charted.modules.openapi.kotlin.dsl.badRequest
import org.noelware.charted.modules.openapi.kotlin.dsl.created
import org.noelware.charted.modules.openapi.kotlin.dsl.json
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.postgresql.controllers.get
import org.noelware.charted.modules.postgresql.controllers.repositories.RepositoryDatabaseController
import org.noelware.charted.server.extensions.addAuthenticationResponses
import org.noelware.charted.server.plugins.sessions.PreconditionResult
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.plugins.sessions.preconditions.canAccessRepository
import org.noelware.charted.server.plugins.sessions.preconditions.canEditMetadata
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController
import org.noelware.charted.server.routing.openapi.ResourceDescription
import org.noelware.charted.server.routing.openapi.describeResource

class CreateRepositoryReleaseTarballRestController(
    private val controller: RepositoryDatabaseController,
    private val charts: HelmChartModule? = null,
    private val config: Config
): RestController("/repositories/{id}/releases/{version}.tar.gz", HttpMethod.Post) {
    override val apiVersion: APIVersion = APIVersion.V1
    override fun Route.init() {
        install(Sessions) {
            this += ApiKeyScope.Repositories.Releases.Create

            condition(::canAccessRepository)
            condition { call -> canEditMetadata(call, controller) }
            condition { call ->
                if (config.features.contains(Feature.DockerRegistry) || config.experimentalFeatures.contains(
                        ExperimentalFeature.ExternalOciRegistry,
                    )
                ) {
                    call.respond(HttpStatusCode.NotFound)
                    return@condition PreconditionResult.Failed()
                }

                PreconditionResult.Success
            }
        }
    }

    override suspend fun call(call: ApplicationCall) {
        val id = call.parameters.getOrFail<Long>("id")
        val repo = controller.get(id)
        val version = call.parameters.getOrFail("version")

        try {
            version.toVersion(true)
        } catch (e: VersionFormatException) {
            return call.respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "INVALID_SEMVER",
                    "Version provided '$version' was not a valid SemVer value",
                ),
            )
        }

        val multipart = call.receiveMultipart()
        val part = multipart.readPart() ?: return call.respond(
            HttpStatusCode.BadRequest,
            ApiResponse.err(
                "MISSING_FILE_PART",
                "Unable to determine file part to use",
            ),
        )

        if (part !is PartData.FileItem) {
            return call.respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "NOT_FILE_PART",
                    "Part [${part.name}] was not a file.",
                ),
            )
        }

        val provenancePart = multipart.readPart()
        if (provenancePart != null && provenancePart !is PartData.FileItem) {
            return call.respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "NOT_FILE_PART",
                    "Part [${provenancePart.name}] was not a file.",
                ),
            )
        }

        charts!!.uploadReleaseTarball {
            this.provenanceFile = if (provenancePart != null) provenancePart as PartData.FileItem else null
            this.version = version
            this.repo = repo

            tarballFile = part
            owner = repo.ownerID
        }

        call.respond(HttpStatusCode.Created, ApiResponse.ok())
    }

    companion object: ResourceDescription by describeResource("/repositories/{id}/releases/{version}.tar.gz", {
        post {
            description = "Uploads a tarball that is a valid Helm tarball that was generated from the `helm package` command."

            requestBody {
                description = "multipart/form-data object with a file part being the actual tarball"
                contentType(ContentType.MultiPart.FormData)
            }

            pathParameter {
                description = "Repository ID to lookup"
                name = "id"

                schema<Long>()
            }

            pathParameter {
                description = "Valid SemVer version to use as the tarball name"
                name = "version"

                schema<VersionConstraint>()
            }

            addAuthenticationResponses()
            created {
                description = "Tarball was stored successfully"
                json {
                    schema(ApiResponse.ok())
                }
            }

            badRequest {
                description = "Invalid multipart/form-data object"
                json {
                    schema(
                        ApiResponse.err(
                            "NOT_FILE_PART",
                            "Part was not a file.",
                        ),
                    )
                }
            }
        }
    })
}
