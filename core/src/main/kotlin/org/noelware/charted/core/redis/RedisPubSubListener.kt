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

@file:JvmName("RedisPubSubListenerBuilderKt")

package org.noelware.charted.core.redis

import io.lettuce.core.pubsub.RedisPubSubListener
import kotlinx.coroutines.runBlocking

fun <K, V> createPubSubListener(
    onMessage: suspend (K, V, K?) -> Unit,
    onSubscribed: (suspend (K, Long) -> Unit)? = null,
    onUnsubscribed: (suspend (K, Long) -> Unit)? = null
): RedisPubSubListener<K, V> = object: RedisPubSubListener<K, V> {
    override fun message(channel: K, message: V) {
        runBlocking {
            onMessage.invoke(channel, message, null)
        }
    }

    override fun message(pattern: K, channel: K, message: V) {
        runBlocking {
            onMessage.invoke(channel, message, pattern)
        }
    }

    override fun subscribed(channel: K, count: Long) {
        if (onSubscribed != null) {
            runBlocking {
                onSubscribed.invoke(channel, count)
            }
        }
    }

    override fun psubscribed(pattern: K, count: Long) {
        if (onSubscribed != null) {
            runBlocking {
                onSubscribed.invoke(pattern, count)
            }
        }
    }

    override fun unsubscribed(channel: K, count: Long) {
        if (onUnsubscribed != null) {
            runBlocking {
                onUnsubscribed.invoke(channel, count)
            }
        }
    }

    override fun punsubscribed(pattern: K, count: Long) {
        if (onUnsubscribed != null) {
            runBlocking {
                onUnsubscribed.invoke(pattern, count)
            }
        }
    }
}
