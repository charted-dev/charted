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

@file:Suppress("MemberVisibilityCanBePrivate")

package org.noelware.charted.configuration.kotlin.dsl

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.ValidationException
import org.noelware.charted.common.Buildable
import org.noelware.charted.common.extensions.string.toUriOrNull
import org.noelware.charted.common.serializers.SecretStringSerializer
import kotlin.properties.Delegates

@Serializable
public data class NoelwareAnalyticsConfig(
    /**
     * The authentication token that is used to authenticate this daemon to the Noelware Analytics API server.
     */
    @SerialName("endpoint_auth")
    val endpointAuth: String? = null,

    /**
     * The bind address when creating the Noelware Analytics gRPC Daemon that is allowed by the [Analytics API Server](https://analytics.noelware.org/docs/api-server/current)
     * and can collect metrics from.
     */
    @SerialName("grpc_bind_ip")
    val grpcBindIp: String? = null,

    @SerialName("service_token")
    @Serializable(with = SecretStringSerializer::class)
    val serviceToken: String,

    /**
     * API server endpoint to connect to. This will use the official instance's endpoint
     * if this is not set (`https://analytics.noelware.org/api`)
     */
    val endpoint: String = "https://analytics.noelware.org/api",

    /**
     * Port range to listen to when the gRPC daemon is listening for incoming requests.
     */
    val port: Short = 10234
) {
    init {
        if (endpoint.toUriOrNull() == null) {
            throw ValidationException("config.analytics.endpoint", "Analytics endpoint must be a valid URI [received: $endpoint]")
        }
    }

    public class Builder: Buildable<NoelwareAnalyticsConfig> {
        /**
         * The authentication token that is used to authenticate this daemon to the Noelware Analytics API server.
         */
        public val endpointAuth: String? = null

        /**
         * The bind address when creating the Noelware Analytics gRPC Daemon that is allowed by the [Analytics API Server](https://analytics.noelware.org/docs/api-server/current)
         * and can collect metrics from.
         */
        public val grpcBindIp: String? = null

        public val serviceToken: String by Delegates.notNull()

        /**
         * API server endpoint to connect to. This will use the official instance's endpoint
         * if this is not set (`https://analytics.noelware.org/api`)
         */
        public val endpoint: String = "https://analytics.noelware.org/api"

        /**
         * Port range to listen to when the gRPC daemon is listening for incoming requests.
         */
        public val port: Short = 10234

        override fun build(): NoelwareAnalyticsConfig = NoelwareAnalyticsConfig(endpointAuth, grpcBindIp, serviceToken, endpoint, port)
    }
}
