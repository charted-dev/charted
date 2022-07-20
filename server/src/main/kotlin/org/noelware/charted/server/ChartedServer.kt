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

package org.noelware.charted.server

import com.charleskorn.kaml.YamlException
import dev.floofy.haru.Scheduler
import dev.floofy.utils.koin.inject
import dev.floofy.utils.koin.retrieve
import dev.floofy.utils.kotlin.ifNotNull
import dev.floofy.utils.kotlin.sizeToStr
import dev.floofy.utils.slf4j.logging
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import io.ktor.server.application.*
import io.ktor.server.engine.*
import io.ktor.server.netty.*
import io.ktor.server.plugins.autohead.*
import io.ktor.server.plugins.contentnegotiation.*
import io.ktor.server.plugins.cors.routing.*
import io.ktor.server.plugins.defaultheaders.*
import io.ktor.server.plugins.doublereceive.*
import io.ktor.server.plugins.statuspages.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.sentry.Sentry
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.debug.DebugProbes
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.SerializationException
import kotlinx.serialization.json.*
import org.koin.core.context.GlobalContext
import org.noelware.charted.common.ChartedInfo
import org.noelware.charted.common.SetOnceGetValue
import org.noelware.charted.common.data.Config
import org.noelware.charted.common.data.Feature
import org.noelware.charted.common.exceptions.ValidationException
import org.noelware.charted.features.docker.registry.DockerRegistryPlugin
import org.noelware.charted.server.endpoints.proxyStorageTrailer
import org.noelware.charted.server.jobs.ReconfigureProxyCdnJob
import org.noelware.charted.server.plugins.Logging
import org.noelware.charted.server.plugins.RequestMdc
import org.noelware.ktor.NoelKtorRouting
import org.noelware.ktor.loader.koin.KoinEndpointLoader
import org.noelware.remi.filesystem.ifExists
import org.slf4j.LoggerFactory
import java.io.File
import java.lang.management.ManagementFactory
import java.security.KeyStore
import io.sentry.Sentry as SentryClient

class ChartedServer(private val config: Config) {
    companion object {
        val bootTime = System.currentTimeMillis()
        val hasStarted: SetOnceGetValue<Boolean> = SetOnceGetValue()
    }

    lateinit var server: NettyApplicationEngine
    private val log by logging<ChartedServer>()

    @OptIn(ExperimentalCoroutinesApi::class)
    fun start() {
        val runtime = Runtime.getRuntime()
        val os = ManagementFactory.getOperatingSystemMXBean()
        val threads = ManagementFactory.getThreadMXBean()

        log.info("Runtime Information:")
        log.info("===> Free / Total Memory [Max]: ${runtime.freeMemory().sizeToStr()}/${runtime.totalMemory().sizeToStr()} [${runtime.maxMemory().sizeToStr()}]")
        log.info("===> Operating System: ${os.name.lowercase()}${os.arch} (${os.availableProcessors} processors)")
        log.info("===> OS Threads: ${threads.threadCount} (${threads.daemonThreadCount} background threads)")
        if (ChartedInfo.dedicatedNode != null) {
            log.info("===> Dedicated Node: ${ChartedInfo.dedicatedNode}")
        }

        log.info("===> Library Versions:")
        log.info("|-  charted: ${ChartedInfo.version} [${ChartedInfo.commitHash}]")
        log.info("|-  Kotlin:  ${KotlinVersion.CURRENT}")
        log.info("|-  Java:    ${System.getProperty("java.version", "Unknown")} [${System.getProperty("java.vendor", "Unknown")}]")

        if (config.debug) {
            log.info("Enabling kotlinx.coroutines debug probe...")
            DebugProbes.install()
        }

        val self = this
        val environment = applicationEngineEnvironment {
            developmentMode = self.config.debug
            log = LoggerFactory.getLogger("org.noelware.charted.server.KtorApplication")

            connector {
                host = self.config.server.host
                port = self.config.server.port
            }

            module {
                install(AutoHeadResponse)
                install(DoubleReceive)
                install(RequestMdc)
                install(Logging)

                if (self.config.isFeatureEnabled(Feature.DOCKER_REGISTRY)) {
                    install(DockerRegistryPlugin) {
                        basicAuth = self.config.ociProxy!!.auth
                        port = self.config.ociProxy!!.port
                        host = self.config.ociProxy!!.host
                        ssl = self.config.ociProxy!!.ssl
                    }
                }

                if (Sentry.isEnabled()) {
                    install(org.noelware.charted.server.plugins.Sentry)
                }

                install(ContentNegotiation) {
                    json(GlobalContext.retrieve())
                }

                install(CORS) {
                    anyHost()
                    allowHeader(HttpHeaders.ContentType)
                    allowHeader(HttpHeaders.Authorization)
                    allowHeader(HttpHeaders.Accept)

                    allowCredentials = true
                    maxAgeInSeconds = 3600
                    methods += setOf(HttpMethod.Get, HttpMethod.Patch, HttpMethod.Delete, HttpMethod.Put, HttpMethod.Post)
                    headers += "X-Forwarded-Proto"
                }

                install(DefaultHeaders) {
                    header("Cache-Control", "public, max-age=7776000")
                    if (self.config.server.securityHeaders) {
                        header("X-Frame-Options", "deny")
                        header("X-Content-Type-Options", "nosniff")
                        header("X-XSS-Protection", "1; mode=block")
                    }

                    for ((key, value) in self.config.server.extraHeaders) {
                        header(key, value)
                    }
                }

                install(StatusPages) {
                    statuses[HttpStatusCode.NotFound] = { call, content, _ ->
                        if (content.contentLength == null) {
                            call.respond(
                                HttpStatusCode.NotFound,
                                buildJsonObject {
                                    put("success", false)
                                    putJsonArray("errors") {
                                        addJsonObject {
                                            put("code", "NOT_FOUND")
                                            put("message", "Route ${call.request.httpMethod.value} ${call.request.uri} was not found.")
                                        }
                                    }
                                }
                            )
                        }
                    }

                    status(HttpStatusCode.MethodNotAllowed) { call, _ ->
                        call.respond(
                            HttpStatusCode.MethodNotAllowed,
                            buildJsonObject {
                                put("success", false)
                                putJsonArray("errors") {
                                    addJsonObject {
                                        put("code", "INVALID_ROUTE")
                                        put("message", "Route ${call.request.httpMethod.value} ${call.request.uri} doesn't implement a handler for that specific method.")
                                    }
                                }
                            }
                        )
                    }

                    exception<SerializationException> { call, cause ->
                        if (SentryClient.isEnabled()) {
                            SentryClient.captureException(cause)
                        }

                        self.log.error("Serialization exception had occurred while handling request [Unable to handle request ${call.request.httpMethod.value} ${call.request.uri}]:", cause)
                        call.respond(
                            HttpStatusCode.NotAcceptable,
                            buildJsonObject {
                                put("success", false)
                                putJsonArray("errors") {
                                    addJsonObject {
                                        put("message", cause.message!!)
                                        put("code", "SERIALIZATION_ERROR")
                                        if (self.config.debug) {
                                            put("stacktrace", cause.stackTraceToString())

                                            cause.cause.ifNotNull {
                                                putJsonObject("caused") {
                                                    put("message", message)
                                                    put("stacktrace", stackTraceToString())
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        )
                    }

                    exception<ValidationException> { call, cause ->
                        if (SentryClient.isEnabled()) {
                            SentryClient.captureException(cause)
                        }

                        self.log.error("Received validation exception in route [${call.request.httpMethod.value} ${call.request.path()}] ${cause.path} [${cause.validationMessage}]")
                        call.respond(
                            HttpStatusCode.NotAcceptable,
                            buildJsonObject {
                                put("success", false)
                                putJsonArray("errors") {
                                    addJsonObject {
                                        put("message", cause.validationMessage)
                                        put("code", "VALIDATION_EXCEPTION")
                                    }
                                }
                            }
                        )
                    }

                    exception<YamlException> { call, cause ->
                        if (SentryClient.isEnabled()) {
                            SentryClient.captureException(cause)
                        }

                        self.log.error("Unknown YAML exception had occurred while handling request [${call.request.httpMethod.value} ${call.request.path()}]:", cause)
                        call.respond(
                            HttpStatusCode.NotAcceptable,
                            buildJsonObject {
                                put("success", false)
                                putJsonArray("errors") {
                                    addJsonObject {
                                        put("code", "INVALID_YAML")
                                        put("message", cause.message)
                                        if (self.config.debug) {
                                            cause.cause.ifNotNull {
                                                putJsonObject("cause") {
                                                    put("message", message)
                                                    put("stacktrace", stackTraceToString())
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        )
                    }

                    exception<Exception> { call, cause ->
                        if (SentryClient.isEnabled()) {
                            SentryClient.captureException(cause)
                        }

                        self.log.error("Unknown exception had occurred while handling request [${call.request.httpMethod.value} ${call.request.path()}]", cause)
                        call.respond(
                            HttpStatusCode.InternalServerError,
                            buildJsonObject {
                                put("success", false)
                                putJsonArray("errors") {
                                    addJsonObject {
                                        put("message", "Unknown exception had occurred.")
                                        put("code", "UNKNOWN_EXCEPTION")
                                        if (self.config.debug) {
                                            put("message", cause.message)
                                            put("stacktrace", cause.stackTraceToString())

                                            cause.cause.ifNotNull {
                                                putJsonObject("cause") {
                                                    put("message", message)
                                                    put("stacktrace", stackTraceToString())
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        )
                    }
                }

                routing {
                    if (self.config.cdn.proxyContents) {
                        runBlocking {
                            proxyStorageTrailer(GlobalContext.retrieve(), self.config)
                        }

                        val scheduler: Scheduler by inject()
                        scheduler.schedule(ReconfigureProxyCdnJob(self, GlobalContext.retrieve(), self.config), start = true)
                    }
                }

                install(NoelKtorRouting) {
                    endpointLoader(KoinEndpointLoader)
                }
            }

            if (self.config.ssl != null) {
                self.log.info("Received SSL configuration! Loading Keystore...")
                val file = File(self.config.ssl!!.path)
                val keystore = file.ifExists {
                    self.log.info("SSL keystore exists in path ${self.config.ssl!!.path}!")
                    val inputStream = inputStream()
                    val keystore = KeyStore.getInstance("JKS")
                    keystore.load(inputStream, self.config.ssl!!.password.toCharArray())

                    keystore
                } ?: error("Keystore path ${self.config.ssl!!.path} doesn't exist.")

                sslConnector(keystore, "charted.ssl", { self.config.ssl!!.password.toCharArray() }, { self.config.ssl!!.password.toCharArray() }, {})
            }
        }

        server = embeddedServer(Netty, environment, configure = {
            requestQueueLimit = config.server.requestQueueLimit
            runningLimit = config.server.runningLimit
            shareWorkGroup = config.server.shareWorkGroup
            responseWriteTimeoutSeconds = config.server.responseWriteTimeoutSeconds
            requestReadTimeoutSeconds = config.server.requestReadTimeout
            tcpKeepAlive = config.server.tcpKeepAlive
        })

        server.start(wait = true)
    }

    fun destroy() {
        if (!::server.isInitialized) return
        server.stop()
    }
}
