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

package org.noelware.charted.server.testing

import dev.floofy.utils.koin.retrieve
import dev.floofy.utils.slf4j.logging
import io.ktor.serialization.kotlinx.json.*
import io.ktor.server.application.*
import io.ktor.server.engine.*
import io.ktor.server.plugins.autohead.*
import io.ktor.server.plugins.contentnegotiation.*
import io.ktor.server.plugins.defaultheaders.*
import io.ktor.server.plugins.doublereceive.*
import io.ktor.server.plugins.statuspages.*
import io.ktor.server.routing.*
import io.ktor.server.testing.*
import kotlinx.coroutines.runBlocking
import org.koin.core.context.GlobalContext
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.server.ChartedServer
import org.noelware.charted.server.endpoints.v1.CdnEndpoints
import org.noelware.charted.server.hasStarted
import org.noelware.charted.server.internal.extensions.configure
import org.noelware.charted.server.plugins.Logging
import org.noelware.charted.server.plugins.RequestMdc
import org.noelware.ktor.loader.koin.KoinEndpointLoader
import org.noelware.ktor.plugin.NoelKtorRouting

typealias ServerTestFunction = suspend ApplicationTestBuilder.() -> Unit

class TestChartedServer(
    private val config: Config,
    private val testFunction: ServerTestFunction = {}
): ChartedServer {
    private val log by logging<TestChartedServer>()

    override val server: ApplicationEngine
        get() = throw IllegalStateException("Server tests don't expose `ApplicationEngine`")

    override val started: Boolean
        get() = hasStarted.get()

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

    override fun start() {
        runBlocking { TestBootstrapPhase.bootstrap(config) }

        log.info("Starting test server!")
        testApplication {
            application { module() }
            testFunction()
        }

        TestBootstrapPhase.cleanup()
    }

    override fun close() {
        // we don't do anything since `testApplication` starts and destroys
        // the server.
    }
}
