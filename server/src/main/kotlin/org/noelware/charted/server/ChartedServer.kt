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
import dev.floofy.utils.kotlin.humanize
import dev.floofy.utils.slf4j.logging
import io.ktor.http.*
import io.ktor.serialization.kotlinx.*
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
import io.netty.util.Version
import io.sentry.Sentry
import kotlinx.coroutines.Job
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.*
import org.koin.core.context.GlobalContext
import org.noelware.charted.analytics.AnalyticsServer
import org.noelware.charted.common.ChartedScope
import org.noelware.charted.common.SetOnceGetValue
import org.noelware.charted.common.data.Config
import org.noelware.charted.common.data.Feature
import org.noelware.charted.common.data.responses.Response
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
import java.io.Closeable
import java.io.File
import java.security.KeyStore
import io.sentry.Sentry as SentryClient

class ChartedServer(private val config: Config, private val analytics: AnalyticsServer? = null): Closeable {
    companion object {
        val bootTime = System.currentTimeMillis()
        val hasStarted: SetOnceGetValue<Boolean> = SetOnceGetValue()
    }

    private lateinit var analyticsJob: Job
    private val _server: SetOnceGetValue<NettyApplicationEngine> = SetOnceGetValue()
    private val log by logging<ChartedServer>()

    val server: NettyApplicationEngine
        get() = _server.value

    fun start() {
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
                                Response.err("NOT_FOUND", "Route ${call.request.httpMethod.value} ${call.request.path()} was not found")
                            )
                        }
                    }

                    status(HttpStatusCode.MethodNotAllowed) { call, _ ->
                        call.respond(
                            HttpStatusCode.MethodNotAllowed,
                            Response.err(
                                "INVALID_ROUTE",
                                "Route ${call.request.httpMethod.value} ${call.request.path()} doesn't have a REST handler"
                            )
                        )
                    }

                    status(HttpStatusCode.UnsupportedMediaType) { call, _ ->
                        val header = call.request.header(HttpHeaders.ContentType)
                        call.respond(
                            HttpStatusCode.MethodNotAllowed,
                            Response.err("UNSUPPORTED_MEDIA_TYPE", "Invalid content type [$header], expecting application/json")
                        )
                    }

                    exception<ValidationException> { call, cause ->
                        if (SentryClient.isEnabled()) {
                            SentryClient.captureException(cause)
                        }

                        self.log.error("Received validation exception in route [${call.request.httpMethod.value} ${call.request.path()}] ${cause.path} [${cause.validationMessage}]")
                        call.respond(
                            HttpStatusCode.NotAcceptable,
                            Response.err("VALIDATION_EXCEPTION", cause.validationMessage)
                        )
                    }

                    exception<YamlException> { call, cause ->
                        if (SentryClient.isEnabled()) {
                            SentryClient.captureException(cause)
                        }

                        self.log.error("Unknown YAML exception had occurred while handling request [${call.request.httpMethod.value} ${call.request.path()}]:", cause)
                        call.respond(
                            HttpStatusCode.NotAcceptable,
                            Response.err(cause)
                        )
                    }

                    exception<Exception> { call, cause ->
                        if (SentryClient.isEnabled()) {
                            SentryClient.captureException(cause)
                        }

                        self.log.error("Unknown exception had occurred while handling request [${call.request.httpMethod.value} ${call.request.path()}]", cause)
                        call.respond(
                            HttpStatusCode.InternalServerError,
                            Response.err(cause)
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

        _server.value = embeddedServer(Netty, environment, configure = {
            requestQueueLimit = config.server.requestQueueLimit
            runningLimit = config.server.runningLimit
            shareWorkGroup = config.server.shareWorkGroup
            responseWriteTimeoutSeconds = config.server.responseWriteTimeoutSeconds
            requestReadTimeoutSeconds = config.server.requestReadTimeout
            tcpKeepAlive = config.server.tcpKeepAlive
        })

        if (analytics != null) {
            analyticsJob = ChartedScope.launch {
                analytics.launch()
            }
        }

        val versions = Version.identify()
        val version = versions[versions.keys.first()]!!

        log.info("Using Netty v${version.artifactVersion()} [${version.shortCommitHash()}, ${(System.currentTimeMillis() - version.buildTimeMillis()).humanize(true)} ago]")
        server.start(wait = true)
    }

    override fun close() {
        if (!_server.wasSet()) return
        if (::analyticsJob.isInitialized) {
            analyticsJob.cancel()
            analytics?.close()
        }

        server.stop()
    }
}
