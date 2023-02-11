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

package org.noelware.charted.modules.analytics

import dev.floofy.utils.java.SetOnce
import dev.floofy.utils.slf4j.logging
import io.grpc.protobuf.services.ProtoReflectionService
import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.decodeFromStream
import org.bouncycastle.util.io.pem.PemReader
import org.noelware.analytics.jvm.server.AnalyticsServer
import org.noelware.analytics.jvm.server.AnalyticsServerBuilder
import org.noelware.analytics.jvm.server.extensions.Extension
import org.noelware.charted.ChartedInfo
import org.noelware.charted.configuration.kotlin.dsl.NoelwareAnalyticsConfig
import org.noelware.charted.extensions.toUri
import org.noelware.charted.modules.analytics.kotlin.dsl.toBuildFlavour
import org.noelware.charted.types.responses.ApiResponse
import java.io.Closeable
import java.io.IOException
import java.io.StringReader
import java.net.http.HttpClient
import java.net.http.HttpRequest
import java.net.http.HttpRequest.BodyPublishers
import java.net.http.HttpResponse.BodyHandlers
import java.security.KeyFactory
import java.security.spec.X509EncodedKeySpec
import java.time.Instant
import java.util.*
import javax.crypto.Cipher

/**
 * Represents a daemon process that runs the analytics server in the background outside the
 * main API server.
 *
 * @param config The configuration object for configuring the daemon
 */
class AnalyticsDaemon(
    private val json: Json,
    private val config: NoelwareAnalyticsConfig,
    private val extension: Extension<*>
) : Closeable {
    private val httpClient: HttpClient = HttpClient.newHttpClient()
    private val server = SetOnce<AnalyticsServer>()
    private val log by logging<AnalyticsDaemon>()

    @OptIn(ExperimentalSerializationApi::class)
    fun start() {
        if (server.wasSet()) {
            log.warn("Analytics daemon is already running! Not doing anything...")
            return
        }

        val bindIP = config.grpcBindIp ?: "127.0.0.1"
        log.info("Starting the protocol server with host [$bindIP:${config.port}]")
        val serverBuilder = AnalyticsServerBuilder(bindIP, config.port)
            .withServiceToken(config.serviceToken)
            .withExtension(extension)
            .withServerBuilder { builder -> builder.addService(ProtoReflectionService.newInstance()) }
            .withServerMetadata { metadata ->
                val info = ChartedInfo

                metadata.setDistributionType(info.distribution.toBuildFlavour())
                metadata.setProductName("charted-server")
                metadata.setCommitHash(info.commitHash)
                metadata.setBuildDate(Instant.parse(info.buildDate))
                metadata.setVersion(info.version)
                metadata.setVendor("Noelware, LLC.")
            }
            .build()

        server.value = serverBuilder
        serverBuilder.start()

        // TODO: Allow setting of bind IP, default 0.0.0.0
        val initReq = Requests.InitRequest("0.0.0.0:${config.port}")
        val request = HttpRequest
            .newBuilder("${config.endpoint}/instances/${serverBuilder.instanceUUID()}/init".toUri())
            .POST(BodyPublishers.ofString(json.encodeToString(initReq)))
            .header("Content-Type", "application/json")
            .build()

        val resp = httpClient.send(request, BodyHandlers.ofInputStream())
        return resp.body().use { stream ->
            val r: ApiResponse<Requests.InitResponse> = json.decodeFromStream(stream)
            if (r.success) {
                val apiToken = Base64.getDecoder().decode(config.serviceToken).decodeToString().split(":")[1]
                val pemReader = PemReader(StringReader((r as ApiResponse.Ok<Requests.InitResponse>).data!!.pubKey))
                val keySpec = X509EncodedKeySpec(pemReader.readPemObject().content)
                val pubKey = KeyFactory.getInstance("RSA").generatePublic(keySpec)
                val cipher = Cipher.getInstance("RSA")
                cipher.init(Cipher.ENCRYPT_MODE, pubKey)

                val encoded = Base64.getEncoder().encodeToString(cipher.doFinal(apiToken.toByteArray()))
                val final = HttpRequest.newBuilder("${config.endpoint}/instances/${serverBuilder.instanceUUID()}/finalize".toUri())
                    .header("Content-Type", "application/json")
                    .POST(BodyPublishers.ofString(json.encodeToString(Requests.FinalizeRequest(encoded))))
                    .build()

                val res = httpClient.send(final, BodyHandlers.ofInputStream())
                res.body().use { `is` ->
                    val decoded: ApiResponse<Unit> = json.decodeFromStream(`is`)
                    if (!decoded.success) {
                        val errors = decoded as ApiResponse.Err
                        val printableErrors = errors.errors.map { err ->
                            "\t* ${err.code}: ${err.message}"
                        }

                        log.error("Unable to finalize request with status [${res.statusCode()}] with errors:\n${printableErrors.joinToString("\n")}")
                    }
                }
            }
        }
    }

    /**
     * Closes this stream and releases any system resources associated
     * with it. If the stream is already closed then invoking this
     * method has no effect.
     *
     * As noted in [AutoCloseable.close], cases where the
     * close may fail require careful attention. It is strongly advised
     * to relinquish the underlying resources and to internally
     * *mark* the `Closeable` as closed, prior to throwing
     * the `IOException`.
     *
     * @throws IOException if an I/O error occurs
     */
    override fun close() {
        if (!server.wasSet()) return
        server.value.close()
    }
}
