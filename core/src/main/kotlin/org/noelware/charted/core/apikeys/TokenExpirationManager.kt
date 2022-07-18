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

package org.noelware.charted.core.apikeys

import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.future.await
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking
import org.noelware.charted.common.ChartedScope
import org.noelware.charted.common.IRedisClient
import org.noelware.charted.core.redis.createPubSubListener
import kotlin.time.Duration.Companion.milliseconds

class TokenExpirationManager(private val redis: IRedisClient) {
    private val jobs = mutableListOf<Job>()

    init {
        val tokens = runBlocking { redis.commands.keys("apikeys:*").await() }
        for (key in tokens) {
            val ttl = runBlocking { redis.commands.ttl(key).await() }

            if (ttl == -2L) continue
            if (ttl == -1L) {
                runBlocking { redis.commands.del(key).await() }
            } else {
                jobs.add(
                    ChartedScope.launch {
                        delay(ttl.milliseconds.inWholeMilliseconds)

                        redis.commands.del(key).await()
                    }
                )
            }
        }

        redis.addPubSubListener(
            createPubSubListener<String, String>({ channel, message, pattern ->
                println("[channel=$channel; $message=message; $pattern=pattern]")

                if (!channel.matches("__keyevent@\\d{1,2}__:(\\w+)".toRegex())) {
                    return@createPubSubListener
                }
            })
        )
    }
}
