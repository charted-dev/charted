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

package org.noelware.charted.server.internal.statuspages

import com.charleskorn.kaml.YamlException
import dev.floofy.utils.slf4j.logging
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.plugins.*
import io.ktor.server.plugins.statuspages.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.sentry.Sentry
import kotlinx.serialization.SerializationException
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import org.noelware.charted.KtorHttpRespondException
import org.noelware.charted.MultiValidationException
import org.noelware.charted.ValidationException
import org.noelware.charted.common.extensions.sentry.ifSentryEnabled
import org.noelware.charted.common.types.responses.ApiError
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.server.internal.DefaultServer

private val log by logging<DefaultServer>()

fun StatusPagesConfig.configure(config: Config) {
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
                    },
                ),
            )
        }
    }

    status(HttpStatusCode.TooManyRequests) { call, _ ->
        val retryAfter = call.response.headers["Retry-After"]
        call.respond(
            HttpStatusCode.TooManyRequests,
            ApiResponse.err(
                "TOO_MANY_REQUESTS", "IP ${call.request.origin.remoteAddress} has hit the global rate-limiter!",
                buildJsonObject {
                    put("retry_after", retryAfter)
                    put("method", call.request.httpMethod.value)
                    put("url", call.request.path())
                },
            ),
        )
    }

    status(HttpStatusCode.MethodNotAllowed) { call, _ ->
        call.respond(
            HttpStatusCode.MethodNotAllowed,
            ApiResponse.err(
                "INVALID_REST_HANDLER", "Route handler was not the right method",
                buildJsonObject {
                    put("method", call.request.httpMethod.value)
                    put("url", call.request.path())
                },
            ),
        )
    }

    status(HttpStatusCode.UnsupportedMediaType) { call, _ ->
        val header = call.request.header("Content-Type")
        call.respond(
            HttpStatusCode.UnsupportedMediaType,
            ApiResponse.err("UNSUPPORTED_CONTENT_TYPE", "Invalid content type [$header], was expecting \"application/json\""),
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
                },
            ),
        )
    }

    exception<KtorHttpRespondException> { call, cause ->
        ifSentryEnabled { Sentry.captureException(cause) }
        call.respond(cause.httpStatusCode(), ApiResponse.err(cause.errors()))
    }

    exception<MultiValidationException> { call, cause ->
        ifSentryEnabled { Sentry.captureException(cause) }

        log.error("Received multiple validation exceptions on REST handler [${call.request.httpMethod.value} ${call.request.path()}]", cause)
        call.respond(
            HttpStatusCode.NotAcceptable,
            cause.exceptions().map { ApiError(it.codeToUse() ?: "VALIDATION_EXCEPTION", it.validationMessage()) },
        )
    }

    exception<ValidationException> { call, cause ->
        ifSentryEnabled { Sentry.captureException(cause) }

        log.error("Received an validation exception on REST handler [${call.request.httpMethod.value} ${call.request.path()}] ~> ${cause.path()} [${cause.validationMessage()}]")
        call.respond(
            HttpStatusCode.NotAcceptable,
            ApiResponse.err(cause.codeToUse() ?: "VALIDATION_EXCEPTION", cause.validationMessage()),
        )
    }

    exception<SerializationException> { call, cause ->
        ifSentryEnabled { Sentry.captureException(cause) }

        log.error("Received serialization exception in handler [${call.request.httpMethod.value} ${call.request.path()}]", cause)
        call.respond(
            HttpStatusCode.PreconditionFailed,
            ApiResponse.err("SERIALIZATION_FAILED", cause.message!!),
        )
    }

    exception<YamlException> { call, cause ->
        ifSentryEnabled { Sentry.captureException(cause) }

        log.error("Unknown YAML exception had occurred while handling request [${call.request.httpMethod.value} ${call.request.path()}]:", cause)
        call.respond(
            HttpStatusCode.NotAcceptable,
            ApiResponse.err(cause),
        )
    }

    exception<MissingRequestParameterException> { call, cause ->
        ifSentryEnabled { Sentry.captureException(cause) }

        call.respond(
            HttpStatusCode.NotAcceptable,
            ApiResponse.err("MISSING_REQUEST_PARAMETER", "Parameter [${cause.parameterName}] was missing in the request"),
        )
    }

    exception<Exception> { call, cause ->
        ifSentryEnabled { Sentry.captureException(cause) }

        log.error("Unknown exception had occurred while handling request [${call.request.httpMethod.value} ${call.request.path()}]", cause)
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
                                if (config.debug) {
                                    put("stacktrace", cause.cause!!.stackTraceToString())
                                }
                            },
                        )
                    }

                    if (config.debug) {
                        put("stacktrace", cause.stackTraceToString())
                    }
                },
            ),
        )
    }
}
