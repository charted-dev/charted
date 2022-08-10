package org.noelware.charted.stats

import org.noelware.charted.stats.collectors.CoroutineStatCollector
import org.noelware.charted.stats.collectors.JvmStatCollector
import org.noelware.charted.stats.collectors.MemoryPoolStatCollector
import org.noelware.charted.stats.collectors.ThreadStatCollector

class StatisticsCollector {
    private val collectors = mutableMapOf<String, StatCollector<*>>()

    init {
        register("memory_pools" to MemoryPoolStatCollector())
        register("coroutines" to CoroutineStatCollector())
        register("threads" to ThreadStatCollector())
        register("jvm" to JvmStatCollector())
    }

    fun <I, C: StatCollector<I>> register(mapping: Pair<String, C>) {
        val (name, collector) = mapping
        collectors[name] = collector
    }

    suspend fun <T> collect(name: String): T? = (collectors[name] as? StatCollector<T>)?.collect()
}
