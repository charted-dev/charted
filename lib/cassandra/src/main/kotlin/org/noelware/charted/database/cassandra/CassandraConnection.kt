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

package org.noelware.charted.database.cassandra

import com.datastax.oss.driver.api.core.CqlSession
import com.datastax.oss.driver.api.core.cql.AsyncResultSet
import com.datastax.oss.driver.api.core.cql.SimpleStatement
import dev.floofy.utils.slf4j.logging
import io.sentry.Sentry
import kotlinx.coroutines.future.await
import org.intellij.lang.annotations.Language
import org.noelware.charted.common.SetOnceGetValue
import org.noelware.charted.configuration.dsl.CassandraConfig
import java.io.Closeable
import java.net.InetSocketAddress
import java.util.concurrent.atomic.AtomicBoolean
import java.util.concurrent.atomic.AtomicLong

/**
 * Represents a wrapper for the connection to Apache Cassandra.
 */
class CassandraConnection(private val config: CassandraConfig): Closeable {
    private val _serverVersion: SetOnceGetValue<String> = SetOnceGetValue()
    private val _connected: AtomicBoolean = AtomicBoolean(false)
    private val _session: SetOnceGetValue<CqlSession> = SetOnceGetValue()
    private val _closed = AtomicBoolean(false)
    private val _calls = AtomicLong(0L)
    private val log by logging<CassandraConnection>()

    /**
     * Returns the server version to Cassandra.
     */
    val serverVersion: String
        get() = _serverVersion.value

    /**
     * Whether the connection is connected or not.
     */
    val connected: Boolean
        get() = _connected.get()

    /**
     * Whether the connection is closed or not.
     */
    val closed: Boolean
        get() = _closed.get()

    /**
     * The [CqlSession] that is connected to Cassandra.
     */
    val session: CqlSession
        get() = _session.value

    /**
     * How many database calls occurred. Does not reflect on using [#execute][CqlSession.execute],
     * [#executeAsync][CqlSession.executeAsync], or [#executeReactive][CqlSession.executeReactive], only
     * on [#sql][sql]
     */
    val calls: Long
        get() = _calls.get()

    /**
     * Closes the underlying [session] that was used in [#connect][connect].
     */
    override fun close() {
        if (_closed.compareAndSet(false, true)) {
            log.warn("Closing connection from Cassandra...")
            session.close()
        }
    }

    /**
     * Executes a line of SQL and returns the [AsyncResultSet] of the underlying call.
     * @param sql The line of SQL to execute.
     */
    suspend fun sql(@Language("sql") sql: String): AsyncResultSet = sql(sql, *arrayOf())

    /**
     * Executes a line of SQL and returns the [AsyncResultSet] of the underlying call.
     * @param sql The line of SQL to execute.
     * @param args variadic arguments representing as parameters in the [sql] code.
     */
    suspend fun sql(@Language("sql") sql: String, vararg args: Any?): AsyncResultSet = sql(SimpleStatement.newInstance(sql, *args))

    /**
     * Execute the [statement][SimpleStatement] to the current session and returns
     * the underlying [AsyncResultSet] object.
     */
    suspend fun sql(stmt: SimpleStatement): AsyncResultSet = try {
        val rs = session.executeAsync(stmt).await()
        _calls.incrementAndGet()

        rs
    } catch (e: Exception) {
        Sentry.captureException(e)
        throw e
    }

    /**
     * Connects to Cassandra. This method can be called multiple times but the first
     * instance of the [#connect][connect] call establishes the connection, any other calls
     * will be dropped and the connection won't be established.
     */
    suspend fun connect() {
        if (config.keyspace.isEmpty()) {
            log.warn("Configuration key 'cassandra.keyspace' is empty, please set this to a keyspace and not globally.")
        }

        if (_connected.get()) {
            log.warn("Connection has been established already.")
        }

        if (_connected.compareAndSet(false, true)) {
            try {
                log.info("Connecting to Cassandra with nodes [${config.nodes.joinToString(", ")}]")
                val builder = CqlSession.builder().apply {
                    withKeyspace(config.keyspace)
                    addContactPoints(
                        config.nodes.map {
                            val mapping = it.split(":", limit = 2)
                            if (mapping.size != 2) {
                                throw IllegalArgumentException("Node mapping must be host:port")
                            }

                            InetSocketAddress(mapping.first(), mapping.last().toInt())
                        }
                    )

                    if (config.username != null) {
                        if (config.password == null) {
                            throw IllegalArgumentException("Missing required `password` property.")
                        }

                        withAuthCredentials(config.username!!, config.password!!)
                    }
                }

                _session.value = builder.build()
                log.info("Established connection with Cassandra! Collecting server version...")

                val rs = sql("SELECT release_version FROM system.local;").one()
                    ?: throw IllegalStateException("system.local didn't return release_version! Are you using the right Cassandra version?")

                _serverVersion.value = rs.getString("release_version")!!
                log.info("Using v$serverVersion of Cassandra.")
            } catch (e: Exception) {
                Sentry.captureException(e)
                throw e
            }
        }
    }
}
