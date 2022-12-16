/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware <team@noelware.org>
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

package org.noelware.charted.configuration.kotlin.dsl.metrics.keys

import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable
import kotlinx.serialization.SerializationException
import kotlinx.serialization.descriptors.PrimitiveKind
import kotlinx.serialization.descriptors.PrimitiveSerialDescriptor
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder

/**
 * Represents all the metric keys that can be toggle-able with the `config.metrics.sets.redis` configuration
 * key.
 */
@Serializable(with = RedisMetricKeysSerializer::class)
enum class RedisMetricKeys(val key: String) {
    TotalConnectionsReceived("charted_redis_total_connections_received"),
    TotalCommandsProcessed("charted_redis_total_commands_processed"),
    TotalNetworkOutput("charted_redis_total_net_output"),
    TotalNetworkInput("charted_redis_total_net_input"),
    Allocator("charted_redis_allocator"),
    Version("charted_redis_version"),
    Mode("charted_redis_mode"),
    Ping("charted_redis_ping"),
    Wildcard("*");
}

object RedisMetricKeysSerializer: KSerializer<RedisMetricKeys> {
    override val descriptor: SerialDescriptor = PrimitiveSerialDescriptor("charted.metrics.RedisKeys", PrimitiveKind.STRING)
    override fun deserialize(decoder: Decoder): RedisMetricKeys {
        val key = decoder.decodeString()
        return RedisMetricKeys.values().find {
            it.key == key
        } ?: throw SerializationException("Unknown key '$key'")
    }

    override fun serialize(encoder: Encoder, value: RedisMetricKeys) {
        encoder.encodeString(value.key)
    }
}
