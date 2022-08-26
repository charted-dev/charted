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

package org.noelware.charted.elasticsearch.apache

import dev.floofy.utils.kotlin.ifNotNull
import dev.floofy.utils.slf4j.logging
import io.sentry.ITransaction
import io.sentry.Sentry
import io.sentry.SpanStatus
import org.apache.commons.lang3.time.StopWatch
import org.apache.http.HttpRequest
import org.apache.http.HttpRequestInterceptor
import org.apache.http.HttpResponse
import org.apache.http.HttpResponseInterceptor
import org.apache.http.client.protocol.HttpClientContext
import org.apache.http.protocol.HttpContext
import org.noelware.charted.common.extensions.ifSentryEnabled
import org.noelware.charted.common.extensions.unsafeCast

private const val SENTRY_TRANSACTION_NAME = "sentry:transaction"
private const val STOPWATCH_NAME = "request:stopwatch"

class SentryApacheHttpClientRequestInterceptor: HttpRequestInterceptor {
    private val log by logging<SentryApacheHttpClientRequestInterceptor>()
    override fun process(request: HttpRequest, context: HttpContext) {
        val requestLine = request.requestLine

        log.info("<- ${requestLine.method} ${requestLine.uri} [${request.protocolVersion}]")
        val transaction = ifSentryEnabled {
            Sentry.startTransaction("apache.http.request", "{requestLine.method} ${requestLine.uri} [${request.protocolVersion}]")
        }

        context.setAttribute(STOPWATCH_NAME, StopWatch.createStarted())
        if (transaction != null) {
            context.setAttribute(SENTRY_TRANSACTION_NAME, transaction)
        }
    }
}

class SentryApacheHttpClientResponseInterceptor: HttpResponseInterceptor {
    private val log by logging<SentryApacheHttpClientResponseInterceptor>()

    override fun process(response: HttpResponse, context: HttpContext) {
        val ctx = HttpClientContext.adapt(context)
        val req = ctx.request

        val stopwatch: StopWatch = ctx.getAttribute(STOPWATCH_NAME).unsafeCast()
        stopwatch.stop()

        val transaction: ITransaction? = ctx.getAttribute(SENTRY_TRANSACTION_NAME).ifNotNull {
            unsafeCast()
        }

        val bytesReceived = response.entity?.contentLength ?: 0
        log.info("-> ${req.requestLine.method} ${req.requestLine.uri} [${req.protocolVersion}] ~> ${response.statusLine.statusCode} ${response.statusLine.reasonPhrase} [$bytesReceived bytes received, ~${stopwatch.formatTime()}]")
        transaction?.finish(SpanStatus.fromHttpStatusCode(response.statusLine.statusCode))
    }
}
