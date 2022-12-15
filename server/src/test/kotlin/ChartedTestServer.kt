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

package org.noelware.charted.server.tests

import com.charleskorn.kaml.YamlException
import dev.floofy.utils.slf4j.logging
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import io.ktor.server.application.*
import io.ktor.server.engine.*
import io.ktor.server.plugins.autohead.*
import io.ktor.server.plugins.contentnegotiation.*
import io.ktor.server.plugins.doublereceive.*
import io.ktor.server.plugins.statuspages.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.ktor.server.testing.*
import kotlinx.serialization.SerializationException
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import org.koin.core.context.GlobalContext.startKoin
import org.koin.dsl.module
import org.noelware.charted.MultiValidationException
import org.noelware.charted.ValidationException
import org.noelware.charted.common.SetOnce
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.server.ChartedServer
import org.noelware.charted.server.hasStarted
import org.noelware.charted.server.plugins.Logging
import org.noelware.charted.types.responses.ApiError
import org.noelware.charted.types.responses.ApiResponse

/**
 * Creates a new [ChartedTestServer] with the given [config] and [testFunction] block.
 * @param config The configuration to apply
 * @param module The module to apply (if any)
 * @param testFunction Function to perform testing on
 */
internal fun withChartedServer(config: Config, module: Application.() -> Unit = {}, testFunction: suspend ApplicationTestBuilder.() -> Unit) {
    val server = ChartedTestServer(config)
    server.testFunction = {
        application {
            module()
        }

        testFunction()
    }

    server.start()
}

internal class ChartedTestServer(private val config: Config): ChartedServer {
    private val _test: SetOnce<suspend ApplicationTestBuilder.() -> Unit> = SetOnce()
    private val log by logging<ChartedServer>()

    /**
     * Returns the function used to test the server.
     */
    var testFunction: suspend ApplicationTestBuilder.() -> Unit
        get() = _test.value
        set(value) {
            _test.value = value
        }

    /**
     * Checks if the server has started or not.
     */
    override val started: Boolean
        get() = hasStarted.get()

    /**
     * The application engine that Ktor is using for the server.
     */
    override val server: ApplicationEngine
        get() = throw IllegalStateException("Server is not allowed to be fetched")

    /**
     * Extension function to tailor the application module for this [ChartedServer]
     * instance.
     */
    override fun Application.module() {
        val self = this@ChartedTestServer

        install(AutoHeadResponse)
        install(DoubleReceive)
        install(Logging)

        install(ContentNegotiation) {
            json(
                Json {
                    ignoreUnknownKeys = true
                    isLenient = true
                }
            )
        }

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
                    HttpStatusCode.NotImplemented,
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
                self.log.error("Received multiple validation exceptions on REST handler [${call.request.httpMethod.value} ${call.request.path()}]")
                call.respond(
                    HttpStatusCode.NotAcceptable,
                    cause.exceptions().map { ApiError("VALIDATION_EXCEPTION", it.validationMessage) }
                )
            }

            exception<ValidationException> { call, cause ->
                self.log.error("Received an validation exception on REST handler [${call.request.httpMethod.value} ${call.request.path()}] ~> ${cause.path} [${cause.validationMessage}]")
                call.respond(
                    HttpStatusCode.NotAcceptable,
                    ApiResponse.err("VALIDATION_EXCEPTION", cause.validationMessage)
                )
            }

            exception<SerializationException> { call, cause ->
                self.log.error("Received serialization exception in handler [${call.request.httpMethod.value} ${call.request.path()}]", cause)
                call.respond(
                    HttpStatusCode.PreconditionFailed,
                    ApiResponse.err("SERIALIZATION_FAILED", cause.message!!)
                )
            }

            exception<YamlException> { call, cause ->
                self.log.error("Unknown YAML exception had occurred while handling request [${call.request.httpMethod.value} ${call.request.path()}]:", cause)
                call.respond(
                    HttpStatusCode.NotAcceptable,
                    ApiResponse.err(cause)
                )
            }

            exception<Exception> { call, cause ->
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

        routing {}
    }

    /**
     * Starts the server, this will be a no-op if [started] was already
     * set to `true`.
     */
    override fun start() {
        // Start Koin with only the configuration (I think?)
        startKoin {
            modules(
                module {
                    single { config }
                }
            )
        }

        // Run the test server
        testApplication {
            application {
                module()
            }

            testFunction()
        }
    }

    override fun close() {
        /* we don't close it since the test application does for us */
    }
}
