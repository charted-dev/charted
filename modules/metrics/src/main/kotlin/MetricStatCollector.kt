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

import io.prometheus.client.Collector
import io.prometheus.client.Predicate

/**
 * Represents a statistics collector that formats the data into what Prometheus can consume. This uses the
 * [GenericStatCollector] object to collect what metrics it needs.
 */
interface MetricStatCollector {
    /**
     * Read the documentation on [Collector.collect] for more information.
     */
    fun collect(): MutableList<Collector.MetricFamilySamples>

    /**
     * Read the documentation on [Collector.collect] for more information.
     */
    fun collect(predicate: Predicate<String>?): MutableList<Collector.MetricFamilySamples>
}
