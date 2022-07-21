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

package org.noelware.charted.metrics

import com.zaxxer.hikari.HikariDataSource
import com.zaxxer.hikari.metrics.prometheus.PrometheusMetricsTrackerFactory
import io.prometheus.client.Collector
import io.prometheus.client.CollectorRegistry
import io.prometheus.client.Counter
import io.prometheus.client.Histogram
import io.prometheus.client.exporter.common.TextFormat
import io.prometheus.client.hotspot.DefaultExports
import java.io.Writer

class PrometheusMetrics(dataStore: HikariDataSource) {
    private val registry: CollectorRegistry = CollectorRegistry()
    val requestLatency: Histogram = Histogram.build("charted_request_latency", "The latency between all requests.")
        .register(registry)

    val requests: Counter = Counter.build("charted_requests", "How many requests the server has handled")
        .register(registry)

    init {
        dataStore.metricsTrackerFactory = PrometheusMetricsTrackerFactory(registry)
        DefaultExports.register(registry)
    }

    fun <T: Collector> addCollector(collector: T) {
        registry.register(collector)
    }

    fun <W: Writer> writeTo(writer: W) {
        TextFormat.write004(writer, registry.metricFamilySamples())
    }
}
