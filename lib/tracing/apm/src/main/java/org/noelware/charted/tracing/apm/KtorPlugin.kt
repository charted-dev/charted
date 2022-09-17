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

package org.noelware.charted.tracing.apm

import co.elastic.apm.api.ElasticApm
import co.elastic.apm.api.Transaction
import io.ktor.server.application.*
import io.ktor.server.application.hooks.*
import io.ktor.server.request.*
import io.ktor.util.*

private val transactionKey: AttributeKey<Transaction> = AttributeKey("Transaction")

/**
 * Returns the APM transaction, if the plugin was enabled. You can create
 * additional spans if this is used.
 */
val ApplicationCall.apmTransaction: Transaction?
    get() = attributes.getOrNull(transactionKey)

/**
 * Ktor Plugin to integrate Elastic APM into the requests.
 */
val APM = createApplicationPlugin("Elastic APM") {
    val ktorVersion = Application::class.java.`package`.implementationVersion ?: "(unknown)"
    onCall { call ->
        val transaction = ElasticApm.startTransaction().apply {
            setName("${call.request.httpMethod.value} ${call.request.path()} [${call.request.httpVersion}]")
            setFrameworkName("Ktor v$ktorVersion")
            setType(Transaction.TYPE_REQUEST)
        }

        call.attributes.put(transactionKey, transaction)
    }

    on(ResponseSent) { call ->
        call.apmTransaction!!.end()
    }
}
