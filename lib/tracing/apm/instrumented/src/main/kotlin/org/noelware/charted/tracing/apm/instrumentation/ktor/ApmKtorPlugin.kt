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

package org.noelware.charted.tracing.apm.instrumentation.ktor

import co.elastic.apm.api.ElasticApm
import co.elastic.apm.api.Span
import co.elastic.apm.api.Transaction
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.application.hooks.*
import io.ktor.server.request.*
import io.ktor.util.*
import org.noelware.charted.common.ChartedInfo

/**
 * Represents the key to retrieve the current APM transaction.
 */
val APM_TRANSACTION_KEY = AttributeKey<Transaction>("APM Transaction")

/**
 * Represents the Ktor plugin to configure Elastic APM for tracing.
 */
val ApmKtorPlugin = createApplicationPlugin("Elastic APM Integration") {
    val ktorVersion = Application::class.java.`package`.implementationVersion
    val requestSpanKey = AttributeKey<Span>("apm span")

    onCall { call ->
        val transaction = ElasticApm.startTransaction().apply {
            setServiceInfo("Noelware/charted-server", "${ChartedInfo.version} (${ChartedInfo.commitHash})")
            setFrameworkName("Ktor v$ktorVersion")
            setLabel("metadata.product", "charted-server")
            setLabel("metadata.vendor", "Noelware")
            setLabel("kotlin.version", KotlinVersion.CURRENT.toString())
        }

        call.attributes.put(APM_TRANSACTION_KEY, transaction)

        val span = transaction.startSpan("http", "request", "${call.request.httpMethod.value} ${call.request.path()} [${call.request.httpVersion}]")
        call.attributes.put(requestSpanKey, span)
    }

    on(CallFailed) { call, cause ->
        val span = call.attributes[requestSpanKey]
        span.end()

        throw cause
    }

    on(ResponseSent) { call ->
        val transaction = call.attributes[APM_TRANSACTION_KEY]
        val requestSpan = call.attributes[requestSpanKey]

        val status = call.response.status() ?: HttpStatusCode.OK
        transaction.setResult("${status.value} ${status.description}")

        requestSpan.end()
        transaction.end()
    }
}
