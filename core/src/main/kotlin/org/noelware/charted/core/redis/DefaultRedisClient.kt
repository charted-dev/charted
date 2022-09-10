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

package org.noelware.charted.core.redis

import dev.floofy.utils.slf4j.logging
import io.lettuce.core.RedisClient
import io.lettuce.core.RedisURI
import io.lettuce.core.api.StatefulRedisConnection
import io.lettuce.core.api.async.RedisAsyncCommands
import kotlinx.coroutines.future.await
import kotlinx.coroutines.runBlocking
import org.apache.commons.lang3.time.StopWatch
import org.noelware.charted.common.IRedisClient
import org.noelware.charted.common.SetOnceGetValue
import org.noelware.charted.common.extensions.associateOrNull
import org.noelware.charted.common.stats.RedisStats
import org.noelware.charted.configuration.dsl.RedisConfig
import java.util.concurrent.TimeUnit
import kotlin.time.DurationUnit
import kotlin.time.toDuration

class DefaultRedisClient(config: RedisConfig): IRedisClient {
    private val _connection: SetOnceGetValue<StatefulRedisConnection<String, String>> = SetOnceGetValue()
    private val _commands: SetOnceGetValue<RedisAsyncCommands<String, String>> = SetOnceGetValue()
    private val log by logging<DefaultRedisClient>()
    private val client: RedisClient

    init {
        val url = if (config.sentinels.isNotEmpty()) {
            val builder = RedisURI.builder()
                .withSentinelMasterId(config.master)
                .withDatabase(config.index.toInt())

            for (host in config.sentinels) {
                val (h, port) = host.split(":")
                builder.withSentinel(h, Integer.parseInt(port))
            }

            if (config.password != null) {
                builder.withPassword(config.password!!.toCharArray())
            }

            builder.build()
        } else {
            val builder = RedisURI.builder()
                .withHost(config.host)
                .withPort(config.port.toInt())
                .withDatabase(config.index.toInt())

            if (config.password != null) {
                builder.withPassword(config.password!!.toCharArray())
            }

            builder.build()
        }

        client = RedisClient.create(url)
    }

    override fun getCommands(): RedisAsyncCommands<String, String> = _commands.value

    override fun getStats(): RedisStats {
        val ping = runBlocking {
            val sw = StopWatch.createStarted()
            commands.ping().await()

            sw.stop(); sw.getTime(TimeUnit.MILLISECONDS)
        }

        val stats = runBlocking { commands.info().await() }
            .split("\r\n?".toRegex())
            .drop(1)
            .dropLast(1)
            .filter { !it.startsWith("#") || it.isNotEmpty() }
            .associateOrNull {
                if (!it.startsWith("#") && it.isNotEmpty()) {
                    val (key, value) = it.split(':')
                    key to value
                } else {
                    null
                }
            }

        return RedisStats(
            stats["total_net_input_bytes"]?.toLong() ?: 0,
            stats["total_net_output_bytes"]?.toLong() ?: 0,
            stats["total_commands_processed"]?.toLong() ?: 0,
            stats["total_connections_received"]?.toLong() ?: 0,
            stats["mem_allocator"]!!,
            stats["uptime_in_seconds"]?.toLong()?.toDuration(DurationUnit.SECONDS)?.inWholeMilliseconds ?: -1,
            stats["redis_version"]!!,
            stats["redis_mode"]!!,
            ping
        )
    }

    override fun connect() {
        if (_connection.valueOrNull != null) return

        log.info("Connecting to Redis...")
        _connection.value = client.connect()
        _commands.value = _connection.value.async()

        log.info("Connected to Redis!")
    }

    override fun close() {
        if (_connection.valueOrNull == null) return

        log.warn("Closing Redis client...")
        runBlocking {
            _connection.value.closeAsync().await()
        }
    }
}
