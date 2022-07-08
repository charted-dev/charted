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

package org.noelware.charted.server.plugins

import io.ktor.server.application.*
import io.ktor.server.application.hooks.*
import io.ktor.server.request.*
import io.ktor.util.*
import io.sentry.ITransaction
import io.sentry.SpanStatus
import io.sentry.Sentry as SentryClient

val transactionKey = AttributeKey<ITransaction>("Sentry Transaction")
val Sentry = createApplicationPlugin("Sentry") {
    onCall { call ->
        val transaction = SentryClient.startTransaction(
            "${call.request.httpMethod.value} ${call.request.uri} [${call.request.httpVersion}]",
            "http.request"
        )

        call.attributes.put(transactionKey, transaction)
    }

    on(ResponseSent) { call ->
        val transaction = call.attributes[transactionKey]
        transaction.status = SpanStatus.fromHttpStatusCode(call.response.status()!!.value)
        transaction.finish()

        call.attributes.remove(transactionKey)
    }
}
