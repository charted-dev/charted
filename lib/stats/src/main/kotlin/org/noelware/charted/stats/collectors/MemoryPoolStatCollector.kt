package org.noelware.charted.stats.collectors

import kotlinx.serialization.SerialName
import org.noelware.charted.common.extensions.formatToSize
import org.noelware.charted.stats.StatCollector
import java.lang.management.ManagementFactory

@kotlinx.serialization.Serializable
data class MemoryPoolStat(
    @SerialName("peak_usage")
    val peakUsage: MemoryPoolPeakUsage,
    val name: String,
    val type: String
)

@kotlinx.serialization.Serializable
data class MemoryPoolPeakUsage(
    @SerialName("used_human")
    val usedHuman: String,
    val used: Long,

    @SerialName("committed_human")
    val committedHuman: String,
    val committed: Long,

    @SerialName("max_human")
    val maxHuman: String,
    val max: Long,

    @SerialName("init_human")
    val initHuman: String,
    val init: Long
)

class MemoryPoolStatCollector: StatCollector<List<MemoryPoolStat>> {
    override suspend fun collect(): List<MemoryPoolStat> {
        val poolStats = mutableListOf<MemoryPoolStat>()
        for (pool in ManagementFactory.getMemoryPoolMXBeans()) {
            poolStats.add(
                MemoryPoolStat(
                MemoryPoolPeakUsage(
                    pool.peakUsage.used.formatToSize(),
                    pool.peakUsage.used,

                    pool.peakUsage.committed.formatToSize(),
                    pool.peakUsage.committed,

                    pool.peakUsage.max.formatToSize(),
                    pool.peakUsage.max,

                    pool.peakUsage.init.formatToSize(),
                    pool.peakUsage.init
                ),

                pool.name,
                pool.type.name
            ))
        }

        return poolStats.toList()
    }
}
