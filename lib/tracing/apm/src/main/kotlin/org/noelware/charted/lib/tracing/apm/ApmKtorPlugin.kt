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

package org.noelware.charted.lib.tracing.apm

import co.elastic.apm.api.ElasticApm
import co.elastic.apm.api.Span
import co.elastic.apm.api.Transaction
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.application.hooks.*
import io.ktor.server.request.*
import io.ktor.util.*
import org.noelware.charted.common.ChartedInfo

val transactionKey = AttributeKey<Transaction>("APM Transaction")
val ApplicationCall.apmTransaction: Transaction?
    get() = attributes.getOrNull(transactionKey)

val ApmKtorPlugin = createApplicationPlugin("Apm") {
    val requestSpanKey = AttributeKey<Span>("Ktor Request Span")

    onCall { call ->
        val transaction = ElasticApm.startTransaction().apply {
            setServiceInfo("Noelware/charted-server", "${ChartedInfo.version} (${ChartedInfo.commitHash})")
            setFrameworkName("Ktor v2.0.3")
            setLabel("metadata.product", "charted-server")
            setLabel("metadata.vendor", "Noelware")
            setLabel("kotlin.version", KotlinVersion.CURRENT.toString())
        }

        call.attributes.put(transactionKey, transaction)

        val span = transaction.startSpan("http", "request", "${call.request.httpMethod.value} ${call.request.path()}")
        call.attributes.put(requestSpanKey, span)
    }

    on(CallFailed) { call, cause ->
        val requestSpan = call.attributes[requestSpanKey]
        requestSpan.end()

        throw cause
    }

    on(ResponseSent) { call ->
        val transaction = call.attributes[transactionKey]
        val requestSpan = call.attributes[requestSpanKey]

        val status = call.response.status() ?: HttpStatusCode.OK
        transaction.setResult("${status.value} ${status.description}")

        requestSpan.end()
        transaction.end()
    }
}
