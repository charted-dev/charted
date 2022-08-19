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

package org.noelware.charted.stats

import org.noelware.charted.stats.collectors.*

class StatisticsCollector {
    private val collectors = mutableMapOf<String, StatCollector<*>>()

    init {
        register("memory_pools", MemoryPoolStatCollector())
        register("coroutines", CoroutineStatCollector())
        register("threads", ThreadStatCollector())
        register("jvm", JvmStatCollector())
        register("os", OperatingSystemStatCollector())
    }

    fun <I: Any, C: StatCollector<I>> register(name: String, collector: C) {
        collectors[name] = collector
    }

    @Suppress("UNCHECKED_CAST")
    suspend fun <T> collect(name: String): T? = (collectors[name] as? StatCollector<T>)?.collect()
}
