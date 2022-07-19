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

package org.noelware.charted.server.metrics

import io.prometheus.client.Collector
import io.prometheus.client.GaugeMetricFamily
import io.prometheus.client.Predicate
import io.prometheus.client.SampleNameFilter
import org.noelware.charted.database.cassandra.CassandraConnection

class CassandraCollector(private val cassandra: CassandraConnection): Collector() {
    override fun collect(): MutableList<MetricFamilySamples> = collect(null)
    override fun collect(sampleNameFilter: Predicate<String>?): MutableList<MetricFamilySamples> {
        val mfs = mutableListOf<MetricFamilySamples>()
        collectCassandraMetrics(mfs, sampleNameFilter ?: SampleNameFilter.ALLOW_ALL)

        return mfs
    }

    private fun collectCassandraMetrics(mfs: MutableList<MetricFamilySamples>, sampleNameFilter: Predicate<String>) {
        if (sampleNameFilter.test("charted_cassandra_database_calls")) {
            mfs.add(
                GaugeMetricFamily(
                    "charted_cassandra_database_calls",
                    "How many database calls did the current connection execute.",
                    cassandra.calls.toDouble()
                )
            )
        }

        if (sampleNameFilter.test("charted_cassandra_version")) {
            mfs.add(
                GaugeMetricFamily(
                    "charted_cassandra_version",
                    "The current cluster version",
                    listOf(cassandra.serverVersion)
                )
            )
        }

        val metrics = cassandra.cluster.metrics
        if (sampleNameFilter.test("charted_cassandra_trashed_connections")) {
            mfs.add(
                GaugeMetricFamily(
                    "charted_cassandra_trashed_connections",
                    "Returns the total number of currently \"trashed\" connections to Cassandra hosts.",
                    metrics.trashedConnections.value.toDouble()
                )
            )
        }

        if (sampleNameFilter.test("charted_cassandra_requests_latency")) {
            mfs.add(
                GaugeMetricFamily(
                    "charted_cassandra_requests_latency",
                    "The total latency between requests from charted <- -> Cassandra",
                    metrics.requestsTimer.count.toDouble()
                )
            )
        }
    }
}
