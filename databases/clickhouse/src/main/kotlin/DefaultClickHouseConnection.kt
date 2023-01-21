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

package org.noelware.charted.databases.clickhouse

import com.clickhouse.jdbc.JdbcConfig
import com.zaxxer.hikari.HikariConfig
import com.zaxxer.hikari.HikariDataSource
import dev.floofy.utils.java.SetOnce
import dev.floofy.utils.slf4j.logging
import io.sentry.Sentry
import io.sentry.SpanStatus
import kotlinx.atomicfu.AtomicBoolean
import kotlinx.atomicfu.AtomicLong
import kotlinx.atomicfu.atomic
import org.apache.commons.lang3.time.StopWatch
import org.noelware.charted.configuration.kotlin.dsl.ClickHouseConfig
import org.noelware.charted.extensions.closeable.closeQuietly
import org.noelware.charted.extensions.doFormatTime
import org.noelware.charted.extensions.ifSentryEnabled
import java.sql.Connection
import kotlin.time.Duration.Companion.seconds

class DefaultClickHouseConnection(private val config: ClickHouseConfig) : ClickHouseConnection {
    private val _serverVersion: SetOnce<String> = SetOnce()
    private val _dataSource: SetOnce<HikariDataSource> = SetOnce()
    private val _closed: AtomicBoolean = atomic(false)
    private val _calls: AtomicLong = atomic(0L)
    private val log by logging<DefaultClickHouseConnection>()

    override val closed: Boolean
        get() = _closed.value

    override val serverVersion: String
        get() = _serverVersion.value

    override val calls: Long
        get() = _calls.value

    /**
     * Creates and uses a new [Connection] to do some queries to the
     * ClickHouse server.
     *
     * @param block connection function to use.
     */
    override fun <T> create(block: Connection.() -> T): T {
        if (!_dataSource.wasSet()) throw IllegalAccessException("Can't grab connection due to no connection being set!")

        val connection = _dataSource.value.connection
        val transaction = ifSentryEnabled { Sentry.startTransaction("ClickHouse#grabConnection", "SQL") }
        return try {
            connection.block().let {
                _calls.incrementAndGet()
                transaction?.finish()
                connection.close()

                it
            }
        } catch (e: Exception) {
            transaction?.finish(SpanStatus.INTERNAL_ERROR)
            ifSentryEnabled { Sentry.captureException(e) }

            throw e
        }
    }

    override fun connect() {
        if (_dataSource.wasSet()) {
            log.warn("Connection was already previously established.")
            return
        }

        if (closed) {
            log.warn("This connection is already closed!")
            return
        }

        val sw = StopWatch.createStarted()
        val jdbcUrl = "jdbc:clickhouse://${config.host}:${config.port}/${config.database}"
        log.debug("Connecting to ClickHouse with JDBC URL [$jdbcUrl]")

        _dataSource.value = HikariDataSource(
            HikariConfig().apply {
                leakDetectionThreshold = 30.seconds.inWholeMilliseconds
                this.jdbcUrl = jdbcUrl
                driverClassName = "com.clickhouse.jdbc.ClickHouseDriver"
                poolName = "ClickHouse-HikariPool"
                username = config.username
                password = config.password

                addDataSourceProperty(JdbcConfig.PROP_WRAPPER_OBJ, "true")
            },
        )

        log.info("Created the connection pool! Checking if we can query...")
        try {
            create {
                val stmt = createStatement()
                stmt.execute("SELECT 1")
                stmt.close()
            }

            sw.stop()
            log.info("Connection was a success! Took ${sw.doFormatTime()} to create data source and query to ClickHouse!")
        } catch (e: Exception) {
            ifSentryEnabled { Sentry.captureException(e) }
            throw e
        }

        val version = create {
            val stmt = createStatement()
            stmt.execute("SELECT version() AS version")

            if (!stmt.resultSet.next()) {
                stmt.close()
                null
            } else {
                val version = stmt.resultSet.getString("version")
                stmt.close()

                version
            }
        } ?: throw IllegalStateException("Connection was successful, but can't get server version?")

        _serverVersion.value = version
        log.info("Server is using ClickHouse v$version!")
    }

    override fun close() {
        if (_closed.compareAndSet(expect = false, update = true)) {
            log.warn("Closing ClickHouse connection...")
            _dataSource.value.closeQuietly()
        }
    }
}
