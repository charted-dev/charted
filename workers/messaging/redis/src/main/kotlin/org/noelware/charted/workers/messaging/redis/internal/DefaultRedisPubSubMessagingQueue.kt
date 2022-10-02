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

package org.noelware.charted.workers.messaging.redis.internal

import dev.floofy.utils.slf4j.logging
import io.lettuce.core.pubsub.StatefulRedisPubSubConnection
import kotlinx.coroutines.future.await
import kotlinx.coroutines.runBlocking
import org.noelware.charted.common.IRedisClient
import org.noelware.charted.common.SetOnceGetValue
import org.noelware.charted.workers.messaging.queue.WorkerQueue
import org.noelware.charted.workers.messaging.redis.RedisPubSubMessagingQueue
import java.util.concurrent.atomic.AtomicBoolean

class DefaultRedisPubSubMessagingQueue(private val redis: IRedisClient, private val config: Any): RedisPubSubMessagingQueue {
    private val _closed: AtomicBoolean = AtomicBoolean(false)
    private val _pubsub: SetOnceGetValue<StatefulRedisPubSubConnection<String, String>> = SetOnceGetValue()
    private val _queues: MutableMap<String, WorkerQueue> = mutableMapOf()
    private val log by logging<DefaultRedisPubSubMessagingQueue>()

    override val queues: Map<String, WorkerQueue> = _queues
    override val closed: Boolean = _closed.get()
    override val name: String = "redis"

    override fun close() {
        if (_closed.compareAndSet(false, true)) {
            log.warn("Closing off PubSub connection (and releasing worker queues from metadata)")

            for (queue in _queues.keys) {
                runBlocking { redis.commands.hdel("queue:$queue").await() }
            }

            runBlocking { _pubsub.value.closeAsync().await() }
        }
    }

    override fun <T: WorkerQueue> registerQueue(queue: T) {
        TODO("Not yet implemented")
    }

    override suspend fun handleTrigger(name: String, vararg args: Any) {
        TODO("Not yet implemented")
    }

    override fun connect() {
    }
}
