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

package org.noelware.charted.server.internal

import com.charleskorn.kaml.YamlException
import dev.floofy.utils.koin.retrieve
import dev.floofy.utils.slf4j.logging
import guru.zoroark.tegral.openapi.dsl.OpenApiVersion
import guru.zoroark.tegral.openapi.dsl.openApi
import guru.zoroark.tegral.openapi.dsl.toJson
import guru.zoroark.tegral.openapi.dsl.toYaml
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
import io.netty.util.Version
import io.sentry.Sentry
import kotlinx.serialization.SerializationException
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import org.koin.core.context.GlobalContext
import org.noelware.charted.MultiValidationException
import org.noelware.charted.ValidationException
import org.noelware.charted.common.SetOnce
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.ServerFeature
import org.noelware.charted.extensions.ifSentryEnabled
import org.noelware.charted.modules.docker.registry.RegistryKtorPlugin
import org.noelware.charted.server.ChartedServer
import org.noelware.charted.server.hasStarted
import org.noelware.charted.server.openapi.charted
import org.noelware.charted.server.plugins.Logging
import org.noelware.charted.server.plugins.RequestMdc
import org.noelware.charted.types.responses.ApiError
import org.noelware.charted.types.responses.ApiResponse
import org.noelware.ktor.NoelKtorRouting
import org.noelware.ktor.loader.koin.KoinEndpointLoader
import org.slf4j.LoggerFactory
import java.io.IOException

class DefaultChartedServer(private val config: Config): ChartedServer {
    private val _server: SetOnce<NettyApplicationEngine> = SetOnce()
    private val log by logging<DefaultChartedServer>()

    /**
     * Checks if the server has started or not.
     */
    override val started: Boolean
        get() = hasStarted.get()

    /**
     * The application engine that Ktor is using for the server.
     */
    override val server: NettyApplicationEngine
        get() = _server.value

    /**
     * Extension function to tailor the application module for this [ChartedServer]
     * instance.
     */
    override fun Application.module() {
        val self = this@DefaultChartedServer // to make this more readable to the viewer and me.

        // So you can use `HEAD https://charts.noelware.org/api` to see if it is
        // running or not.
        install(AutoHeadResponse)

        // So we can consume the body multiple times, since the request logger
        // consumes the body to see how many (in bytes) it is.
        install(DoubleReceive)

        // So we can have additional slf4j MDC properties during the lifecycle.
        install(RequestMdc)

        // Logging middleware, nothing to expect here.
        install(Logging)

        // Installs Sentry onto the middleware for tracing purposes,
        // though we also need to add APM and OpenTelemetry here.

        // Installs the Docker Registry proxy plugin if it is available
        if (config.features.contains(ServerFeature.DOCKER_REGISTRY) && config.dockerRegistry != null) {
            install(RegistryKtorPlugin)
        }

        // So we can use kotlinx.serialization for the `application/json` content type
        install(ContentNegotiation) {
            json(GlobalContext.retrieve())
        }

        // TODO(@auguwu): make this optional with `server.cors` configuration key
//        install(CORS) {
//            anyHost()
//            allowHeader(HttpHeaders.ContentType)
//            allowHeader(HttpHeaders.Authorization)
//            allowHeader(HttpHeaders.Accept)
//
//            allowCredentials = true
//            maxAgeInSeconds = 3600
//            methods += setOf(HttpMethod.Get, HttpMethod.Patch, HttpMethod.Delete, HttpMethod.Put, HttpMethod.Post)
//            headers += "X-Forwarded-Proto"
//        }

        // Adds caching and security headers (if enabled)
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

        // Adds error handling for status codes and exceptions that are
        // the most frequent.
        install(StatusPages) {
            // We have to do this to guard the content length since it can be null! If it is,
            // display a generic 404 message.
            statuses[HttpStatusCode.NotFound] = { call, content, _ ->
                if (content.contentLength == null) {
                    call.respond(
                        HttpStatusCode.NotFound,
                        ApiResponse.err(
                            "REST_HANDLER_NOT_FOUND", "Route handler was not found",
                            buildJsonObject {
                                put("method", call.request.httpMethod.value)
                                put("url", call.request.path())
                            }
                        )
                    )
                }
            }

            status(HttpStatusCode.MethodNotAllowed) { call, _ ->
                call.respond(
                    HttpStatusCode.MethodNotAllowed,
                    ApiResponse.err(
                        "INVALID_REST_HANDLER", "Route handler was not the right method",
                        buildJsonObject {
                            put("method", call.request.httpMethod.value)
                            put("url", call.request.path())
                        }
                    )
                )
            }

            status(HttpStatusCode.UnsupportedMediaType) { call, _ ->
                val header = call.request.header("Content-Type")
                call.respond(
                    HttpStatusCode.UnsupportedMediaType,
                    ApiResponse.err("UNSUPPORTED_CONTENT_TYPE", "Invalid content type [$header], was expecting \"application/json\"")
                )
            }

            status(HttpStatusCode.NotImplemented) { call, _ ->
                call.respond(
                    HttpStatusCode.MethodNotAllowed,
                    ApiResponse.err(
                        "REST_HANDLER_UNAVAILABLE", "Route handler is not implemented at this moment!",
                        buildJsonObject {
                            put("method", call.request.httpMethod.value)
                            put("url", call.request.path())
                        }
                    )
                )
            }

            exception<MultiValidationException> { call, cause ->
                ifSentryEnabled { Sentry.captureException(cause) }

                self.log.error("Received multiple validation exceptions on REST handler [${call.request.httpMethod.value} ${call.request.path()}]")
                call.respond(
                    HttpStatusCode.NotAcceptable,
                    cause.exceptions().map { ApiError("VALIDATION_EXCEPTION", it.validationMessage) }
                )
            }

            exception<ValidationException> { call, cause ->
                ifSentryEnabled { Sentry.captureException(cause) }

                self.log.error("Received an validation exception on REST handler [${call.request.httpMethod.value} ${call.request.path()}] ~> ${cause.path} [${cause.validationMessage}]")
                call.respond(
                    HttpStatusCode.NotAcceptable,
                    ApiResponse.err("VALIDATION_EXCEPTION", cause.validationMessage)
                )
            }

            exception<SerializationException> { call, cause ->
                ifSentryEnabled { Sentry.captureException(cause) }

                self.log.error("Received serialization exception in handler [${call.request.httpMethod.value} ${call.request.path()}]", cause)
                call.respond(
                    HttpStatusCode.PreconditionFailed,
                    ApiResponse.err("SERIALIZATION_FAILED", cause.message!!)
                )
            }

            exception<YamlException> { call, cause ->
                ifSentryEnabled { Sentry.captureException(cause) }

                self.log.error("Unknown YAML exception had occurred while handling request [${call.request.httpMethod.value} ${call.request.path()}]:", cause)
                call.respond(
                    HttpStatusCode.NotAcceptable,
                    ApiResponse.err(cause)
                )
            }

            exception<Exception> { call, cause ->
                ifSentryEnabled { Sentry.captureException(cause) }

                self.log.error("Unknown exception had occurred while handling request [${call.request.httpMethod.value} ${call.request.path()}]", cause)
                call.respond(
                    HttpStatusCode.InternalServerError,
                    ApiResponse.err(
                        "INTERNAL_SERVER_ERROR", cause.message ?: "(unknown)",
                        buildJsonObject {
                            if (cause.cause != null) {
                                put(
                                    "cause",
                                    buildJsonObject {
                                        put("message", cause.cause!!.message ?: "(unknown)")
                                        if (self.config.debug) {
                                            put("stacktrace", cause.cause!!.stackTraceToString())
                                        }
                                    }
                                )
                            }

                            if (self.config.debug) {
                                put("stacktrace", cause.stackTraceToString())
                            }
                        }
                    )
                )
            }
        }

        routing {
            val openapi = openApi { charted() }
            get("/openapi") {
                val format = call.request.queryParameters["format"]
                val result = if (format == null || format == "json") {
                    openapi.toJson(OpenApiVersion.V3_1)
                } else {
                    openapi.toYaml(OpenApiVersion.V3_1)
                }

                call.respondText(result, if (format == null || format == "json") { ContentType.parse("application/json; charset=utf-8") } else { ContentType.parse("text/yaml; charset=utf-8") })
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
        if (started) return

        log.info("Starting API server!")
        val self = this
        val environment = applicationEngineEnvironment {
            developmentMode = self.config.debug
            log = LoggerFactory.getLogger("org.noelware.charted.ktor.Application")

            connector {
                host = self.config.server.host
                port = self.config.server.port
            }

            module {
                module()
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

        val versions = Version.identify()
        val netty = versions[versions.keys.first()]!!
        log.info("Server is using Netty v${netty.artifactVersion()} (${netty.shortCommitHash()})")

        server.start(wait = true)
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

        log.warn("Shutting down API server...")
        server.stop()
    }
}
