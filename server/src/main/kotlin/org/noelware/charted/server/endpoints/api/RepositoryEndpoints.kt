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
import dev.floofy.utils.slf4j.logging
import io.ktor.http.*
import io.ktor.http.content.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import kotlinx.datetime.Clock
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import kotlinx.serialization.decodeFromString
import kotlinx.serialization.json.addJsonObject
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import kotlinx.serialization.json.putJsonArray
import org.noelware.charted.common.SHAUtils
import org.noelware.charted.common.data.Config
import org.noelware.charted.common.data.Feature
import org.noelware.charted.common.data.formatCdnUrl
import org.noelware.charted.common.data.helm.ChartIndexSpec
import org.noelware.charted.common.data.helm.ChartIndexYaml
import org.noelware.charted.common.data.helm.ChartSpec
import org.noelware.charted.core.StorageWrapper
import org.noelware.charted.database.controllers.RepositoryController
import org.noelware.charted.server.plugins.Sessions
import org.noelware.charted.server.session
import org.noelware.charted.server.utils.createOutgoingContentWithBytes
import org.noelware.ktor.endpoints.*
import org.noelware.remi.core.figureContentType
import java.io.ByteArrayInputStream
import java.io.ByteArrayOutputStream

class RepositoryEndpoints(
    private val storage: StorageWrapper,
    private val config: Config,
    private val yaml: Yaml
): AbstractEndpoint("/repositories") {
    private val log by logging<RepositoryEndpoints>()

    init {
        install(HttpMethod.Put, "/repositories", Sessions) {
            addScope("repo:create")
        }

        install(HttpMethod.Put, "/repositories/{id}/Chart.yaml", Sessions) {
            addScope("repo:update")
        }

        install(HttpMethod.Put, "/repositories/{id}/values.yaml", Sessions) {
            addScope("repo:update")
        }

        install(HttpMethod.Put, "/repositories/{id}/tarballs/{version}", Sessions) {
            addScope("repo:update")
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
                        put("docs", "https://charts.noelware.org/docs/api/repositories")
                    }
                )
            }
        )
    }

    @Put
    suspend fun create(call: ApplicationCall) {
        val (status, result) = RepositoryController.create(call.session.userID, call.receive())
        call.respond(status, result)
    }

    @Get("/{id}")
    suspend fun get(call: ApplicationCall) {
        val id = call.parameters["id"]!!.toLong()
        val repository = RepositoryController.get(id) ?: return call.respond(
            HttpStatusCode.NotFound,
            buildJsonObject {
                put("success", false)
                putJsonArray("errors") {
                    addJsonObject {
                        put("code", "UNKNOWN_REPOSITORY")
                        put("message", "Couldn't find repository by ID [$id]")
                    }
                }
            }
        )

        call.respond(
            HttpStatusCode.OK,
            buildJsonObject {
                put("success", true)
                put("data", repository.toJsonObject())
            }
        )
    }

    @Get("/{id}/Chart.yaml")
    suspend fun getChartMetadata(call: ApplicationCall) {
        if (config.isFeatureEnabled(Feature.DOCKER_REGISTRY)) {
            return call.respond(
                HttpStatusCode.Forbidden,
                buildJsonObject {
                    put("success", false)
                    putJsonArray("errors") {
                        addJsonObject {
                            put("code", "CHARTS_NOT_AVAILABLE")
                            put("message", "This instance is running off a local OCI-based registry, so this endpoint is not available.")
                        }
                    }
                }
            )
        }

        val id = call.parameters["id"]!!.toLong()
        val repo = RepositoryController.get(id) ?: return call.respond(
            HttpStatusCode.NotFound,
            buildJsonObject {
                put("success", false)
                putJsonArray("errors") {
                    addJsonObject {
                        put("code", "UNKNOWN_REPOSITORY")
                        put("message", "Couldn't find repository by ID [$id]")
                    }
                }
            }
        )

        val chart = storage.trailer.fetch("./metadata/${repo.ownerID}/$id/Chart.yaml") ?: return call.respond(
            HttpStatusCode.NotFound,
            buildJsonObject {
                put("success", false)
                putJsonArray("errors") {
                    addJsonObject {
                        put("code", "MISSING_CHART_FILE")
                        put("message", "The repository is missing a Chart.yaml file! Did you upload it via PUT /$id/Chart.yaml?")
                    }
                }
            }
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
                contentType = ContentType.parse("application/yaml; charset=utf-8")
            )
        )
    }

    @Get("/{id}/values.yaml")
    suspend fun getChartValues(call: ApplicationCall) {
        if (config.isFeatureEnabled(Feature.DOCKER_REGISTRY)) {
            return call.respond(
                HttpStatusCode.Forbidden,
                buildJsonObject {
                    put("success", false)
                    putJsonArray("errors") {
                        addJsonObject {
                            put("code", "CHARTS_NOT_AVAILABLE")
                            put("message", "This instance is running off a local OCI-based registry, so this endpoint is not available.")
                        }
                    }
                }
            )
        }

        val id = call.parameters["id"]!!.toLong()
        val repo = RepositoryController.get(id) ?: return call.respond(
            HttpStatusCode.NotFound,
            buildJsonObject {
                put("success", false)
                putJsonArray("errors") {
                    addJsonObject {
                        put("code", "UNKNOWN_REPOSITORY")
                        put("message", "Couldn't find repository by ID [$id]")
                    }
                }
            }
        )

        val values = storage.trailer.fetch("./metadata/${repo.ownerID}/$id/values.yaml") ?: return call.respond(
            HttpStatusCode.NotFound,
            buildJsonObject {
                put("success", false)
                putJsonArray("errors") {
                    addJsonObject {
                        put("code", "MISSING_VALUES_FILE")
                        put("message", "The repository is missing a values.yaml file! Did you upload it via PUT /$id/values.yaml?")
                    }
                }
            }
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
                contentType = ContentType.parse("application/yaml; charset=utf-8")
            )
        )
    }

    @Put("/{id}/Chart.yaml")
    suspend fun uploadChart(call: ApplicationCall) {
        if (config.isFeatureEnabled(Feature.DOCKER_REGISTRY)) {
            return call.respond(
                HttpStatusCode.Forbidden,
                buildJsonObject {
                    put("success", false)
                    putJsonArray("errors") {
                        addJsonObject {
                            put("code", "CHARTS_NOT_AVAILABLE")
                            put("message", "This instance is running off a local OCI-based registry, so this endpoint is not available.")
                        }
                    }
                }
            )
        }

        val id = call.parameters["id"]!!.toLong()
        val repo = RepositoryController.get(id) ?: return call.respond(
            HttpStatusCode.NotFound,
            buildJsonObject {
                put("success", false)
                putJsonArray("errors") {
                    addJsonObject {
                        put("code", "UNKNOWN_REPOSITORY")
                        put("message", "Couldn't find repository by ID [$id]")
                    }
                }
            }
        )

        val multipart = call.receiveMultipart()
        val parts = multipart.readAllParts()
        val first = parts.firstOrNull() ?: return call.respond(
            HttpStatusCode.BadRequest,
            buildJsonObject {
                put("success", false)
                putJsonArray("errors") {
                    addJsonObject {
                        put("code", "MORE_THAN_ONE_PART_SPECIFIED")
                        put("message", "There can be only one part or there was no parts.")
                    }
                }
            }
        )

        if (first !is PartData.FileItem) {
            return call.respond(
                HttpStatusCode.NotAcceptable,
                buildJsonObject {
                    put("success", false)
                    putJsonArray("errors") {
                        addJsonObject {
                            put("code", "NOT_FILE_PART")
                            put("message", "The multipart item was not a file.")
                        }
                    }
                }
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

        // Merge the owner's helm charts with the `Chart.yaml` file
        val indexYaml = storage.open("./metadata/${repo.ownerID}/index.yaml")!!
        val helmChart = yaml.decodeFromStream<ChartIndexYaml>(indexYaml)
        if (helmChart.entries.containsKey(chartSpec.name)) {
            // Check if the version already exists in this spec
            val foundVersion = helmChart.entries[chartSpec.name]!!.singleOrNull { it.version == chartSpec.version }
            if (foundVersion != null) {
                return call.respond(
                    HttpStatusCode.NotAcceptable,
                    buildJsonObject {
                        put("success", false)
                        putJsonArray("errors") {
                            addJsonObject {
                                put("code", "VERSION_EXISTS")
                                put("message", "Can't override current Chart.yaml for version ${chartSpec.version}. This endpoint is mainly for uploading new versions of the `Chart.yaml` file.")
                            }
                        }
                    }
                )
            }

            val url = formatCdnUrl(config, "/tarballs/${repo.ownerID}/$id/${repo.name}-${chartSpec.version}.tar.gz")
            val checksum = SHAUtils.sha256Checksum(ByteArrayInputStream(data))

            helmChart.entries[chartSpec.name]!!.add(
                ChartIndexSpec.fromSpec(
                    listOf(if (url.startsWith("http") || url.startsWith("https")) url else "http://${config.server.host}:${config.server.port}$url"),
                    Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault()),
                    false,
                    checksum,
                    chartSpec
                )
            )
        } else {
            val url = formatCdnUrl(config, "/tarballs/${repo.ownerID}/$id/${repo.name}-${chartSpec.version}.tar.gz")
            val checksum = SHAUtils.sha256Checksum(ByteArrayInputStream(data))

            // TODO: configure checksum for the tarball
            helmChart.entries[chartSpec.name] = mutableListOf(
                ChartIndexSpec.fromSpec(
                    listOf(if (url.startsWith("http") || url.startsWith("https")) url else "http://${config.server.host}:${config.server.port}$url"),
                    Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault()),
                    false,
                    checksum,
                    chartSpec
                )
            )
        }

        helmChart.generated = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
        val os = ByteArrayOutputStream()
        yaml.encodeToStream(ChartIndexYaml.serializer(), helmChart, os)

        storage.upload("./metadata/${repo.ownerID}/$id/Chart.yaml", ByteArrayInputStream(data), "application/yaml")
        storage.upload("./metadata/${repo.ownerID}/index.yaml", ByteArrayInputStream(os.toByteArray()), "application/yaml")
        first.dispose() // dispose of it to release it to the wild~

        call.respond(
            HttpStatusCode.Accepted,
            buildJsonObject {
                put("success", true)
            }
        )
    }

    @Put("/{id}/values.yaml")
    suspend fun uploadChartValues(call: ApplicationCall) {
        if (config.isFeatureEnabled(Feature.DOCKER_REGISTRY)) {
            return call.respond(
                HttpStatusCode.Forbidden,
                buildJsonObject {
                    put("success", false)
                    putJsonArray("errors") {
                        addJsonObject {
                            put("code", "CHARTS_NOT_AVAILABLE")
                            put("message", "This instance is running off a local OCI-based registry, so this endpoint is not available.")
                        }
                    }
                }
            )
        }

        val id = call.parameters["id"]!!.toLong()
        val repo = RepositoryController.get(id) ?: return call.respond(
            HttpStatusCode.NotFound,
            buildJsonObject {
                put("success", false)
                putJsonArray("errors") {
                    addJsonObject {
                        put("code", "UNKNOWN_REPOSITORY")
                        put("message", "Couldn't find repository by ID [$id]")
                    }
                }
            }
        )

        val multipart = call.receiveMultipart()
        val parts = multipart.readAllParts()
        val first = parts.firstOrNull() ?: return call.respond(
            HttpStatusCode.BadRequest,
            buildJsonObject {
                put("success", false)
                putJsonArray("errors") {
                    addJsonObject {
                        put("code", "MORE_THAN_ONE_PART_SPECIFIED")
                        put("message", "There can be only one part or there was no parts.")
                    }
                }
            }
        )

        if (first !is PartData.FileItem) {
            return call.respond(
                HttpStatusCode.NotAcceptable,
                buildJsonObject {
                    put("success", false)
                    putJsonArray("errors") {
                        addJsonObject {
                            put("code", "NOT_FILE_PART")
                            put("message", "The multipart item was not a file.")
                        }
                    }
                }
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

        storage.upload("./metadata/${repo.ownerID}/$id/values.yaml", ByteArrayInputStream(data), "application/yaml")
        first.dispose() // dispose of it to release it to the wild~

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
                buildJsonObject {
                    put("success", false)
                    putJsonArray("errors") {
                        addJsonObject {
                            put("code", "CHARTS_NOT_AVAILABLE")
                            put("message", "This instance is running off a local OCI-based registry, so this endpoint is not available.")
                        }
                    }
                }
            )
        }

        val id = call.parameters["id"]!!.toLong()
        val version = call.parameters["version"]!!
        val repo = RepositoryController.get(id) ?: return call.respond(
            HttpStatusCode.NotFound,
            buildJsonObject {
                put("success", false)
                putJsonArray("errors") {
                    addJsonObject {
                        put("code", "UNKNOWN_REPOSITORY")
                        put("message", "Couldn't find repository by ID [$id]")
                    }
                }
            }
        )

        val multipart = call.receiveMultipart()
        val parts = multipart.readAllParts()
        val first = parts.firstOrNull() ?: return call.respond(
            HttpStatusCode.BadRequest,
            buildJsonObject {
                put("success", false)
                putJsonArray("errors") {
                    addJsonObject {
                        put("code", "MORE_THAN_ONE_PART_SPECIFIED")
                        put("message", "There can be only one part or there was no parts.")
                    }
                }
            }
        )

        if (first !is PartData.FileItem) {
            return call.respond(
                HttpStatusCode.NotAcceptable,
                buildJsonObject {
                    put("success", false)
                    putJsonArray("errors") {
                        addJsonObject {
                            put("code", "NOT_FILE_PART")
                            put("message", "The multipart item was not a file.")
                        }
                    }
                }
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

        storage.upload("./tarballs/${repo.ownerID}/$id/${repo.name}-$version.tar.gz", ByteArrayInputStream(data), contentType)
        first.dispose()
        call.respond(
            HttpStatusCode.Accepted,
            buildJsonObject {
                put("success", true)
            }
        )
    }

    @Patch("/{id}")
    suspend fun patch(call: ApplicationCall) {}

    @Delete("/{id}")
    suspend fun delete(call: ApplicationCall) {}

    @Get("/{id}/members")
    suspend fun members(call: ApplicationCall) {}

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
}
