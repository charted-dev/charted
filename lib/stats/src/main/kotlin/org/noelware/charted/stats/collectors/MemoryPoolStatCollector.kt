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
                )
            )
        }

        return poolStats.toList()
    }
}
