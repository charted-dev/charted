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

package org.noelware.charted.database.cassandra

import io.prometheus.client.Collector
import io.prometheus.client.GaugeMetricFamily
import io.prometheus.client.Predicate
import io.prometheus.client.SampleNameFilter
import kotlinx.coroutines.runBlocking

class CassandraMetricsCollector(private val cassandra: CassandraConnection): Collector() {
    override fun collect(): MutableList<MetricFamilySamples> = collect(null)
    override fun collect(sampleNameFilter: Predicate<String>?): MutableList<MetricFamilySamples> {
        val mfs = mutableListOf<MetricFamilySamples>()
        collectSamples(mfs, sampleNameFilter ?: SampleNameFilter.ALLOW_ALL)

        return mfs
    }

    private fun collectSamples(mfs: MutableList<MetricFamilySamples>, sampleNameFilter: Predicate<String>) {
        val rs = runBlocking { cassandra.sql("SELECT data_center FROM system.local;").one()!! }

        if (sampleNameFilter.test(CASSANDRA_DB_CALLS)) {
            mfs.add(
                GaugeMetricFamily(
                    CASSANDRA_DB_CALLS,
                    "How many database calls did the current connection execute.",
                    cassandra.calls.toDouble()
                )
            )
        }

        if (sampleNameFilter.test(CASSANDRA_VERSION)) {
            mfs.add(
                GaugeMetricFamily(
                    CASSANDRA_VERSION,
                    "The current cluster version",
                    listOf("version")
                ).apply { addMetric(listOf(cassandra.serverVersion), 1.0) }
            )
        }

        if (sampleNameFilter.test(CASSANDRA_DATA_CENTER)) {
            mfs.add(
                GaugeMetricFamily(
                    CASSANDRA_DATA_CENTER,
                    "The current data center that the Cassandra instance is in.",
                    listOf("data_center")
                ).apply { addMetric(listOf(rs.getString("data_center")), 1.0) }
            )
        }

//        if (sampleNameFilter.test(CASSANDRA_CLUSTER_NAME)) {
//            mfs.add(
//                GaugeMetricFamily(
//                    CASSANDRA_CLUSTER_NAME,
//                    "Returns the cluster name of this Cassandra instance.",
//                    listOf("cluster_name")
//                ).apply { addMetric(listOf(cluster.clusterName), 1.0) }
//            )
//        }

        val metrics = cassandra.session.metrics.get()
//        if (sampleNameFilter.test(CASSANDRA_TRASHED_CONNECTIONS)) {
//            mfs.add(
//                GaugeMetricFamily(
//                    CASSANDRA_TRASHED_CONNECTIONS,
//                    "Returns the total number of currently \"trashed\" connections to Cassandra hosts.",
//                    metrics.trashedConnections.value.toDouble()
//                )
//            )
//        }
//
//        if (sampleNameFilter.test(CASSANDRA_REQUEST_LATENCY)) {
//            mfs.add(
//                GaugeMetricFamily(
//                    CASSANDRA_REQUEST_LATENCY,
//                    "The total latency between requests from charted to/from Cassandra",
//                    metrics.requestsTimer.count.toDouble()
//                )
//            )
//        }
    }

    companion object {
        const val CASSANDRA_DB_CALLS = "charted_cassandra_database_calls"
        const val CASSANDRA_VERSION = "charted_cassandra_version"
        const val CASSANDRA_DATA_CENTER = "charted_cassandra_data_center"
        const val CASSANDRA_CLUSTER_NAME = "charted_cassandra_cluster_name"
        const val CASSANDRA_TRASHED_CONNECTIONS = "charted_cassandra_trashed_connections"
        const val CASSANDRA_REQUEST_LATENCY = "charted_cassandra_requests_latency"
    }
}
