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
 *  Unless required by applicable law or agreed to in writing, software
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
import io.sentry.Sentry
import org.intellij.lang.annotations.Language
import org.noelware.charted.common.config.ClickHouseConfig
import java.sql.Connection
import java.sql.ResultSet
import java.sql.SQLException
import java.time.LocalDateTime

/**
 * Represents the abstraction of using ClickHouse as the connection. Since Exposed
 * doesn't support ClickHouse, we have to do our own migrations and such.
 */
class ClickHouseConnection(private val config: ClickHouseConfig): AutoCloseable {
    private lateinit var dataSource: HikariDataSource
    private val log by logging<ClickHouseConnection>()

    private var _serverVersion: String? = null
    private var _closed: Boolean = false
    private var _calls: Long = 0

    val version: String
        get() = _serverVersion ?: error("Connection is not available.")

    /**
     * Returns if the connection is closed or not.
     */
    val closed: Boolean
        get() = _closed

    /**
     * Returns how many calls the database has used up. This is useful for Prometheus
     * metrics.
     */
    val calls: Long
        get() = _calls

    /**
     * Grabs the connection and calls the underlying [block].
     * @param block The block of code to execute while the connection is being in use,
     * then the connection closes.
     * @return The result as [T].
     * @throws java.sql.SQLException If the connection can't be grabbed.
     * @throws Throwable Unknown exception that happened (that isn't a [SQLException][java.sql.SQLException])
     */
    private fun <T> grabConnection(block: Connection.() -> T): T {
        val connection = dataSource.connection
        _calls++

        try {
            return connection.block()
        } catch (e: Throwable) {
            if (Sentry.isEnabled()) Sentry.captureException(e)
            throw e
        }
    }

    /**
     * Executes a block of SQL and returns the [ResultSet] that was thrown at us.
     * @param sql The SQL to execute.
     * @param args The arguments to use.
     * @return The [ResultSet] that was returned from the connection.
     */
    fun sql(
        @Language("sql") sql: String,
        vararg args: Any
    ): ResultSet = grabConnection {
        log.debug("Grabbed connection, now executing SQL query: $sql")

        if (args.isEmpty()) {
            val stmt = createStatement()
            stmt.execute(sql)

            return@grabConnection stmt.resultSet
        }

        val stmt = prepareStatement(sql)
        for ((i, arg) in args.withIndex()) {
            val index = if (i == 0) 1 else i

            when (arg) {
                is String -> stmt.setString(index, arg)
                is Int -> stmt.setInt(index, arg)
                is Long -> stmt.setLong(index, arg)
                is LocalDateTime -> stmt.setObject(index, arg)
                else -> throw IllegalStateException("Cannot use type ${arg::class} due to it not being supported!")
            }
        }

        stmt.execute()
        stmt.resultSet
    }

    /**
     * Sends the SQL statement in a batched statement and returns the underlying
     * [IntArray] that was returned from [PreparedStatement.executeBatch][java.sql.PreparedStatement.executeBatch].
     *
     * @param sql The SQL statement that should be batched in.
     * @param args A list of arguments to parameterize.
     * @throws java.sql.SQLException If any SQL exception have occurred.
     * @throws IllegalStateException If the argument type was not a valid one to set.
     */
    fun sqlInBatch(
        @Language("sql") sql: String,
        vararg args: Any
    ): IntArray = grabConnection {
        val stmt = prepareStatement(sql)
        for ((i, arg) in args.withIndex()) {
            val index = if (i == 0) 1 else i

            when (arg) {
                is String -> stmt.setString(index, arg)
                is Int -> stmt.setInt(index, arg)
                is Long -> stmt.setLong(index, arg)
                is LocalDateTime -> stmt.setObject(index, arg)
                else -> throw IllegalStateException("Cannot use type ${arg::class} due to it not being supported!")
            }
        }

        stmt.addBatch()
        stmt.executeBatch()
    }

    /**
     * Connects to the ClickHouse server that we need to connect to.
     */
    fun connect() {
        if (this::dataSource.isInitialized && !closed) {
            log.warn("Connection was already established")
            return
        }

        val jdbcUri = "jdbc:clickhouse://${config.host}:${config.port}/${config.database}"

        log.info("Connecting to JDBC URI: $jdbcUri...")
        dataSource = HikariDataSource(
            HikariConfig().apply {
                jdbcUrl = jdbcUri
                driverClassName = "com.clickhouse.jdbc.ClickHouseDriver"
                username = config.username
                password = config.password

                addDataSourceProperty(JdbcConfig.PROP_WRAPPER_OBJ, "true")
                addDataSourceProperty(JdbcConfig.PROP_CREATE_DATABASE, "${config.createDbIfNotExists}")
            }
        )

        log.info("Created the data source! Now checking if we can connect...")
        try {
            sql("SELECT 1;")

            log.info("Connection was successful, retrieving server info...")
        } catch (e: SQLException) {
            log.error("Unable to connect to ClickHouse:", e)
            throw e
        }

        val version = sql("SELECT version() AS version;")
        if (!version.next())
            throw IllegalStateException("Connection is successful but can't retrieve server version?")

        _serverVersion = version.getString("version")
        log.info("We are using ClickHouse v${this.version}!")
    }

    override fun close() {
        if (closed) return

        log.warn("Closing connection...")
        dataSource.close()
        _closed = true

        log.info("Closed!~")
    }
}
