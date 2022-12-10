package org.noelware.charted.server.ratelimiting

import io.ktor.server.plugins.ratelimit.*
import kotlinx.serialization.json.Json
import org.noelware.charted.modules.redis.RedisClient

/**
 * Represents a [RateLimiter] that uses Redis as the backend for it. You can define it in your `config.yml` file
 * with:
 *
 * ```yaml
 * server:
 *   ratelimit:
 *     time_window: 1hr # time window
 *     requests: 100    # limit of requests to run before a 429 is hit
 *     in_memory: false # Setting this to `false` will use Redis
 * ```
 */
class RedisRateLimiter(private val redis: RedisClient, private val json: Json): RateLimiter {
    override suspend fun tryConsume(tokens: Int): RateLimiter.State {
        TODO("Not yet implemented")
    }
}
