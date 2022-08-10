package org.noelware.charted.stats.collectors

import dev.floofy.utils.kotlin.humanize
import org.noelware.charted.common.extensions.formatToSize
import org.noelware.charted.stats.StatCollector
import java.lang.management.ManagementFactory
import java.lang.management.RuntimeMXBean

@kotlinx.serialization.Serializable
data class JvmStats(
    val totalMemoryBytes: Long,
    val totalMemoryHuman: String,
    val maxMemoryBytes: Long,
    val maxMemoryHuman: String,
    val freeMemoryBytes: Long,
    val freeMemoryHuman: String,
    val startTimeMs: Long,
    val startTimeHuman: String,
    val version: String,
    val vendor: String,
    val name: String,
    val date: String,
    val pid: Long
)

class JvmStatCollector: StatCollector<JvmStats> {
    private val runtimeBean: RuntimeMXBean = ManagementFactory.getRuntimeMXBean()
    private val runtime: Runtime
        get() = Runtime.getRuntime()

    override suspend fun collect(): JvmStats {
        val totalMemory = runtime.totalMemory()
        val maxMemory = runtime.maxMemory()
        val freeMemory = runtime.freeMemory()
        val uptime = System.currentTimeMillis() - runtimeBean.startTime
        val version = Runtime.version().toString()
        val vendor = runtimeBean.vmVendor
        val name = runtimeBean.vmName
        val date = System.getProperty("java.version.date")
        val pid = runtimeBean.uptime

        return JvmStats(
            totalMemory,
            totalMemory.formatToSize(),
            maxMemory,
            maxMemory.formatToSize(),
            freeMemory,
            freeMemory.formatToSize(),
            uptime,
            uptime.humanize(),
            version,
            vendor,
            name,
            date,
            pid
        )
    }
}
