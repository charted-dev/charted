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

package org.noelware.charted.stats.collectors

import dev.floofy.utils.kotlin.humanize
import kotlinx.serialization.SerialName
import org.noelware.charted.stats.StatCollector
import java.lang.management.ManagementFactory
import java.lang.management.ThreadMXBean
import java.time.Duration

@kotlinx.serialization.Serializable
data class ThreadStats(
    val count: Int,
    val background: Int,
    val threads: List<ThreadInfo>
)

@kotlinx.serialization.Serializable
data class ThreadInfo(
    val stacktrace: List<ThreadStackTrace> = listOf(),

    @SerialName("user_time_ms")
    val userTimeMs: Long,

    @SerialName("user_time_human")
    val userTimeHuman: String? = null,

    @SerialName("cpu_time_ms")
    val cpuTimeMs: Long,

    @SerialName("cpu_time_human")
    val cpuTimeHuman: String? = null,
    val suspended: Boolean,
    val background: Boolean,
    val state: String,
    val name: String,
    val id: Long
)

@kotlinx.serialization.Serializable
data class ThreadStackTrace(
    @SerialName("class_loader_name")
    val classLoaderName: String? = null,

    @SerialName("module_name")
    val moduleName: String? = null,

    @SerialName("module_version")
    val moduleVersion: String? = null,

    @SerialName("declaring_class")
    val declaringClass: String,

    @SerialName("method_name")
    val methodName: String,
    val file: String? = null,
    val line: Int? = null,

    @SerialName("is_native_method")
    val isNativeMethod: Boolean
)

class ThreadStatCollector: StatCollector<ThreadStats> {
    private val threads: ThreadMXBean
        get() = ManagementFactory.getThreadMXBean()

    override suspend fun collect(): ThreadStats {
        val infos = threads.dumpAllThreads(true, true)
        val threadInfos = infos.map {
            val userTimeMs = threads.getThreadUserTime(it.threadId)
            val cpuTimeMs = if (threads.isThreadCpuTimeEnabled) threads.getThreadCpuTime(it.threadId) else -1

            ThreadInfo(
                it.stackTrace.map { element ->
                    ThreadStackTrace(
                        element.classLoaderName,
                        element.moduleName,
                        element.moduleVersion,
                        element.className,
                        element.methodName,
                        element.fileName,
                        element.lineNumber,
                        element.isNativeMethod
                    )
                },

                if (userTimeMs == -1L) -1 else Duration.ofNanos(userTimeMs).toMillis(),
                if (userTimeMs == -1L) null else Duration.ofNanos(userTimeMs).toMillis().humanize(),
                if (cpuTimeMs == -1L) -1 else Duration.ofNanos(cpuTimeMs).toMillis(),
                if (cpuTimeMs == -1L) null else Duration.ofNanos(cpuTimeMs).toMillis().humanize(),
                it.isSuspended,
                it.isDaemon,
                it.threadState.name,
                it.threadName,
                it.threadId
            )
        }

        return ThreadStats(
            threads.threadCount,
            threads.daemonThreadCount,
            threadInfos
        )
    }
}
