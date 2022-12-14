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

package org.noelware.charted.configuration.kotlin.dsl

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.ValidationException
import org.noelware.charted.extensions.toUriOrNull
import kotlin.properties.Delegates

@Serializable
data class NoelwareAnalyticsConfig(
    @SerialName("service_token")
    val serviceToken: String,
    val port: Int = 10234,
    val endpoint: String = "https://analytics.noelware.org",
    @SerialName("grpc_bind_ip")
    val grpcBindIp: String?,
    @SerialName("endpoint_auth")
    val endpointAuth: String?
) {
    init {
        if (endpoint.toUriOrNull() == null) throw ValidationException("config.analytics.endpoint", String.format("Analytics endpoint must be a valid URI, instead got %s", endpoint));
        if (port !in 1024..65535) {
            throw ValidationException("config.analytics.port", "Analytics server port must be in range of [1024..65535]")
        }
    }

    class Builder: org.noelware.charted.common.Builder<NoelwareAnalyticsConfig> {
        private var serviceToken: String by Delegates.notNull()
        private var port: Int = 10234
        private var endpoint: String = "https://analytics.noelware.org"

        override fun build(): NoelwareAnalyticsConfig = NoelwareAnalyticsConfig(serviceToken, port, endpoint, null, null)
    }
}
