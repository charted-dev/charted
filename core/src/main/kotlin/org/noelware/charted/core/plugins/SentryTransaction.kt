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

package org.noelware.charted.core.plugins

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.application.hooks.*
import io.ktor.server.request.*
import io.ktor.util.*
import io.sentry.ITransaction
import io.sentry.SentryTraceHeader
import io.sentry.SpanStatus
import io.sentry.TransactionContext
import io.sentry.Sentry as SentryClient

val SentryTransactionKey = AttributeKey<ITransaction>("Sentry transaction")
val Sentry = createApplicationPlugin("Sentry") {
    onCall { call ->
        // Check if we can receive the transparent data
        val sentryTraceHeader = call.request.header("sentry-trace")
        var sentryTrace: SentryTraceHeader? = null

        if (sentryTraceHeader != null) {
            sentryTrace = SentryTraceHeader(sentryTraceHeader)
        }

        val transaction = if (sentryTraceHeader != null)
            SentryClient.startTransaction(
                TransactionContext.fromSentryTrace(
                    "${call.request.httpMethod.value} ${call.request.path()} | ${call.request.httpVersion}",
                    "http.request",
                    sentryTrace!!
                )
            )
        else
            SentryClient.startTransaction(
                "${call.request.httpMethod.value} ${call.request.path()} | ${call.request.httpVersion}",
                "http.request"
            )

        SentryClient.configureScope {
            it.transaction = transaction
        }

        // We might add more additional spans, so we can retrieve
        call.attributes.put(SentryTransactionKey, transaction)
    }

    on(ResponseSent) { call ->
        val transaction = call.attributes[SentryTransactionKey]
        transaction.status = SpanStatus.fromHttpStatusCode(call.response.status()?.value ?: HttpStatusCode.OK.value)
        transaction.finish()

        call.attributes.remove(SentryTransactionKey)
    }
}
