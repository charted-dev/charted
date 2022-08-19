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

import kotlinx.serialization.SerialName
import org.noelware.charted.stats.StatCollector

@kotlinx.serialization.Serializable
data class CassandraStats(
    @SerialName("trashed_connections")
    val trashedConnections: Long,

    @SerialName("opened_connections")
    val openedConnections: Long,

    @SerialName("bytes_received")
    val bytesReceived: Long,
    val datacenter: String,

    @SerialName("bytes_sent")
    val bytesSent: Long,

    @SerialName("request_latency_ms")
    val latency: Long,
    val cluster: String,
    val version: String,
    val calls: Long
)

class CassandraStatCollector(private val cassandra: CassandraConnection): StatCollector<CassandraStats> {
    override suspend fun collect(): CassandraStats {
        val rs = cassandra.sql("SELECT data_center FROM system.local;").all().first()
        val cluster = cassandra.cluster
        val metrics = cassandra.cluster.metrics

        return CassandraStats(
            metrics.trashedConnections.value.toLong(),
            metrics.openConnections.value.toLong(),
            metrics.bytesReceived.count,
            rs.getString("data_center"),
            metrics.bytesSent.count,
            metrics.requestsTimer.snapshot.values.fold(0L) { acc, curr -> acc + curr } /
                metrics.requestsTimer.snapshot.values.size,

            cluster.clusterName,
            cassandra.serverVersion,
            cassandra.calls
        )
    }
}
