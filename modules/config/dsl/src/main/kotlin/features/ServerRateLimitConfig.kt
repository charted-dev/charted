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

package org.noelware.charted.configuration.kotlin.dsl.features

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.common.TimeSpanValue
import org.noelware.charted.serializers.TimeSpanValueSerializer

/**
 * Feature to implement server-side rate-limiting when calling API requests. This doesn't apply
 * to CDN endpoints.
 *
 * This is useful if you have a public Helm Chart registry, and you want to limit API calls that
 * might hurt performance. This is enabled on the [official instance](https://charts.noelware.org),
 * though Noelware uses 750 requests for non-users and 1500 requests for logged-in users.
 *
 * @param authenticatedMaxRequests `config.server.ratelimit.authenticated_max_requests` determines the max requests for authenticated users
 * @param maxRequests              `config.server.ratelimit.max_requests` determines the max requests for non-authenticated users.
 * @param timeWindow               `config.server.ratelimit.time_window` is a [TimeSpanValue] of the calculated time to hit per request quota.
 * @param inMemory                 `config.server.ratelimit.in_memory` determines if the rate-limit cache should be in Redis (if `in_memory` is false) or in-memory (if `in_memory` is true)
 */
@Serializable
public data class ServerRateLimitConfig(
    @SerialName("authenticated_max_requests")
    val authenticatedMaxRequests: Int = 300,

    @SerialName("max_requests")
    val maxRequests: Int = 100,

    @Serializable(with = TimeSpanValueSerializer::class)
    @SerialName("time_window")
    val timeWindow: Long = TimeSpanValue.fromString("1hr"),

    @SerialName("in_memory")
    val inMemory: Boolean = false
) {
    @Suppress("MemberVisibilityCanBePrivate")
    public class Builder: org.noelware.charted.common.Builder<ServerRateLimitConfig> {
        /** Determines the max requests for authenticated users */
        public var authenticatedMaxRequests: Int = 300

        /** Determines the max requests for unauthenticated users */
        public var maxRequests: Int = 100

        /** [TimeSpanValue] of the calculated time to hit per request quota. */
        public var timeWindow: Long = TimeSpanValue.fromString("1hr")

        /** Determines if the rate-limit cache should be in Redis (if `in_memory` is false) or in-memory (if `in_memory` is true) */
        public var inMemory: Boolean = false

        override fun build(): ServerRateLimitConfig = ServerRateLimitConfig(authenticatedMaxRequests, maxRequests, timeWindow, inMemory)
    }
}
