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

import com.clickhouse.jdbc.JdbcConfig
import com.zaxxer.hikari.HikariConfig
import com.zaxxer.hikari.HikariDataSource
import dev.floofy.utils.slf4j.logging
import io.sentry.ITransaction
import io.sentry.Sentry
import io.sentry.SpanStatus
import org.intellij.lang.annotations.Language
import org.noelware.charted.common.SetOnceGetValue
import org.noelware.charted.common.data.ClickHouseConfig
import org.noelware.charted.common.extensions.ifSentryEnabled
import java.io.Closeable
import java.sql.Connection
import java.sql.ResultSet
import java.time.LocalDateTime
import java.util.concurrent.atomic.AtomicBoolean
import java.util.concurrent.atomic.AtomicLong

class ClickHouseConnection(private val config: ClickHouseConfig): Closeable {
    private val _serverVersion: SetOnceGetValue<String> = SetOnceGetValue()
    private val _dataSource: SetOnceGetValue<HikariDataSource> = SetOnceGetValue()
    private val _closed = AtomicBoolean(false)
    private val _calls = AtomicLong(0)
    private val log by logging<ClickHouseConnection>()

    /**
     * Returns the server version this connection is running off from.
     */
    val serverVersion: String
        get() = _serverVersion.value

    /**
     * Returns if the connection is closed or not.
     */
    val closed: Boolean
        get() = _closed.get()

    /**
     * Returns how many database calls that have been used.
     */
    val calls: Long
        get() = _calls.get()

    /**
     * Grabs the connection, and calls the underlying [block] that is used within this [Connection].
     * @param block The block of code to execute while this connection is being in use.
     * @return The result, cast as [T]
     * @throws java.sql.SQLException
     */
    fun <T> grabConnection(block: Connection.(ITransaction?) -> T): T {
        if (!_dataSource.wasSet()) {
            throw IllegalStateException("#connect() was never called, cannot grab connection.")
        }

        val connection = _dataSource.value.connection
        val sentryTransaction = ifSentryEnabled {
            Sentry.startTransaction("charted.database.GrabConnection", "Retrieves the connection to execute a block of SQL.")
        }

        return try {
            connection.block(sentryTransaction).let {
                _calls.incrementAndGet()
                sentryTransaction?.finish(SpanStatus.OK)
                it
            }
        } catch (e: Throwable) {
            sentryTransaction?.throwable = e
            sentryTransaction?.finish(SpanStatus.UNKNOWN_ERROR)

            if (Sentry.isEnabled()) Sentry.captureException(e)
            throw e
        }
    }

    /**
     * Executes a block of SQL, transforms the [ResultSet] into the object cast as [T]
     * from a closure that represents a [TransformResultSetInto] transformer.
     *
     * @param sql The SQL string to execute
     * @param args The list of arguments to use if the SQL string is parameterized.
     * @param transform The transform closure to transform the [ResultSet] into [T].
     */
    fun <T> sql(
        @Language("sql") sql: String,
        vararg args: Any?,
        transform: (ResultSet) -> T
    ): T? = sql(sql, *args)?.let(transform)

    /**
     * Executes a block of SQL, transforms the [ResultSet] into the object cast as [T]
     * from a [TransformResultSetInto] transformer.
     *
     * @param sql The SQL string to execute.
     * @param transform The transform object to transform the underlying result set.
     * @param args The list of arguments to use if the SQL string is parameterized.
     */
    fun <T> sql(
        @Language("sql") sql: String,
        transform: TransformResultSetInto<T>,
        vararg args: Any?
    ): T? = sql(sql, *args)?.let(transform::transform)

    /**
     * Executes a block of SQL and returns the [ResultSet] that was collected from
     * the statement.
     *
     * @param sql The SQL string to execute.
     * @param args The list of arguments to use if the SQL string is parameterized.
     */
    fun sql(
        @Language("sql") sql: String,
        vararg args: Any?
    ): ResultSet? = grabConnection { transaction ->
        log.debug("Grabbed connection, now executing SQL query [$sql]")
        val span = transaction?.startChild("Execute SQL [$sql with ${args.size} arguments]")

        if (args.isEmpty()) {
            val stmt = createStatement()
            stmt.execute(sql)

            span?.finish(SpanStatus.OK)
            if (!stmt.resultSet.next()) {
                throw IllegalStateException("There is no items available.")
            }

            return@grabConnection stmt.resultSet
        }

        val stmt = prepareStatement(sql)
        for ((i, arg) in args.withIndex()) {
            when (arg) {
                is String -> stmt.setString(i + 1, arg)
                is Int -> stmt.setInt(i + 1, arg)
                is Long -> stmt.setLong(i + 1, arg)
                is LocalDateTime -> stmt.setObject(i + 1, arg)
                else -> error(if (arg == null) "Nullable properties aren't supported at this time." else "Cannot use type ${arg::class} as a parameter due to it not being supported at this time.")
            }
        }

        stmt.execute()
        if (!stmt.resultSet.next()) {
            return@grabConnection null
        }

        stmt.resultSet
    }

    fun connect() {
        if (_dataSource.wasSet()) {
            log.warn("The connection was already previously established, doing nothing!")
            return
        }

        if (closed) {
            log.warn("The connection was closed due to some reason, skipping.")
            return
        }

        val jdbcUrl = "jdbc:clickhouse://${config.host}:${config.port}/${config.database}"
        log.debug("Connecting to JDBC URL [$jdbcUrl]")

        _dataSource.value = HikariDataSource(
            HikariConfig().apply {
                this.jdbcUrl = jdbcUrl
                driverClassName = "com.clickhouse.jdbc.ClickHouseDriver"
                username = config.username
                password = config.password

                addDataSourceProperty(JdbcConfig.PROP_CREATE_DATABASE, "${config.createDbIfNotExists}")
                addDataSourceProperty(JdbcConfig.PROP_WRAPPER_OBJ, "true")
            }
        )

        log.info("Created the Hikari data source, checking if we can connect...")
        try {
            sql("SELECT 1;")
            log.info("Connection was a success! Retrieving server information...")
        } catch (e: Exception) {
            log.error("Unable to connect to ClickHouse:", e)
            throw e
        }

        val version = sql("SELECT version() AS version;") {
            it.getString("version")
        } ?: throw IllegalStateException("Connection was successful, but can't retrieve server version?")

        _serverVersion.value = version
        log.warn("Using ClickHouse v$version")
    }

    override fun close() {
        if (closed) return

        log.warn("Closing connection...")
        _dataSource.value.close()
        _closed.set(true)

        log.info("Closed! ^-^")
    }
}
