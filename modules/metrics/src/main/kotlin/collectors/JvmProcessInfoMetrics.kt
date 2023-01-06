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

package org.noelware.charted.modules.metrics.collectors

import com.google.protobuf.Value
import io.prometheus.client.Predicate
import io.prometheus.client.SampleNameFilter
import kotlinx.datetime.Instant
import kotlinx.datetime.toKotlinInstant
import kotlinx.serialization.Serializable
import org.noelware.charted.modules.analytics.kotlin.dsl.Struct
import org.noelware.charted.modules.analytics.kotlin.dsl.put
import org.noelware.charted.modules.analytics.kotlin.dsl.toGrpcValue

@Serializable
data class JvmProcessInfoMetrics(
    val startTime: Instant,
    val pid: Long
) : org.noelware.analytics.jvm.server.serialization.Serializable {
    override fun toGrpcValue(): Value = Struct {
        put(this, JvmProcessInfoMetrics::startTime)
        put(this, JvmProcessInfoMetrics::pid)
    }.toGrpcValue()

    class Collector : org.noelware.charted.modules.metrics.Collector<JvmProcessInfoMetrics>, io.prometheus.client.Collector() {
        private val current: ProcessHandle
            get() = ProcessHandle.current()

        override val name: String = "process"
        override suspend fun supply(): JvmProcessInfoMetrics = JvmProcessInfoMetrics(
            current.info().startInstant().map { it.toKotlinInstant() }.orElseThrow(),
            current.pid(),
        )

        override fun collect(): MutableList<MetricFamilySamples> = collect(null)
        override fun collect(sampleNameFilter: Predicate<String>?): MutableList<MetricFamilySamples> {
            val mfs = mutableListOf<MetricFamilySamples>()
            val predicate = sampleNameFilter ?: SampleNameFilter.ALLOW_ALL

            collect0(predicate, mfs)
            return mfs
        }

        private fun collect0(predicate: Predicate<String>, mfs: MutableList<MetricFamilySamples>) {}
    }
}
