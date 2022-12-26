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

import org.jetbrains.exposed.sql.TextColumnType
import org.jetbrains.exposed.sql.transactions.transaction
import org.noelware.charted.databases.postgres.entities.OrganizationEntity
import org.noelware.charted.databases.postgres.entities.RepositoryEntity
import org.noelware.charted.databases.postgres.entities.UserEntity
import org.noelware.charted.modules.metrics.GenericStatCollector
import kotlin.time.DurationUnit
import kotlin.time.toDuration

object PostgresStatsCollector: GenericStatCollector<PostgresServerStats> {
    override val name: String = "postgres"
    override fun collect(): PostgresServerStats = transaction {
        val users = UserEntity.count()
        val organizations = OrganizationEntity.count()
        val repositories = RepositoryEntity.count()

        val uptime = exec("SELECT extract(epoch FROM current_timestamp - pg_postmaster_start_time()) AS uptime;") { rs ->
            if (!rs.next()) return@exec -1L
            rs.getLong("uptime").toDuration(DurationUnit.MILLISECONDS).inWholeMilliseconds
        }

        val version = exec("SELECT version();") { rs ->
            if (!rs.next()) return@exec "???"

            val version = rs.getString("version").trim()
            version
                .split(" ")
                .first { it matches "\\d{0,9}.\\d{0,9}?\\d{0,9}".toRegex() }
        }

        // Get database size (in bytes)
        val dbSize = exec("SELECT pg_database_size(?);", listOf(TextColumnType() to "charted")) { rs ->
            if (!rs.next()) return@exec -1

            rs.getLong("pg_database_size")
        }

        PostgresServerStats(
            organizations,
            repositories,
            version!!,
            uptime!!,
            dbSize!!,
            users
        )
    }
}
