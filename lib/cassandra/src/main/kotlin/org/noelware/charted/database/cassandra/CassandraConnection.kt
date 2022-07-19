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

import com.datastax.driver.core.Cluster
import com.datastax.driver.core.ResultSet
import com.datastax.driver.core.Session
import dev.floofy.utils.slf4j.logging
import okhttp3.internal.closeQuietly
import org.noelware.charted.common.SetOnceGetValue
import org.noelware.charted.common.data.CassandraConfig
import java.util.concurrent.atomic.AtomicBoolean
import java.util.concurrent.atomic.AtomicLong

class CassandraConnection(private val config: CassandraConfig): AutoCloseable {
    private val _serverVersion: SetOnceGetValue<String> = SetOnceGetValue()
    private val _session: SetOnceGetValue<Session> = SetOnceGetValue()
    private val _cluster: SetOnceGetValue<Cluster> = SetOnceGetValue()
    private val _closed = AtomicBoolean(false)
    private val _calls = AtomicLong(0L)
    private val log by logging<CassandraConnection>()

    val serverVersion: String
        get() = _serverVersion.value

    val closed: Boolean
        get() = _closed.get()

    val cluster: Cluster
        get() = _cluster.value

    val session: Session
        get() = _session.value

    val calls: Long
        get() = _calls.get()

    fun sql(sql: String): ResultSet = sql(sql, *arrayOf())
    fun sql(sql: String, vararg args: Any?): ResultSet = try {
        val rs = session.execute(sql, *args)
        _calls.incrementAndGet()

        rs
    } catch (e: Exception) {
        log.error("Unable to execute SQL [$sql]", e)
        throw e
    }

    fun connect() {
        log.info("Connecting to Cassandra with nodes [${config.nodes.joinToString(", ")}] on port ${config.port}!")
        val builder = Cluster.builder().apply {
            for (node in config.nodes) {
                addContactPoint(node)
            }

            if (config.username != null && config.password != null) {
                withCredentials(config.username, config.password)
            }

            withPort(config.port)
        }

        _cluster.value = builder.build()
        log.info("Created cluster connection, now creating session!")

        try {
            val sess = if (config.keyspace.isEmpty()) cluster.connect() else cluster.connect(config.keyspace)
            log.info("Established session with Cassandra! Collecting server version...")

            _session.value = sess

            val data = sql("SELECT release_version FROM system.local;").all()
            if (data.isEmpty()) {
                throw IllegalStateException("Can't retrieve server version due to no data.")
            }

            _serverVersion.value = data.first().getString("release_version")
            log.info("Using Cassandra v$serverVersion!")
        } catch (e: Exception) {
            log.error("Unable to connect to Cassandra:", e)
            throw e
        }
    }

    override fun close() {
        if (closed) return

        session.closeQuietly()
        cluster.closeQuietly()
        _closed.set(true)
    }
}
