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

package org.noelware.charted.server.endpoints

import dev.floofy.utils.kotlin.humanize
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import kotlinx.serialization.SerialName
import org.noelware.charted.common.ChartedInfo
import org.noelware.charted.common.data.responses.Response
import org.noelware.charted.common.stats.RedisStats
import org.noelware.charted.database.PostgresStats
import org.noelware.charted.database.cassandra.CassandraStats
import org.noelware.charted.search.elasticsearch.ElasticsearchStats
import org.noelware.charted.server.ChartedServer
import org.noelware.charted.stats.StatisticsCollector
import org.noelware.charted.stats.collectors.*
import org.noelware.ktor.endpoints.AbstractEndpoint
import org.noelware.ktor.endpoints.Get
import org.noelware.ktor.endpoints.Post

@kotlinx.serialization.Serializable
private data class AdminResponse(
    val message: String,

    @SerialName("docs_uri")
    val docs: String
)

/**
 * Represents the response type for `GET /admin/stats`.
 *
 * @param elasticsearch The statistics from the Elasticsearch cluster, if the connection was established.
 * @param memoryPools   The statistics for the JVM memory pools.
 * @param coroutines    The statistics for the coroutines that were spawned.
 * @param cassandra     The statistics from the Cassandra cluster, if the connection was established.
 * @param postgres      The statistics from the Postgres database.
 * @param threads       The statistics from the threads that the JVM created.
 * @param redis         The statistics from the Redis cluster.
 * @param jvm           The statistics from the JVM.
 * @param os            The statistics from the operating system.
 */
@kotlinx.serialization.Serializable
private data class AdminStatsResponse(
    val elasticsearch: ElasticsearchStats?,

    @SerialName("memory_pools")
    val memoryPools: List<MemoryPoolStat>,
    val coroutines: List<CoroutineStats>,
    val cassandra: CassandraStats?,
    val postgres: PostgresStats,
    val threads: ThreadStats,
    val redis: RedisStats,
    val jvm: JvmStats,
    val os: OsStats,

    val uptimeMillis: Long,
    val distribution: String,
    val commitHash: String,
    val buildDate: String,
    val uptime: String,
    val version: String,
    val product: String,
    val vendor: String
)

class AdminEndpoint(private val statistics: StatisticsCollector): AbstractEndpoint("/admin") {
//    init {
//        install(Sessions)
//        install(IsAdminGuard)
//    }

    @Get
    suspend fun main(call: ApplicationCall) {
        call.respond(
            HttpStatusCode.OK,
            Response.ok(
                AdminResponse(
                    message = "Welcome to the Admin API!",
                    docs = "https://charts.noelware.org/docs/server/api/admin"
                )
            )
        )
    }

    @Suppress("UNCHECKED_CAST")
    @Get("/stats")
    suspend fun stats(call: ApplicationCall) {
        val elasticsearch = statistics.collect<ElasticsearchStats>("elasticsearch")
        val memoryPools = statistics.collect<List<MemoryPoolStat>>("memory_pools")!!
        val coroutines = statistics.collect<Any>("coroutines")!!
        val cassandra = statistics.collect<CassandraStats>("cassandra")
        val postgres = statistics.collect<PostgresStats>("postgres")!!
        val threads = statistics.collect<ThreadStats>("threads")!!
        val redis = statistics.collect<RedisStats>("redis")!!
        val jvm = statistics.collect<JvmStats>("jvm")!!
        val os = statistics.collect<OsStats>("os")!!

        call.respond(
            HttpStatusCode.OK,
            Response.ok(
                AdminStatsResponse(
                    elasticsearch,
                    memoryPools,
                    (coroutines as List<CoroutineStats>),
                    cassandra,
                    postgres,
                    threads,
                    redis,
                    jvm,
                    os,

                    System.currentTimeMillis() - ChartedServer.bootTime,
                    ChartedInfo.distribution.key,
                    ChartedInfo.commitHash,
                    ChartedInfo.buildDate,
                    (System.currentTimeMillis() - ChartedServer.bootTime).humanize(),
                    ChartedInfo.version,
                    "charted-server",
                    "Noelware"
                )
            )
        )
    }

    @Post("/gc")
    suspend fun runGarbageCollector(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented, Response.err("NOT_SUPPORTED", "REST handler POST /admin/gc is not supported."))
    }
}
