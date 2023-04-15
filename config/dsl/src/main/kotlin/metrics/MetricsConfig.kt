/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

package org.noelware.charted.configuration.kotlin.dsl.metrics

import kotlinx.serialization.Serializable
import org.noelware.charted.common.Buildable
import org.noelware.charted.configuration.kotlin.dsl.metrics.keysets.PostgresKeysets
import org.noelware.charted.configuration.kotlin.dsl.search.elasticsearch.MetricKeyset as ElasticsearchMetricKeysets
import org.noelware.charted.configuration.kotlin.dsl.server.MetricKeysets as ServerMetricKeysets

@Serializable
public data class Metricsets(
    /** Key-set for configuring Elasticsearch metrics that are specific to charted-server. */
    val elasticsearch: List<ElasticsearchMetricKeysets> = listOf(),

    /** Key-set for configuring PostgreSQL metrics specific to charted-server. */
    val postgres: List<PostgresKeysets> = listOf(),

    /** Key-set for configuring API server metrics. */
    val server: List<ServerMetricKeysets> = listOf()
) {
    public class Builder: Buildable<Metricsets> {
        private val elasticsearch = mutableListOf<ElasticsearchMetricKeysets>()
        private val postgres = mutableListOf<PostgresKeysets>()
        private val server = mutableListOf<ServerMetricKeysets>()

        /**
         * Enables one or more key-sets for our Elasticsearch connection
         * to produce.
         *
         * @param keys List of [ElasticsearchMetricKeysets] to configure
         */
        public fun elasticsearch(vararg keys: ElasticsearchMetricKeysets): Builder {
            for (key in keys) {
                if (elasticsearch.contains(key)) continue
                elasticsearch.add(key)
            }

            return this
        }

        /**
         * Enables one or more key-sets for our Elasticsearch connection
         * to produce.
         *
         * @param keys List of [PostgresKeysets] to configure
         */
        public fun postgres(vararg keys: PostgresKeysets): Builder {
            for (key in keys) {
                if (postgres.contains(key)) continue
                postgres.add(key)
            }

            return this
        }

        /**
         * Enables one or more key-sets for the API server to produce
         * metrics.
         *
         * @param keys List of [ServerMetricKeysets] to configure
         */
        public fun server(vararg keys: ServerMetricKeysets): Builder {
            for (key in keys) {
                if (server.contains(key)) continue
                server.add(key)
            }

            return this
        }

        override fun build(): Metricsets = Metricsets(elasticsearch, postgres, server)
    }
}

@Serializable
public data class MetricsConfig(
    /** A subset of metric key-sets that should be enabled. */
    val metricSets: Metricsets = Metricsets(),

    /** If the Prometheus metrics endpoint is enabled or not */
    val enabled: Boolean = true,

    /** Path to locate the metrics endpoint. */
    val path: String = "/metrics"
) {
    init {
        check(path.startsWith('/')) { "Path [$path] must start with '/'" }
    }

    @Suppress("MemberVisibilityCanBePrivate")
    public class Builder: Buildable<MetricsConfig> {
        private var metricSets = Metricsets()

        /** If the Prometheus metrics endpoint is enabled or not */
        public var enabled: Boolean = false

        /** Path to locate the metrics endpoint. */
        public var path: String = "/metrics"

        /**
         * Configure the key-set for this [MetricsConfig].
         * @param builder Builder DSL block to execute to produce a [Metricsets] object.
         */
        public fun metricSets(builder: Metricsets.Builder.() -> Unit = {}): Builder {
            metricSets = Metricsets.Builder().apply(builder).build()
            return this
        }

        override fun build(): MetricsConfig = MetricsConfig(metricSets, enabled, path)
    }
}
