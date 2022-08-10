package org.noelware.charted.stats.collectors

import dev.floofy.utils.kotlin.humanize
import kotlinx.serialization.SerialName
import org.noelware.charted.stats.StatCollector
import java.lang.management.ManagementFactory
import java.lang.management.ThreadMXBean
import kotlin.time.Duration.Companion.nanoseconds

@kotlinx.serialization.Serializable
data class ThreadStats(
    val count: Int,
    val background: Int,
    val threads: List<ThreadInfo>
)

@kotlinx.serialization.Serializable
data class ThreadInfo(
    @SerialName("user_time_ms")
    val userTimeMs: Long,

    @SerialName("user_time_human")
    val userTimeHuman: String? = null,

    @SerialName("cpu_time_ms")
    val cpuTimeMs: Long,

    @SerialName("cpu_time_human")
    val cpuTimeHuman: String? = null,
    val stacktrace: List<ThreadStackTrace> = listOf(),
    val suspended: Boolean,
    val background: Boolean,
    val state: String,
    val name: String,
    val id: Long
)

@kotlinx.serialization.Serializable
data class ThreadStackTrace(
    @SerialName("class_loader_name")
    val classLoaderName: String,

    @SerialName("module_name")
    val moduleName: String,

    @SerialName("module_version")
    val moduleVersion: String,

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
                if (userTimeMs == -1L) -1 else userTimeMs.nanoseconds.inWholeMilliseconds,
                if (userTimeMs == -1L) null else userTimeMs.nanoseconds.inWholeMilliseconds.humanize(),
                if (cpuTimeMs == -1L) -1 else cpuTimeMs.nanoseconds.inWholeMilliseconds,
                if (cpuTimeMs == -1L) null else cpuTimeMs.nanoseconds.inWholeMilliseconds.humanize(),
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
