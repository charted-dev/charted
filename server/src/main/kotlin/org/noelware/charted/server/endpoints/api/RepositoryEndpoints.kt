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

@file:Suppress("UNUSED")

package org.noelware.charted.server.endpoints.api

import com.charleskorn.kaml.Yaml
import com.charleskorn.kaml.YamlException
import com.charleskorn.kaml.decodeFromStream
import dev.floofy.utils.exposed.asyncTransaction
import dev.floofy.utils.slf4j.logging
import io.github.z4kn4fein.semver.VersionFormatException
import io.github.z4kn4fein.semver.toVersion
import io.ktor.http.*
import io.ktor.http.content.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.withContext
import kotlinx.datetime.Clock
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import kotlinx.serialization.decodeFromString
import kotlinx.serialization.json.*
import okhttp3.internal.closeQuietly
import org.noelware.charted.common.ChartedScope
import org.noelware.charted.common.SHAUtils
import org.noelware.charted.common.data.helm.ChartIndexSpec
import org.noelware.charted.common.data.helm.ChartIndexYaml
import org.noelware.charted.common.data.helm.ChartSpec
import org.noelware.charted.common.data.responses.Response
import org.noelware.charted.configuration.dsl.Config
import org.noelware.charted.configuration.dsl.features.Feature
import org.noelware.charted.core.StorageWrapper
import org.noelware.charted.database.controllers.RepositoryController
import org.noelware.charted.database.controllers.RepositoryMemberController
import org.noelware.charted.database.entities.RepositoryEntity
import org.noelware.charted.database.models.Repository
import org.noelware.charted.database.tables.RepositoryTable
import org.noelware.charted.email.EmailService
import org.noelware.charted.features.webhooks.WebhooksFeature
import org.noelware.charted.server.currentUser
import org.noelware.charted.server.plugins.Sessions
import org.noelware.charted.server.utils.createOutgoingContentWithBytes
import org.noelware.ktor.endpoints.*
import org.noelware.remi.core.figureContentType
import java.io.ByteArrayInputStream
import java.io.ByteArrayOutputStream

class RepositoryEndpoints(
    private val webhooks: WebhooksFeature? = null,
    private val storage: StorageWrapper,
    private val config: Config,
    private val yaml: Yaml,
    private val email: EmailService? = null
): AbstractEndpoint("/repositories") {
    private val log by logging<RepositoryEndpoints>()

    init {
        install(HttpMethod.Put, "/repositories", Sessions) {
            addScope("repo:create")
        }

        install(HttpMethod.Put, "/repositories/{id}/Chart.yaml", Sessions) {
            addScope("repo:write")
        }

        install(HttpMethod.Put, "/repositories/{id}/values.yaml", Sessions) {
            addScope("repo:write")
        }

        install(HttpMethod.Put, "/repositories/{id}/tarballs/{version}", Sessions) {
            addScope("repo:write")
        }

        install(HttpMethod.Patch, "/repositories/{id}", Sessions) {
            addScope("repo:update")
        }

        install(HttpMethod.Delete, "/repositories/{id}", Sessions) {
            addScope("repo:delete")
        }

        install(HttpMethod.Put, "/repositories/{id}/members", Sessions) {
            addScope("repo:member:join")
        }

        install(HttpMethod.Patch, "/repositories/{id}/members/{memberId}", Sessions) {
            addScope("repo:member:update")
        }

        install(HttpMethod.Delete, "/repositories/{id}/members/{memberId}", Sessions) {
            addScope("repo:member:kick")
        }

        install(HttpMethod.Put, "/repositories/{id}/releases", Sessions) {
            addScope("repo:release:create")
        }

        install(HttpMethod.Patch, "/repositories/{id}/releases/{tag}", Sessions) {
            addScope("repo:release:update")
        }

        install(HttpMethod.Delete, "/repositories/{id}/releases/{tag}", Sessions) {
            addScope("repo:release:delete")
        }

        install(HttpMethod.Get, "/repositories/{id}/webhooks", Sessions) {
            addScope("repo:webhooks:list")
        }

        install(HttpMethod.Put, "/repositories/{id}/webhooks", Sessions) {
            addScope("repo:webhooks:create")
        }

        install(HttpMethod.Patch, "/repositories/{id}/webhooks/{webhookId}", Sessions) {
            addScope("repo:webhooks:update")
        }

        install(HttpMethod.Delete, "/repositories/{id}/webhooks/{webhookId}", Sessions) {
            addScope("repo:webhooks:delete")
        }
    }

    @Get
    suspend fun main(call: ApplicationCall) {
        call.respond(
            HttpStatusCode.OK,
            buildJsonObject {
                put("success", true)
                put(
                    "data",
                    buildJsonObject {
                        put("message", "Welcome to the Repositories API!")
                        put("docs", "https://charts.noelware.org/docs/server/api/repositories")
                    }
                )
            }
        )
    }

    @Put
    suspend fun create(call: ApplicationCall) {
        val (status, result) = RepositoryController.create(call.currentUser!!.id.toLong(), call.receive())
        call.respond(status, result)
    }

    @Get("/{id}")
    suspend fun get(call: ApplicationCall) {
        val id = call.parameters["id"]!!
        val repository = if (id.toLongOrNull() != null) {
            RepositoryController.get(id.toLong())
        } else {
            asyncTransaction(ChartedScope) {
                RepositoryEntity.find { RepositoryTable.name eq id }.firstOrNull()?.let { entity -> Repository.fromEntity(entity) }
            }
        } ?: return call.respond(
            HttpStatusCode.NotFound,
            Response.err("UNKNOWN_REPOSITORY", "Unable to find repository by ID [$id]")
        )

        call.respond(
            HttpStatusCode.OK,
            Response.ok(repository)
        )
    }

    @Get("/{id}/Chart.yaml")
    suspend fun getChartMetadata(call: ApplicationCall) {
        if (config.isFeatureEnabled(Feature.DOCKER_REGISTRY)) {
            return call.respond(
                HttpStatusCode.Forbidden,
                Response.err("CHARTS_NOT_AVAILABLE", "This instance is running off a local OCI-based registry, so this endpoint is not available.")
            )
        }

        val id = call.parameters["id"]!!.toLong()
        val repo = RepositoryController.get(id) ?: return call.respond(
            HttpStatusCode.NotFound,
            Response.err("UNKNOWN_REPOSITORY", "Unable to find repository by ID [$id]")
        )

        val chart = storage.trailer.fetch("./metadata/${repo.ownerID}/$id/Chart.yaml") ?: return call.respond(
            HttpStatusCode.NotFound,
            Response.err("MISSING_CHART_FILE", "The repository is missing a Chart.yaml file! Did you push to the registry with the Helm plugin?")
        )

        val baos = ByteArrayOutputStream()
        withContext(Dispatchers.IO) {
            chart.inputStream!!.transferTo(baos)
        }

        val data = baos.toByteArray()
        call.response.header("Chart-Last-Modified", chart.lastModifiedAt.toString())
        call.response.header("ETag", chart.etag)

        call.respond(
            createOutgoingContentWithBytes(
                data,
                contentType = ContentType.parse("text/plain; charset=utf-8")
            )
        )
    }

    @Get("/{id}/values.yaml")
    suspend fun getChartValues(call: ApplicationCall) {
        if (config.isFeatureEnabled(Feature.DOCKER_REGISTRY)) {
            return call.respond(
                HttpStatusCode.Forbidden,
                Response.err("CHARTS_NOT_AVAILABLE", "This instance is running off a local OCI-based registry, so this endpoint is not available.")
            )
        }

        val id = call.parameters["id"]!!.toLong()
        val repo = RepositoryController.get(id) ?: return call.respond(
            HttpStatusCode.NotFound,
            Response.err("UNKNOWN_REPOSITORY", "Unable to find repository by ID [$id]")
        )

        val values = storage.trailer.fetch("./metadata/${repo.ownerID}/$id/values.yaml") ?: return call.respond(
            HttpStatusCode.NotFound,
            Response.err("MISSING_VALUES_FILE", "The repository is missing a values.yaml file! Did you push to the registry with the Helm plugin?")
        )

        val baos = ByteArrayOutputStream()
        withContext(Dispatchers.IO) {
            values.inputStream!!.transferTo(baos)
        }

        val data = baos.toByteArray()
        call.response.header("Values-Last-Modified", values.lastModifiedAt.toString())
        call.response.header("ETag", values.etag)

        call.respond(
            createOutgoingContentWithBytes(
                data,
                contentType = ContentType.parse("text/plain; charset=utf-8")
            )
        )
    }

    @Put("/{id}/Chart.yaml")
    suspend fun uploadChart(call: ApplicationCall) {
        if (config.isFeatureEnabled(Feature.DOCKER_REGISTRY)) {
            return call.respond(
                HttpStatusCode.Forbidden,
                Response.err("CHARTS_NOT_AVAILABLE", "This instance is running off a local OCI-based registry, so this endpoint is not available.")
            )
        }

        val id = call.parameters["id"]!!.toLong()
        val repo = RepositoryController.get(id) ?: return call.respond(
            HttpStatusCode.NotFound,
            Response.err("UNKNOWN_REPOSITORY", "Unable to find repository by ID [$id]")
        )

        val multipart = call.receiveMultipart()
        val parts = multipart.readAllParts()
        val first = parts.firstOrNull() ?: return call.respond(
            HttpStatusCode.BadRequest,
            Response.err("EXCESSIVE_MULTIPART_AMOUNT", "There can be only one multipart in this request.")
        )

        if (first !is PartData.FileItem) {
            return call.respond(
                HttpStatusCode.NotAcceptable,
                Response.err("NOT_FILE_PART", "The multipart object must be a File object.")
            )
        }

        val inputStream = first.streamProvider()
        val baos = ByteArrayOutputStream()
        withContext(Dispatchers.IO) {
            inputStream.transferTo(baos)
        }

        val data = baos.toByteArray()

        // See if the contents is a Chart.yaml file type.
        // It'll throw an error if it's not valid.
        val chartSpec = yaml.decodeFromString<ChartSpec>(String(data))
        val indexYaml = storage.open("./metadata/${repo.ownerID}/index.yaml")!!
        val chartIndex = yaml.decodeFromStream<ChartIndexYaml>(indexYaml)
        val mergedIndex: ChartIndexYaml = if (chartIndex.entries.containsKey(chartSpec.name)) {
            // There can be only one version (or should it merge either way?)
            val foundVersion = chartIndex.entries[chartSpec.name]!!.singleOrNull { it.version == chartSpec.version }
            if (foundVersion != null) {
                return call.respond(
                    HttpStatusCode.NotAcceptable,
                    Response.err("VERSION_ALREADY_RELEASED", "There is a version for ${chartSpec.version} already published.")
                )
            }

            // Versions should abide by SemVer
            // https://helm.sh/docs/topics/charts/#charts-and-versioning
            val semver = try {
                chartSpec.version.toVersion(true)
            } catch (e: VersionFormatException) {
                return call.respond(HttpStatusCode.NotAcceptable, Response.err("INVALID_SEMVER", e.message!!))
            }

            val url = if (config.cdn) {
                "${config.baseUrl ?: "http://localhost:${config.server.port}"}/cdn/tarballs/${repo.ownerID}/$id/${repo.name}-${chartSpec.version}.tar.gz"
            } else {
                "${config.baseUrl ?: "http://localhost:${config.server.port}"}/repositories/$id/tarballs/${repo.name}-${chartSpec.version}.tar.gz"
            }

            val checksum = SHAUtils.sha256Checksum(ByteArrayInputStream(data))
            val entries = chartIndex.entries.toMutableMap()
            entries[chartSpec.name]!!.add(
                ChartIndexSpec.fromSpec(
                    listOf(if (url.startsWith("http") || url.startsWith("https")) url else "http://${config.server.host}:${config.server.port}$url"),
                    Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault()),
                    false,
                    checksum,
                    chartSpec
                )
            )

            chartIndex.copy(entries = entries, generated = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault()))
        } else {
            val url = if (config.cdn) {
                "${config.baseUrl ?: "http://localhost:${config.server.port}"}/cdn/tarballs/${repo.ownerID}/$id/${repo.name}-${chartSpec.version}.tar.gz"
            } else {
                "${config.baseUrl ?: "http://localhost:${config.server.port}"}/repositories/$id/tarballs/${repo.name}-${chartSpec.version}.tar.gz"
            }

            val checksum = SHAUtils.sha256Checksum(ByteArrayInputStream(data))
            val entries = chartIndex.entries.toMutableMap()
            entries[chartSpec.name]!!.add(
                ChartIndexSpec.fromSpec(
                    listOf(if (url.startsWith("http") || url.startsWith("https")) url else "http://${config.server.host}:${config.server.port}$url"),
                    Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault()),
                    false,
                    checksum,
                    chartSpec
                )
            )

            chartIndex.copy(entries = entries, generated = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault()))
        }

        val os = ByteArrayOutputStream()
        yaml.encodeToStream(ChartIndexYaml.serializer(), mergedIndex, os)

        ByteArrayInputStream(data).use {
            runBlocking {
                storage.upload("./metadata/${repo.ownerID}/$id/index.yaml", it, "")
            }
        }

        os.use {
            runBlocking {
                storage.upload("./metadata/${repo.ownerID}/index.yaml", ByteArrayInputStream(it.toByteArray()), "")
            }
        }

        first.dispose()
        call.respond(HttpStatusCode.Accepted, Response.ok())
    }

    @Put("/{id}/values.yaml")
    suspend fun uploadChartValues(call: ApplicationCall) {
        if (config.isFeatureEnabled(Feature.DOCKER_REGISTRY)) {
            return call.respond(
                HttpStatusCode.Forbidden,
                Response.err("CHARTS_NOT_AVAILABLE", "This instance is running off a local OCI-based registry, so this endpoint is not available.")
            )
        }

        val id = call.parameters["id"]!!.toLong()
        val repo = RepositoryController.get(id) ?: return call.respond(
            HttpStatusCode.NotFound,
            Response.err("UNKNOWN_REPOSITORY", "Unable to find repository by ID [$id]")
        )

        val multipart = call.receiveMultipart()
        val parts = multipart.readAllParts()
        val first = parts.firstOrNull() ?: return call.respond(
            HttpStatusCode.BadRequest,
            Response.err("EXCESSIVE_MULTIPART_AMOUNT", "There can be only one multipart in this request.")
        )

        if (first !is PartData.FileItem) {
            return call.respond(
                HttpStatusCode.NotAcceptable,
                Response.err("NOT_FILE_PART", "The multipart object must be a File object.")
            )
        }

        val inputStream = first.streamProvider()
        val baos = ByteArrayOutputStream()
        withContext(Dispatchers.IO) {
            inputStream.transferTo(baos)
        }

        val data = baos.toByteArray()
        try {
            yaml.parseToYamlNode(String(data))
            baos.closeQuietly()
        } catch (e: YamlException) {
            return call.respond(
                HttpStatusCode.NotAcceptable,
                buildJsonObject {
                    put("success", false)
                    putJsonArray("errors") {
                        addJsonObject {
                            put("code", "INVALID_YAML")
                            put("message", "Unable to encode data into YAML.")
                        }
                    }
                }
            )
        }

        ByteArrayInputStream(data).use {
            storage.upload("./metadata/${repo.ownerID}/$id/values.yaml", it, "application/yaml")
        }

        first.dispose()
        call.respond(
            HttpStatusCode.Accepted,
            buildJsonObject {
                put("success", true)
            }
        )
    }

    @Put("/{id}/tarballs/{version}")
    suspend fun uploadTarballFor(call: ApplicationCall) {
        if (config.isFeatureEnabled(Feature.DOCKER_REGISTRY)) {
            return call.respond(
                HttpStatusCode.Forbidden,
                Response.err("CHARTS_NOT_AVAILABLE", "This instance is running off a local OCI-based registry, so this endpoint is not available.")
            )
        }

        val id = call.parameters["id"]!!.toLong()
        val version = call.parameters["version"]!!

        // Versions should abide by SemVer
        // https://helm.sh/docs/topics/charts/#charts-and-versioning
        val semver = try {
            version.toVersion(true)
        } catch (e: VersionFormatException) {
            return call.respond(HttpStatusCode.NotAcceptable, Response.err("INVALID_SEMVER", e.message!!))
        }

        val repo = RepositoryController.get(id) ?: return call.respond(
            HttpStatusCode.NotFound,
            Response.err("UNKNOWN_REPOSITORY", "Unable to find repository by ID [$id]")
        )

        val multipart = call.receiveMultipart()
        val parts = multipart.readAllParts()
        val first = parts.firstOrNull() ?: return call.respond(
            HttpStatusCode.BadRequest,
            Response.err("EXCESSIVE_MULTIPART_AMOUNT", "There can be only one multipart in this request.")
        )

        if (first !is PartData.FileItem) {
            return call.respond(
                HttpStatusCode.NotAcceptable,
                Response.err("NOT_FILE_PART", "The multipart object must be a File object.")
            )
        }

        val inputStream = first.streamProvider()
        val baos = ByteArrayOutputStream()
        withContext(Dispatchers.IO) {
            inputStream.transferTo(baos)
        }

        val data = baos.toByteArray()
        val contentType = storage.trailer.figureContentType(data)
        if (!(listOf("application/gzip", "application/tar+gzip", "application/tar").contains(contentType))) {
            return call.respond(
                HttpStatusCode.NotAcceptable,
                buildJsonObject {
                    put("success", false)
                    putJsonArray("errors") {
                        addJsonObject {
                            put("code", "INVALID_TARBALL")
                            put("message", "File provided was not a tarball")
                        }
                    }
                }
            )
        }

        storage.upload("./tarballs/${repo.ownerID}/$id/${repo.name}-$semver.tar.gz", ByteArrayInputStream(data), contentType)
        first.dispose()
        call.respond(
            HttpStatusCode.Accepted,
            Response.ok()
        )
    }

    @Patch("/{id}")
    suspend fun patch(call: ApplicationCall) {
        RepositoryController.update(call.parameters["id"]!!.toLong(), call.receive())
        call.respond(
            HttpStatusCode.Accepted,
            Response.ok()
        )
    }

    @Delete("/{id}")
    suspend fun delete(call: ApplicationCall) {
        val success = RepositoryController.delete(call.parameters["id"]!!.toLong())
        call.respond(
            HttpStatusCode.Accepted,
            Response.ok()
        )
    }

    @Get("/{id}/members")
    suspend fun members(call: ApplicationCall) {
        val members = RepositoryMemberController.getAll(call.parameters["id"]!!.toLong()).map { it.toJsonObject() }
        call.respond(
            HttpStatusCode.OK,
            Response.ok(members)
        )
    }

    @Put("/{id}/members")
    suspend fun inviteMember(call: ApplicationCall) {}

    @Post("/{id}/members/invitations/{inviteId}")
    suspend fun joinRepository(call: ApplicationCall) {}

    @Patch("/{id}/members/{memberId}")
    suspend fun patchMember(call: ApplicationCall) {}

    @Delete("/{id}/members/{memberId}")
    suspend fun kickMember(call: ApplicationCall) {}

    @Get("/{id}/members/{memberId}")
    suspend fun memberById(call: ApplicationCall) {}

    @Get("/{id}/releases")
    suspend fun releases(call: ApplicationCall) {}

    @Put("/{id}/releases")
    suspend fun createRelease(call: ApplicationCall) {}

    @Get("/{id}/releases/{tag}")
    suspend fun release(call: ApplicationCall) {}

    @Patch("/{id}/releases/{tag}")
    suspend fun patchRelease(call: ApplicationCall) {}

    @Delete("/{id}/releases/{tag}")
    suspend fun deleteRelease(call: ApplicationCall) {}

    @Get("/{id}/webhooks")
    suspend fun webhooks(call: ApplicationCall) {}

    @Get("/{id}/webhooks/{webhookId}")
    suspend fun webhook(call: ApplicationCall) {}

    @Put("/{id}/webhooks")
    suspend fun createWebhook(call: ApplicationCall) {}

    @Patch("/{id}/webhooks/{webhookId}")
    suspend fun patchWebhook(call: ApplicationCall) {}

    @Delete("/{id}/webhooks/{webhookId}")
    suspend fun deleteWebhook(call: ApplicationCall) {}
}
