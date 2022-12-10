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
