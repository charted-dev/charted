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

package org.noelware.charted.modules.metrics

import com.zaxxer.hikari.HikariDataSource
import com.zaxxer.hikari.metrics.prometheus.PrometheusMetricsTrackerFactory
import io.prometheus.client.*
import io.prometheus.client.exporter.common.TextFormat
import io.prometheus.client.hotspot.DefaultExports
import java.io.Writer

class PrometheusMetrics(dataSource: HikariDataSource) {
    private val _collectors: MutableList<GenericStatCollector<*>> = mutableListOf()
    private val registry: CollectorRegistry = CollectorRegistry()

    val collectors: List<GenericStatCollector<*>>
        get() = _collectors.toList()

    val requestLatency: Histogram = Histogram.build("charted_request_latency", "The request latency between this request")
        .labelNames("method", "url", "protocol")
        .register(registry)

    val requests: Counter = Counter.build("charted_requests", "How many requests the server has handled")
        .register(registry)

    init {
        dataSource.metricsTrackerFactory = PrometheusMetricsTrackerFactory(registry)
        DefaultExports.register(registry)
    }

    fun <T> addGenericCollector(collector: GenericStatCollector<T>) {
        _collectors.add(collector)
    }

    fun addMetricCollector(collector: MetricStatCollector) {
        registry.register(object: Collector() {
            override fun toString(): String = "ProxyCollector(${collector::class})"
            override fun collect(): MutableList<MetricFamilySamples> = collector.collect()
            override fun collect(sampleNameFilter: Predicate<String>?): MutableList<MetricFamilySamples> = collector.collect(sampleNameFilter)
        })
    }

    fun collect(): Map<String, Any> {
        val items = mutableMapOf<String, Any>()
        for (collector in collectors) {
            items[collector.name] = collector.collect() as Any
        }

        return items
    }

    fun <W: Writer> writeIn(writer: W) {
        TextFormat.write004(writer, registry.metricFamilySamples())
    }
}
