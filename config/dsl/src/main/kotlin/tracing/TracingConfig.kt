/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

import dev.floofy.utils.kotlin.every
import kotlinx.serialization.*
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.descriptors.buildClassSerialDescriptor
import kotlinx.serialization.descriptors.element
import kotlinx.serialization.encoding.*
import org.noelware.charted.ValidationException
import org.noelware.charted.common.Buildable
import org.noelware.charted.common.extensions.string.toUriOrNull
import org.noelware.charted.configuration.kotlin.dsl.tracing.apm.ApmSystemProperty
import org.noelware.charted.configuration.kotlin.dsl.tracing.apm.CircuitBreakerConfig
import org.noelware.charted.configuration.kotlin.dsl.tracing.apm.Instrumentation
import org.noelware.charted.configuration.kotlin.dsl.tracing.otel.Instrumentation as OtelInstumentation
import kotlin.properties.Delegates

@Serializable
public enum class TracerType {
    @SerialName("otel")
    OpenTelemetry,

    @SerialName("apm")
    ElasticAPM,

    @SerialName("sentry")
    Sentry,
    Unknown
}

@Serializable(with = TracingConfig.Serializer::class)
public sealed class TracingConfig(public val type: TracerType) {
    @Suppress("unused")
    private constructor(): this(TracerType.Unknown)

    @Serializable
    public object Sentry: TracingConfig(TracerType.Sentry)

    /**
     * Configuration for configuring [Elastic's APM integration](https://www.elastic.co/observability/application-performance-monitoring) with charted-server if your stack
     * includes Elasticsearch and Kibana, which will make it easier to trace if you or your company
     * heavily uses Kibana.
     */
    @Serializable
    public data class ElasticAPM(
        /**
         * A list of instrumentations which should be selectively enabled.
         */
        @ApmSystemProperty("enable_instrumentation")
        @SerialName("enabled_instrumentation")
        val enabledInstrumentation: List<Instrumentation> = Instrumentation.values().toList(),

        /**
         * A list of instrumentations which should be selectively disabled.
         */
        @ApmSystemProperty("disable_instrumentation")
        @SerialName("disabled_instrumentation")
        val disabledInstrumentation: List<Instrumentation> = listOf(),

        /**
         * Limits the amount of spans that are recorded per transaction.
         * This is helpful in cases where a transaction creates a very high amount of spans (e.g. thousands of SQL queries).
         * Setting an upper limit will prevent overloading the agent and the APM server with too much work for such edge cases.
         * A message will be logged when the max number of spans has been exceeded but only at a rate of once every 5 minutes to ensure performance is not impacted.
         */
        @ApmSystemProperty("transaction_max_spans")
        @SerialName("transaction_max_spans")
        val transactionMaxSpans: Int = 500,

        /**
         * By default, the agent will sample every transaction (e.g. request to your service). To reduce overhead and storage requirements,
         * you can set the sample rate to a value between 0.0 and 1.0.
         *
         * Value will be rounded with 4 significant digits, as an example, value 0.55555 will be rounded to 0.5556
         */
        @ApmSystemProperty("transaction_sample_rate")
        @SerialName("transaction_sample_rate")
        val transactionSampleRate: Double = 1.0,

        /**
         * Configuration for configuring [Elastic APM's Java agent](https://www.elastic.co/guide/en/apm/agent/java/current/intro.html)'s
         * [circuit breaker configuration](https://www.elastic.co/guide/en/apm/agent/java/current/config-circuit-breaker.html)
         */
        @SerialName("circuit_breaker")
        val circuitBreaker: CircuitBreakerConfig? = null,

        /**
         * List of global labels to append to all events.
         */
        @ApmSystemProperty("global_labels")
        @SerialName("global_labels")
        val globalLabels: List<String> = listOf(),

        /**
         * If set, this name is used to distinguish between different nodes of a service, therefore it should be unique for each
         * JVM within a service. If not set, data aggregations will be done based on a container ID (where valid)
         * or on the reported hostname.
         */
        @ApmSystemProperty("service_node_name")
        @SerialName("service_node_name")
        val serviceNodeName: String? = null,

        /**
         * If the agent should instrument the application to collect traces for the app. When set to false,
         * most built-in instrumentation plugins are disabled, which would minimize the effect on your application.
         *
         * However, the agent would still apply instrumentation related to manual tracing options, and it would still collect and
         * send metrics to APM Server.
         */
        @ApmSystemProperty("instrument")
        val instrument: Boolean = true,

        /**
         * This is used to keep all the errors and transactions of your service together and is the primary filter in
         * the Elastic APM user interface.
         */
        @ApmSystemProperty("service_name")
        @SerialName("service_name")
        val serviceName: String = "charted-server",

        /**
         * Qualified URLs to provide fail-over when one APM server in this list
         * has failed to be executed. This will only provide `server_url` if this
         * contains only one element.
         */
        @ApmSystemProperty("server_urls")
        @SerialName("server_urls")
        val serverUrls: List<String>
    ): TracingConfig(TracerType.ElasticAPM) {
        init {
            if (!serverUrls.every { it.toUriOrNull() != null }) {
                throw ValidationException("config.tracing.apm.server_urls", "Not all URLs listed [${serverUrls.joinToString(", ")}] are valid Java URIs.")
            }
        }

        @Suppress("MemberVisibilityCanBePrivate")
        public class Builder: Buildable<ElasticAPM> {
            private val enabledInstrumentation: MutableList<Instrumentation> = mutableListOf()
            private val disabledInstrumentation: MutableList<Instrumentation> = mutableListOf()
            private var circuitBreaker: CircuitBreakerConfig? = null
            private val globalLabels: MutableList<String> = mutableListOf()
            private val serverUrls: MutableList<String> = mutableListOf()

            /**
             * Limits the amount of spans that are recorded per transaction.
             * This is helpful in cases where a transaction creates a very high amount of spans (e.g. thousands of SQL queries).
             * Setting an upper limit will prevent overloading the agent and the APM server with too much work for such edge cases.
             * A message will be logged when the max number of spans has been exceeded but only at a rate of once every 5 minutes to ensure performance is not impacted.
             */
            public var transactionMaxSpans: Int = 500

            /**
             * By default, the agent will sample every transaction (e.g. request to your service). To reduce overhead and storage requirements,
             * you can set the sample rate to a value between 0.0 and 1.0.
             *
             * Value will be rounded with 4 significant digits, as an example, value 0.55555 will be rounded to 0.5556
             */
            public val transactionSampleRate: Double = 1.0

            /**
             * If set, this name is used to distinguish between different nodes of a service, therefore it should be unique for each
             * JVM within a service. If not set, data aggregations will be done based on a container ID (where valid)
             * or on the reported hostname.
             */
            public val serviceNodeName: String? = null

            /**
             * If the agent should instrument the application to collect traces for the app. When set to false,
             * most built-in instrumentation plugins are disabled, which would minimize the effect on your application.
             *
             * However, the agent would still apply instrumentation related to manual tracing options, and it would still collect and
             * send metrics to APM Server.
             */
            @ApmSystemProperty("instrument")
            public val instrument: Boolean = true

            /**
             * This is used to keep all the errors and transactions of your service together and is the primary filter in
             * the Elastic APM user interface.
             */
            @ApmSystemProperty("service_name")
            @SerialName("service_name")
            public val serviceName: String = "charted-server"

            public fun servers(server: String, vararg servers: String): Builder {
                // de-couple duplicate entries
                serverUrls += setOf(*(listOf(server) + servers).toTypedArray()).toList()
                return this
            }

            public fun globalLabels(vararg labels: String): Builder {
                globalLabels += setOf(*labels).toList()
                return this
            }

            public fun circuitBreaker(block: CircuitBreakerConfig.Builder.() -> Unit = {}): Builder {
                circuitBreaker = CircuitBreakerConfig.Builder().apply(block).build()
                return this
            }

            public fun enabledInstrumentation(vararg instrumentation: Instrumentation): Builder {
                enabledInstrumentation += instrumentation.toList()
                return this
            }

            public fun disabledInstrumentation(vararg instrumentation: Instrumentation): Builder {
                disabledInstrumentation += instrumentation.toList()
                return this
            }

            override fun build(): ElasticAPM = ElasticAPM(
                enabledInstrumentation,
                disabledInstrumentation,
                transactionMaxSpans,
                transactionSampleRate,
                circuitBreaker,
                globalLabels,
                serviceNodeName,
                instrument,
                serviceName,
                serverUrls,
            )
        }
    }

    @Serializable
    public data class OpenTelemetry(
        // agent options
        /**
         * A comma-separated list of HTTP header names. HTTP server instrumentations will capture HTTP response header
         * values for all configured header names.
         */
        @SerialName("capture_client_response_headers")
        val captureClientResponseHeaders: List<String> = listOf(),

        /**
         * A comma-separated list of HTTP header names. HTTP client instrumentations will capture HTTP response header
         * values for all configured header names.
         */
        @SerialName("capture_server_response_headers")
        val captureServerResponseHeaders: List<String> = listOf(),

        /**
         * A comma-separated list of HTTP header names. HTTP client instrumentations will capture HTTP response header
         * values for all configured header names.
         */
        @SerialName("capture_client_request_headers")
        val captureClientRequestHeaders: List<String> = listOf(),

        /**
         * A comma-separated list of HTTP header names. HTTP server instrumentations will capture HTTP response header
         * values for all configured header names.
         */
        @SerialName("capture_server_request_headers")
        val captureServerRequestHeaders: List<String> = listOf(),

        /**
         * List of disabled instrumentation regardless if it was set in [enabledInstrumentation].
         */
        @SerialName("disabled_instrumentation")
        val disabledInstrumentation: List<OtelInstumentation> = listOf(),

        /**
         * List of enabled instrumentation, unless if it is in the [disabled instrumentation list][disabledInstrumentation].
         */
        @SerialName("enabled_instrumentation")
        val enabledInstrumentation: List<OtelInstumentation> = listOf(),

        /**
         * The agent sanitizes all database queries/statements before setting the db.statement semantic attribute.
         * All values (strings, numbers) in the query string are replaced with a question mark (?).
         *
         * Note: JDBC bind parameters are not captured in db.statement. See the corresponding issue if you are looking to capture
         * bind parameters.
         *
         * ## Examples:
         *
         * - SQL query `SELECT a from b where password="secret" will appear as SELECT a from b where password=?` in the exported span;
         * - Redis command `HSET map password "secret"` will appear as `HSET map password ?` in the exported span.
         *
         * This behavior is turned on by default for all database instrumentations.
         */
        @SerialName("db_stmt_sanitizer")
        val dbStatementSanitizer: Boolean = true,

        /**
         * Used to specify a mapping from host names or IP addresses to peer services, as a comma-separated list of
         * <host_or_ip>=<user_assigned_name> pairs. The peer service is added as an attribute to a span whose host or
         * IP address match the mapping.
         *
         * For example, if set to the following:
         *
         * ```
         * 1.2.3.4=cats-service,dogs-abcdef123.serverlessapis.com=dogs-api
         * ```
         *
         * Then, requests to 1.2.3.4 will have a peer.service attribute of cats-service and requests to dogs-abcdef123.serverlessapis.com will have an attribute of dogs-api.
         */
        @SerialName("peer_service_mapping")
        val peerServiceMapping: List<String> = listOf(),

        // non-agent options
        /**
         * URL to connect to a OTLP collector instance.
         */
        @SerialName("otlp_url")
        val otlpUrl: String = "localhost:4132",

        /**
         * Connection type to use when connecting to the [otlpUrl]
         */
        @SerialName("otlp_type")
        val otlpType: OtlpConnectionType
    ): TracingConfig(TracerType.OpenTelemetry) {
        @Serializable
        public enum class OtlpConnectionType {
            @SerialName("http")
            HTTP,

            @SerialName("grpc")
            GRPC
        }

        @Suppress("MemberVisibilityCanBePrivate")
        public class Builder: Buildable<OpenTelemetry> {
            private val captureServerResponseHeaders: MutableList<String> = mutableListOf()
            private val captureClientResponseHeaders: MutableList<String> = mutableListOf()
            private val captureServerRequestHeaders: MutableList<String> = mutableListOf()
            private val captureClientRequestHeaders: MutableList<String> = mutableListOf()
            private val disabledInstrumentation: MutableList<OtelInstumentation> = mutableListOf()
            private val enabledInstrumentation: MutableList<OtelInstumentation> = mutableListOf()
            private val peerServiceMapping: MutableList<String> = mutableListOf()

            public var otlpUrl: String by Delegates.notNull()
            public var otlpType: OtlpConnectionType = OtlpConnectionType.HTTP

            /**
             * The agent sanitizes all database queries/statements before setting the db.statement semantic attribute.
             * All values (strings, numbers) in the query string are replaced with a question mark (?).
             *
             * Note: JDBC bind parameters are not captured in db.statement. See the corresponding issue if you are looking to capture
             * bind parameters.
             *
             * ## Examples:
             *
             * - SQL query `SELECT a from b where password="secret" will appear as SELECT a from b where password=?` in the exported span;
             * - Redis command `HSET map password "secret"` will appear as `HSET map password ?` in the exported span.
             *
             * This behavior is turned on by default for all database instrumentations.
             */
            public var dbStatementSanitizer: Boolean = true

            public fun captureServerResponseHeaders(vararg headers: String): Builder {
                if (headers.isNotEmpty()) {
                    captureServerResponseHeaders += headers.toList()
                }

                return this
            }

            public fun captureClient1ResponseHeaders(vararg headers: String): Builder {
                if (headers.isNotEmpty()) {
                    captureClientResponseHeaders += headers.toList()
                }

                return this
            }

            public fun captureClientRequestHeaders(vararg headers: String): Builder {
                if (headers.isNotEmpty()) {
                    captureClientRequestHeaders += headers.toList()
                }

                return this
            }

            public fun captureServerRequestHeaders(vararg headers: String): Builder {
                if (headers.isNotEmpty()) {
                    captureServerRequestHeaders += headers.toList()
                }

                return this
            }

            public fun peerServiceMappings(vararg map: String): Builder {
                if (map.isNotEmpty()) {
                    peerServiceMapping += map.toList()
                }

                return this
            }

            override fun build(): OpenTelemetry = OpenTelemetry(
                captureClientResponseHeaders,
                captureServerResponseHeaders,
                captureClientRequestHeaders,
                captureServerRequestHeaders,
                disabledInstrumentation,
                enabledInstrumentation,
                dbStatementSanitizer,
                peerServiceMapping,
                otlpUrl,
                otlpType,
            )
        }
    }

    internal class Serializer: KSerializer<TracingConfig> {
        override val descriptor: SerialDescriptor = buildClassSerialDescriptor("charted.tracing.TracingConfig") {
            element<TracerType>("type")
            element<ElasticAPM>("apm", isOptional = true)
            element<OpenTelemetry>("otel", isOptional = true)
        }

        override fun deserialize(decoder: Decoder): TracingConfig = decoder.decodeStructure(descriptor) {
            var type: TracerType? = null
            var config: TracingConfig? = null

            loop@ while (true) {
                when (val index = decodeElementIndex(descriptor)) {
                    CompositeDecoder.DECODE_DONE -> break@loop
                    0 -> {
                        type = decodeSerializableElement(descriptor, index, TracerType.serializer())
                        if (type == TracerType.Sentry) {
                            config = Sentry
                            break@loop
                        }
                    }

                    1 -> {
                        require(type == TracerType.ElasticAPM) { "Expected 'config.tracing.apm' to be 'apm', but got $type [$index]" }
                        config = decodeSerializableElement(descriptor, index, ElasticAPM.serializer())
                    }

                    2 -> {
                        require(type == TracerType.OpenTelemetry) { "Expected 'config.tracing.otel' to be 'otel', but got $type [$index]" }
                        config = decodeSerializableElement(descriptor, index, OpenTelemetry.serializer())
                    }

                    else -> throw SerializationException("Unable to serialize at index [$index]")
                }
            }

            checkNotNull(config) { "Configuration should be specified." }
            config
        }

        override fun serialize(encoder: Encoder, value: TracingConfig): Unit = encoder.encodeStructure(descriptor) {
            encodeSerializableElement(descriptor, 0, TracerType.serializer(), value.type)
            when (value) {
                is Sentry -> {}
                is ElasticAPM -> {
                    encodeSerializableElement(descriptor, 1, ElasticAPM.serializer(), value)
                }

                is OpenTelemetry -> {
                    encodeSerializableElement(descriptor, 2, OpenTelemetry.serializer(), value)
                }
            }
        }
    }
}
