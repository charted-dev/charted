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

package org.noelware.charted.configuration.kotlin.dsl

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.common.ByteSizeValue
import org.noelware.charted.configuration.kotlin.dsl.features.ServerRateLimitConfig
import org.noelware.charted.serializers.ByteSizeValueSerializer

@Serializable
public data class KtorServerConfig(
    /**
     * If we should add additional security headers to the response.
     */
    @SerialName("security_headers")
    val securityHeaders: Boolean = true,

    /**
     * Size of the queue to store all the application call instances
     * that cannot be immediately processed.
     */
    @SerialName("request_queue_limit")
    val requestQueueLimit: Int = 16,

    /**
     * Number of concurrently running requests from the same HTTP pipeline
     */
    @SerialName("running_limit")
    val runningLimit: Int = 10,

    /**
     * Do not create separate call event groups and reuse worker
     * groups for processing calls.
     */
    @SerialName("share_work_group")
    val shareWorkGroup: Boolean = false,

    /**
     * Timeout in seconds for sending responses to the client.
     */
    @SerialName("response_write_timeout")
    val responseWriteTimeoutSeconds: Int = 60, // expand this to 60 so /metrics can properly work (since elasticsearch takes a while)

    /**
     * Timeout in seconds to read incoming requests from the client, "0" = infinite.
     */
    @SerialName("request_read_timeout")
    val requestReadTimeout: Int = 0,

    /**
     * If this is set to `true`, this will enable TCP keep alive for
     * connections that are so-called "dead" and can be easily discarded.
     *
     * The timeout period is configured by the system, so configure
     * the end host accordingly.
     */
    @SerialName("keep_alive")
    val tcpKeepAlive: Boolean = false,

    /**
     * Append extra headers when sending out a response.
     */
    @SerialName("extra_headers")
    val extraHeaders: Map<String, String> = mapOf(),

    /**
     * Returns how many bytes that any request can send back to the server. The default
     * is 50MB before the server will throw a INTERNAL_SERVER_ERROR code. This is tailored to
     * your liking, 50MB is just a nice default.
     */
    @SerialName("max_data_payload")
    @Serializable(with = ByteSizeValueSerializer::class)
    val maxDataPayload: Long = ByteSizeValue.fromString("50mb"),

    /**
     * Configures SSL on the server.
     */
    val ssl: KtorSSLConfig? = null,

    /**
     * The connector host to use. Defaults to `0.0.0.0` for all connections
     * to pass through. Use `127.0.0.1` to only allow the connection via your
     * network.
     */
    val host: String = "0.0.0.0",

    /**
     * The port to listen on. Defaults to `3651`.
     */
    val port: Int = 3651,

    /**
     * Represents the configuration for configurating server-side rate-limiting.
     */
    val rateLimit: ServerRateLimitConfig? = null
) {
    @Suppress("MemberVisibilityCanBePrivate")
    public class Builder: org.noelware.charted.common.Builder<KtorServerConfig> {
        /**
         * If we should add additional security headers to the response.
         */
        public var securityHeaders: Boolean = true

        /**
         * Size of the queue to store all the application call instances
         * that cannot be immediately processed.
         */
        public var requestQueueLimit: Int = 16

        /**
         * Number of concurrently running requests from the same HTTP pipeline
         */
        public var runningLimit: Int = 10

        /**
         * Do not create separate call event groups and reuse worker
         * groups for processing calls.
         */
        public var shareWorkGroup: Boolean = false

        /**
         * Timeout in seconds for sending responses to the client.
         */
        public var responseWriteTimeoutSeconds: Int = 10

        /**
         * Timeout in seconds to read incoming requests from the client, "0" = infinite.
         */
        public var requestReadTimeout: Int = 0

        /**
         * If this is set to `true`, this will enable TCP keep alive for
         * connections that are so-called "dead" and can be easily discarded.
         *
         * The timeout period is configured by the system, so configure
         * the end host accordingly.
         */
        public var tcpKeepAlive: Boolean = false

        /**
         * Append extra headers when sending out a response.
         */
        private val extraHeaders: MutableMap<String, String> = mutableMapOf()

        /**
         * Represents the configuration for configuring server-side rate-limiting.
         */
        private var rateLimit: ServerRateLimitConfig? = null

        /**
         * Configures SSL on the server.
         */
        private var ssl: KtorSSLConfig? = null

        /**
         * Returns how many bytes that any request can send back to the server. The default
         * is 50MB before the server will throw a INTERNAL_SERVER_ERROR code. This is tailoured to
         * your liking, 50MB is just a nice default.
         */
        public var maxDataPayload: Long = ByteSizeValue.fromString("50mb")

        /**
         * The connector host to use. Defaults to `0.0.0.0` for all connections
         * to pass through. Use `127.0.0.1` to only allow the connection via your
         * network.
         */
        public var host: String = "0.0.0.0"

        /**
         * The port to listen on. Defaults to `3651`.
         */
        public var port: Int = 3651

        /**
         * Appends a custom header to the server when a request is made
         * @param key The header key
         * @param value The header value
         */
        public fun addHeader(key: String, value: String): Builder {
            if (extraHeaders.containsKey(key)) return this

            extraHeaders[key] = value
            return this
        }

        /**
         * Configures SSL connections
         * @param builder Builder DSL to configure SSL
         */
        public fun ssl(builder: KtorSSLConfig.Builder.() -> Unit = {}): Builder {
            ssl = KtorSSLConfig.Builder().apply(builder).build()
            return this
        }

        /**
         * Configures server-side rate-limiting on API calls, not CDN endpoints
         * @param builder Builder DSL to configure server side rate-limiting
         */
        public fun rateLimit(builder: ServerRateLimitConfig.Builder.() -> Unit = {}): Builder {
            rateLimit = ServerRateLimitConfig.Builder().apply(builder).build()
            return this
        }

        override fun build(): KtorServerConfig = KtorServerConfig(
            securityHeaders,
            requestQueueLimit,
            runningLimit,
            shareWorkGroup,
            responseWriteTimeoutSeconds,
            requestReadTimeout,
            tcpKeepAlive,
            extraHeaders,
            maxDataPayload,
            ssl,
            host,
            port,
            rateLimit
        )
    }
}
