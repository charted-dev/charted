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
import io.sentry.Sentry
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.withContext
import kotlinx.serialization.SerializationException
import org.koin.core.context.GlobalContext
import org.noelware.charted.common.SetOnceGetValue
import org.noelware.charted.common.data.responses.Response
import org.noelware.charted.common.exceptions.ValidationException
import org.noelware.charted.common.extensions.ifSentryEnabled
import org.noelware.charted.configuration.dsl.Config
import org.noelware.charted.core.DebugUtils
import org.noelware.charted.core.StorageWrapper
import org.noelware.charted.core.server.ChartedServer
import org.noelware.charted.server.endpoints.proxyStorageTrailer
import org.noelware.charted.server.jobs.ReconfigureProxyCdnJob
import org.noelware.charted.server.plugins.Logging
import org.noelware.charted.server.plugins.RequestMdc
import org.noelware.charted.server.utils.createOutgoingContentWithBytes
import org.noelware.charted.tracing.apm.APM
import org.noelware.charted.tracing.apm.apmTransaction
import org.noelware.ktor.NoelKtorRouting
import org.noelware.ktor.loader.koin.KoinEndpointLoader
import org.slf4j.LoggerFactory
import java.io.IOException
import java.util.concurrent.atomic.AtomicBoolean

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
        val self = this@DefaultChartedServer // to make it more readable

        install(AutoHeadResponse)
        install(DoubleReceive)
        install(RequestMdc)
        install(Logging)

        if (Sentry.isEnabled()) install(org.noelware.charted.server.plugins.Sentry)
        if (self.config.tracing.apm != null) install(APM)

        install(ContentNegotiation) {
            json(GlobalContext.retrieve())
        }

        // TODO: make this optional with `server.cors`
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
            // We have to do this to guard the content length since it can be null! If it is,
            // display a generic 404 message.
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

            // The server only supports application/json for most REST endpoints!
            status(HttpStatusCode.UnsupportedMediaType) { call, _ ->
                val header = call.request.header(HttpHeaders.ContentType)
                call.respond(
                    HttpStatusCode.MethodNotAllowed,
                    Response.err("UNSUPPORTED_MEDIA_TYPE", "Invalid content type [$header], expecting application/json")
                )
            }

            status(HttpStatusCode.NotImplemented) { call, _ ->
                call.respond(
                    HttpStatusCode.NotAcceptable,
                    Response.err("NOT_IMPLEMENTED", "REST handler ${call.request.httpMethod.value} ${call.request.path()} is not implemented!")
                )
            }

            exception<ValidationException> { call, cause ->
                ifSentryEnabled { Sentry.captureException(cause) }

                self.log.error("Received validation exception(s) in route [${call.request.httpMethod.value} ${call.request.path()}] ~> ${cause.path} [${cause.validationMessage}]")
                call.apmTransaction?.captureException(cause)
                call.respond(
                    HttpStatusCode.NotAcceptable,
                    Response.err("VALIDATION_EXCEPTION", cause.validationMessage)
                )
            }

            exception<SerializationException> { call, cause ->
                ifSentryEnabled { Sentry.captureException(cause) }

                self.log.error("Received serialization exception in handler [${call.request.httpMethod.value} ${call.request.path()}]", cause)
                call.apmTransaction?.captureException(cause)
                call.respond(
                    HttpStatusCode.PreconditionFailed,
                    Response.err("SERIALIZATION_FAILED", cause.message!!)
                )
            }

            exception<YamlException> { call, cause ->
                ifSentryEnabled { Sentry.captureException(cause) }

                self.log.error("Unknown YAML exception had occurred while handling request [${call.request.httpMethod.value} ${call.request.path()}]:", cause)
                call.apmTransaction?.captureException(cause)
                call.respond(
                    HttpStatusCode.NotAcceptable,
                    Response.err(cause)
                )
            }

            exception<Exception> { call, cause ->
                ifSentryEnabled { Sentry.captureException(cause) }

                self.log.error("Unknown exception had occurred while handling request [${call.request.httpMethod.value} ${call.request.path()}]", cause)
                call.apmTransaction?.captureException(cause)
                call.respond(
                    HttpStatusCode.InternalServerError,
                    Response.err(cause)
                )
            }
        }

        routing {
            get("/openapi.json") {
                val resource = withContext(Dispatchers.IO) {
                    self::class.java.getResource("/openapi/openapi.json")!!.openStream()
                }

                val bytes = resource.use { it.readBytes() }
                call.respond(HttpStatusCode.OK, createOutgoingContentWithBytes(bytes, ContentType.Application.Json))
            }

            get("/openapi.yaml") {
                val resource = withContext(Dispatchers.IO) {
                    self::class.java.getResource("/openapi/openapi.yaml")!!.openStream()
                }

                val bytes = resource.use { it.readBytes() }
                call.respond(HttpStatusCode.OK, createOutgoingContentWithBytes(bytes, ContentType.parse("text/plain; charset=utf-8")))
            }

            val storage: StorageWrapper by inject()
            if (self.config.cdn) {
                runBlocking { proxyStorageTrailer(storage) }
            }
        }

        install(NoelKtorRouting) {
            endpointLoader(KoinEndpointLoader)
        }
    }

    /**
     * Starts the server, this will be a no-op if [started] was already
     * set to `true`.
     */
    override fun start() {
        log.info("Starting API server...")

        if (config.cdn) {
            val scheduler: Scheduler by inject()
            scheduler.schedule(
                ReconfigureProxyCdnJob(this, GlobalContext.retrieve(), config),
                true
            )
        }

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
        server.stop()
    }

    companion object {
        val hasStarted: AtomicBoolean = AtomicBoolean(false)
        val bootTime: Long = System.nanoTime()
    }
}
