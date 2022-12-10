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
data class ServerRateLimitConfig(
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
    class Builder: org.noelware.charted.common.Builder<ServerRateLimitConfig> {
        /** Determines the max requests for authenticated users */
        var authenticatedMaxRequests: Int = 300

        /** Determines the max requests for unauthenticated users */
        var maxRequests: Int = 100

        /** [TimeSpanValue] of the calculated time to hit per request quota. */
        var timeWindow: Long = TimeSpanValue.fromString("1hr")

        /** Determines if the rate-limit cache should be in Redis (if `in_memory` is false) or in-memory (if `in_memory` is true) */
        var inMemory: Boolean = false

        override fun build(): ServerRateLimitConfig = ServerRateLimitConfig(authenticatedMaxRequests, maxRequests, timeWindow, inMemory)
    }
}
