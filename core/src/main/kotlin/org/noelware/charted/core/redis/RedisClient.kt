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
import io.lettuce.core.RedisURI
import io.lettuce.core.api.StatefulRedisConnection
import io.lettuce.core.api.async.RedisAsyncCommands
import kotlinx.coroutines.future.await
import kotlinx.coroutines.runBlocking
import org.apache.commons.lang3.time.StopWatch
import org.noelware.charted.common.SetOnceGetValue
import org.noelware.charted.common.config.RedisConfig
import kotlin.time.Duration
import kotlin.time.DurationUnit
import kotlin.time.toDuration
import io.lettuce.core.RedisClient as LettuceRedisClient

class RedisClient(config: RedisConfig): IRedisClient {
    private val _commands: SetOnceGetValue<RedisAsyncCommands<String, String>> = SetOnceGetValue()
    private val _connection: SetOnceGetValue<StatefulRedisConnection<String, String>> = SetOnceGetValue()
    private val log by logging<RedisClient>()
    private val client: LettuceRedisClient

    override val commands: RedisAsyncCommands<String, String>
        get() = _commands.value

    init {
        log.debug("Creating Redis client...")

        val uri = if (config.sentinels.isNotEmpty()) {
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

        client = LettuceRedisClient.create(uri)
    }

    override suspend fun stats(): RedisStats {
        try {
            _connection.value
        } catch (e: IllegalStateException) {
            null
        } catch (e: Throwable) {
            throw e
        } ?: return RedisStats(mapOf(), mapOf(), Duration.ZERO)

        val serverStats = commands.info("server").await()
        val stats = commands.info("stats").await()

        return RedisStats(
            serverStats!!.split("\r\n?".toRegex()).drop(1).dropLast(1).associate {
                val (key, value) = it.split(":")
                key to value
            },

            stats!!.split("\r\n?".toRegex()).drop(1).dropLast(1).associate {
                val (key, value) = it.split(":")
                key to value
            },

            ping()
        )
    }

    override suspend fun ping(): Duration {
        try {
            _connection.value
        } catch (e: IllegalStateException) {
            null
        } catch (e: Throwable) {
            throw e
        } ?: return Duration.ZERO

        val sw = StopWatch.createStarted()
        commands.ping().await()
        sw.stop()

        return sw.time.toDuration(DurationUnit.MICROSECONDS)
    }

    override fun connect() {
        if (_connection.valueOrNull != null) return

        log.info("Connecting to Redis...")
        _connection.value = client.connect()
        _commands.value = _connection.value.async()

        log.info("Connected to Redis!")
    }

    override fun close() {
        try {
            _connection.value
        } catch (e: IllegalStateException) {
            null
        } catch (e: Throwable) {
            throw e
        } ?: return

        log.warn("Closing Redis client...")
        runBlocking { _connection.value.closeAsync().await() }
    }
}
