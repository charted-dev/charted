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

@file:JvmName("ConfigurePostgreSQLKt")

package org.noelware.charted.modules.postgresql

import com.zaxxer.hikari.HikariConfig
import com.zaxxer.hikari.HikariDataSource
import dev.floofy.utils.kotlin.ifNotNull
import dev.floofy.utils.slf4j.logging
import org.apache.commons.lang3.time.StopWatch
import org.jetbrains.exposed.sql.Database
import org.jetbrains.exposed.sql.DatabaseConfig
import org.jetbrains.exposed.sql.SchemaUtils
import org.jetbrains.exposed.sql.Slf4jSqlDebugLogger
import org.jetbrains.exposed.sql.transactions.transaction
import org.noelware.charted.common.extensions.formatting.doFormatTime
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.modules.postgresql.tables.*
import kotlin.time.Duration.Companion.seconds

private val log by logging("org.noelware.charted.modules.postgresql.ConfigurePostgreSQLKt")
fun configure(config: Config, sw: StopWatch? = null) {
    if (sw?.isSuspended == true) sw.resume()
    val ds = HikariDataSource(
        HikariConfig().apply {
            leakDetectionThreshold = 30.seconds.inWholeMilliseconds
            driverClassName = "org.postgresql.Driver"
            isAutoCommit = false
            poolName = "Charted-Postgres-HikariPool"
            username = config.database.username
            password = config.database.password
            jdbcUrl = "jdbc:postgresql://${config.database.host}:${config.database.port}/${config.database.database}"
            schema = config.database.schema

            addDataSourceProperty("reWriteBatchedInserts", "true")
        },
    )

    Database.connect(
        ds,
        databaseConfig = DatabaseConfig {
            defaultRepetitionAttempts = 5
            sqlLogger = if (config.debug || System.getProperty("org.noelware.charted.debug", "false") == "true") {
                Slf4jSqlDebugLogger
            } else {
                null
            }
        },
    )

    sw.ifNotNull {
        suspend()
        log.info("Connected to PostgreSQL in [${doFormatTime()}], running migrations...")

        resume()
    }

    transaction {
        createOrUpdateEnums()
        SchemaUtils.createMissingTablesAndColumns(
            ApiKeyTable,
            OrganizationTable,
            OrganizationMemberTable,
            RepositoryTable,
            RepositoryMemberTable,
            RepositoryReleaseTable,
            UserTable,
            UserConnectionsTable,
        )
    }

    sw.ifNotNull {
        suspend()
        log.info("Ran all migrations in [${doFormatTime()}]")
    }
}
