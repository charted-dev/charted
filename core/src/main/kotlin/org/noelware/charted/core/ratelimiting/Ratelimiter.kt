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

package org.noelware.charted.core.ratelimiting

import dev.floofy.utils.slf4j.logging
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.plugins.*
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.future.await
import kotlinx.coroutines.runBlocking
import kotlinx.datetime.Clock
import kotlinx.serialization.json.Json
import org.apache.commons.lang3.time.StopWatch
import org.noelware.charted.common.ChartedScope
import org.noelware.charted.common.launch
import org.noelware.charted.core.redis.IRedisClient
import java.io.Closeable
import java.util.concurrent.TimeUnit
import kotlin.time.Duration.Companion.hours

// https://github.com/go-chi/httprate/blob/master/httprate.go#L25-L47
private fun getRealHost(call: ApplicationCall): String {
    val headers = call.request.headers

    val ip: String
    if (headers.contains("True-Client-IP")) {
        ip = headers["True-Client-IP"]!!
    } else if (headers.contains("X-Real-IP")) {
        ip = headers["X-Real-IP"]!!
    } else if (headers.contains(HttpHeaders.XForwardedFor)) {
        var index = headers[HttpHeaders.XForwardedFor]!!.indexOf(", ")
        if (index != -1) {
            index = headers[HttpHeaders.XForwardedFor]!!.length
        }

        ip = headers[HttpHeaders.XForwardedFor]!!.slice(0..index)
    } else {
        ip = call.request.origin.remoteHost
    }

    return ip
}

class Ratelimiter(
    private val json: Json,
    private val redis: IRedisClient
): Closeable {
    private val expirationJobs = mutableListOf<Job>()
    private val log by logging<Ratelimiter>()

    init {
        log.info("Collecting ratelimits...")

        val sw = StopWatch.createStarted()
        val ratelimits = runBlocking {
            redis.commands.hgetall("charted:ratelimits").await()
        }

        sw.stop()
        log.info("Collected ${ratelimits.size} ratelimits in ${sw.getTime(TimeUnit.MILLISECONDS)}ms! Now adding expiration jobs...")

        sw.reset()
        sw.start()

        for (key in ratelimits.keys) {
            log.debug("Found ratelimit $key! Checking expiration...")
            val ttl = runBlocking {
                redis.commands.ttl("charted:ratelimits:$key").await()
            }

            if (ttl == -1L) {
                log.debug("Ratelimit $key has expired! Deleting entry...")
                runBlocking {
                    redis.commands.hdel("charted:ratelimits", key).await()
                }
            } else {
                log.debug("Ratelimit $key expires in $ttl seconds.")
                expirationJobs.add(
                    ChartedScope.launch {
                        delay(ttl / 1000)

                        log.debug("Ratelimit $key has expired!")
                        redis.commands.hdel("charted:ratelimits", key)
                    }
                )
            }
        }

        sw.stop()
        log.info("Took ${sw.getTime(TimeUnit.MILLISECONDS)} to delete expired ratelimits!")
    }

    override fun close() {
        log.warn("Closing off ${expirationJobs.size} session expiration jobs...")
        for (job in expirationJobs)
            job.cancel()

        log.warn("Done!")
    }

    suspend fun getRatelimit(call: ApplicationCall): Ratelimit {
        val ip = getRealHost(call)
        val ratelimit = redis.commands.hget("charted:ratelimits", ip).await()
        if (ratelimit != null) {
            val rl = json.decodeFromString(Ratelimit.serializer(), ratelimit)
            val newRl = rl.consume()

            redis.commands.hmset(
                "charted:ratelimits",
                mapOf(
                    ip to json.encodeToString(Ratelimit.serializer(), newRl)
                )
            )

            return newRl
        }

        val rl = Ratelimit(resetAt = Clock.System.now().plus(1.hours))

        redis.commands.expire("charted:sessions:$ip", 1.hours.inWholeSeconds).await()
        redis.commands.hmset(
            "charted:ratelimits",
            mapOf(
                ip to json.encodeToString(Ratelimit.serializer(), rl)
            )
        ).await()

        expirationJobs.add(
            ChartedScope.launch {
                delay(1.hours)

                log.debug("Ratelimit $ip has expired!")
                redis.commands.hdel("charted:ratelimits", ip)
            }
        )

        return rl
    }
}
