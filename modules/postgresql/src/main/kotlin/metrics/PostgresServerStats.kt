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

package org.noelware.charted.modules.postgresql.metrics

import com.fasterxml.jackson.annotation.JsonProperty
import com.google.protobuf.Value
import io.prometheus.client.GaugeMetricFamily
import io.prometheus.client.Predicate
import io.prometheus.client.SampleNameFilter
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.jetbrains.exposed.sql.TextColumnType
import org.jetbrains.exposed.sql.transactions.transaction
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.enumSets.serialName
import org.noelware.charted.configuration.kotlin.dsl.metrics.keysets.PostgresKeysets
import org.noelware.charted.configuration.kotlin.dsl.metrics.keysets.enumSet
import org.noelware.charted.modules.analytics.kotlin.dsl.*
import org.noelware.charted.modules.postgresql.entities.OrganizationEntity
import org.noelware.charted.modules.postgresql.entities.RepositoryEntity
import org.noelware.charted.modules.postgresql.entities.UserEntity
import kotlin.time.Duration.Companion.milliseconds

@Serializable
data class PostgresServerStats(
    val organizations: Long,
    val repositories: Long,
    val version: String,

    @JsonProperty("db_size")
    @SerialName("db_size")
    val dbSize: Long,
    val uptime: Long,
    val users: Long
): org.noelware.analytics.jvm.server.serialization.Serializable {
    override fun toGrpcValue(): Value = Struct {
        put(this, PostgresServerStats::organizations)
        put(this, PostgresServerStats::repositories)
        put(this, PostgresServerStats::version)
        put(this, PostgresServerStats::dbSize)
        put(this, PostgresServerStats::uptime)
        put(this, PostgresServerStats::users)
    }.toGrpcValue()

    class Collector(private val config: Config): org.noelware.charted.modules.metrics.Collector<PostgresServerStats>, io.prometheus.client.Collector() {
        override val name: String = "postgresql"
        override suspend fun supply(): PostgresServerStats = transaction {
            val organizations = OrganizationEntity.count()
            val repositories = RepositoryEntity.count()
            val users = UserEntity.count()

            val uptime = exec("SELECT extract(epoch FROM current_timestamp - pg_postmaster_start_time()) AS uptime;") { rs ->
                if (rs.next()) rs.getLong("uptime").milliseconds.inWholeMilliseconds else -1
            }

            val postgresVersion = exec("SELECT version();") { rs ->
                if (!rs.next()) return@exec "Unknown"

                val version = rs.getString("version").trim()
                version
                    .split(" ")
                    .first { it matches "\\d{0,9}.\\d{0,9}?\\d{0,9}".toRegex() }
            }

            val databaseSize = exec("SELECT pg_database_size(?);", listOf(TextColumnType() to config.database.database)) { rs ->
                if (rs.next()) rs.getLong("pg_database_size") else -1
            }

            PostgresServerStats(
                organizations,
                repositories,
                postgresVersion!!,
                uptime!!,
                databaseSize!!,
                users,
            )
        }

        override fun collect(): MutableList<MetricFamilySamples> = collect {
            PostgresKeysets.EnumSet.enabled(config.metrics.metricSets.postgres, it)
        }

        override fun collect(predicate: Predicate<String>?): MutableList<MetricFamilySamples> {
            val mfs = mutableListOf<MetricFamilySamples>()
            collect0(predicate ?: SampleNameFilter.ALLOW_ALL, mfs)

            return mfs
        }

        private fun collect0(predicate: Predicate<String>, mfs: MutableList<MetricFamilySamples>) {
            val stats = runBlocking { supply() }
            if (predicate.test(PostgresKeysets.TotalOrganizationsAvailable.serialName!!)) {
                mfs.add(
                    GaugeMetricFamily(
                        PostgresKeysets.TotalOrganizationsAvailable.serialName!!,
                        "Returns how many registered organizations are available",
                        stats.organizations.toDouble(),
                    ),
                )
            }

            if (predicate.test(PostgresKeysets.TotalRepositoriesAvailable.serialName!!)) {
                mfs.add(
                    GaugeMetricFamily(
                        PostgresKeysets.TotalRepositoriesAvailable.serialName!!,
                        "Returns how many registered repositories are available",
                        stats.repositories.toDouble(),
                    ),
                )
            }

            if (predicate.test(PostgresKeysets.TotalUsersAvailable.serialName!!)) {
                mfs.add(
                    GaugeMetricFamily(
                        PostgresKeysets.TotalUsersAvailable.serialName!!,
                        "Returns how many registered users are available",
                        stats.users.toDouble(),
                    ),
                )
            }

            if (predicate.test(PostgresKeysets.DatabaseSize.serialName!!)) {
                mfs.add(
                    GaugeMetricFamily(
                        PostgresKeysets.DatabaseSize.serialName!!,
                        "The database size (in bytes), or -1 if it couldn't be calculated",
                        stats.dbSize.toDouble(),
                    ),
                )
            }

            if (predicate.test(PostgresKeysets.ServerUptime.serialName!!)) {
                mfs.add(
                    GaugeMetricFamily(
                        PostgresKeysets.ServerUptime.serialName!!,
                        "Returns the uptime (in milliseconds) of the Postgres server.",
                        stats.uptime.toDouble(),
                    ),
                )
            }

            if (predicate.test(PostgresKeysets.Version.serialName!!)) {
                mfs.add(
                    GaugeMetricFamily(
                        PostgresKeysets.Version.serialName!!,
                        "Returns the current PostgresSQL server version",
                        listOf("version"),
                    ).apply { addMetric(listOf(stats.version), 1.0) },
                )
            }
        }
    }
}
