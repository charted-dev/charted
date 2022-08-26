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

package org.noelware.charted.elasticsearch

import io.prometheus.client.Collector
import io.prometheus.client.GaugeMetricFamily
import io.prometheus.client.Predicate
import io.prometheus.client.SampleNameFilter
import kotlinx.coroutines.runBlocking

class ElasticsearchMetricsCollector(private val elasticsearch: ElasticsearchService): Collector() {
    override fun collect(): MutableList<MetricFamilySamples> = collect(null)
    override fun collect(sampleNameFilter: Predicate<String>?): MutableList<MetricFamilySamples> {
        val mfs = mutableListOf<MetricFamilySamples>()
        collectSamples(mfs, sampleNameFilter ?: SampleNameFilter.ALLOW_ALL)

        return mfs
    }

    private fun collectSamples(mfs: MutableList<MetricFamilySamples>, sampleNameFilter: Predicate<String>) {
        val stats = runBlocking { elasticsearch.collect() }
        if (sampleNameFilter.test(ELASTICSEARCH_DOCUMENTS)) {
            mfs.add(
                GaugeMetricFamily(
                    ELASTICSEARCH_DOCUMENTS,
                    "How many documents are in all indexes on the available Elasticsearch cluster.",
                    stats.documents.toDouble()
                )
            )
        }

        if (sampleNameFilter.test(ELASTICSEARCH_SERVER_VERSION)) {
            mfs.add(
                GaugeMetricFamily(
                    ELASTICSEARCH_SERVER_VERSION,
                    "Returns the current version of the Elasticsearch cluster the server is connected to.",
                    listOf("version")
                ).apply { addMetric(listOf(elasticsearch.serverVersion), 1.0) }
            )
        }

        if (sampleNameFilter.test(ELASTICSEARCH_CLUSTER)) {
            mfs.add(
                GaugeMetricFamily(
                    ELASTICSEARCH_CLUSTER,
                    "Returns the current Elasticsearch cluster's name and UUID.",
                    listOf("cluster")
                ).apply { addMetric(listOf("${elasticsearch.clusterName} (${elasticsearch.clusterUUID})"), 1.0) }
            )
        }

        if (sampleNameFilter.test(ELASTICSEARCH_STORE_SIZE_IN_BYTES)) {
            mfs.add(
                GaugeMetricFamily(
                    ELASTICSEARCH_STORE_SIZE_IN_BYTES,
                    "Returns the Elasticsearch datastore size in bytes. (this includes all documents, not the ones used by the server.)",
                    stats.sizeInBytes.toDouble()
                )
            )
        }

        if (sampleNameFilter.test(ELASTICSEARCH_DELETED_DOCUMENTS)) {
            mfs.add(
                GaugeMetricFamily(
                    ELASTICSEARCH_DELETED_DOCUMENTS,
                    "Returns how many documents were deleted, this includes all documents, not the ones indexed by the server.",
                    stats.deleted.toDouble()
                )
            )
        }

        if (sampleNameFilter.test(ELASTICSEARCH_CLUSTER_MEMORY_FREE)) {
            mfs.add(
                GaugeMetricFamily(
                    ELASTICSEARCH_CLUSTER_MEMORY_FREE,
                    "Returns how much free memory the cluster has (in bytes)",
                    stats.memory.freeBytes.toDouble()
                )
            )
        }

        if (sampleNameFilter.test(ELASTICSEARCH_CLUSTER_MEMORY_USED)) {
            mfs.add(
                GaugeMetricFamily(
                    ELASTICSEARCH_CLUSTER_MEMORY_FREE,
                    "Returns how much used memory the cluster has used (in bytes)",
                    stats.memory.usedBytes.toDouble()
                )
            )
        }

        if (sampleNameFilter.test(ELASTICSEARCH_CLUSTER_MEMORY_TOTAL)) {
            mfs.add(
                GaugeMetricFamily(
                    ELASTICSEARCH_CLUSTER_MEMORY_FREE,
                    "Returns how much total memory the cluster has (in bytes)",
                    stats.memory.totalBytes.toDouble()
                )
            )
        }

        for ((key, stat) in stats.indexes) {
            if (sampleNameFilter.test(String.format(ELASTICSEARCH_INDEXED_DOCUMENTS, key.replace('-', '_')))) {
                mfs.add(
                    GaugeMetricFamily(
                        String.format(ELASTICSEARCH_INDEXED_DOCUMENTS, key.replace('-', '_')),
                        "Returns how many documents were indexed in index ${key.replace('-', '_')}.",
                        stat.documents.toDouble()
                    )
                )
            }

            if (sampleNameFilter.test(String.format(ELASTICSEARCH_INDEX_HEALTH, key.replace('-', '_')))) {
                mfs.add(
                    GaugeMetricFamily(
                        String.format(ELASTICSEARCH_INDEX_HEALTH, key.replace('-', '_')),
                        "Returns the health (red, yellow, green) of index ${key.replace('-', '_')}.",
                        listOf("health")
                    ).apply { addMetric(listOf(stat.health), 1.0) }
                )
            }

            if (sampleNameFilter.test(String.format(ELASTICSEARCH_INDEX_SIZE_IN_BYTES, key.replace('-', '_')))) {
                mfs.add(
                    GaugeMetricFamily(
                        String.format(ELASTICSEARCH_INDEX_SIZE_IN_BYTES, key.replace('-', '_')),
                        "Returns the datastore size for index ${key.replace('-', '_')}.",
                        stat.sizeInBytes.toDouble()
                    )
                )
            }

            if (sampleNameFilter.test(String.format(ELASTICSEARCH_INDEX_DELETED_DOCUMENTS, key.replace('-', '_')))) {
                mfs.add(
                    GaugeMetricFamily(
                        String.format(ELASTICSEARCH_INDEX_DELETED_DOCUMENTS, key.replace('-', '_')),
                        "Returns how many documents were deleted by index ${key.replace('-', '_')}.",
                        stat.deleted.toDouble()
                    )
                )
            }
        }
    }

    companion object {
        const val ELASTICSEARCH_DOCUMENTS = "charted_elasticsearch_documents"
        const val ELASTICSEARCH_SERVER_VERSION = "charted_elasticsearch_server_version"
        const val ELASTICSEARCH_CLUSTER = "charted_elasticsearch_cluster"
        const val ELASTICSEARCH_STORE_SIZE_IN_BYTES = "charted_elasticsearch_store_size"
        const val ELASTICSEARCH_DELETED_DOCUMENTS = "charted_elasticsearch_deleted_documents"
        const val ELASTICSEARCH_INDEX_SIZE_IN_BYTES = "charted_elasticsearch_%s_size"
        const val ELASTICSEARCH_INDEXED_DOCUMENTS = "charted_elasticsearch_%s_documents"
        const val ELASTICSEARCH_INDEX_DELETED_DOCUMENTS = "charted_elasticsearch_%s_deleted_documents"
        const val ELASTICSEARCH_INDEX_HEALTH = "charted_elasticsearch_%s_health"
        const val ELASTICSEARCH_CLUSTER_MEMORY_FREE = "charted_elasticsearch_memory_free_bytes"
        const val ELASTICSEARCH_CLUSTER_MEMORY_USED = "charted_elasticsearch_memory_used_bytes"
        const val ELASTICSEARCH_CLUSTER_MEMORY_TOTAL = "charted_elasticsearch_memory_total_bytes"
    }
}
