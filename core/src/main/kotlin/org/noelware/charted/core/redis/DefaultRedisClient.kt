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
import io.lettuce.core.RedisClient
import io.lettuce.core.RedisURI
import io.lettuce.core.api.StatefulRedisConnection
import io.lettuce.core.api.async.RedisAsyncCommands
import io.lettuce.core.pubsub.RedisPubSubListener
import io.lettuce.core.pubsub.StatefulRedisPubSubConnection
import io.lettuce.core.pubsub.api.sync.RedisPubSubCommands
import kotlinx.coroutines.future.await
import kotlinx.coroutines.runBlocking
import org.noelware.charted.common.IRedisClient
import org.noelware.charted.common.SetOnceGetValue
import org.noelware.charted.common.data.RedisConfig

class DefaultRedisClient(private val config: RedisConfig): IRedisClient {
    private val _connection: SetOnceGetValue<StatefulRedisConnection<String, String>> = SetOnceGetValue()
    private val _commands: SetOnceGetValue<RedisAsyncCommands<String, String>> = SetOnceGetValue()
    private val _pubsub: SetOnceGetValue<StatefulRedisPubSubConnection<String, String>> = SetOnceGetValue()
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
    override fun getPubSubCommands(): RedisPubSubCommands<String, String> = _pubsub.value.sync()
    override fun addPubSubListener(listener: RedisPubSubListener<String, String>) {
        _pubsub.value.addListener(listener)
    }

    override fun connect() {
        if (_connection.valueOrNull != null) return

        log.info("Connecting to Redis...")
        _connection.value = client.connect()
        _commands.value = _connection.value.async()

        log.info("Connected to Redis! Enabling keyspace notifications...")
        runBlocking {
            commands.configSet("notify-keyspace-events", "A").await()
        }

        log.info("Hopefully keyspace notifications are enabled, creating PubSub connection...")
        _pubsub.value = client.connectPubSub()

        val pubsubClient = _pubsub.value
        pubsubClient.sync().subscribe("__keyspace@:${config.index}__:*")
    }

    override fun close() {
        if (_connection.valueOrNull == null) return

        log.warn("Closing Redis client...")
        runBlocking {
            _connection.value.closeAsync().await()
        }
    }
}
