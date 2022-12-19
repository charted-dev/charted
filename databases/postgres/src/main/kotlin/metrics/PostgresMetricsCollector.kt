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

package org.noelware.charted.databases.postgres.metrics

import io.prometheus.client.Collector
import io.prometheus.client.GaugeMetricFamily
import io.prometheus.client.Predicate
import io.prometheus.client.SampleNameFilter
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.metrics.keys.PostgresMetricKeys
import org.noelware.charted.modules.metrics.MetricStatCollector

class PostgresMetricsCollector(private val config: Config): MetricStatCollector {
    override fun collect(): MutableList<Collector.MetricFamilySamples> = collect {
        config.metrics.metricSets.postgres.firstOrNull() == PostgresMetricKeys.Wildcard ||
            config.metrics.metricSets.postgres.contains(PostgresMetricKeys.values().find { f -> f.key == it })
    }

    override fun collect(predicate: Predicate<String>?): MutableList<Collector.MetricFamilySamples> {
        val mfs = mutableListOf<Collector.MetricFamilySamples>()
        collect0(predicate ?: SampleNameFilter.ALLOW_ALL, mfs)

        return mfs
    }

    private fun collect0(predicate: Predicate<String>, mfs: MutableList<Collector.MetricFamilySamples>) {
        val stats = PostgresStatsCollector.collect()
        if (predicate.test(PostgresMetricKeys.TotalOrganizationsAvailable.key)) {
            mfs.add(
                GaugeMetricFamily(
                    PostgresMetricKeys.TotalOrganizationsAvailable.key,
                    "Returns how many registered organizations are available",
                    stats.organizations.toDouble()
                )
            )
        }

        if (predicate.test(PostgresMetricKeys.TotalRepositoriesAvailable.key)) {
            mfs.add(
                GaugeMetricFamily(
                    PostgresMetricKeys.TotalRepositoriesAvailable.key,
                    "Returns how many registered repositories are available",
                    stats.repositories.toDouble()
                )
            )
        }

        if (predicate.test(PostgresMetricKeys.TotalUsersAvailable.key)) {
            mfs.add(
                GaugeMetricFamily(
                    PostgresMetricKeys.TotalUsersAvailable.key,
                    "Returns how many registered users are available",
                    stats.users.toDouble()
                )
            )
        }

        if (predicate.test(PostgresMetricKeys.ServerUptime.key)) {
            mfs.add(
                GaugeMetricFamily(
                    PostgresMetricKeys.ServerUptime.key,
                    "Returns the uptime (in milliseconds) of the Postgres server.",
                    stats.uptime.toDouble()
                )
            )
        }

        if (predicate.test(PostgresMetricKeys.Version.key)) {
            mfs.add(
                GaugeMetricFamily(
                    PostgresMetricKeys.Version.key,
                    "Returns the current PostgresSQL server version",
                    listOf("version")
                ).apply { addMetric(listOf(stats.version), 1.0) }
            )
        }
    }
}
