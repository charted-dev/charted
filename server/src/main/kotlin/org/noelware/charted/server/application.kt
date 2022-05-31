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

package org.noelware.charted.server

import dev.floofy.utils.koin.retrieve
import dev.floofy.utils.kotlin.ifNotNull
import dev.floofy.utils.slf4j.logging
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import io.ktor.server.application.*
import io.ktor.server.plugins.autohead.*
import io.ktor.server.plugins.contentnegotiation.*
import io.ktor.server.plugins.cors.routing.*
import io.ktor.server.plugins.defaultheaders.*
import io.ktor.server.plugins.doublereceive.*
import io.ktor.server.plugins.statuspages.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import kotlinx.serialization.json.*
import org.koin.core.context.GlobalContext
import org.noelware.charted.common.config.Config
import org.noelware.charted.server.plugins.KtorLogging
import org.noelware.charted.server.plugins.Ratelimit
import org.noelware.charted.server.plugins.RequestMdc
import org.noelware.charted.server.plugins.Sentry
import org.noelware.ktor.NoelKtorRoutingPlugin
import org.noelware.ktor.loader.koin.KoinEndpointLoader
import io.sentry.Sentry as SentryInstance

fun Application.module(config: Config) {
    val log by logging<ChartedServer>()

    install(DoubleReceive)
    install(AutoHeadResponse)
    install(KtorLogging)
    install(RequestMdc)
    install(Ratelimit)

    if (SentryInstance.isEnabled())
        install(Sentry)

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

    install(StatusPages) {
        statuses[HttpStatusCode.NotFound] = { call, content, _ ->
            if (content.contentLength == null) {
                call.respond(
                    HttpStatusCode.NotFound,
                    buildJsonObject {
                        put("success", false)
                        putJsonArray("errors") {
                            addJsonObject {
                                put("code", "UNKNOWN_ROUTE")
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
                            put("code", "INVALID_METHOD")
                            put(
                                "message",
                                "Route ${call.request.httpMethod.value} ${call.request.uri} doesn't implement a handler for that specific method."
                            )
                        }
                    }
                }
            )
        }

        exception<Exception> { call, cause ->
            if (SentryInstance.isEnabled())
                SentryInstance.captureException(cause)

            log.error(
                "Unable to handle request ${call.request.httpMethod.value} ${call.request.uri}:",
                cause
            )

            call.respond(
                HttpStatusCode.MethodNotAllowed,
                buildJsonObject {
                    put("success", false)
                    putJsonArray("errors") {
                        addJsonObject {
                            put("message", "Unknown exception has occurred")
                            put("code", "INTERNAL_SERVER_ERROR")

                            if (config.debug) {
                                putJsonObject("exception") {
                                    put("message", cause.message)
                                    put("stacktrace", cause.stackTraceToString())

                                    cause.cause.ifNotNull {
                                        putJsonObject("caused_by") {
                                            put("message", it.message)
                                            put("stacktrace", it.stackTraceToString())
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            )
        }
    }

    install(Routing)
    install(NoelKtorRoutingPlugin) {
        endpointLoader(KoinEndpointLoader)
    }
}
