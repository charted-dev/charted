/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware <team@noelware.org>
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

package org.noelware.charted.server.plugins.tracing

import co.elastic.apm.api.ElasticApm
import co.elastic.apm.api.Transaction
import io.ktor.server.application.*
import io.ktor.server.application.hooks.*
import io.ktor.server.request.*
import io.ktor.util.*
import org.noelware.charted.ChartedInfo

private val ELASTIC_APM_TRACING_KEY = AttributeKey<Transaction>("Elastic APM Transaction")

/**
 * Returns the transaction for Elastic APM. Can return `null` if the plugin was never
 * registered.
 */
val ApplicationCall.apmTransaction: Transaction?
    get() = attributes.getOrNull(ELASTIC_APM_TRACING_KEY)

val ElasticApmTracingPlugin = createApplicationPlugin("ElasticApmTracingPlugin") {
    val ktorVersion = Application::class.java.`package`.implementationVersion

    onCall { call ->
        call.attributes.put(
            ELASTIC_APM_TRACING_KEY,
            ElasticApm.startTransaction()
                .setName("${call.request.httpMethod.value} ${call.request.path()} [${call.request.httpVersion}]")
                .setFrameworkName("Ktor ${ktorVersion ?: "(unknown)"}")
                .setServiceInfo("Noelware/charted-server", "${ChartedInfo.version}+${ChartedInfo.commitHash}")
        )

        call.apmTransaction!!.activate()
    }

    on(CallFailed) { call, cause ->
        call.apmTransaction!!.captureException(cause)
    }

    on(ResponseSent) { call ->
        call.apmTransaction!!.end()
    }
}
