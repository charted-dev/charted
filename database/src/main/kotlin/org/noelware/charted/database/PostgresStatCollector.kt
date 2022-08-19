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

package org.noelware.charted.database

import dev.floofy.utils.exposed.asyncTransaction
import org.jetbrains.exposed.sql.TextColumnType
import org.noelware.charted.common.ChartedScope
import org.noelware.charted.common.data.Config
import org.noelware.charted.database.entities.OrganizationEntity
import org.noelware.charted.database.entities.RepositoryEntity
import org.noelware.charted.database.entities.UserEntity
import org.noelware.charted.stats.StatCollector
import kotlin.time.DurationUnit
import kotlin.time.toDuration

@kotlinx.serialization.Serializable
data class PostgresStats(
    val organizations: Long,
    val serverVersion: String,
    val repositories: Long,
    val numBackends: Int,
    val inserted: Long,
    val deleted: Long,
    val fetched: Long,
    val uptime: Long,
    val users: Long
)

class PostgresStatCollector(private val config: Config): StatCollector<PostgresStats> {
    override suspend fun collect(): PostgresStats = asyncTransaction(ChartedScope) {
        val o = OrganizationEntity.count()
        val r = RepositoryEntity.count()
        val u = UserEntity.count()

        // this will be in seconds, so we need to transform it into milliseconds to get the humanized
        // time correctly.
        val uptime = exec("SELECT extract(epoch FROM current_timestamp - pg_postmaster_start_time()) AS uptime;") { rs ->
            if (!rs.next()) return@exec 0L

            rs.getLong("uptime").toDuration(DurationUnit.MILLISECONDS).inWholeMilliseconds
        }

        val version = exec("SELECT version();") { rs ->
            if (!rs.next()) return@exec "???"

            val version = rs.getString("version").trim()
            version.split(" ").first { it.matches("\\d{0,9}.\\d{0,9}?\\d{0,9}".toRegex()) }
        }

        exec(
            "SELECT numbackends, tup_fetched, tup_inserted, tup_deleted FROM pg_stat_database WHERE datname = ?;",
            listOf(TextColumnType() to config.postgres.name)
        ) { rs ->
            if (!rs.next()) {
                return@exec PostgresStats(
                    o,
                    version!!,
                    r,
                    0,
                    0,
                    0,
                    0,
                    uptime!!,
                    u
                )
            }

            PostgresStats(
                o,
                version!!,
                r,
                rs.getInt("numbackends"),
                rs.getLong("tup_inserted"),
                rs.getLong("tup_deleted"),
                rs.getLong("tup_fetched"),
                uptime!!,
                u
            )
        }!!
    }
}
