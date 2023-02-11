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

package org.noelware.charted.server.internal

import dev.floofy.utils.java.SetOnce
import dev.floofy.utils.koin.retrieve
import dev.floofy.utils.slf4j.logging
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import io.ktor.server.application.*
import io.ktor.server.engine.*
import io.ktor.server.netty.*
import io.ktor.server.plugins.*
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
import org.koin.core.context.GlobalContext
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.extensions.ifSentryEnabled
import org.noelware.charted.server.ChartedServer
import org.noelware.charted.server.endpoints.v1.CdnEndpoints
import org.noelware.charted.server.hasStarted
import org.noelware.charted.server.internal.extensions.configure
import org.noelware.charted.server.plugins.Logging
import org.noelware.charted.server.plugins.RequestMdc
import org.noelware.charted.server.plugins.SentryPlugin
import org.noelware.ktor.loader.koin.KoinEndpointLoader
import org.noelware.ktor.plugin.NoelKtorRouting
import org.slf4j.LoggerFactory
import java.io.IOException

class DefaultChartedServer(private val config: Config) : ChartedServer {
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
        ifSentryEnabled {
            install(SentryPlugin)
        }

        // So we can use kotlinx.serialization for the `application/json` content type
        install(ContentNegotiation) {
            json(GlobalContext.retrieve())
        }

        install(CORS) {
            anyHost()

            exposeHeader("")

            allowHeader("X-Forwarded-Proto")
            allowHeader(HttpHeaders.Authorization)
            allowHeader(HttpHeaders.ContentType)
            allowHeader(HttpHeaders.Accept)

            for (method in setOf(HttpMethod.Get, HttpMethod.Patch, HttpMethod.Delete, HttpMethod.Put, HttpMethod.Post)) {
                allowMethod(method)
            }

            allowCredentials = true
            maxAgeInSeconds = 3600
        }

        // Adds caching and security headers (if enabled)
        install(DefaultHeaders) {
            header("Cache-Control", "public, max-age=7776000")
            if (config.server.securityHeaders) {
                header("X-Frame-Options", "deny")
                header("X-Content-Type-Options", "nosniff")
                header("X-XSS-Protection", "1; mode=block")
            }

            for ((key, value) in config.server.extraHeaders) {
                header(key, value)
            }
        }

        // Adds error handling for status codes and exceptions that are
        // the most frequent.
        install(StatusPages) {
            configure(config)
        }

        routing {}
        install(NoelKtorRouting) {
            endpointLoader = KoinEndpointLoader
            if (config.cdn != null && config.cdn!!.enabled) {
                val prefix = config.cdn!!.prefix
                assert(prefix.startsWith('/')) { "CDN endpoint must start with a trailing slash" }

                endpoints(CdnEndpoints(GlobalContext.retrieve(), prefix))
            }
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
