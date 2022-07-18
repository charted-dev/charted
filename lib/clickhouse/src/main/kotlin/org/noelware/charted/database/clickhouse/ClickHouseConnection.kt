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

package org.noelware.charted.database.clickhouse

import com.zaxxer.hikari.HikariConfig
import com.zaxxer.hikari.HikariDataSource
import dev.floofy.utils.slf4j.logging
import io.sentry.ITransaction
import io.sentry.Sentry
import io.sentry.SpanStatus
import org.noelware.charted.common.SetOnceGetValue
import org.noelware.charted.common.data.ClickHouseConfig
import org.noelware.charted.common.extensions.ifSentryEnabled
import org.noelware.charted.common.extensions.measureTime
import java.io.Closeable
import java.sql.Connection
import java.util.concurrent.atomic.AtomicBoolean
import java.util.concurrent.atomic.AtomicLong

class ClickHouseConnection(private val config: ClickHouseConfig): Closeable {
    private val _serverVersion: SetOnceGetValue<String> = SetOnceGetValue()
    private val _dataSource: SetOnceGetValue<HikariDataSource> = SetOnceGetValue()
    private val _closed: AtomicBoolean = AtomicBoolean(false)
    private val _calls: AtomicLong = AtomicLong(0L)
    private val log by logging<ClickHouseConnection>()

    /**
     * Returns the server version the connection is running from.
     */
    val serverVersion: String
        get() = _serverVersion.value

    /**
     * Returns if the connection is closed or not
     */
    val closed: Boolean
        get() = _closed.get()

    /**
     * Returns how many database calls the connection has used.
     */
    val calls: Long
        get() = _calls.get()

    /**
     * Grabs a connection from the connection pool and calls the underlying [block].
     * @param block The block of code to execute
     * @return The result, as [T].
     * @throws java.sql.SQLException If any SQL exception occurred.
     */
    fun <T> grabConnection(block: Connection.(ITransaction?) -> T): T {
        if (!_dataSource.wasSet()) {
            throw IllegalStateException("#connect() was never called, can't grab connection!")
        }

        val connection = _dataSource.value.connection
        val transaction = ifSentryEnabled {
            Sentry.startTransaction("charted.database.Connection", "Retrieve connection to execute a SQL statement.")
        }

        try {
            return connection.block(transaction).let {
                _calls.incrementAndGet()
                transaction?.finish(SpanStatus.OK)

                connection.close()
                it
            }
        } catch (e: Throwable) {
            transaction?.throwable = e
            transaction?.finish(SpanStatus.UNKNOWN_ERROR)

            if (Sentry.isEnabled()) Sentry.captureException(e)

            log.error("Unable to execute connection block:", e)
            _dataSource.value.evictConnection(connection)

            throw e
        }
    }

    /**
     * Connects to the ClickHouse server.
     */
    fun connect() {
        if (_dataSource.wasSet()) {
            log.warn("Connection was already established!")
            return
        }

        if (closed) {
            log.warn("Connection was previously closed, not re-connecting.")
            return
        }

        val jdbcUrl = "jdbc:clickhouse://${config.host}:${config.port}/${config.database}"
        log.debug("Connecting to JDBC URL -> [$jdbcUrl]")
        _dataSource.value = HikariDataSource(
            HikariConfig().apply {
                driverClassName = "com.github.housepower.jdbc.ClickHouseDriver"
                this.jdbcUrl = jdbcUrl
                username = config.username
                password = config.password
            }
        )

        log.info("Created data source! Can we connect?")
        log.measureTime("Took %T to execute sql ['SELECT 1;']") {
            try {
                grabConnection {
                    val stmt = createStatement()
                    stmt.execute("SELECT 1;")
                }
            } catch (e: Exception) {
                log.error("Unable to connect to ClickHouse:", e)
                throw e
            }
        }

        val version = grabConnection {
            val stmt = prepareStatement("SELECT version() AS version;")
            stmt.execute()

            if (!stmt.resultSet.next()) {
                return@grabConnection null
            }

            stmt.resultSet.getString("version")
        } ?: throw IllegalStateException("Connection was successful, but can't retrieve server version?")

        _serverVersion.value = version
        log.info("Using ClickHouse v$version!")
    }

    override fun close() {
        if (closed) return

        log.warn("Closing connection...")
        _dataSource.value.close()
        _closed.set(true)

        log.info("Closed connection! ^~^")
    }
}
