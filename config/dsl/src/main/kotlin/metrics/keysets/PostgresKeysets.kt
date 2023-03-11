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

package org.noelware.charted.configuration.kotlin.dsl.metrics.keysets

import kotlinx.serialization.KSerializer
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.SerializationException
import kotlinx.serialization.descriptors.PrimitiveKind
import kotlinx.serialization.descriptors.PrimitiveSerialDescriptor
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import org.noelware.charted.configuration.kotlin.dsl.server.MetricKeysets

@Serializable(with = PostgresKeysets.Companion.Serializer::class)
public enum class PostgresKeysets(public val key: String) {
    @SerialName("charted_postgres_total_organizations")
    TotalOrganizationsAvailable("charted_postgres_total_organizations"),

    @SerialName("charted_postgres_total_repositories")
    TotalRepositoriesAvailable("charted_postgres_total_repositories"),

    @SerialName("charted_postgres_total_users")
    TotalUsersAvailable("charted_postgres_total_users"),

    @SerialName("charted_postgres_database_size")
    DatabaseSize("charted_postgres_database_size"),

    @SerialName("charted_postgres_server_uptime")
    ServerUptime("charted_postgres_server_uptime"),

    @SerialName("*")
    Wildcard("*"),

    @SerialName("charted_postgres_version")
    Version("charted_postgres_version");

    internal companion object {
        internal object Serializer: KSerializer<PostgresKeysets> {
            override val descriptor: SerialDescriptor = PrimitiveSerialDescriptor("charted.metrics.PostgresKeysets", PrimitiveKind.STRING)
            override fun deserialize(decoder: Decoder): PostgresKeysets {
                val key = decoder.decodeString()
                return PostgresKeysets.values().find { it.key == key } ?: throw SerializationException("Unknown enum key '$key'")
            }

            override fun serialize(encoder: Encoder, value: PostgresKeysets) {
                encoder.encodeString(value.key)
            }
        }
    }
}

/**
 * Determine if the given list of [MetricKeysets] is a wildcard, so we list
 * all the options in the key-set instead of specific keys.
 */
public fun List<PostgresKeysets>.isWildcard(): Boolean {
    if (isEmpty()) return false
    if (size == 1) return first() == PostgresKeysets.Wildcard

    return any { it == PostgresKeysets.Wildcard }
}
