/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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
import io.prometheus.client.Predicate
import io.prometheus.client.SampleNameFilter
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.common.OperatingSystem
import org.noelware.charted.modules.analytics.kotlin.dsl.Struct
import org.noelware.charted.modules.analytics.kotlin.dsl.put
import org.noelware.charted.modules.analytics.kotlin.dsl.toGrpcValue
import org.noelware.charted.modules.metrics.Collector
import java.io.File
import java.lang.management.ManagementFactory
import java.nio.charset.Charset

@Serializable
data class OperatingSystemMetrics(
    @SerialName("system_load_avg")
    val systemLoadAverage: Double,
    val processors: Int,
    val version: String,
    val distro: String? = null,
    val arch: String,
    val name: String
) : org.noelware.analytics.jvm.server.serialization.Serializable {
    class Collector : org.noelware.charted.modules.metrics.Collector<OperatingSystemMetrics>, io.prometheus.client.Collector() {
        override val name: String = "os"

        private val os = ManagementFactory.getOperatingSystemMXBean()
        private val linuxDistro: String? by lazy {
            if (!OperatingSystem.current().isLinux) return@lazy null

            try {
                val releases = File("/etc/os-release").readText(Charset.defaultCharset())
                val data = releases.split("\n\r?".toRegex()).dropLast(1).associate { value ->
                    val (key, v) = value.split("=")

                    key to v.replace("\"", "")
                }

                "${data["NAME"]} (${data["ID"]})"
            } catch (e: Exception) {
                // return blank because we probably can't read the file :(
                ""
            }
        }

        override suspend fun supply(): OperatingSystemMetrics = OperatingSystemMetrics(
            os.systemLoadAverage,
            os.availableProcessors,
            os.version,
            linuxDistro,
            os.arch,
            os.name,
        )

        override fun collect(): MutableList<MetricFamilySamples> = collect(null)
        override fun collect(sampleNameFilter: Predicate<String>?): MutableList<MetricFamilySamples> {
            val mfs = mutableListOf<MetricFamilySamples>()
            val predicate = sampleNameFilter ?: SampleNameFilter.ALLOW_ALL

            collect0(predicate, mfs)
            return mfs
        }

        private fun collect0(predicate: Predicate<String>, mfs: MutableList<MetricFamilySamples>) {
        }
    }

    override fun toGrpcValue(): Value = Struct {
        put(this, OperatingSystemMetrics::systemLoadAverage)
        put(this, OperatingSystemMetrics::processors)
        put(this, OperatingSystemMetrics::version)
        put(this, OperatingSystemMetrics::distro)
        put(this, OperatingSystemMetrics::arch)
        put(this, OperatingSystemMetrics::name)
    }.toGrpcValue()
}
