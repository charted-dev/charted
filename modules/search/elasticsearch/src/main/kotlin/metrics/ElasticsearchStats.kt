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

package org.noelware.charted.modules.search.elasticsearch.metrics

import io.prometheus.client.GaugeMetricFamily
import io.prometheus.client.Predicate
import io.prometheus.client.SampleNameFilter
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.enumSets.serialName
import org.noelware.charted.configuration.kotlin.dsl.search.elasticsearch.MetricKeyset
import org.noelware.charted.modules.search.elasticsearch.ElasticsearchModule

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
) {
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
    )

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
    )

    class Collector(
        private val elasticsearchModule: ElasticsearchModule,
        private val config: Config
    ): org.noelware.charted.modules.metrics.Collector<ElasticsearchStats>, io.prometheus.client.Collector() {
        override val name: String = "elasticsearch"
        override suspend fun supply(): ElasticsearchStats = elasticsearchModule.stats()

        override fun collect(): MutableList<MetricFamilySamples> = collect {
            MetricKeyset.EnumSet.enabled(config.metrics.metricSets.elasticsearch, it)
        }

        override fun collect(sampleNameFilter: Predicate<String>?): MutableList<MetricFamilySamples> {
            val mfs = mutableListOf<MetricFamilySamples>()
            collect0(mfs, sampleNameFilter ?: SampleNameFilter.ALLOW_ALL)

            return mfs
        }

        private fun collect0(mfs: MutableList<MetricFamilySamples>, predicate: Predicate<String>) {
            val stats = runBlocking { supply() }
            for ((index, indexStats) in stats.indexes) {
                if (predicate.test(MetricKeyset.IndexDeletedDocuments.serialName!!.format(index))) {
                    mfs.add(
                        GaugeMetricFamily(
                            MetricKeyset.IndexDeletedDocuments.serialName!!.format(index),
                            "Total amount of documents that were deleted in this index",
                            indexStats.deletedDocuments.toDouble(),
                        ),
                    )
                }

                if (predicate.test(MetricKeyset.IndexAvgQueryTime.serialName!!.format(index))) {
                    mfs.add(
                        GaugeMetricFamily(
                            MetricKeyset.IndexAvgQueryTime.serialName!!.format(index),
                            "Average query time (in milliseconds) that this index processes search",
                            indexStats.averageQueryTimeInMs.toDouble(),
                        ),
                    )
                }

                if (predicate.test(MetricKeyset.IndexTotalDocuments.serialName!!.format(index))) {
                    mfs.add(
                        GaugeMetricFamily(
                            MetricKeyset.IndexTotalDocuments.serialName!!.format(index),
                            "Total amount of documents that are present in this index",
                            indexStats.documents.toDouble(),
                        ),
                    )
                }

                if (predicate.test(MetricKeyset.IndexHealth.serialName!!.format(index))) {
                    mfs.add(
                        GaugeMetricFamily(
                            MetricKeyset.IndexHealth.serialName!!.format(index),
                            "Current health status of this index",
                            listOf("health"),
                        ).apply { addMetric(listOf(indexStats.health), 1.0) },
                    )
                }

                if (predicate.test(MetricKeyset.IndexSize.serialName!!.format(index))) {
                    mfs.add(
                        GaugeMetricFamily(
                            MetricKeyset.IndexSize.serialName!!.format(index),
                            "Size (in bytes) of how big this index is",
                            indexStats.size.toDouble(),
                        ),
                    )
                }

                if (predicate.test(MetricKeyset.IndexUUID.serialName!!.format(index))) {
                    mfs.add(
                        GaugeMetricFamily(
                            MetricKeyset.IndexUUID.serialName!!.format(index),
                            "UUID of this index, which can be used to search via Elasticsearch's REST API",
                            listOf("uuid"),
                        ).apply { addMetric(listOf(indexStats.uuid), 1.0) },
                    )
                }
            }

            for ((node, nodeStats) in stats.nodes) {
                if (predicate.test(MetricKeyset.NodeDeletedDocuments.serialName!!.format(node))) {
                    mfs.add(
                        GaugeMetricFamily(
                            MetricKeyset.NodeDeletedDocuments.serialName!!.format(node),
                            "Total deleted documents in all of this node's managed indices",
                            nodeStats.deletedDocuments.toDouble(),
                        ),
                    )
                }

                if (predicate.test(MetricKeyset.NodeAvgIndexTime.serialName!!.format(node))) {
                    mfs.add(
                        GaugeMetricFamily(
                            MetricKeyset.NodeAvgIndexTime.serialName!!.format(node),
                            "Average indexing time (in milliseconds) that this node's indices takes",
                            nodeStats.averageIndexTimeInMs.toDouble(),
                        ),
                    )
                }

                if (predicate.test(MetricKeyset.NodeCpuPercentage.serialName!!.format(node))) {
                    mfs.add(
                        GaugeMetricFamily(
                            MetricKeyset.NodeCpuPercentage.serialName!!.format(node),
                            "Total CPU percentage that this node is using",
                            nodeStats.cpuPercentage,
                        ),
                    )
                }

                if (predicate.test(MetricKeyset.NodeIndexSize.serialName!!.format(node))) {
                    mfs.add(
                        GaugeMetricFamily(
                            MetricKeyset.NodeIndexSize.serialName!!.format(node),
                            "Total memory (in bytes) that all of this node's managed indices holds",
                            nodeStats.totalIndexesSize.toDouble(),
                        ),
                    )
                }

                if (predicate.test(MetricKeyset.NodeShards.serialName!!.format(node))) {
                    mfs.add(
                        GaugeMetricFamily(
                            MetricKeyset.NodeShards.serialName!!.format(node),
                            "Total shards that this node manages",
                            nodeStats.shards.toDouble(),
                        ),
                    )
                }
            }
        }
    }
}
