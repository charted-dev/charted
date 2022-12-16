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

package org.noelware.charted.modules.redis.metrics

import io.prometheus.client.Collector
import io.prometheus.client.GaugeMetricFamily
import io.prometheus.client.Predicate
import io.prometheus.client.SampleNameFilter
import org.noelware.charted.configuration.kotlin.dsl.metrics.MetricsConfig
import org.noelware.charted.configuration.kotlin.dsl.metrics.keys.RedisMetricKeys
import org.noelware.charted.modules.metrics.MetricStatCollector
import org.noelware.charted.modules.redis.RedisClient

class RedisMetricsCollector(private val redis: RedisClient, private val config: MetricsConfig): MetricStatCollector {
    override fun collect(): MutableList<Collector.MetricFamilySamples> = collect {
        config.metricsets.redis.firstOrNull() == RedisMetricKeys.Wildcard ||
            config.metricsets.redis.contains(RedisMetricKeys.values().find { f -> f.key == it })
    }

    override fun collect(predicate: Predicate<String>?): MutableList<Collector.MetricFamilySamples> {
        val mfs = mutableListOf<Collector.MetricFamilySamples>()
        collect0(predicate ?: SampleNameFilter.ALLOW_ALL, mfs)

        return mfs
    }

    private fun collect0(
        predicate: Predicate<String>,
        mfs: MutableList<Collector.MetricFamilySamples>
    ) {
        val stats = redis.stats()
        if (predicate.test(RedisMetricKeys.TotalNetworkInput.key)) {
            mfs.add(
                GaugeMetricFamily(
                    RedisMetricKeys.TotalNetworkInput.key,
                    "Returns the total network input in bytes.",
                    stats.totalNetworkInput.toDouble()
                )
            )
        }

        if (predicate.test(RedisMetricKeys.TotalNetworkOutput.key)) {
            mfs.add(
                GaugeMetricFamily(
                    RedisMetricKeys.TotalNetworkOutput.key,
                    "Returns the total network output in bytes.",
                    stats.totalNetworkOutput.toDouble()
                )
            )
        }

        if (predicate.test(RedisMetricKeys.TotalCommandsProcessed.key)) {
            mfs.add(
                GaugeMetricFamily(
                    RedisMetricKeys.TotalCommandsProcessed.key,
                    "Returns how many Redis commands were processed.",
                    stats.totalCommandsProcessed.toDouble()
                )
            )
        }

        if (predicate.test(RedisMetricKeys.TotalConnectionsReceived.key)) {
            mfs.add(
                GaugeMetricFamily(
                    RedisMetricKeys.TotalConnectionsReceived.key,
                    "Returns how many connections were received by the Redis server.",
                    stats.totalConnectionsReceived.toDouble()
                )
            )
        }

        if (predicate.test(RedisMetricKeys.Allocator.key)) {
            mfs.add(
                GaugeMetricFamily(
                    RedisMetricKeys.Allocator.key,
                    "Returns the current memory allocator Redis is using.",
                    listOf("allocator")
                ).apply { addMetric(listOf(stats.allocator), 1.0) }
            )
        }

        if (predicate.test(RedisMetricKeys.Version.key)) {
            mfs.add(
                GaugeMetricFamily(
                    RedisMetricKeys.Version.key,
                    "Returns the current Redis server version.",
                    listOf("version")
                ).apply { addMetric(listOf(stats.version), 1.0) }
            )
        }

        if (predicate.test(RedisMetricKeys.Mode.key)) {
            mfs.add(
                GaugeMetricFamily(
                    RedisMetricKeys.Mode.key,
                    "Returns the current mode the Redis server is using. [standalone, clustered]",
                    listOf("mode")
                ).apply { addMetric(listOf(stats.mode), 1.0) }
            )
        }

        if (predicate.test(RedisMetricKeys.Ping.key)) {
            mfs.add(
                GaugeMetricFamily(
                    RedisMetricKeys.Ping.key,
                    "Returns the latency (in nanoseconds) from us to Redis",
                    stats.ping.toDouble()
                )
            )
        }
    }
}
