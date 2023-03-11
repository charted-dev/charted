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

package org.noelware.charted.server.ratelimit

import io.ktor.server.plugins.ratelimit.*
import io.ktor.util.date.*
import kotlinx.coroutines.future.await
import kotlinx.serialization.decodeFromString
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import org.noelware.charted.modules.redis.RedisClient
import kotlin.time.Duration
import kotlin.time.Duration.Companion.hours
import kotlin.time.Duration.Companion.milliseconds

/**
 * Represents a [RateLimiter] that uses Redis as the backend for it.
 */
// This has been modified from Noelware's Accounting RateLimiter, which was written by
// Ice (https://winterfox.tech)
class RedisRateLimiter(
    private val redis: RedisClient,
    private val json: Json,
    private val rateLimitKey: String,
    private val rateLimitName: String,
    private val timeWindow: Duration = 1.hours
): RateLimiter {
    private val maxRequests = if (rateLimitName == "authenticated") 1500 else 45

    override suspend fun tryConsume(tokens: Int): RateLimiter.State {
        // Check if the given IP has a state already
        val old = redis.commands.hget("charted:ratelimits:$rateLimitName", rateLimitKey).await()
        if (old == null) {
            val rateLimit = RateLimit(getTimeMillis(), maxRequests - 1)
            redis.commands.hset(
                "charted:ratelimits:$rateLimitName",
                mapOf(
                    rateLimitKey to json.encodeToString(rateLimit),
                ),
            )

            return RateLimiter.State.Available(rateLimit.limit, maxRequests, rateLimit.lastRefillAt + timeWindow.inWholeMilliseconds)
        }

        val rateLimit = json.decodeFromString<RateLimit>(old)
        var remain = rateLimit.limit
        var lastRefillAt = rateLimit.lastRefillAt

        if (timeToWait(rateLimit.lastRefillAt) < 0) {
            remain = maxRequests
            lastRefillAt = getTimeMillis()
        }

        if ((remain - tokens) < 0) return RateLimiter.State.Exhausted(timeToWait(lastRefillAt).milliseconds)

        val newRateLimit = RateLimit(lastRefillAt, remain)
        redis.commands.hset(
            "charted:ratelimits:$rateLimitName",
            mapOf(
                rateLimitKey to json.encodeToString(newRateLimit),
            ),
        ).await()

        return RateLimiter.State.Available(remain, maxRequests, lastRefillAt + timeWindow.inWholeMilliseconds)
    }

    private fun timeToWait(refill: Long): Long = timeWindow.inWholeMilliseconds - (getTimeMillis() - refill)
}
