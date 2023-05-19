/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

package org.noelware.charted.modules.caching.redis

import dev.floofy.utils.slf4j.logging
import io.lettuce.core.SetArgs
import kotlinx.coroutines.future.await
import kotlinx.serialization.*
import kotlinx.serialization.json.*
import org.noelware.charted.modules.caching.CacheWorker
import org.noelware.charted.modules.redis.RedisClient
import kotlin.reflect.KClass
import kotlin.time.Duration.Companion.minutes

class RedisCacheWorker(private val redis: RedisClient, private val json: Json): CacheWorker {
    private val log by logging<RedisCacheWorker>()

    override suspend fun invalidateAll() {
        val keys = redis.commands.keys("charted:cache:*").await()
        redis.commands.del(*keys.toTypedArray()).await()
    }

    @OptIn(InternalSerializationApi::class)
    @Suppress("UNCHECKED_CAST")
    override suspend fun <T: Any> getOrPut(key: String, klazz: KClass<T>, push: suspend (key: String) -> T?): T? {
        val value = redis.commands.get("charted:cache:$key").await()
        if (value == null) {
            log.trace("Cache miss: [charted:cache:$key]")

            val newObject = push(key) ?: return null
            val serializer = newObject::class.serializerOrNull()
                ?: throw IllegalStateException("Unable to populate cache for object [${newObject::class}] as it doesn't have a @Serializable attached to it.")

            val asJsonElement = json.encodeToJsonElement(serializer as KSerializer<T>, newObject)
            val serialized = json.encodeToString(
                buildJsonObject {
                    put("class_name", klazz.qualifiedName)
                    put("data", asJsonElement)
                },
            )

            log.debug(serialized)
            redis.commands.set(
                "charted:cache:$key", serialized,
                SetArgs().apply {
                    ex(15.minutes.inWholeSeconds)
                },
            ).await()

            return newObject
        }

        val obj: JsonObject = json.decodeFromString(value)
        if (obj["class_name"] == null) {
            redis.commands.del("charted:cache:$key").await()
            log.warn("Cache key [charted:cache:$key] was corrupted: was missing 'class_name' property.")

            return null
        }

        val registeredClassName: KClass<T>? = try {
            Class.forName(obj["class_name"]!!.jsonPrimitive.content).kotlin as? KClass<T>
        } catch (ignored: ClassNotFoundException) {
            null
        }

        if (registeredClassName != klazz) {
            redis.commands.del("charted:cache:$key").await()
            throw IllegalStateException("Tried to cast inferred class [${obj["class_name"]!!.jsonPrimitive.content}] from cache object to $klazz")
        }

        return null
    }
}
