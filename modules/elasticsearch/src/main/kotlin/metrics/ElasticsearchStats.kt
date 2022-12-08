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

package org.noelware.charted.modules.elasticsearch.metrics

import com.google.protobuf.Value
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.modules.analytics.kotlin.dsl.Struct
import org.noelware.charted.modules.analytics.kotlin.dsl.put
import org.noelware.charted.modules.analytics.kotlin.dsl.toGrpcValue
import org.noelware.charted.modules.elasticsearch.ElasticsearchModule
import org.noelware.charted.modules.metrics.GenericStatCollector

/**
 * Represents the statistics object for the Elasticsearch cluster relevant to
 * **charted-server**. This will collect node and index statistics that the server
 * uses for searching.
 *
 * @param indexes statistics for the indexes charted-server uses
 * @param nodes   node statistics
 */
@Serializable
data class ElasticsearchStats(
    val indexes: Map<String, IndexStats> = mapOf(),
    val nodes: Map<String, NodeStats> = mapOf()
): org.noelware.analytics.jvm.server.serialization.Serializable {
    override fun toGrpcValue(): Value = Struct {
        put(this, ElasticsearchStats::indexes)
        put(this, ElasticsearchStats::nodes)
    }.toGrpcValue()

    /**
     * Statistics object for a specific index
     *
     * @param averageQueryTimeInMs average query time in milliseconds
     * @param deletedDocuments     amount of deleted documents in this index
     * @param documents            amount of documents in this index
     * @param queries              amount of queries this index has performed (use [averageQueryTimeInMs] to check the average time)
     * @param health               health of this index
     * @param size                 size (in bytes) of the index
     * @param uuid                 index uuid (I don't know if u need this, but I don't care)
     */
    @Serializable
    data class IndexStats(
        @SerialName("avg_query_time_ms")
        val averageQueryTimeInMs: Long,

        @SerialName("deleted_documents")
        val deletedDocuments: Long,
        val documents: Long,
        val queries: Long,
        val health: String,
        val size: Long,
        val uuid: String
    ): org.noelware.analytics.jvm.server.serialization.Serializable {
        override fun toGrpcValue(): Value = Struct {
            put(this, IndexStats::averageQueryTimeInMs)
            put(this, IndexStats::deletedDocuments)
            put(this, IndexStats::documents)
            put(this, IndexStats::queries)
            put(this, IndexStats::health)
            put(this, IndexStats::size)
            put(this, IndexStats::uuid)
        }.toGrpcValue()
    }

    /**
     * Statistics object of all the Elasticsearch nodes that are used to perform indexing
     * and searching on the [indexes][ElasticsearchStats.indexes].
     *
     * @param averageIndexTimeInMs average time the node indexes documents (in milliseconds)
     * @param deletedDocuments     amount of deleted documents in all indices in this node
     * @param cpuPercentage        cpu percentage of the node.
     * @param totalIndexesSize     total indices size (in bytes)
     * @param shards               how many shards this index is using
     */
    @Serializable
    data class NodeStats(
        @SerialName("avg_index_time_ms")
        val averageIndexTimeInMs: Long,

        @SerialName("deleted_documents")
        val deletedDocuments: Long,

        @SerialName("cpu_percentage")
        val cpuPercentage: Double,

        @SerialName("total_indexes_size")
        val totalIndexesSize: Long,
        val documents: Long,
        val shards: Long
    ): org.noelware.analytics.jvm.server.serialization.Serializable {
        override fun toGrpcValue(): Value = Struct {
            put(this, NodeStats::averageIndexTimeInMs)
            put(this, NodeStats::totalIndexesSize)
            put(this, NodeStats::deletedDocuments)
            put(this, NodeStats::documents)
            put(this, NodeStats::shards)
        }.toGrpcValue()
    }

    /**
     * [GenericStatCollector] for getting the [elasticsearch stats][ElasticsearchModule.stats].
     */
    class Collector(private val elasticsearch: ElasticsearchModule): GenericStatCollector<ElasticsearchStats> {
        override val name: String = "elasticsearch"
        override fun collect(): ElasticsearchStats = runBlocking { elasticsearch.stats() }
    }
}
