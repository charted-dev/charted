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

package org.noelware.charted.core.redis

import dev.floofy.utils.slf4j.logging
import io.lettuce.core.api.StatefulRedisConnection
import io.lettuce.core.api.async.RedisAsyncCommands
import org.noelware.charted.core.SetOnceGetValue
import kotlin.time.Duration
import io.lettuce.core.RedisClient as LettuceRedisClient

class RedisClient: IRedisClient {
    private val _commands: SetOnceGetValue<RedisAsyncCommands<String, String>> = SetOnceGetValue()
    private val _connection: SetOnceGetValue<StatefulRedisConnection<String, String>> = SetOnceGetValue()
    private val log by logging<RedisClient>()
    private val client: LettuceRedisClient

    init {
        log.debug("Creating Redis client...")

        client = LettuceRedisClient.create()
    }

    override suspend fun stats(): RedisStats {
        TODO("Not yet implemented")
    }

    override suspend fun ping(): Duration {
        TODO("Not yet implemented")
    }

    override fun connect() {
        TODO("Not yet implemented")
    }

    override fun close() {
        TODO("Not yet implemented")
    }
}

/*
/*
class RedisManager(config: HanaConfig): AutoCloseable {
    private lateinit var connection: StatefulRedisConnection<String, String>
    lateinit var commands: RedisAsyncCommands<String, String>
    private val log by logging<RedisManager>()
    private val client: RedisClient

    init {
        log.info("* Creating Redis client...")

        val redisUri: RedisURI = if (config.redis.sentinels.isNotEmpty()) {
            val builder = RedisURI.builder()
                .withSentinelMasterId(config.redis.master)
                .withDatabase(config.redis.index)

            for (host in config.redis.sentinels) {
                val (h, port) = host.split(":")
                builder.withSentinel(h, Integer.parseInt(port))
            }

            if (config.redis.password != null) {
                builder.withPassword(config.redis.password.toCharArray())
            }

            builder.build()
        } else {
            val builder = RedisURI.builder()
                .withHost(config.redis.host)
                .withPort(config.redis.port)
                .withDatabase(config.redis.index)

            if (config.redis.password != null) {
                builder.withPassword(config.redis.password.toCharArray())
            }

            builder.build()
        }

        client = RedisClient.create(redisUri)
    }

    override fun close() {
        if (!::connection.isInitialized) return

        log.warn("Closing Redis connection...")
        connection.close()
        client.shutdown()
    }

    fun connect() {
        // Check if the connection was already established
        if (::connection.isInitialized) return

        log.info("* Creating the Redis connection...")
        connection = client.connect()
        commands = connection.async()

        log.info("* Connected to Redis!")
    }

    suspend fun getPing(): Duration {
        // If the connection wasn't established,
        // let's return 0.
        if (!::connection.isInitialized) return Duration.ZERO

        val sw = StopWatch.createStarted()
        commands.ping().await()
        sw.stop()

        return sw.time.toDuration(DurationUnit.MICROSECONDS)
    }

    suspend fun getStats(): RedisStats {
        val ping = getPing()

        // get statistics from connection
        val serverStats = commands.info("server").await()
        val stats = commands.info("stats").await()

        val mappedServerStats = serverStats!!
            .split("\r\n?".toRegex())
            .drop(1)
            .dropLast(1)
            .associate {
                val (key, value) = it.split(":")
                key to value
            }

        val mappedStats = stats!!
            .split("\r\n?".toRegex())
            .drop(1)
            .dropLast(1)
            .associate {
                val (key, value) = it.split(":")
                key to value
            }

        return RedisStats(
            mappedServerStats,
            mappedStats,
            ping
        )
    }
}
 */

 */
