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

package org.noelware.charted.configuration.kotlin.dsl.metrics

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
public data class MetricsConfig(
    @SerialName("metricsets")
    val metricSets: MetricSets = MetricSets(),
    val enabled: Boolean = true,
    val path: String = "/metrics"
) {
    @Suppress("MemberVisibilityCanBePrivate")
    public class Builder: org.noelware.charted.common.Builder<MetricsConfig> {
        private var _metricSets: MetricSets = MetricSets()
        public var enabled: Boolean = true
        public var path: String = "/metrics"

        public fun metricSets(builder: MetricSets.Builder.() -> Unit = {}): Builder {
            _metricSets = MetricSets.Builder().apply(builder).build()
            return this
        }

        override fun build(): MetricsConfig = MetricsConfig(_metricSets, enabled, path)
    }
}
