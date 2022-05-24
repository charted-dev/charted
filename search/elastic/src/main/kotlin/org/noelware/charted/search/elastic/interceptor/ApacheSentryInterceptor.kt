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

package org.noelware.charted.search.elastic.interceptor

import io.sentry.*
import org.apache.http.HttpRequest
import org.apache.http.HttpRequestInterceptor
import org.apache.http.HttpResponse
import org.apache.http.HttpResponseInterceptor
import org.apache.http.client.protocol.HttpClientContext
import org.apache.http.protocol.HttpContext

object ApacheSentryRequestInterceptor: HttpRequestInterceptor {
    override fun process(request: HttpRequest, context: HttpContext) {
        val currentTransaction = Sentry.startTransaction("charted.server.elastic.http", "Request ${request.requestLine.uri} ${request.requestLine.uri} [${request.requestLine.protocolVersion}]")
        currentTransaction.latestActiveSpan?.toSentryTrace()?.let {
            request.addHeader(it.name, it.value)
        }

        context.setAttribute("sentry.transaction", currentTransaction)
    }
}

object ApacheSentryResponseInterceptor: HttpResponseInterceptor {
    override fun process(response: HttpResponse, context: HttpContext) {
        val transaction = context.getAttribute("sentry.transaction") as? ITransaction ?: return
        val clientContext = HttpClientContext.adapt(context)

        val hub = HubAdapter.getInstance()
        val breadcrumb = Breadcrumb.http(clientContext.request.requestLine.uri, clientContext.request.requestLine.method)

        breadcrumb.level = if (response.statusLine.statusCode !in 200..300) SentryLevel.ERROR else SentryLevel.INFO
        hub.addBreadcrumb(breadcrumb)

        transaction.finish(SpanStatus.fromHttpStatusCode(response.statusLine.statusCode))
        context.removeAttribute("sentry.transaction")
    }
}
