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

import dev.floofy.utils.slf4j.logging
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import io.ktor.server.application.*
import io.ktor.server.engine.*
import io.ktor.server.http.content.*
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
import org.noelware.charted.common.SetOnceGetValue
import org.noelware.charted.configuration.dsl.Config
import org.noelware.charted.core.DebugUtils
import org.noelware.charted.core.server.ChartedServer
import org.slf4j.LoggerFactory
import java.io.IOException
import java.util.concurrent.TimeUnit
import java.util.concurrent.atomic.AtomicBoolean
import kotlin.jvm.Throws

class DefaultChartedServer(private val config: Config): ChartedServer<NettyApplicationEngine> {
    private val _server: SetOnceGetValue<NettyApplicationEngine> = SetOnceGetValue()
    private val log by logging<DefaultChartedServer>()

    /**
     * Checks if the server has been previously started or not.
     */
    override val started: Boolean
        get() = hasStarted.get()

    /**
     * The server instance, for whatever reason you need it for.
     */
    override val server: NettyApplicationEngine
        get() = _server.value

    /**
     * Extension function to tailor the application module for this [ChartedServer]
     * instance.
     */
    override fun Application.module() {
        TODO("Not yet implemented")
    }

    /*
                    install(AutoHeadResponse)
                install(DoubleReceive)
                install(RequestMdc)
                install(Logging)

//                if (self.config.isFeatureEnabled(Feature.DOCKER_REGISTRY)) {
//                    install(DockerRegistryPlugin) {
//                        basicAuth = self.config.ociProxy!!.auth
//                        port = self.config.ociProxy!!.port
//                        host = self.config.ociProxy!!.host
//                        ssl = self.config.ociProxy!!.ssl
//                    }
//                }

                if (Sentry.isEnabled()) {
                    install(org.noelware.charted.server.plugins.Sentry)
                }

                if (self.config.tracing.apm != null) {
                    install(APM)
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

                        call.apmTransaction?.captureException(cause)
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

                        call.apmTransaction?.captureException(cause)
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

                        call.apmTransaction?.captureException(cause)
                        self.log.error("Unknown exception had occurred while handling request [${call.request.httpMethod.value} ${call.request.path()}]", cause)
                        call.respond(
                            HttpStatusCode.InternalServerError,
                            Response.err(cause)
                        )
                    }
                }

                routing {
                    val storage: StorageWrapper by inject()
                    if (self.config.cdn) {
                        runBlocking {
                            proxyStorageTrailer(storage)
                        }

                        val scheduler: Scheduler by inject()
                        scheduler.schedule(
                            ReconfigureProxyCdnJob(self, GlobalContext.retrieve(), self.config),
                            true
                        )
                    }
                }

                install(NoelKtorRouting) {
                    endpointLoader(KoinEndpointLoader)
                }
     */

    /**
     * Starts the server, this will be a no-op if [started] was already
     * set to `true`.
     */
    override fun start() {
        log.info("Starting API server...")

        val self = this
        val environment = applicationEngineEnvironment {
            developmentMode = DebugUtils.isDebugEnabled(self.config)
            log = LoggerFactory.getLogger("org.noelware.charted.ktor.Application")

            connector {
                host = self.config.server.host
                port = self.config.server.port
            }

            module { module() }
        }

        _server.value = embeddedServer(Netty, environment, configure = {
            requestQueueLimit = config.server.requestQueueLimit
            runningLimit = config.server.runningLimit
            shareWorkGroup = config.server.shareWorkGroup
            responseWriteTimeoutSeconds = config.server.responseWriteTimeoutSeconds
            requestReadTimeoutSeconds = config.server.requestReadTimeout
            tcpKeepAlive = config.server.tcpKeepAlive
        })

        val versions = Version.identify()
        val netty = versions[versions.keys.first()]!!
        log.info("Server is using Netty v${netty.artifactVersion()} (commit ${netty.shortCommitHash()}) [${netty.artifactId()}]")

        server.start(true)
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
     * @throws java.io.IOException if an I/O error occurs
     */
    @Throws(IOException::class)
    override fun close() {
        if (!started) return

        log.warn("Shutting down server...")
        server.stop(500, 10, TimeUnit.SECONDS)
    }

    companion object {
        val hasStarted: AtomicBoolean = AtomicBoolean(false)
        val bootTime: Long = System.nanoTime()
    }
}
