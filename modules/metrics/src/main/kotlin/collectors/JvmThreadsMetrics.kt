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

import com.fasterxml.jackson.annotation.JsonProperty
import com.google.protobuf.Value
import io.prometheus.client.Predicate
import io.prometheus.client.SampleNameFilter
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.modules.analytics.kotlin.dsl.Struct
import org.noelware.charted.modules.analytics.kotlin.dsl.put
import org.noelware.charted.modules.analytics.kotlin.dsl.toGrpcValue
import org.noelware.charted.modules.metrics.Collector
import java.lang.management.ManagementFactory
import kotlin.time.Duration.Companion.nanoseconds

@Serializable
data class JvmThreadsMetrics(
    val current: Int,
    val peak: Int,
    val background: Int,
    val threads: List<ThreadInfo>
): org.noelware.analytics.jvm.server.serialization.Serializable {
    @Serializable
    data class ThreadInfo(
        val stacktrace: List<StackTrace> = listOf(),

        @JsonProperty("user_time_ms")
        @SerialName("user_time_ms")
        val userTimeMs: Long,

        @JsonProperty("cpu_time_ms")
        @SerialName("cpu_time_ms")
        val cpuTimeMs: Long,
        val suspended: Boolean,
        val background: Boolean,
        val state: String,
        val name: String,
        val id: Long
    ): org.noelware.analytics.jvm.server.serialization.Serializable {
        override fun toGrpcValue(): Value = Struct {
            put(this, ThreadInfo::stacktrace)
            put(this, ThreadInfo::userTimeMs)
            put(this, ThreadInfo::cpuTimeMs)
            put(this, ThreadInfo::suspended)
            put(this, ThreadInfo::background)
            put(this, ThreadInfo::state)
            put(this, ThreadInfo::name)
            put(this, ThreadInfo::id)
        }.toGrpcValue()
    }

    @Serializable
    data class StackTrace(
        @JsonProperty("class_loader_name")
        @SerialName("class_loader_name")
        val classLoaderName: String? = null,

        @JsonProperty("module_name")
        @SerialName("module_name")
        val moduleName: String? = null,

        @JsonProperty("module_version")
        @SerialName("module_version")
        val moduleVersion: String? = null,

        @JsonProperty("declaring_class")
        @SerialName("declaring_class")
        val declaringClass: String,

        @JsonProperty("method_name")
        @SerialName("method_name")
        val methodName: String,
        val file: String? = null,
        val line: Int? = null,

        @JsonProperty("is_native_method")
        @SerialName("is_native_method")
        val isNativeMethod: Boolean
    ): org.noelware.analytics.jvm.server.serialization.Serializable {
        override fun toGrpcValue(): Value = Struct {
            put(this, StackTrace::classLoaderName)
            put(this, StackTrace::moduleVersion)
            put(this, StackTrace::moduleName)
            put(this, StackTrace::declaringClass)
            put(this, StackTrace::methodName)
            put(this, StackTrace::isNativeMethod)
            put(this, StackTrace::line)
            put(this, StackTrace::file)
        }.toGrpcValue()
    }

    class Collector: org.noelware.charted.modules.metrics.Collector<JvmThreadsMetrics>, io.prometheus.client.Collector() {
        private val threads = ManagementFactory.getThreadMXBean()

        override val name: String = "threads"
        override suspend fun supply(): JvmThreadsMetrics {
            val allThreads = threads.dumpAllThreads(true, true)
            val infos = allThreads.map {
                val userTimeNanos = threads.getThreadUserTime(it.threadId)
                val cpuTimeNanos = if (threads.isThreadCpuTimeEnabled) threads.getThreadCpuTime(it.threadId) else -1

                ThreadInfo(
                    it.stackTrace.map { element ->
                        StackTrace(
                            element.classLoaderName,
                            element.moduleName,
                            element.moduleVersion,
                            element.className,
                            element.methodName,
                            element.fileName,
                            element.lineNumber,
                            element.isNativeMethod,
                        )
                    },

                    if (userTimeNanos != -1L) userTimeNanos.nanoseconds.inWholeMilliseconds else -1,
                    if (cpuTimeNanos != -1L) cpuTimeNanos.nanoseconds.inWholeMilliseconds else -1,
                    it.isSuspended,
                    it.isDaemon,
                    it.threadState.name,
                    it.threadName,
                    it.threadId,
                )
            }

            return JvmThreadsMetrics(
                threads.threadCount,
                threads.peakThreadCount,
                threads.daemonThreadCount,
                infos,
            )
        }

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
        put(this, JvmThreadsMetrics::threads)
    }.toGrpcValue()
}
