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

package org.noelware.charted.core.ratelimiter

import dev.floofy.utils.slf4j.logging
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.plugins.*
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.future.await
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking
import kotlinx.datetime.Clock
import kotlinx.serialization.decodeFromString
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import org.apache.commons.lang3.time.StopWatch
import org.noelware.charted.common.ChartedScope
import org.noelware.charted.common.IRedisClient
import org.noelware.ktor.realIP
import java.io.Closeable
import java.util.concurrent.TimeUnit
import kotlin.time.Duration.Companion.hours

class Ratelimiter(private val json: Json, private val redis: IRedisClient): Closeable {
    private val expirationJobs = mutableListOf<Job>()
    private val log by logging<Ratelimiter>()

    init {
        log.debug("Collecting all ratelimits...")

        val sw = StopWatch.createStarted()
        val ratelimits = runBlocking {
            redis.commands.hgetall("charted:ratelimits").await()
        }

        sw.stop()
        log.debug("Took ${sw.getTime(TimeUnit.MILLISECONDS)}ms to collect ${ratelimits.size} ratelimits.")
        for ((index, key) in ratelimits.keys.withIndex()) {
            log.debug("|- Checking TTL limit for ratelimit #$index")
            sw.reset()
            sw.start()

            val ttl = runBlocking { redis.commands.ttl("ratelimit:$key").await() }
            sw.stop()

            log.debug("|- Took ${sw.getTime(TimeUnit.MILLISECONDS)}ms to find TTL for ratelimit #$index: [$ttl]")

            if (ttl == -2L) {
                log.debug("|  |- TTL key didn't exist for ratelimit #$index!")
                continue
            }

            if (ttl == -1L) {
                log.debug("|  |- TTL key exists but it has expired, deleting!")
                runBlocking { redis.commands.hdel("charted:ratelimit", key).await() }
            } else {
                log.debug("|  |- Ratelimit with key [$key] expires in $ttl seconds!")
                expirationJobs.add(
                    ChartedScope.launch {
                        delay(ttl / 1000)
                        redis.commands.hdel("charted:ratelimits", key).await()
                    }
                )
            }
        }
    }

    override fun close() {
        val sw = StopWatch.createStarted()
        log.warn("Closing off ${expirationJobs.size} ratelimit expiration jobs!")
        for (job in expirationJobs) job.cancel()

        log.warn("Completed in [${sw.getTime(TimeUnit.MILLISECONDS)}ms]")
    }

    suspend fun retrieve(call: ApplicationCall): Ratelimit {
        val ip = call.realIP
        val ratelimit = redis.commands.hget("charted:ratelimits", ip).await()
        if (ratelimit != null) {
            val rl = json.decodeFromString<Ratelimit>(ratelimit)
            val newRl = rl.consume()

            redis.commands.hmset(
                "charted:ratelimits",
                mapOf(ip to json.encodeToString(newRl))
            ).await()

            return newRl
        }

        val rl = Ratelimit(resetAt = Clock.System.now().plus(1.hours))
        redis.commands.expire(ip, 1.hours.inWholeSeconds).await()
        redis.commands.hmset(
            "charted:ratelimits",
            mapOf(ip to json.encodeToString(rl))
        ).await()

        expirationJobs.add(
            ChartedScope.launch {
                delay(1.hours)
                redis.commands.hdel("charted:ratelimits", ip).await()
            }
        )

        return rl
    }
}
