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
import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.ktor.server.util.*
import io.swagger.v3.oas.models.PathItem
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.features.ExperimentalFeature
import org.noelware.charted.configuration.kotlin.dsl.features.Feature
import org.noelware.charted.modules.helm.charts.HelmChartModule
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.openapi.toPaths
import org.noelware.charted.modules.postgresql.controllers.get
import org.noelware.charted.modules.postgresql.controllers.repositories.RepositoryDatabaseController
import org.noelware.charted.server.extensions.addAuthenticationResponses
import org.noelware.charted.server.plugins.sessions.PreconditionResult
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.plugins.sessions.preconditions.canAccessRepository
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController
import org.noelware.charted.server.util.createBodyFromInputStream

class GetSingleRepositoryReleaseTemplateRestController(
    private val controller: RepositoryDatabaseController,
    private val charts: HelmChartModule? = null,
    private val config: Config
): RestController("/repositories/{id}/releases/{version}/template/{template}") {
    override val apiVersion: APIVersion = APIVersion.V1
    override fun Route.init() {
        install(Sessions) {
            allowNonAuthorizedRequests = true

            condition { call -> canAccessRepository(call, false) }
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
        val versionToLookup = call.parameters.getOrFail("version")
        val allowPrereleases = call.request.queryParameters["allow_prerelease"]?.let { it matches "^(yes|true|1)".toRegex() } ?: false
        val version = if (versionToLookup == "latest") {
            // If we were unable to look up the latest version, it's probably not a valid
            // SemVer version available or if the repository doesn't include any release tarballs
            // that were uploaded.
            charts!!.getLatestVersion(repo.ownerID, repo.id, allowPrereleases)
                ?: return call.respond(HttpStatusCode.NotFound)
        } else {
            try {
                versionToLookup.toVersion(false)
                versionToLookup
            } catch (e: VersionFormatException) {
                return call.respond(
                    HttpStatusCode.BadRequest,
                    ApiResponse.err(
                        "INVALID_SEMVER",
                        "Version provided '$versionToLookup' was not a valid SemVer value",
                    ),
                )
            }
        }

        val template = charts!!.getTemplate(repo.ownerID, repo.id, version, call.parameters.getOrFail("template"))
            ?: return call.respond(HttpStatusCode.NotFound)

        call.respond(createBodyFromInputStream(template, ContentType.parse("text/yaml; charset=utf-8")))
    }

    override fun toPathDsl(): PathItem = toPaths("/repositories/{id}/releases/{version}/templates/{template}") {
        get {
            description = "List of all available templates of a given release"
            pathParameter {
                description = "Repository ID to lookup"
                name = "id"

                schema<Long>()
            }

            pathParameter {
                description = "Valid SemVer version to lookup"
                name = "version"

                schema<String>()
            }

            pathParameter {
                description = "Valid template to look for"
                name = "template"

                schema<String>()
            }

            queryParameter {
                description = "If we are allowed to look-up for pre-releases if the version path parameter is \"latest\""
                name = "allow_prerelease"

                schema<Boolean>()
            }

            addAuthenticationResponses()
            response(HttpStatusCode.OK) {
                description = "Valid Kubernetes API object as YAML"
                contentType(ContentType.parse("text/yaml; charset=utf-8")) {
                    schema<Map<String, Any>>()
                }
            }

            response(HttpStatusCode.BadRequest) {
                description = "If the version path parameter wasn't a valid SemVer version"
                contentType(ContentType.Application.Json) {
                    schema(
                        ApiResponse.err(
                            "INVALID_SEMVER",
                            "Version provided 'v1.noel-is-cute' was not a valid SemVer value",
                        ),
                    )
                }
            }

            response(HttpStatusCode.NotFound) {
                description = "If the template wasn't found"
                contentType(ContentType.Application.Json)
            }
        }
    }
}
