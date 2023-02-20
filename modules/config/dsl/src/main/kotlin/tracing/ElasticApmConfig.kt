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

package org.noelware.charted.configuration.kotlin.dsl.tracing

import kotlinx.serialization.KSerializer
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.descriptors.PrimitiveKind
import kotlinx.serialization.descriptors.PrimitiveSerialDescriptor
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import org.noelware.charted.ChartedInfo
import org.noelware.charted.serializers.SecretStringSerializer
import java.net.InetAddress

/**
 * [read more here](https://www.elastic.co/guide/en/apm/agent/java/current/config-core.html#config-enable-instrumentations)
 */
@kotlinx.serialization.Serializable
public enum class Instrumentation(public val key: String) {
    ANNOTATIONS("annotations"),
    ANNOTATIONS_CAPTURE_SPAN("annotations-capture-span"),
    ANNOTATIONS_CAPTURE_TRANSACTION("annotations-capture-transaction"),
    ANNOTATIONS_TRACED("annotations-traced"),
    APACHE_HTTP_CLIENT("apache-httpclient"),
    AWS_SDK("aws-sdk"),
    ELASTICSEARCH_REST("elasticsearch-restclient"),
    EXCEPTION_HANDLER("exception-handler"),
    JAVA_EXECUTOR("executor"),
    EXECUTOR_COLLECTION("executor-collection"),
    GRPC("grpc"),
    LETTUCE("lettuce"),
    JDBC("jdbc"),
    OKHTTP("okhttp"),
    PROCESS("process"),
    SLF4J_ERROR("slf4j-error");

    public companion object : KSerializer<Instrumentation> {
        override val descriptor: SerialDescriptor = PrimitiveSerialDescriptor("charted.apm.Instrumentation", PrimitiveKind.STRING)
        override fun deserialize(decoder: Decoder): Instrumentation {
            val key = decoder.decodeString()
            return values().find { it.key == key } ?: error("Unknown key [$key]")
        }

        override fun serialize(encoder: Encoder, value: Instrumentation) {
            encoder.encodeString(value.key)
        }
    }
}

/**
 * Represents the configuration for using Elastic APM for the tracing mechanism.
 */
@kotlinx.serialization.Serializable
public data class ElasticAPMConfig(
    /**
     * A boolean specifying if the agent should be recording or not. When recording, the agent instruments incoming HTTP requests,
     * tracks errors and collects and sends metrics. When not recording, the agent works as a noop, not collecting data and not
     * communicating with the APM sever, except for polling the central configuration endpoint. As this is a reversible switch,
     * agent threads are not being killed when inactivated, but they will be mostly idle in this state, so the overhead should be
     * negligible.
     *
     * [read more here](https://www.elastic.co/guide/en/apm/agent/java/current/config-core.html#config-recording)
     */
    val recording: Boolean = true,

    /**
     * A boolean specifying if the agent should instrument the application to collect traces for the app. When set to false,
     * most built-in instrumentation plugins are disabled, which would minimize the effect on your application. However, the
     * agent would still apply instrumentation related to manual tracing options, and it would still collect and send metrics
     * to APM Server.
     */
    @SerialName("enable_instrumentation")
    val enableInstrumentation: Boolean = true,

    /**
     * If set, this name is used to distinguish between different nodes of a service, therefore it should be unique for
     * each JVM within a service. If not set, data aggregations will be done based on a container ID (where valid).
     *
     * This is also used when the [dedi node][org.noelware.charted.common.ChartedInfo.dedicatedNode] is specified. If not,
     * it'll return the host name if this is `null`.
     *
     * [read more here](https://www.elastic.co/guide/en/apm/agent/java/current/config-core.html#config-service-node-name)
     */
    @SerialName("service_node_name")
    val serviceNodeName: String? = null,

    /**
     * By default, the agent will sample every transaction (e.g. request to your service). To reduce overhead and storage
     * requirements, you can set the sample rate to a value between 0.0 and 1.0. We still record overall time and the
     * result for unsampled transactions, but no context information, labels, or spans.
     *
     * Value will be rounded with 4 significant digits, as an example, value 0.55555 will be rounded to 0.5556
     *
     * [read more here](https://www.elastic.co/guide/en/apm/agent/java/current/config-core.html#config-transaction-sample-rate)
     */
    @SerialName("transaction_sample_rate")
    val transactionSampleRate: Double = 0.5,

    /**
     * Limits the amount of spans that are recorded per transaction.
     *
     * This is helpful in cases where a transaction creates a very high amount of spans (e.g. thousands of SQL queries).
     * Setting an upper limit will prevent overloading the agent and the APM server with too much work for such edge cases.
     * A message will be logged when the max number of spans has been exceeded but only at a rate of once every 5 minutes to ensure performance is not impacted.
     *
     * [read more here](https://www.elastic.co/guide/en/apm/agent/java/current/config-core.html#config-transaction-max-spans)
     */
    @SerialName("transaction_max_spans")
    val transactionMaxSpans: Int = 500,

    /**
     * A list of instrumentations which should be selectively enabled. By default, all instrumentations are enabled.
     * [read more here](https://www.elastic.co/guide/en/apm/agent/java/current/config-core.html#config-enable-instrumentations)
     */
    val instrumentations: List<Instrumentation> = Instrumentation.values().toList(),

    /**
     * For transactions that are HTTP requests, the Java agent can optionally capture the request body (e.g. POST variables). For transactions that are initiated by receiving a message from a message broker, the agent can capture the textual message body.
     * If the HTTP request or the message has a body and this setting is disabled, the body will be shown as [REDACTED]. This option is case-insensitive.
     *
     * By default, this is set to false for security reasons.
     *
     * [read more here](https://www.elastic.co/guide/en/apm/agent/java/current/config-core.html#config-capture-body)
     */
    @SerialName("capture_body")
    val captureBody: Boolean = false,

    /**
     * If set to true, the agent will capture HTTP request and response headers (including cookies), as well as messages'
     * headers/properties when using messaging frameworks like Kafka or JMS.
     *
     * [read more here](https://www.elastic.co/guide/en/apm/agent/java/current/config-core.html#config-capture-headers)
     */
    val captureHeaders: Boolean = false,

    /**
     * **This requires APM Server 7.2 or higher!**
     *
     * Labels added to all events, with the format `key=value[,key=value[,...]]`. Any labels set by application via the API
     * will override global labels with the same keys.
     *
     * [read more here](https://www.elastic.co/guide/en/apm/agent/java/current/config-core.html#config-global-labels)
     */
    @SerialName("global_labels")
    val globalLabels: Map<String, String> = mapOf(),

    /**
     * APM server URL to connect to Elastic APM. Must be a valid HTTP or HTTPS url. You will need to
     * expose :8200 from the Elastic Agent container if using Docker, if APM server was installed
     * with Fleet.
     */
    @SerialName("server_url")
    val serverUrl: String,

    /**
     * API Key authentication method to use when fanning out events to APM server.
     */
    @Serializable(with = SecretStringSerializer::class)
    @SerialName("api_key")
    val apiKey: String? = null,

    /**
     * If a secret token was configured from the APM server configuration.
     */
    @Serializable(with = SecretStringSerializer::class)
    @SerialName("secret_token")
    val secretToken: String? = null
) {
    @Suppress("MemberVisibilityCanBePrivate")
    public class Builder : org.noelware.charted.common.Builder<ElasticAPMConfig> {
        private var enabledInstrumentations = mutableListOf<Instrumentation>()

        /**
         * A boolean specifying if the agent should be recording or not. When recording, the agent instruments incoming HTTP requests,
         * tracks errors and collects and sends metrics. When not recording, the agent works as a noop, not collecting data and not
         * communicating with the APM sever, except for polling the central configuration endpoint. As this is a reversible switch,
         * agent threads are not being killed when inactivated, but they will be mostly idle in this state, so the overhead should be
         * negligible.
         *
         * [read more here](https://www.elastic.co/guide/en/apm/agent/java/current/config-core.html#config-recording)
         */
        public var recording: Boolean = true

        /**
         * A boolean specifying if the agent should instrument the application to collect traces for the app. When set to false,
         * most built-in instrumentation plugins are disabled, which would minimize the effect on your application. However, the
         * agent would still apply instrumentation related to manual tracing options, and it would still collect and send metrics
         * to APM Server.
         */
        @SerialName("enable_instrumentation")
        public var enableInstrumentation: Boolean = true

        /**
         * If set, this name is used to distinguish between different nodes of a service, therefore it should be unique for
         * each JVM within a service. If not set, data aggregations will be done based on a container ID (where valid).
         *
         * This is also used when the [dedi node][org.noelware.charted.common.ChartedInfo.dedicatedNode] is specified. If not,
         * it'll try to resolve the host name, or an empty string if that errors out.
         *
         * [read more here](https://www.elastic.co/guide/en/apm/agent/java/current/config-core.html#config-service-node-name)
         */
        @SerialName("service_node_name")
        public var serviceNodeName: String = if (ChartedInfo.dedicatedNode != null) {
            ChartedInfo.dedicatedNode!!
        } else {
            try {
                InetAddress.getLocalHost().hostName
            } catch (e: Exception) {
                ""
            }
        }

        /**
         * By default, the agent will sample every transaction (e.g. request to your service). To reduce overhead and storage
         * requirements, you can set the sample rate to a value between 0.0 and 1.0. We still record overall time and the
         * result for unsampled transactions, but no context information, labels, or spans.
         *
         * Value will be rounded with 4 significant digits, as an example, value 0.55555 will be rounded to 0.5556
         *
         * [read more here](https://www.elastic.co/guide/en/apm/agent/java/current/config-core.html#config-transaction-sample-rate)
         */
        @SerialName("transaction_sample_rate")
        public var transactionSampleRate: Double = 0.5

        /**
         * Limits the amount of spans that are recorded per transaction.
         *
         * This is helpful in cases where a transaction creates a very high amount of spans (e.g. thousands of SQL queries).
         * Setting an upper limit will prevent overloading the agent and the APM server with too much work for such edge cases.
         * A message will be logged when the max number of spans has been exceeded but only at a rate of once every 5 minutes to ensure performance is not impacted.
         *
         * [read more here](https://www.elastic.co/guide/en/apm/agent/java/current/config-core.html#config-transaction-max-spans)
         */
        @SerialName("transaction_max_spans")
        public var transactionMaxSpans: Int = 500

        /**
         * For transactions that are HTTP requests, the Java agent can optionally capture the request body (e.g. POST variables). For transactions that are initiated by receiving a message from a message broker, the agent can capture the textual message body.
         * If the HTTP request or the message has a body and this setting is disabled, the body will be shown as [REDACTED]. This option is case-insensitive.
         *
         * By default, this is set to false for security reasons.
         *
         * [read more here](https://www.elastic.co/guide/en/apm/agent/java/current/config-core.html#config-capture-body)
         */
        public var captureBody: Boolean = false

        /**
         * If set to true, the agent will capture HTTP request and response headers (including cookies), as well as messages'
         * headers/properties when using messaging frameworks like Kafka or JMS.
         *
         * [read more here](https://www.elastic.co/guide/en/apm/agent/java/current/config-core.html#config-capture-headers)
         */
        public var captureHeaders: Boolean = false

        /**
         * **This requires APM Server 7.2 or higher!**
         *
         * Labels added to all events, with the format `key=value[,key=value[,...]]`. Any labels set by application via the API
         * will override global labels with the same keys.
         *
         * [read more here](https://www.elastic.co/guide/en/apm/agent/java/current/config-core.html#config-global-labels)
         */
        public val globalLabels: MutableMap<String, String> = mutableMapOf()
        public var serverUrl: String = "http://localhost:8200"
        public var apiKey: String? = null
        public var secretToken: String? = null

        public fun enable(vararg instrumentations: Instrumentation): Builder {
            for (instrumentation in instrumentations) {
                if (enabledInstrumentations.contains(instrumentation)) {
                    continue
                }

                enabledInstrumentations.add(instrumentation)
            }

            return this
        }

        /**
         * Enables a single auto-instrumentation.
         */
        public fun enable(instrumentation: Instrumentation): Builder {
            if (enabledInstrumentations.contains(instrumentation)) return this

            enabledInstrumentations.add(instrumentation)
            return this
        }

        override fun build(): ElasticAPMConfig = ElasticAPMConfig(
            recording,
            enableInstrumentation,
            serviceNodeName,
            transactionSampleRate,
            transactionMaxSpans,
            enabledInstrumentations,
            captureBody,
            captureHeaders,
            globalLabels,
            serverUrl,
            apiKey,
            secretToken,
        )
    }
}
