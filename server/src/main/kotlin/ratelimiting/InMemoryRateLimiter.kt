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

package org.noelware.charted.server.ratelimiting

import com.github.benmanes.caffeine.cache.Caffeine
import com.github.benmanes.caffeine.cache.stats.CacheStats
import io.ktor.server.plugins.ratelimit.*
import io.ktor.util.date.*
import kotlin.time.Duration
import kotlin.time.Duration.Companion.hours
import kotlin.time.Duration.Companion.milliseconds
import kotlin.time.toJavaDuration

// This has been modified from Noelware's Accounting RateLimiter, which was written by
// Ice (https://winterfox.tech)
class InMemoryRateLimiter(
    rateLimitName: String,
    private val rateLimitKey: String,
    private val timeWindow: Duration = 1.hours
): RateLimiter {
    private val maxRequests = if (rateLimitName == "authenticated") 1500 else 45
    private val cache = Caffeine.newBuilder()
        .expireAfterAccess(timeWindow.toJavaDuration())
        .build<String, RateLimit>()

    /**
     * Returns the statistics for the in-memory cache for the rate-limits.
     */
    fun stats(): CacheStats = cache.stats()
    override suspend fun tryConsume(tokens: Int): RateLimiter.State {
        val old = cache.getIfPresent(rateLimitKey) ?: return run {
            val rateLimit = RateLimit(getTimeMillis(), maxRequests - 1)
            cache.put(rateLimitKey, rateLimit)

            RateLimiter.State.Available(rateLimit.limit, maxRequests, rateLimit.lastRefillAt + timeWindow.inWholeMilliseconds)
        }

        var remain = old.limit
        var lastRefillAt = old.lastRefillAt

        if (timeToWait(old.lastRefillAt) < 0) {
            remain = maxRequests
            lastRefillAt = getTimeMillis()
        }

        if ((remain - tokens) < 0) {
            cache.invalidate(rateLimitKey)
            return RateLimiter.State.Exhausted(timeToWait(lastRefillAt).milliseconds)
        }

        cache.invalidate(rateLimitKey)
        cache.put(rateLimitKey, old.copy(lastRefillAt = lastRefillAt, limit = remain))

        return RateLimiter.State.Available(remain, maxRequests, lastRefillAt + timeWindow.inWholeMilliseconds)
    }

    private fun timeToWait(refill: Long): Long = timeWindow.inWholeMilliseconds - (getTimeMillis() - refill)
}
