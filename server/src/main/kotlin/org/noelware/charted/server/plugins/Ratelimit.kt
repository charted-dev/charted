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
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.server.plugins

import dev.floofy.utils.koin.inject
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import kotlinx.datetime.Clock
import kotlinx.serialization.json.addJsonObject
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import kotlinx.serialization.json.putJsonArray
import org.noelware.charted.core.ratelimiting.Ratelimiter
import org.noelware.charted.core.ratelimiting.exceeded

val Ratelimit = createApplicationPlugin("Ratelimit") {
    val ratelimiter by inject<Ratelimiter>()

    onCall { call ->
        // Was it already handled? Let's not continue.
        if (call.isHandled) return@onCall

        val record = ratelimiter.getRatelimit(call)
        call.response.header("X-RateLimit-Limit", record.limit)
        call.response.header("X-RateLimit-Reset", record.resetAt.toEpochMilliseconds())
        call.response.header("X-RateLimit-Remaining", record.remaining)
        call.response.header("X-RateLimit-Reset-Date", record.resetAt.toString())

        if (record.exceeded) {
            val retryAfter = (record.resetAt.epochSeconds - Clock.System.now().epochSeconds).coerceAtLeast(0)
            call.response.header(HttpHeaders.RetryAfter, retryAfter)
            call.respond(
                HttpStatusCode.TooManyRequests,
                buildJsonObject {
                    put("success", false)
                    putJsonArray("errors") {
                        addJsonObject {
                            put("code", "RATELIMITED")
                            put("message", "You have been rate-limited! ((ヾ(≧皿≦；)ノ＿))Fuuuuuu—-！")
                        }
                    }
                }
            )

            return@onCall
        }
    }
}
