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
import io.github.z4kn4fein.semver.VersionFormatException
import io.github.z4kn4fein.semver.toVersion
import io.ktor.http.*
import io.ktor.http.content.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.features.ServerFeature
import org.noelware.charted.databases.postgres.asyncTransaction
import org.noelware.charted.databases.postgres.entities.RepositoryReleaseEntity
import org.noelware.charted.databases.postgres.models.Repository
import org.noelware.charted.databases.postgres.models.RepositoryRelease
import org.noelware.charted.databases.postgres.tables.RepositoryReleasesTable
import org.noelware.charted.modules.helm.charts.HelmChartModule
import org.noelware.charted.server.createKtorContentWithInputStream
import org.noelware.charted.server.openapi.extensions.addSessionResponses
import org.noelware.charted.server.openapi.extensions.externalDocsUrl
import org.noelware.charted.server.plugins.PreconditionResult
import org.noelware.charted.server.plugins.SessionsPlugin
import org.noelware.charted.types.responses.ApiError
import org.noelware.charted.types.responses.ApiResponse
import org.noelware.ktor.endpoints.*

class RepositoryReleasesEndpoints(
    private val config: Config,
    private val helmCharts: HelmChartModule? = null
): AbstractEndpoint("/repositories/{id}/releases") {
    init {
        install(HttpMethod.Get, "/repositories/{id}/releases/{version}/templates/{template}", SessionsPlugin) {
            condition { call ->
                val repository = call.getRepositoryEntityById() ?: return@condition PreconditionResult.Failed(
                    ApiError.EMPTY, HttpStatusCode.BadRequest,
                )

                call.canAccessRepository(repository)
            }
        }

        install(HttpMethod.Get, "/repositories/{id}/releases/{version}/templates", SessionsPlugin) {
            condition { call ->
                val repository = call.getRepositoryEntityById() ?: return@condition PreconditionResult.Failed(
                    ApiError.EMPTY, HttpStatusCode.BadRequest,
                )

                call.canAccessRepository(repository)
            }
        }

        install(HttpMethod.Get, "/repositories/{id}/releases/{version}/values.yaml", SessionsPlugin) {
            condition { call ->
                val repository = call.getRepositoryEntityById() ?: return@condition PreconditionResult.Failed(
                    ApiError.EMPTY, HttpStatusCode.BadRequest,
                )

                call.canAccessRepository(repository)
            }
        }

        install(HttpMethod.Get, "/repositories/{id}/releases/{version}/Chart.yaml", SessionsPlugin) {
            condition { call ->
                val repository = call.getRepositoryEntityById() ?: return@condition PreconditionResult.Failed(
                    ApiError.EMPTY, HttpStatusCode.BadRequest,
                )

                call.canAccessRepository(repository)
            }
        }

        install(HttpMethod.Get, "/repositories/{id}/releases/{version}.tar.gz", SessionsPlugin) {
            condition { call ->
                val repository = call.getRepositoryEntityById() ?: return@condition PreconditionResult.Failed(ApiError.EMPTY, HttpStatusCode.BadRequest)
                call.canAccessRepository(repository)
            }
        }

        install(HttpMethod.Post, "/repositories/{id}/releases/{version}.tar.gz", SessionsPlugin) {
            this += "repo:releases:create"
            condition { call ->
                val repository = call.getRepositoryEntityById() ?: return@condition PreconditionResult.Failed(ApiError.EMPTY, HttpStatusCode.BadRequest)
                call.repoHasPermission(repository, "metadata:update")
            }
        }

        install(HttpMethod.Delete, "/repositories/{id}/releases/{releaseId}", SessionsPlugin) {
            this += "repo:releases:delete"

            condition { call ->
                val repository = call.getRepositoryEntityById() ?: return@condition PreconditionResult.Failed(ApiError.EMPTY, HttpStatusCode.BadRequest)
                call.repoHasPermission(repository, "metadata:delete")
            }
        }

        install(HttpMethod.Patch, "/repositories/{id}/releases/{releaseId}", SessionsPlugin) {
            this += "repo:releases:update"

            condition { call ->
                val repository = call.getRepositoryEntityById() ?: return@condition PreconditionResult.Failed(ApiError.EMPTY, HttpStatusCode.BadRequest)
                call.repoHasPermission(repository, "metadata:update")
            }
        }

        install(HttpMethod.Put, "/repositories/{id}/releases", SessionsPlugin) {
            this += "repo:releases:create"

            condition { call ->
                val repository = call.getRepositoryEntityById() ?: return@condition PreconditionResult.Failed(ApiError.EMPTY, HttpStatusCode.BadRequest)
                call.repoHasPermission(repository, "metadata:update")
            }
        }

        install(HttpMethod.Get, "/repositories/{id}/releases/{releaseId}", SessionsPlugin) {
            condition { call ->
                val repository = call.getRepositoryEntityById() ?: return@condition PreconditionResult.Failed(
                    ApiError.EMPTY, HttpStatusCode.BadRequest,
                )

                call.canAccessRepository(repository)
            }
        }

        install(HttpMethod.Get, "/repositories/{id}/releases", SessionsPlugin) {
            condition { call ->
                val repository = call.getRepositoryEntityById() ?: return@condition PreconditionResult.Failed(
                    ApiError.EMPTY, HttpStatusCode.BadRequest,
                )

                call.canAccessRepository(repository)
            }
        }
    }

    @Get
    suspend fun getAll(call: ApplicationCall) {
        val repo = call.getRepositoryById() ?: return
        val releases = asyncTransaction {
            RepositoryReleaseEntity.find {
                RepositoryReleasesTable.repository eq repo.id
            }.toList().map { entity -> RepositoryRelease.fromEntity(entity) }
        }

        call.respond(HttpStatusCode.OK, ApiResponse.ok(releases))
    }

    @Get("/{releaseId}")
    suspend fun getById(call: ApplicationCall) {
        val id = call.parameters["releaseId"]?.toLongOrNull() ?: return call.respond(
            HttpStatusCode.BadRequest,
            ApiResponse.err(
                "INVALID_SNOWFLAKE",
                "Provided repo release ID was not a snowflake.",
            ),
        )

        val release = asyncTransaction {
            RepositoryReleaseEntity.findById(id)?.ifNotNull { RepositoryRelease.fromEntity(this) }
        } ?: return call.respond(
            HttpStatusCode.NotFound,
            ApiResponse.err(
                "UNKNOWN_RELEASE",
                "Repository release with ID [$id] was not found",
            ),
        )

        call.respond(HttpStatusCode.OK, ApiResponse.ok(release))
    }

    @Put
    suspend fun createRelease(call: ApplicationCall) {}

    @Patch("/{releaseId}")
    suspend fun patchRelease(call: ApplicationCall) {}

    @Delete("/{releaseId}")
    suspend fun deleteRelease(call: ApplicationCall) {}

    @Get("/{version}.tar.gz")
    suspend fun getVersionTarball(call: ApplicationCall) {
        if (config.features.contains(ServerFeature.DOCKER_REGISTRY)) {
            return call.respond(HttpStatusCode.NotFound)
        }

        val version = call.parameters["version"]!!
        try {
            version.toVersion(true)
        } catch (e: VersionFormatException) {
            return call.respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "INVALID_SEMVER",
                    "Version [$version] was not a valid SemVer v2 version",
                ),
            )
        }

        val repository = call.getRepositoryEntityById() ?: return
        val stream = helmCharts!!.getReleaseTarball(repository.owner, repository.id.value, version)
            ?: return call.respond(
                HttpStatusCode.NotFound,
                ApiResponse.err(
                    "UNKNOWN_RELEASE",
                    "Release $version doesn't have a Helm chart tarball.",
                ),
            )

        call.respond(createKtorContentWithInputStream(stream, ContentType.parse("application/tar+gzip")))
    }

    @Post("/{version}.tar.gz")
    suspend fun uploadTarball(call: ApplicationCall) {
        if (config.features.contains(ServerFeature.DOCKER_REGISTRY)) {
            return call.respond(HttpStatusCode.NotFound)
        }

        val repository = call.getRepositoryEntityById() ?: return
        val multipart = call.receiveMultipart()
        val part = multipart.readPart() ?: return call.respond(
            HttpStatusCode.BadRequest,
            ApiResponse.err("EXCESSIVE_MULTIPART_AMOUNT", "There can be only one multipart in this request."),
        )

        if (part !is PartData.FileItem) {
            return call.respond(
                HttpStatusCode.NotAcceptable,
                ApiResponse.err("NOT_FILE_PART", "The multipart object must be a File object."),
            )
        }

        val version = call.parameters["version"]!!
        try {
            version.toVersion(true)
        } catch (e: VersionFormatException) {
            return call.respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "INVALID_SEMVER",
                    "Version [$version] was not a valid SemVer v2 version",
                ),
            )
        }

        helmCharts!!.uploadReleaseTarball(repository.owner, Repository.fromEntity(repository), version, part)
        call.respond(HttpStatusCode.Accepted, ApiResponse.ok())
    }

    @Get("/{version}/Chart.yaml")
    suspend fun getReleaseChartYaml(call: ApplicationCall) {
        if (config.features.contains(ServerFeature.DOCKER_REGISTRY)) {
            return call.respond(HttpStatusCode.NotFound)
        }

        val version = call.parameters["version"]!!
        try {
            version.toVersion(true)
        } catch (e: VersionFormatException) {
            return call.respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "INVALID_SEMVER",
                    "Version [$version] was not a valid SemVer v2 version",
                ),
            )
        }

        val repository = call.getRepositoryEntityById() ?: return
        val stream = helmCharts!!.getChartYaml(repository.owner, repository.id.value, version) ?: return call.respond(HttpStatusCode.NotFound)

        call.respond(createKtorContentWithInputStream(stream, ContentType.parse("text/yaml; charset=utf-8")))
    }

    @Get("/{version}/values.yaml")
    suspend fun getReleaseValuesYaml(call: ApplicationCall) {
        if (config.features.contains(ServerFeature.DOCKER_REGISTRY)) {
            return call.respond(HttpStatusCode.NotFound)
        }

        val version = call.parameters["version"]!!
        try {
            version.toVersion(true)
        } catch (e: VersionFormatException) {
            return call.respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "INVALID_SEMVER",
                    "Version [$version] was not a valid SemVer v2 version",
                ),
            )
        }

        val repository = call.getRepositoryEntityById() ?: return
        val stream = helmCharts!!.getValuesYaml(repository.owner, repository.id.value, version) ?: return call.respond(HttpStatusCode.NotFound)

        call.respond(createKtorContentWithInputStream(stream, ContentType.parse("text/yaml; charset=utf-8")))
    }

    @Get("/{version}/templates")
    suspend fun getReleaseTemplates(call: ApplicationCall) {
        if (config.features.contains(ServerFeature.DOCKER_REGISTRY)) {
            return call.respond(HttpStatusCode.NotFound)
        }

        val version = call.parameters["version"]!!
        try {
            version.toVersion(true)
        } catch (e: VersionFormatException) {
            return call.respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "INVALID_SEMVER",
                    "Version [$version] was not a valid SemVer v2 version",
                ),
            )
        }

        val repository = call.getRepositoryEntityById() ?: return
        val templates = helmCharts!!.getAllTemplates(repository.owner, repository.id.value, version)
        call.respond(HttpStatusCode.OK, ApiResponse.ok(templates))
    }

    @Get("/{version}/templates/{template}")
    suspend fun getReleaseTemplate(call: ApplicationCall) {
        if (config.features.contains(ServerFeature.DOCKER_REGISTRY)) {
            return call.respond(HttpStatusCode.NotFound)
        }

        val version = call.parameters["version"]!!
        try {
            version.toVersion(true)
        } catch (e: VersionFormatException) {
            return call.respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "INVALID_SEMVER",
                    "Version [$version] was not a valid SemVer v2 version",
                ),
            )
        }

        val repository = call.getRepositoryEntityById() ?: return
        val stream = helmCharts!!.getTemplate(repository.owner, repository.id.value, version, call.parameters["template"]!!) ?: return call.respond(HttpStatusCode.NotFound)
        call.respond(createKtorContentWithInputStream(stream, ContentType.parse("text/plain; charset=utf-8")))
    }

    companion object {
        /**
         * Transforms the [RepositoryReleasesEndpoints] with the necessary data that is applicable
         * for the OpenAPI specification. This is used in the [charted][org.noelware.charted.server.openapi.charted] DSL
         * function.
         */
        fun RootDsl.toOpenAPI() {
            "/repositories/{id}/releases" get {
                description = "Returns all the releases that this repository has created"
                externalDocsUrl("repository", "GET-/repositories/{id}/releases")

                200 response {
                    "application/json" content {
                        schema<ApiResponse.Ok<List<RepositoryRelease>>>()
                    }
                }

                404 response {
                    "application/json" content {
                        schema<ApiResponse.Err>()
                    }
                }

                addSessionResponses()
            }

            "/repositories/{id}/releases/{releaseId}" {
                get {
                }

                patch {
                }

                delete {
                }
            }

            "/repositories/{id}/releases/{version}.tar.gz" {
                get {
                }

                post {
                }
            }

            "/repositories/{id}/releases/{version}/Chart.yaml" get {
            }

            "/repositories/{id}/releases/{version}/values.yaml" get {
            }

            "/repositories/{id}/releases/{version}/templates" get {
            }

            "/repositories/{id}/releases/{version}/templates/{template}" get {
            }
        }
    }
}
