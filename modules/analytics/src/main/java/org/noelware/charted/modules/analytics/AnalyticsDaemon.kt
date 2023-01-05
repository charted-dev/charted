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

package org.noelware.charted.modules.analytics

import dev.floofy.utils.java.SetOnce
import io.grpc.protobuf.services.ProtoReflectionService
import kotlinx.serialization.decodeFromString
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.OkHttpClient
import okhttp3.Request
import okhttp3.RequestBody.Companion.toRequestBody
import org.bouncycastle.util.io.pem.PemReader
import org.noelware.analytics.jvm.server.AnalyticsServer
import org.noelware.analytics.jvm.server.AnalyticsServerBuilder
import org.noelware.analytics.jvm.server.extensions.Extension
import org.noelware.analytics.protobufs.v1.BuildFlavour
import org.noelware.charted.ChartedInfo
import org.noelware.charted.DistributionType
import org.noelware.charted.configuration.kotlin.dsl.NoelwareAnalyticsConfig
import org.noelware.charted.types.responses.ApiResponse
import org.slf4j.LoggerFactory
import java.io.Closeable
import java.io.IOException
import java.io.StringReader
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
class AnalyticsDaemon(private val config: NoelwareAnalyticsConfig, private val extension: Extension<*>) : Closeable {
    private val server = SetOnce<AnalyticsServer>()
    private val logger = LoggerFactory.getLogger(AnalyticsDaemon::class.java)
    private val httpClient = OkHttpClient.Builder()
        .addInterceptor { chain -> chain.proceed(chain.request().newBuilder().header("Authorization", config.endpointAuth!!).build()) }
        .build()

    fun start() {
        if (server.wasSet()) {
            logger.warn("Analytics daemon is already running! Not doing anything...")
            return
        }

        logger.info("Starting the protocol server with host [0.0.0.0:{}]", config.port)
        val serverBuilder = AnalyticsServerBuilder(config.grpcBindIp ?: "127.0.0.1", config.port)
            .withServiceToken(config.serviceToken)
            .withExtension(extension)
            .withServerMetadata { metadata ->
                val info = ChartedInfo
                metadata.setDistributionType(
                    when (info.distribution) {
                        DistributionType.UNKNOWN, DistributionType.AUR -> BuildFlavour.UNRECOGNIZED
                        DistributionType.KUBERNETES -> BuildFlavour.KUBERNETES
                        DistributionType.DOCKER -> BuildFlavour.DOCKER
                        DistributionType.RPM -> BuildFlavour.RPM
                        DistributionType.DEB -> BuildFlavour.DEB
                        DistributionType.GIT -> BuildFlavour.GIT
                    },
                )

                metadata.setProductName("charted-server")
                metadata.setCommitHash(info.commitHash)
                metadata.setBuildDate(Instant.parse(info.buildDate))
                metadata.setVersion(info.version)
                metadata.setVendor("Noelware")
            }
            .withServerBuilder { builder -> builder.addService(ProtoReflectionService.newInstance()) }
            .build()

        server.value = serverBuilder
        serverBuilder.start()

        // TODO: Allow setting of bind IP, default 0.0.0.0
        val initReq = Requests.InitRequest(String.format("0.0.0.0:%s", config.port))
        val request: Request = Request.Builder()
            .post(Json.encodeToString(initReq).toRequestBody("application/json".toMediaType()))
            .url("${config.endpoint}/instances/${serverBuilder.instanceUUID()}/init")
            .build()

        httpClient.newCall(request).execute().use { resp ->
            val initApiResponse = Json.decodeFromString<ApiResponse<Requests.InitResponse>>(resp.body!!.string())
            if (initApiResponse.success) {
                val apiToken = Base64.getDecoder().decode(config.serviceToken).decodeToString().split(":")[1]
                val okRes = initApiResponse as ApiResponse.Ok<Requests.InitResponse>
                val pemReader = PemReader(StringReader(okRes.data!!.pubKey))
                val keySpec = X509EncodedKeySpec(pemReader.readPemObject().content)
                val pubKey = KeyFactory.getInstance("RSA").generatePublic(keySpec)
                val cipher = Cipher.getInstance("RSA")
                cipher.init(Cipher.ENCRYPT_MODE, pubKey)

                val encoded = Base64.getEncoder().encodeToString(cipher.doFinal(apiToken.toByteArray()))
                val finalReq: Request = Request.Builder()
                    .post(Json.encodeToString(Requests.FinalizeRequest(encoded)).toRequestBody("application/json".toMediaType()))
                    .url("${config.endpoint}/instances/${serverBuilder.instanceUUID()}/finalize")
                    .build()

                httpClient.newCall(finalReq).execute().use { res ->
                    try {
                        val decoded = Json.decodeFromString<ApiResponse<Unit>>(res.body!!.string())
                        if (!decoded.success) {
                            val errors = decoded as ApiResponse.Err
                            logger.info("Finalize request failed with status: {}, errors: {}", res.code, errors.errors)
                        }
                    } catch (_: Exception) {
                        logger.info("Response code when finalizing: {} {}", res.code, res.message)
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
