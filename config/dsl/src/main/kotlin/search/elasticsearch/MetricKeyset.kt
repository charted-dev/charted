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

package org.noelware.charted.configuration.kotlin.dsl.search.elasticsearch

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

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
 * - `index_%s_deleted_documents` :: How many documents that were deleted in this index
 * - `index_%s_avg_query_time`    :: Average query time in milliseconds
 * - `index_%s_documents`         :: How many live documents are available in this index
 * - `index_%s_health`            :: Current health of this index
 * - `index_%s_size`              :: Size (in bytes) of the index
 * - `index_%s_uuid`              :: UUID of the index
 * - `index_%s_*`                 :: Wildcard to enable all index-related keys
 *
 * ## Elasticsearch Node Keys
 * - `node_%s_deleted_documents` :: Count of deleted documents in all of this node's managed indexes
 * - `node_%s_avg_indexing_time` :: Average indexing time in milliseconds
 * - `node_%s_cpu_percentage`    :: CPU percentage that this node is taking up
 */
@Serializable
public enum class MetricKeyset {
    /** How many documents that were deleted in this index */
    @SerialName("index_%s_deleted_documents")
    IndexDeletedDocuments,

    /** Average query time in milliseconds */
    @SerialName("index_%s_avg_query_time")
    IndexAvgQueryTime,

    /** How many live documents are available in this index */
    @SerialName("index_%s_documents")
    IndexTotalDocuments,

    /** Wildcard type to enable all metrics for a specific index */
    @SerialName("index_%s_*")
    IndexWildcard,

    /** Current health of this index */
    @SerialName("es_index_%s_health")
    IndexHealth,

    /** Size (in bytes) of the index */
    @SerialName("index_%s_size")
    IndexSize,

    /** UUID of the index */
    @SerialName("index_%s_uuid")
    IndexUUID,

    // === Node-related Keys ===
    /** Count of deleted documents in all of this node's managed indexes */
    @SerialName("node_%s_deleted_documents")
    NodeDeletedDocuments,

    /** Average indexing time in milliseconds */
    @SerialName("node_%s_avg_indexing_time")
    NodeAvgIndexTime,

    /** CPU percentage that this node is taking up */
    @SerialName("node_%s_cpu_percentage")
    NodeCpuPercentage,

    /** Size of all the indexes in this node combined */
    @SerialName("node_%s_index_size")
    NodeIndexSize,

    /** Total shards in this node */
    @SerialName("%s_total_shards")
    NodeShards,

    /** Wildcard to enable all node metrics */
    @SerialName("node_%s_*")
    NodeWildcard,

    /** Wildcard to enable all index and node metrics */
    @SerialName("*")
    Wildcard
}

private val allWildcardsAvailable = listOf(MetricKeyset.Wildcard, MetricKeyset.IndexWildcard, MetricKeyset.NodeWildcard)

public fun List<MetricKeyset>.isNodeWildcard(): Boolean {
    if (isEmpty()) return false
    return if (size == 1) first() == MetricKeyset.NodeWildcard else any { it == MetricKeyset.NodeWildcard }
}

public fun List<MetricKeyset>.isIndexWildcard(): Boolean {
    if (isEmpty()) return false
    return if (size == 1) first() == MetricKeyset.IndexWildcard else any { it == MetricKeyset.IndexWildcard }
}

/**
 * Determine if the given list of [MetricKeyset] is a wildcard, so we list
 * all the options in the key-set instead of specific keys.
 */
public fun List<MetricKeyset>.isWildcard(): Boolean {
    if (isEmpty()) return false
    return if (size == 1) allWildcardsAvailable.contains(first()) else any { allWildcardsAvailable.contains(it) }
}
