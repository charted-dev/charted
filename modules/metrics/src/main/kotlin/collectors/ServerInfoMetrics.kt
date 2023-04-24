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

package org.noelware.charted.modules.metrics.collectors

import com.google.protobuf.Value
import io.ktor.server.application.*
import io.prometheus.client.GaugeMetricFamily
import io.prometheus.client.Predicate
import io.prometheus.client.SampleNameFilter
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.ChartedInfo
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.enumSets.serialName
import org.noelware.charted.configuration.kotlin.dsl.metrics.keysets.APIServerKeysets
import org.noelware.charted.modules.analytics.kotlin.dsl.Struct
import org.noelware.charted.modules.analytics.kotlin.dsl.put
import org.noelware.charted.modules.analytics.kotlin.dsl.toGrpcValue
import org.noelware.charted.modules.metrics.Collector

@Serializable
data class ServerInfoMetrics(
    val distribution: ChartedInfo.Distribution,

    @SerialName("ktor_version")
    val ktorVersion: String,

    @SerialName("commit_sha")
    val commitHash: String,
    val requests: Long,

    @SerialName("build_date")
    val buildDate: String,
    val product: String,
    val version: String,
    val vendor: String
): org.noelware.analytics.jvm.server.serialization.Serializable {
    override fun toGrpcValue(): Value = Struct {
        put(this, ServerInfoMetrics::distribution)
        put(this, ServerInfoMetrics::ktorVersion)
        put(this, ServerInfoMetrics::commitHash)
        put(this, ServerInfoMetrics::buildDate)
        put(this, ServerInfoMetrics::requests)
        put(this, ServerInfoMetrics::product)
        put(this, ServerInfoMetrics::version)
        put(this, ServerInfoMetrics::vendor)
    }.toGrpcValue()

    class Collector(
        private val config: Config,
        private val getRequestCounter: () -> Long
    ): org.noelware.charted.modules.metrics.Collector<ServerInfoMetrics>, io.prometheus.client.Collector() {
        override val name: String = "server"
        override suspend fun supply(): ServerInfoMetrics {
            val ktorVersion = Application::class.java.`package`.implementationVersion!!
            return ServerInfoMetrics(
                ChartedInfo.distribution,
                ktorVersion,
                ChartedInfo.commitHash,
                getRequestCounter(),
                ChartedInfo.buildDate,
                "charted-server",
                ChartedInfo.version,
                "Noelware, LLC.",
            )
        }

        override fun collect(): MutableList<MetricFamilySamples> = collect {
            APIServerKeysets.EnumSet.enabled(config.metrics.metricSets.server, it)
        }

        override fun collect(sampleNameFilter: Predicate<String>?): MutableList<MetricFamilySamples> {
            val mfs = mutableListOf<MetricFamilySamples>()
            val predicate = sampleNameFilter ?: SampleNameFilter.ALLOW_ALL

            collect0(mfs, predicate)
            return mfs
        }

        private fun collect0(mfs: MutableList<MetricFamilySamples>, predicate: Predicate<String>) {
            val stats = runBlocking { supply() }
            if (predicate.test(APIServerKeysets.Distribution.serialName!!)) {
                mfs.add(
                    GaugeMetricFamily(
                        APIServerKeysets.Distribution.serialName!!,
                        "Current distribution",
                        listOf("distribution"),
                    ).addMetric(listOf(stats.distribution.serialName!!), 1.0),
                )
            }

            if (predicate.test(APIServerKeysets.KtorVersion.serialName!!)) {
                mfs.add(
                    GaugeMetricFamily(
                        APIServerKeysets.KtorVersion.serialName!!,
                        "Current version of Ktor that charted-server is using",
                        listOf("ktor_version"),
                    ).addMetric(listOf(stats.ktorVersion), 1.0),
                )
            }

            if (predicate.test(APIServerKeysets.CommitHash.serialName!!)) {
                mfs.add(
                    GaugeMetricFamily(
                        APIServerKeysets.CommitHash.serialName!!,
                        "Valid Git commit hash",
                        listOf("commit_hash"),
                    ).addMetric(listOf(stats.commitHash), 1.0),
                )
            }

            if (predicate.test(APIServerKeysets.BuildDate.serialName!!)) {
                mfs.add(
                    GaugeMetricFamily(
                        APIServerKeysets.BuildDate.serialName!!,
                        "RFC3339-encoded value of when this version of charted-server was last built at",
                        listOf("build_date"),
                    ).addMetric(listOf(stats.buildDate), 1.0),
                )
            }

            if (predicate.test(APIServerKeysets.Product.serialName!!)) {
                mfs.add(
                    GaugeMetricFamily(
                        APIServerKeysets.Product.serialName!!,
                        "Product name, will always be \"charted-server\"",
                        listOf("product"),
                    ).addMetric(listOf(stats.product), 1.0),
                )
            }

            if (predicate.test(APIServerKeysets.Vendor.serialName!!)) {
                mfs.add(
                    GaugeMetricFamily(
                        APIServerKeysets.Vendor.serialName!!,
                        "Distribution vendor that distributed this instance",
                        listOf("vendor"),
                    ).addMetric(listOf(stats.vendor), 1.0),
                )
            }

            if (predicate.test(APIServerKeysets.Version.serialName!!)) {
                mfs.add(
                    GaugeMetricFamily(
                        APIServerKeysets.Version.serialName!!,
                        "Current API server version that this instance is running",
                        listOf("version"),
                    ).addMetric(listOf(stats.version), 1.0),
                )
            }
        }
    }
}
