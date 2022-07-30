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

package org.noelware.charted.metrics

import io.prometheus.client.Collector
import io.prometheus.client.GaugeMetricFamily
import io.prometheus.client.Predicate
import io.prometheus.client.SampleNameFilter
import org.noelware.charted.common.IRedisClient

class RedisMetricsCollector(private val redis: IRedisClient): Collector() {
    override fun collect(): MutableList<MetricFamilySamples> = collect(null)
    override fun collect(sampleNameFilter: Predicate<String>?): MutableList<MetricFamilySamples> {
        val mfs = mutableListOf<MetricFamilySamples>()
        collectMetrics(mfs, sampleNameFilter ?: SampleNameFilter.ALLOW_ALL)

        return mfs
    }

    private fun collectMetrics(mfs: MutableList<MetricFamilySamples>, predicate: Predicate<String>) {
        val stats = redis.stats

        if (predicate.test(REDIS_TOTAL_NETWORK_INPUT)) {
            mfs.add(
                GaugeMetricFamily(
                    REDIS_TOTAL_NETWORK_INPUT,
                    "Returns the total network input in bytes.",
                    stats.totalNetworkInput.toDouble()
                )
            )
        }

        if (predicate.test(REDIS_TOTAL_NETWORK_OUTPUT)) {
            mfs.add(
                GaugeMetricFamily(
                    REDIS_TOTAL_NETWORK_OUTPUT,
                    "Returns the total network output in bytes.",
                    stats.totalNetworkOutput.toDouble()
                )
            )
        }

        if (predicate.test(REDIS_TOTAL_COMMANDS_PROCESSED)) {
            mfs.add(
                GaugeMetricFamily(
                    REDIS_TOTAL_COMMANDS_PROCESSED,
                    "Returns how many Redis commands were processed.",
                    stats.totalCommandsProcessed.toDouble()
                )
            )
        }

        if (predicate.test(REDIS_TOTAL_CONNECTIONS_RECEIVED)) {
            mfs.add(
                GaugeMetricFamily(
                    REDIS_TOTAL_CONNECTIONS_RECEIVED,
                    "Returns how many connections were received by the Redis server.",
                    stats.totalConnectionsReceived.toDouble()
                )
            )
        }

        if (predicate.test(REDIS_ALLOCATOR)) {
            mfs.add(
                GaugeMetricFamily(
                    REDIS_ALLOCATOR,
                    "Returns the current memory allocator Redis is using.",
                    listOf("allocator")
                ).apply { addMetric(listOf(stats.allocator), 1.0) }
            )
        }

        if (predicate.test(REDIS_VERSION)) {
            mfs.add(
                GaugeMetricFamily(
                    REDIS_VERSION,
                    "Returns the current Redis server version.",
                    listOf("version")
                ).apply { addMetric(listOf(stats.version), 1.0) }
            )
        }

        if (predicate.test(REDIS_MODE)) {
            mfs.add(
                GaugeMetricFamily(
                    REDIS_MODE,
                    "Returns the current mode the Redis server is using. [standalone, sentinel, clustered]",
                    listOf("mode")
                ).apply { addMetric(listOf(stats.mode), 1.0) }
            )
        }

        if (predicate.test(REDIS_PING)) {
            mfs.add(
                GaugeMetricFamily(
                    REDIS_PING,
                    "Returns the ping from charted from/to Redis",
                    stats.ping.toDouble()
                )
            )
        }
    }

    companion object {
        const val REDIS_TOTAL_NETWORK_INPUT = "charted_redis_total_net_input"
        const val REDIS_TOTAL_NETWORK_OUTPUT = "charted_redis_total_net_output"
        const val REDIS_TOTAL_COMMANDS_PROCESSED = "charted_redis_total_commands_processed"
        const val REDIS_TOTAL_CONNECTIONS_RECEIVED = "charted_redis_total_connections_received"
        const val REDIS_ALLOCATOR = "charted_redis_allocator"
        const val REDIS_VERSION = "charted_redis_version"
        const val REDIS_MODE = "charted_redis_mode"
        const val REDIS_PING = "charted_redis_ping"
    }
}
