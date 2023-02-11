/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

package org.noelware.charted.server.plugins

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.application.hooks.*
import io.ktor.server.request.*
import io.ktor.util.*
import io.sentry.ITransaction
import io.sentry.Sentry
import io.sentry.SpanStatus

private val SENTRY_TRANSACTION_KEY: AttributeKey<ITransaction> = AttributeKey("Sentry Transaction")

val ApplicationCall.sentryTransaction: ITransaction?
    get() = attributes.getOrNull(SENTRY_TRANSACTION_KEY)

val SentryPlugin = createApplicationPlugin("Sentry") {
    onCall { call ->
        call.attributes.put(
            SENTRY_TRANSACTION_KEY,
            Sentry.startTransaction(
                "HTTP Request",
                "${call.request.httpMethod.value} ${call.request.path()} [${call.request.httpVersion}]",
            ),
        )
    }

    on(CallFailed) { call, _ ->
        call.sentryTransaction!!.finish(SpanStatus.UNKNOWN_ERROR)
    }

    on(ResponseSent) { call ->
        call.sentryTransaction!!.finish(SpanStatus.fromHttpStatusCode(call.response.status()?.value ?: HttpStatusCode.OK.value))
    }
}
