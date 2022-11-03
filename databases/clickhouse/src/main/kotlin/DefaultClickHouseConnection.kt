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

package org.noelware.charted.databases.clickhouse

import co.elastic.apm.api.CaptureSpan
import co.elastic.apm.api.CaptureTransaction
import com.zaxxer.hikari.HikariDataSource
import dev.floofy.utils.slf4j.logging
import io.sentry.Sentry
import io.sentry.SpanStatus
import kotlinx.datetime.LocalDateTime
import kotlinx.datetime.toJavaLocalDateTime
import org.noelware.charted.common.SetOnce
import org.noelware.charted.extensions.ifSentryEnabled
import java.sql.Connection
import java.sql.ResultSet
import java.util.concurrent.atomic.AtomicBoolean
import java.util.concurrent.atomic.AtomicLong

class DefaultClickHouseConnection: ClickHouseConnection {
    private val _serverVersion: SetOnce<String> = SetOnce()
    private val _dataSource: SetOnce<HikariDataSource> = SetOnce()
    private val _closed: AtomicBoolean = AtomicBoolean()
    private val _calls: AtomicLong = AtomicLong(0)
    private val log by logging<DefaultClickHouseConnection>()

    override val closed: Boolean
        get() = _closed.get()

    override val serverVersion: String
        get() = _serverVersion.value

    override val calls: Long
        get() = _calls.get()

    @CaptureTransaction("DefaultClickHouseConnection#grabConnection", type = "database")
    private fun <T> connection(block: Connection.() -> T): T {
        if (!_dataSource.wasSet()) throw IllegalAccessException("Can't grab connection due to no connection being set!")

        val connection = _dataSource.value.connection
        val transaction = ifSentryEnabled { Sentry.startTransaction("ClickHouse#grabConnection", "SQL") }

        return try {
            connection.block().let {
                _calls.incrementAndGet()
                transaction?.finish()

                it
            }
        } catch (e: Exception) {
            transaction?.finish(SpanStatus.INTERNAL_ERROR)
            log.error("Received SQL exception when running:", e)
            ifSentryEnabled { Sentry.captureException(e) }

            throw e
        }
    }

    override fun <U: Any> sql(sql: String, from: FromResultSet<U>, vararg args: Any?): U? =
        sql(sql, *args)?.let(from::fromResultSet)

    @CaptureSpan("DefaultClickHouseConnection#sql", type = "database", action = "sql")
    override fun sql(sql: String, vararg args: Any?): ResultSet? = connection {
        log.debug("Received connection! Now executing SQL code...")
        if (args.isEmpty()) {
            val stmt = createStatement()
            return@connection stmt.executeQuery(sql)
                ?: return@connection null
        }

        val stmt = prepareStatement(sql)
        for ((i, arg) in args.withIndex()) {
            when (arg) {
                is String -> stmt.setString(i + 1, arg)
                is Int -> stmt.setInt(i + 1, arg)
                is Long -> stmt.setLong(i + 1, arg)
                is LocalDateTime -> stmt.setObject(i + 1, arg.toJavaLocalDateTime())
                else -> {
                    log.warn("Discarding argument in index #$i (${if (arg == null) "(null)" else "(${arg::class})"})")
                }
            }
        }

        stmt.executeQuery(sql) ?: return@connection null
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

        val jdbcUrl = "jdbc:clickhouse://"
    }

    /*
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
     */

    override fun close() {
    }
}
