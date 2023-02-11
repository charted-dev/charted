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

package org.noelware.charted.modules.metrics.default

import dev.floofy.utils.kotlin.ifNotNull
import org.noelware.charted.modules.metrics.Collector
import org.noelware.charted.modules.metrics.MetricsSupport
import org.noelware.charted.modules.metrics.collectors.JvmThreadsMetrics
import kotlin.reflect.KClass

class DefaultMetricsSupport : MetricsSupport {
    private val _collectors: MutableList<Collector<*>> = mutableListOf()

    init {
        add(JvmThreadsMetrics.Collector())
    }

    override val collectors: List<Collector<*>> = _collectors
    override fun add(collector: Collector<*>) {
        _collectors.add(collector)
    }

    override suspend fun collect(): Map<String, Any> {
        val result = mutableMapOf<String, Any>()
        for (collector in _collectors) {
            result[collector.name] = collector.supply()
        }

        return result
    }

    @Suppress("UNCHECKED_CAST")
    override suspend fun <U : Any> collectFrom(collector: KClass<Collector<U>>): U? {
        val collectorInstance = _collectors.find { it.javaClass.isAssignableFrom(collector::class.java) }?.ifNotNull {
            this as? Collector<U>
        } ?: return null

        return collectorInstance.supply()
    }
}
