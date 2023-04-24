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

package org.noelware.charted.modules.redis.metrics

import com.google.protobuf.Value
import io.prometheus.client.GaugeMetricFamily
import io.prometheus.client.Predicate
import io.prometheus.client.SampleNameFilter
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.enumSets.serialName
import org.noelware.charted.configuration.kotlin.dsl.metrics.keysets.RedisKeysets
import org.noelware.charted.modules.analytics.kotlin.dsl.Struct
import org.noelware.charted.modules.analytics.kotlin.dsl.put
import org.noelware.charted.modules.analytics.kotlin.dsl.toGrpcValue
import org.noelware.charted.modules.redis.RedisClient

/**
 * Represents the collected statistics the metrics collector use.
 * @param totalConnectionsReceived Returns how many total connections the server has received
 * @param totalCommandsProcessed   Returns how many Redis commands were processed
 * @param totalNetworkOutput       Returns the total network output (in bytes) the server has sent out to us or any other client
 * @param totalNetworkInput        Returns the total network input (in bytes) we or other clients had sent out to the server
 * @param allocator                The allocator that the Redis server uses
 * @param version                  Redis server version
 * @param uptime                   The uptime (in milliseconds) how long the server has been up for
 * @param mode                     The server mode it is in ("standalone" or "clustered")
 * @param ping                     Returns the latency (in nanoseconds) from the server to us.
 */
@Serializable
data class RedisServerStats(
    @SerialName("total_network_input")
    val totalNetworkInput: Long,

    @SerialName("total_network_output")
    val totalNetworkOutput: Long,

    @SerialName("total_commands_processed")
    val totalCommandsProcessed: Long,

    @SerialName("total_connections_received")
    val totalConnectionsReceived: Long,
    val allocator: String,
    val uptime: Long,
    val version: String,
    val mode: String,
    val ping: Long
): org.noelware.analytics.jvm.server.serialization.Serializable {
    override fun toGrpcValue(): Value = Struct {
        put(this, RedisServerStats::totalConnectionsReceived)
        put(this, RedisServerStats::totalCommandsProcessed)
        put(this, RedisServerStats::totalNetworkOutput)
        put(this, RedisServerStats::totalNetworkInput)
        put(this, RedisServerStats::allocator)
        put(this, RedisServerStats::version)
        put(this, RedisServerStats::uptime)
        put(this, RedisServerStats::mode)
        put(this, RedisServerStats::ping)
    }.toGrpcValue()

    class Collector(
        private val redis: RedisClient,
        private val config: Config
    ): org.noelware.charted.modules.metrics.Collector<RedisServerStats>, io.prometheus.client.Collector() {
        override val name: String = "redis"
        override suspend fun supply(): RedisServerStats = redis.stats()

        override fun collect(): MutableList<MetricFamilySamples> = collect {
            RedisKeysets.EnumSet.enabled(config.metrics.metricSets.redis, it)
        }

        override fun collect(sampleNameFilter: Predicate<String>?): MutableList<MetricFamilySamples> {
            val mfs = mutableListOf<MetricFamilySamples>()
            collect0(mfs, sampleNameFilter ?: SampleNameFilter.ALLOW_ALL)

            return mfs
        }

        private fun collect0(mfs: MutableList<MetricFamilySamples>, predicate: Predicate<String>) {
            val stats = runBlocking { supply() }
            if (predicate.test(RedisKeysets.TotalConnectionsReceived.serialName!!)) {
                mfs.add(
                    GaugeMetricFamily(
                        RedisKeysets.TotalConnectionsReceived.serialName!!,
                        "Returns how many total connections we had made",
                        stats.totalConnectionsReceived.toDouble(),
                    ),
                )
            }

            if (predicate.test(RedisKeysets.TotalCommandsProcessed.serialName!!)) {
                mfs.add(
                    GaugeMetricFamily(
                        RedisKeysets.TotalCommandsProcessed.serialName!!,
                        "How many commands were processed during this session",
                        stats.totalCommandsProcessed.toDouble(),
                    ),
                )
            }

            if (predicate.test(RedisKeysets.TotalNetworkOutput.serialName!!)) {
                mfs.add(
                    GaugeMetricFamily(
                        RedisKeysets.TotalNetworkOutput.serialName!!,
                        "Network output (in bytes) of the whole Redis server",
                        stats.totalNetworkOutput.toDouble(),
                    ),
                )
            }

            if (predicate.test(RedisKeysets.TotalNetworkInput.serialName!!)) {
                mfs.add(
                    GaugeMetricFamily(
                        RedisKeysets.TotalCommandsProcessed.serialName!!,
                        "Network input (in bytes) of the whole Redis server",
                        stats.totalNetworkInput.toDouble(),
                    ),
                )
            }

            if (predicate.test(RedisKeysets.Allocator.serialName!!)) {
                mfs.add(
                    GaugeMetricFamily(
                        RedisKeysets.Allocator.serialName!!,
                        "The allocator that this Redis server is configured to use",
                        listOf("allocator"),
                    ).apply { addMetric(listOf(stats.allocator), 1.0) },
                )
            }

            if (predicate.test(RedisKeysets.Version.serialName!!)) {
                mfs.add(
                    GaugeMetricFamily(
                        RedisKeysets.Version.serialName!!,
                        "Current Redis server version",
                        listOf("version"),
                    ).apply { addMetric(listOf(stats.version), 1.0) },
                )
            }

            if (predicate.test(RedisKeysets.Uptime.serialName!!)) {
                mfs.add(
                    GaugeMetricFamily(
                        RedisKeysets.Uptime.serialName!!,
                        "The uptime (in milliseconds) how long the server has been up for",
                        stats.uptime.toDouble(),
                    ),
                )
            }

            if (predicate.test(RedisKeysets.Mode.serialName!!)) {
                mfs.add(
                    GaugeMetricFamily(
                        RedisKeysets.Mode.serialName!!,
                        "The server mode it is in (\"standalone\" or \"clustered\")",
                        listOf("mode"),
                    ).apply { addMetric(listOf(stats.mode), 1.0) },
                )
            }

            if (predicate.test(RedisKeysets.Ping.serialName!!)) {
                mfs.add(
                    GaugeMetricFamily(
                        RedisKeysets.Ping.serialName!!,
                        "Ping (in milliseconds) that determines the latency between the Redis server to ourselves.",
                        stats.ping.toDouble(),
                    ),
                )
            }
        }
    }
}
