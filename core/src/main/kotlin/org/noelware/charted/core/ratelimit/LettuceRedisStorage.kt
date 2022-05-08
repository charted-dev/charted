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

package org.noelware.charted.core.ratelimit

import app.softwork.ratelimit.Storage
import dev.floofy.utils.koin.inject
import kotlinx.coroutines.future.await
import kotlinx.datetime.Clock
import kotlinx.datetime.Instant
import kotlinx.serialization.json.*
import org.noelware.charted.core.extensions.instant
import org.noelware.charted.core.redis.IRedisClient

class LettuceRedisStorage: Storage {
    private val client: IRedisClient by inject()
    private val json: Json by inject()

    override val clock: Clock = Clock.System
    override suspend fun getOrNull(host: String): Storage.Requested? {
        val data = client.commands.hget("charted:ratelimit", host).await() ?: return null
        val serialized = json.decodeFromString(JsonObject.serializer(), data)

        return object: Storage.Requested {
            override val lastRequest: Instant = serialized["last_request"]!!.jsonPrimitive.instant
            override val trial: Int = serialized["trials"]!!.jsonPrimitive.int
        }
    }

    override suspend fun set(host: String, trial: Int, lastRequest: Instant) {
        val data = buildJsonObject {
            put("last_request", lastRequest.toString())
            put("trials", trial)
        }

        val serialized = json.encodeToString(JsonObject.serializer(), data)
        client.commands.hset(
            "charted:ratelimit",
            mapOf(
                host to serialized
            )
        )
    }

    override suspend fun remove(host: String) {
        client.commands.hdel("charted:ratelimit", host)
    }
}
