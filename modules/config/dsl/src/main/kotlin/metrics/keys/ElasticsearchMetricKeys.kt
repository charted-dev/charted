/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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
 * Represents all the metric set keys for using the Elasticsearch metrics collector. The `%s` is
 * necessary to fill in the index/node name.
 *
 * - `*` :: Enables all index and node-related metrics keys
 *
 * ## Managed Index Keys
 * The indexes that are available in the index keys are only associated with the ones that were created
 * by the API server, other indexes that were not created or used by the API server are never outputted here.
 *
 * - `charted_es_index_%s_deleted_documents` :: How many documents that were deleted in this index
 * - `charted_es_index_%s_avg_query_time`    :: Average query time in milliseconds
 * - `charted_es_index_%s_documents`         :: How many live documents are available in this index
 * - `charted_es_index_%s_health`            :: Current health of this index
 * - `charted_es_index_%s_size`              :: Size (in bytes) of the index
 * - `charted_es_index_%s_uuid`              :: UUID of the index
 * - `charted_es_index_%s_*`                 :: Wildcard to enable all index-related keys
 *
 * ## Elasticsearch Node Keys
 * - `charted_es_node_%s_deleted_documents` :: Count of deleted documents in all of this node's managed indexes
 * - `charted_es_node_%s_avg_indexing_time` :: Average indexing time in milliseconds
 * - `charted_es_node_%s_cpu_percentage`    :: CPU percentage that this node is taking up
 */
@Serializable(with = ElasticsearchMetricKeysSerializer::class)
@Suppress("ktlint:no-semi")
public enum class ElasticsearchMetricKeys(public val key: String) {
    // === Index-related Keys ===
    IndexDeletedDocuments("charted_es_index_%s_deleted_documents"),
    IndexAvgQueryTime("charted_es_index_%s_avg_query_time"),
    IndexTotalDocuments("charted_es_index_%s_documents"),
    IndexWildcard("charted_es_index_%s_*"),
    IndexHealth("charted_es_index_%s_health"),
    IndexSize("charted_es_index_%s_size"),
    IndexUUID("charted_es_index_%s_uuid"),

    // === Node-related Keys ===
    NodeDeletedDocuments("charted_es_node_%s_deleted_documents"),
    NodeAvgIndexTime("charted_es_node_%s_avg_indexing_time"),
    NodeCpuPercentage("charted_es_node_%s_cpu_percentage"),
    NodeIndexSize("charted_es_node_%s_index_size"),
    NodeShards("charted_es_node_%s_total_shards"),
    NodeWildcard("charted_es_node_%s_*"),

    Wildcard("*");
}

internal object ElasticsearchMetricKeysSerializer : KSerializer<ElasticsearchMetricKeys> {
    override val descriptor: SerialDescriptor = PrimitiveSerialDescriptor("charted.metrics.ElasticsearchMetricsKeys", PrimitiveKind.STRING)
    override fun deserialize(decoder: Decoder): ElasticsearchMetricKeys {
        val key = decoder.decodeString()
        return ElasticsearchMetricKeys.values().find {
            it.key == key
        } ?: throw SerializationException("Unknown key '$key'")
    }

    override fun serialize(encoder: Encoder, value: ElasticsearchMetricKeys) {
        encoder.encodeString(value.key)
    }
}
