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
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.core

import app.softwork.ratelimit.RateLimit
import dev.floofy.utils.koin.retrieve
import dev.floofy.utils.kotlin.sizeToStr
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
import io.ktor.server.plugins.statuspages.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.sentry.Sentry
import kotlinx.serialization.json.buildJsonArray
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import org.koin.core.context.GlobalContext
import org.noelware.charted.analytics.AnalyticsDaemonApplication
import org.noelware.charted.core.config.AnalyticsConfig
import org.noelware.charted.core.config.Config
import org.noelware.charted.core.plugins.KtorLogging
import org.noelware.charted.core.plugins.UserAgentMdc
import org.noelware.charted.core.ratelimit.LettuceRedisStorage
import org.noelware.ktor.NoelKtorRoutingPlugin
import org.noelware.ktor.loader.koin.KoinEndpointLoader
import org.slf4j.LoggerFactory
import org.springframework.boot.Banner
import org.springframework.boot.SpringApplication
import org.springframework.boot.WebApplicationType
import org.springframework.boot.context.event.ApplicationEnvironmentPreparedEvent
import org.springframework.boot.context.event.ApplicationFailedEvent
import org.springframework.context.ApplicationListener
import java.lang.management.ManagementFactory
import java.util.concurrent.ExecutorService
import java.util.concurrent.Executors
import kotlin.time.Duration.Companion.hours

/**
 * Represents the server that is bootstrapped.
 */
class ChartedServer {
    companion object {
        val executorPool: ExecutorService = Executors.newCachedThreadPool(createThreadFactory("ChartedServer-Executor"))
    }

    private lateinit var server: NettyApplicationEngine
    private val log by logging<ChartedServer>()

    private fun startSpringServer(config: AnalyticsConfig) {
        log.debug("Analytics daemon was enabled, so starting daemon server!")

        val app = SpringApplication(AnalyticsDaemonApplication::class.java)
        app.webApplicationType = WebApplicationType.SERVLET
        app.setBannerMode(Banner.Mode.OFF) // we don't need to print this!

        app.addListeners(
            ApplicationListener { event: Any ->
                if (event is ApplicationEnvironmentPreparedEvent) {
                    log.debug("Daemon servlet has been bootstrapped successfully.")
                }
            },

            ApplicationListener { event: Any ->
                if (event is ApplicationFailedEvent) {
                    log.error("FATAL: Spring Boot application failed to start:", event.exception)
                }
            }
        )

        app.run(
            *listOf(
                "--server.port=${config.port}",
                "--spring.application.name=Noelware Analytics Daemon Service"
            ).toTypedArray()
        )
    }

    suspend fun start() {
        val runtime = Runtime.getRuntime()
        val os = ManagementFactory.getOperatingSystemMXBean()
        val threads = ManagementFactory.getThreadMXBean()

        log.info("Runtime Information:")
        log.info("  * Free / Total Memory [Max]: ${runtime.freeMemory().sizeToStr()}/${runtime.totalMemory().sizeToStr()} [${runtime.maxMemory().sizeToStr()}]")
        log.info("  * Threads: ${threads.threadCount} (${threads.daemonThreadCount} background threads)")
        log.info("  * Operating System: ${os.name} with ${os.availableProcessors} processors (${os.arch}; ${os.version})")
        log.info("  * Versions:")
        log.info("      * JVM [JRE]: v${System.getProperty("java.version", "Unknown")} (${System.getProperty("java.vendor", "Unknown")}) [${Runtime.version()}]")
        log.info("      * Kotlin:    v${KotlinVersion.CURRENT}")
        log.info("      * charted:   v${ChartedInfo.version} (${ChartedInfo.commitHash} -- ${ChartedInfo.buildDate})")

        if (ChartedInfo.dediNode != null)
            log.info("  * Dedicated Node: ${ChartedInfo.dediNode}")

        val config: Config = GlobalContext.retrieve()
        val self = this

        val environment = applicationEngineEnvironment {
            developmentMode = System.getProperty("org.noelware.charted.debug", "false") == "true"
            log = LoggerFactory.getLogger("org.noelware.charted.server.KtorApplicationEnvironmentKt")

            connector {
                host = config.server.host
                port = config.server.port.toInt()
            }

            module {
                install(AutoHeadResponse)
                install(KtorLogging)
                install(UserAgentMdc)
                install(ContentNegotiation) {
                    json(GlobalContext.retrieve())
                }

                install(CORS) {
                    anyHost()
                    headers += "X-Forwarded-Proto"
                }

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

                install(RateLimit(LettuceRedisStorage())) {
                    limit = 1000
                    timeout = 1.hours

                    host { call ->
                        var ip = ""
                        val trueClientIpHeader = call.request.header("True-Client-IP")
                        if (trueClientIpHeader != null)
                            ip = trueClientIpHeader

                        val realIpHeader = call.request.header("X-Real-IP")
                        if (realIpHeader != null)
                            ip = realIpHeader

                        val xForwardedFor = call.request.header("X-Forwarded-For")
                        if (xForwardedFor != null) {
                            var i = xForwardedFor.indexOf(',')
                            if (i == -1) {
                                i = xForwardedFor.length
                            }

                            ip = xForwardedFor.slice(0..i)
                        }

                        ip.ifEmpty { call.request.origin.remoteHost }
                    }
                }

                if (Sentry.isEnabled()) {
                    install(org.noelware.charted.core.plugins.Sentry)
                }

                install(StatusPages) {
                    status(HttpStatusCode.NotFound) { call, _ ->
                        call.respond(
                            HttpStatusCode.NotFound,
                            buildJsonObject {
                                put("success", false)
                                put(
                                    "errors",
                                    buildJsonArray {
                                        add(
                                            buildJsonObject {
                                                put("code", "UNKNOWN_ROUTE")
                                                put("message", "Route ${call.request.httpMethod.value} ${call.request.uri} was not found.")
                                            }
                                        )
                                    }
                                )
                            }
                        )
                    }

                    status(HttpStatusCode.MethodNotAllowed) { call, _ ->
                        call.respond(
                            HttpStatusCode.MethodNotAllowed,
                            buildJsonObject {
                                put("success", false)
                                put(
                                    "errors",
                                    buildJsonArray {
                                        add(
                                            buildJsonObject {
                                                put("code", "INVALID_METHOD")
                                                put("message", "Route ${call.request.httpMethod.value} ${call.request.uri} doesn't implement a handler for that specific method.")
                                            }
                                        )
                                    }
                                )
                            }
                        )
                    }

                    exception<Exception> { call, cause ->
                        if (Sentry.isEnabled()) {
                            Sentry.captureException(cause)
                        }

                        self.log.error("Unable to handle request ${call.request.httpMethod.value} ${call.request.uri}:", cause)
                        call.respond(
                            HttpStatusCode.InternalServerError,
                            buildJsonObject {
                                put("success", false)
                                put(
                                    "errors",
                                    buildJsonArray {
                                        add(
                                            buildJsonObject {
                                                put("message", "Unknown exception has occurred")
                                                put("code", "INTERNAL_SERVER_ERROR")
                                            }
                                        )
                                    }
                                )
                            }
                        )
                    }
                }

                routing {}

                install(NoelKtorRoutingPlugin) {
                    endpointLoader(KoinEndpointLoader)
                }
            }
        }

        server = embeddedServer(Netty, environment, configure = {
            requestQueueLimit = config.server.requestQueueLimit.toInt()
            runningLimit = config.server.runningLimit.toInt()
            shareWorkGroup = config.server.shareWorkGroup
            responseWriteTimeoutSeconds = config.server.responseWriteTimeoutSeconds.toInt()
            requestReadTimeoutSeconds = config.server.requestReadTimeout.toInt()
            tcpKeepAlive = config.server.tcpKeepAlive
        })

        // we have to do this before we call the actual rest server
        // so it doesn't block this coroutine
        if (config.analytics != null) {
            ChartedScope.launch {
                startSpringServer(config.analytics)
            }
        }

        server.start(wait = true)
    }

    fun destroy() {
        if (!::server.isInitialized) return

        log.warn("Destroying server...")
        server.stop()
    }
}
