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

package org.noelware.charted.core.interceptors

import io.sentry.*
import okhttp3.Interceptor
import okhttp3.Response

object SentryInterceptor: Interceptor {
    private val hub: IHub = HubAdapter.getInstance()

    override fun intercept(chain: Interceptor.Chain): Response {
        var request = chain.request()
        val url = "${request.method} ${request.url.encodedPath}"
        val transaction = Sentry.startTransaction("charted.server.http", "Request $url")
        var statusCode = 200
        var response: Response? = null

        return try {
            transaction.latestActiveSpan?.toSentryTrace()?.let {
                request = request.newBuilder()
                    .addHeader(it.name, it.value)
                    .build()
            }

            response = chain.proceed(request)
            statusCode = response.code
            transaction.status = SpanStatus.OK

            response
        } catch (e: Exception) {
            Sentry.captureException(e)
            transaction.apply {
                status = SpanStatus.fromHttpStatusCode(statusCode)
            }

            throw e
        } finally {
            val breb = Breadcrumb.http(request.url.toString(), request.method, statusCode)
            breb.level = if (response?.isSuccessful == true) SentryLevel.INFO else SentryLevel.ERROR
            hub.addBreadcrumb(breb)

            transaction.finish()
        }
    }
}
