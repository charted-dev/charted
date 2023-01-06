/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware <team@noelware.org>
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

package org.noelware.charted.modules.metrics.prometheus

import com.zaxxer.hikari.HikariDataSource
import com.zaxxer.hikari.metrics.prometheus.PrometheusMetricsTrackerFactory
import dev.floofy.utils.slf4j.logging
import io.prometheus.client.CollectorRegistry
import io.prometheus.client.Counter
import io.prometheus.client.Histogram
import io.prometheus.client.exporter.common.TextFormat
import io.prometheus.client.hotspot.DefaultExports
import org.noelware.charted.modules.metrics.Collector
import org.noelware.charted.modules.metrics.MetricsSupport
import org.noelware.charted.modules.metrics.default.DefaultMetricsSupport
import java.io.Writer
import kotlin.reflect.KClass

class PrometheusMetricsSupport(dataSource: HikariDataSource) : MetricsSupport {
    private val _delegate: MetricsSupport = DefaultMetricsSupport()
    private val registry: CollectorRegistry = CollectorRegistry()
    private val log by logging<PrometheusMetricsSupport>()

    val serverRequests: Counter = Counter.build("charted_server_ktor_requests", "How many requests have been executed")
        .register(registry)

    val serverRequestLatency: Histogram = Histogram.build("charted_server_ktor_request_latency", "The latency between each request")
        .labelNames("method", "endpoint", "version")
        .register(registry)

    init {
        dataSource.metricsTrackerFactory = PrometheusMetricsTrackerFactory()
        DefaultExports.register(registry)
    }

    override val collectors: List<Collector<*>> = _delegate.collectors
    override fun add(collector: Collector<*>) {
        _delegate.add(collector)
        if (collector is io.prometheus.client.Collector) {
            registry.register(collector)
        } else {
            log.warn("Collector $collector doesn't implement [io.prometheus.client.Collector], collector might not be available in final Prometheus output!")
        }
    }

    override suspend fun collect(): Map<String, Any> = _delegate.collect()
    override suspend fun <U : Any> collectFrom(collector: KClass<Collector<U>>): U? = _delegate.collectFrom(collector)

    fun <W : Writer> writeIn(writer: W): Unit = TextFormat.write004(writer, registry.metricFamilySamples())
}
