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

import co.elastic.apm.attach.ElasticApmAttacher
import dev.floofy.utils.slf4j.logging
import org.noelware.charted.common.ChartedInfo
import org.noelware.charted.common.extensions.toStringMap
import org.noelware.charted.configuration.dsl.tracing.ElasticAPMConfig
import java.net.InetAddress
import java.util.Properties

object ElasticAPM {
    private val log by logging<ElasticAPM>()

    fun install(config: ElasticAPMConfig) {
        log.info("Enabling tracing with Elastic APM...")

        val nodeName = when {
            ChartedInfo.dedicatedNode != null -> ChartedInfo.dedicatedNode!!
            config.serviceNodeName != null -> config.serviceNodeName!!
            else -> {
                log.warn("Getting service node name from hostname!")
                try {
                    InetAddress.getLocalHost().hostName ?: ""
                } catch (e: Exception) {
                    ""
                }
            }
        }

        val apmConfig = Properties().apply {
            put("recording", config.recording)
            put("instrument", config.enableInstrumentation)
            put("service_name", "charted-server")
            put("service_version", "${ChartedInfo.version} [${ChartedInfo.commitHash}]")
            put("transaction_sample_rate", config.transactionSampleRate)
            put("transaction_max_spans", config.transactionMaxSpans)
            put("capture_body", if (config.captureBody) "ON" else "OFF")
            put("global_labels", config.globalLabels)
            put("capture_headers", config.captureHeaders)
            put("enable_instrumentations", config.instrumentations.joinToString(", ") { it.key })
            put("global_labels", config.globalLabels.map { "${it.key}=${it.value}" }.joinToString(","))
            put("application_packages", "org.noelware.charted")
            put("server_url", config.serverUrl)

            if (config.apiKey != null) put("api_key", config.apiKey)
            if (config.secretToken != null) put("secret_token", config.secretToken)
            if (nodeName.isNotEmpty()) {
                put("service_node_name", nodeName)
            }
        }

        ElasticApmAttacher.attach(apmConfig.toStringMap())
    }
}
