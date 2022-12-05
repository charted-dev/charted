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

package org.noelware.charted.modules.redis

import co.elastic.apm.api.Traced
import dev.floofy.utils.slf4j.logging
import io.lettuce.core.RedisURI
import io.lettuce.core.api.StatefulRedisConnection
import io.lettuce.core.api.async.RedisAsyncCommands
import kotlinx.coroutines.future.await
import kotlinx.coroutines.runBlocking
import org.apache.commons.lang3.time.StopWatch
import org.noelware.charted.common.SetOnce
import org.noelware.charted.configuration.kotlin.dsl.RedisConfig
import org.noelware.charted.extensions.associateOrNull
import org.noelware.charted.extensions.doFormatTime
import org.noelware.charted.modules.redis.metrics.RedisServerStats
import java.util.concurrent.TimeUnit
import kotlin.time.DurationUnit
import kotlin.time.toDuration
import io.lettuce.core.RedisClient as LettuceRedisClient

class DefaultRedisClient(config: RedisConfig): RedisClient {
    private val _connection: SetOnce<StatefulRedisConnection<String, String>> = SetOnce()
    private val _commands: SetOnce<RedisAsyncCommands<String, String>> = SetOnce()
    private val client: LettuceRedisClient
    private val log by logging<DefaultRedisClient>()

    init {
        log.debug("Creating Redis client...")

        val redisURI = if (config.sentinels.isNotEmpty() && config.master != null) {
            log.debug("Redis configuration is set to be using a Sentinel connection!")
            val builder = RedisURI.builder()
                .withSentinelMasterId(config.master)
                .withDatabase(config.index)

            for (host in config.sentinels) {
                if (host.startsWith("redis://")) {
                    builder.withSentinel(RedisURI.create(host))
                    continue
                }

                val h = host.split(":", limit = 2)
                if (h.size != 2) {
                    throw IllegalArgumentException("Sentinel host format must be 'host:port', received $host")
                }

                builder.withSentinel(h.first(), h.last().toInt(), config.password)
            }

            if (config.password != null) {
                builder.withPassword(config.password!!.toCharArray())
            }

            builder.build()
        } else {
            log.debug("Configuration is set to use a standalone, single Redis server connection!")

            val builder = RedisURI.builder()
                .withHost(config.host)
                .withPort(config.port)
                .withDatabase(config.index)

            if (config.password != null) {
                builder.withPassword(config.password!!.toCharArray())
            }

            builder.build()
        }

        log.debug("Configured Redis URI ~> [$redisURI]")
        client = LettuceRedisClient.create(redisURI)
    }

    /**
     * Returns all the commands you can execute on the Redis server.
     */
    override val commands: RedisAsyncCommands<String, String>
        get() = _commands.value

    /**
     * Returns the [RedisServerStats] object from the Redis server itself.
     */
    @Traced
    override fun stats(): RedisServerStats {
        val ping = runBlocking {
            val sw = StopWatch.createStarted()
            commands.ping().await()

            sw.stop()
            sw.getTime(TimeUnit.NANOSECONDS)
        }

        val stats = runBlocking { commands.info().await() }
            .split("\r\n?".toRegex())
            .drop(1)
            .dropLast(1)
            .filter { it.isNotBlank() || !it.startsWith("#") }
            .associateOrNull {
                if (!it.startsWith("#") && it.isNotEmpty()) {
                    val (key, value) = it.split(':')
                    key to value
                } else {
                    null
                }
            }

        return RedisServerStats(
            stats["total_net_input_bytes"]?.toLong() ?: 0,
            stats["total_net_output_bytes"]?.toLong() ?: 0,
            stats["total_commands_processed"]?.toLong() ?: 0,
            stats["total_connections_received"]?.toLong() ?: 0,
            stats["mem_allocator"] ?: "unknown",
            stats["uptime_in_seconds"]?.toLong()?.toDuration(DurationUnit.SECONDS)?.inWholeMilliseconds ?: -1,
            stats["redis_version"] ?: "unknown",
            stats["redis_mode"] ?: "unknown",
            ping
        )
    }

    /**
     * Connects to the Redis server
     */
    override suspend fun connect() {
        if (_connection.wasSet()) return

        val sw = StopWatch.createStarted()
        log.info("Connecting to the Redis server...")

        val connection = client.connect()
        log.info("Checking connection...")

        connection.async().ping().await()

        sw.stop()
        log.info("Connected to Redis in [${sw.doFormatTime()}]")

        _connection.value = connection
        _commands.value = connection.async()
    }

    /**
     * Closes this stream and releases any system resources associated
     * with it. If the stream is already closed then invoking this
     * method has no effect.
     *
     * As noted in [AutoCloseable.close], cases where the
     * close may fail require careful attention. It is strongly advised
     * to relinquish the underlying resources and to internally
     * *mark* the `Closeable` as closed, prior to throwing
     * the `IOException`.
     *
     * @throws java.io.IOException if an I/O error occurs
     */
    override fun close() {
        if (!_connection.wasSet()) return

        log.warn("Shutting down Redis connection...")
        _connection.value.close()
    }
}
