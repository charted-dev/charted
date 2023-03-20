/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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
import dev.floofy.utils.koin.inject
import dev.floofy.utils.koin.retrieve
import dev.floofy.utils.koin.retrieveAll
import dev.floofy.utils.slf4j.logging
import io.ktor.http.*
import io.ktor.http.content.*
import io.ktor.serialization.kotlinx.json.*
import io.ktor.server.application.*
import io.ktor.server.engine.*
import io.ktor.server.netty.*
import io.ktor.server.plugins.autohead.*
import io.ktor.server.plugins.contentnegotiation.*
import io.ktor.server.plugins.defaultheaders.*
import io.ktor.server.plugins.doublereceive.*
import io.ktor.server.plugins.ratelimit.*
import io.ktor.server.plugins.statuspages.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.netty.util.Version
import io.swagger.v3.oas.models.OpenAPI
import kotlinx.atomicfu.AtomicBoolean
import kotlinx.atomicfu.atomic
import org.koin.core.context.GlobalContext
import org.noelware.charted.Server
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.server.KtorRateLimitBackend
import org.noelware.charted.modules.openapi.toJson
import org.noelware.charted.modules.openapi.toYaml
import org.noelware.charted.server.extensions.realIP
import org.noelware.charted.server.internal.statuspages.configure
import org.noelware.charted.server.plugins.Log
import org.noelware.charted.server.ratelimit.InMemoryRateLimiter
import org.noelware.charted.server.ratelimit.RedisRateLimiter
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController
import org.noelware.charted.server.routing.v1.CdnRestController
import org.slf4j.LoggerFactory
import kotlin.time.DurationUnit

/** Returns a [AtomicBoolean] of if the server has started. */
val hasStarted: AtomicBoolean = atomic(false)

/** The boot time (in nanoseconds) */
val bootTime: Long = System.nanoTime()

class DefaultServer(private val config: Config): Server {
    internal val requestsHandled = atomic(0L)

    val requests: Long get() = requestsHandled.value
    private val log by logging<DefaultServer>()

    private val _server: SetOnce<NettyApplicationEngine> = SetOnce()
    override val started: Boolean get() = hasStarted.value

    override fun Application.module() {
        val log = this@DefaultServer.log

        // Allows `HEAD` requests to pass through
        install(AutoHeadResponse)

        // So we can consume the body multiple times, since the request logger
        // consumes the body to see how many (in bytes) it is.
        install(DoubleReceive)

        // Logging middleware, nothing to expect here.
        install(Log)

        // So we can use kotlinx.serialization for the `application/json` content type
        install(ContentNegotiation) {
            json(GlobalContext.retrieve())
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

        if (config.server.rateLimit != null) {
            val rateLimitConfig = config.server.rateLimit!!
            install(RateLimit) {
                register(RateLimitName("authenticated")) {
                    requestKey { call -> call.realIP }
                    rateLimiter { _, key ->
                        when (config.server.rateLimit!!.backend) {
                            KtorRateLimitBackend.InMemory -> InMemoryRateLimiter(
                                "authenticated", key as String,
                                rateLimitConfig.timeWindow.toDuration(
                                    DurationUnit.MILLISECONDS,
                                ),
                            )
                            KtorRateLimitBackend.Redis -> RedisRateLimiter(GlobalContext.retrieve(), GlobalContext.retrieve(), key as String, "authenticated", rateLimitConfig.timeWindow.toDuration(DurationUnit.MILLISECONDS))
                            else -> throw IllegalStateException("Backend must be [InMemoryRateLimiter] or [RedisRateLimiter]")
                        }
                    }
                }

                global {
                    requestKey { call -> call.realIP }
                    rateLimiter { _, key ->
                        when (config.server.rateLimit!!.backend) {
                            KtorRateLimitBackend.InMemory -> InMemoryRateLimiter("authenticated", key as String, rateLimitConfig.timeWindow.toDuration(DurationUnit.MILLISECONDS))
                            KtorRateLimitBackend.Redis -> RedisRateLimiter(GlobalContext.retrieve(), GlobalContext.retrieve(), key as String, "authenticated", rateLimitConfig.timeWindow.toDuration(DurationUnit.MILLISECONDS))
                            else -> throw IllegalStateException("Backend must be [InMemoryRateLimiter] or [RedisRateLimiter]")
                        }
                    }
                }
            }
        }

        val openapiDoc: OpenAPI by inject()
        val controllers: List<RestController> = GlobalContext.retrieveAll()

        routing {
            for (controller in controllers) {
                log.trace("Configuring REST controller [${controller.method.value} ${controller.path}] (${controller::class.toString().replace("class ", "")})")
                if (APIVersion.default() == controller.apiVersion) {
                    route(controller.path, controller.method) {
                        controller.initRoute(this)
                        handle { controller.call(call) }
                    }
                }

                route("${controller.apiVersion.toRoutePath()}${controller.path}", controller.method) {
                    controller.initRoute(this)
                    handle { controller.call(call) }
                }
            }

            if (config.cdn?.enabled == true) {
                val controller = CdnRestController(config, GlobalContext.retrieve())
                route(controller.path, controller.method) {
                    controller.initRoute(this)
                    handle { controller.call(call) }
                }
            }

            get("/_/openapi") {
                val pretty = call.request.queryParameters["pretty"] != null
                val format = when (call.request.queryParameters["format"]) {
                    "json" -> "json"
                    "yaml" -> "yaml"
                    else -> "json"
                }

                val (contentType, doc) = if (format == "json") {
                    ContentType.Application.Json to openapiDoc.toJson(pretty)
                } else {
                    ContentType.parse("text/yaml; charset=utf-8") to openapiDoc.toYaml()
                }

                call.response.header(HttpHeaders.ContentType, contentType.toString())
                call.respond(HttpStatusCode.OK, doc)
            }
        }
    }

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

        _server.value.start(wait = true)
    }

    override fun close() {
        if (!started) return

        log.warn("Shutting down API server...")
        _server.value.stop()
    }
}
