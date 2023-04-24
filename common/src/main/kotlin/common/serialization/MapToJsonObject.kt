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

package org.noelware.charted.common.serialization

import dev.floofy.utils.koin.retrieveOrNull
import kotlinx.serialization.*
import kotlinx.serialization.json.*
import org.koin.core.context.GlobalContext

private val json: Json
    get() = GlobalContext.retrieveOrNull() ?: Json {
        ignoreUnknownKeys = false
        isLenient = true
        encodeDefaults = false
    }

public fun <V> Map<String, V>.toJsonObject(): JsonObject = buildJsonObject {
    for ((key, value) in entries) {
        put(key, value.toJsonElement())
    }
}

@OptIn(ExperimentalSerializationApi::class, InternalSerializationApi::class)
public fun Any?.toJsonElement(): JsonElement = when (this) {
    null -> JsonNull
    is String -> JsonPrimitive(this)
    is Number -> JsonPrimitive(this)
    is Boolean -> JsonPrimitive(this)
    is List<*> -> JsonArray(map { toJsonElement() })
    else -> {
        val serializer = json.serializersModule.getContextual(this::class)
            ?: this::class.serializerOrNull()
            ?: error("Unable to grab contextual serializer for [${this::class}]")

        @Suppress("UNCHECKED_CAST")
        json.encodeToJsonElement(serializer as KSerializer<Any>, this)
    }
}
