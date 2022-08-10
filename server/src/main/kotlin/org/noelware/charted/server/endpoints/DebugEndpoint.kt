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

import dev.floofy.utils.exposed.asyncTransaction
import dev.floofy.utils.kotlin.humanize
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.debug.DebugProbes
import kotlinx.coroutines.debug.State
import kotlinx.serialization.json.*
import org.noelware.charted.common.ChartedInfo
import org.noelware.charted.common.ChartedScope
import org.noelware.charted.common.IRedisClient
import org.noelware.charted.common.data.Config
import org.noelware.charted.common.data.Feature
import org.noelware.charted.common.extensions.formatToSize
import org.noelware.charted.core.StorageWrapper
import org.noelware.charted.database.cassandra.CassandraConnection
import org.noelware.charted.search.elasticsearch.ElasticsearchClient
import org.noelware.charted.search.meilisearch.MeilisearchClient
import org.noelware.charted.server.ChartedServer
import org.noelware.ktor.endpoints.AbstractEndpoint
import org.noelware.ktor.endpoints.Get
import org.noelware.remi.filesystem.FilesystemStorageTrailer
import java.lang.management.ManagementFactory
import kotlin.time.Duration.Companion.nanoseconds

// TODO: move this into an "admin" endpoint
// TODO: add seperate sections like /admin/stats?sections=threads,jvm
class DebugEndpoint(
    private val elastic: ElasticsearchClient? = null,
    private val meili: MeilisearchClient? = null,
    private val cassandra: CassandraConnection? = null,
    private val storage: StorageWrapper,
    private val config: Config,
    private val redis: IRedisClient
): AbstractEndpoint("/debug") {
    private val runtime = ManagementFactory.getRuntimeMXBean()
    private val threads = ManagementFactory.getThreadMXBean()
    private val os = ManagementFactory.getOperatingSystemMXBean()

    @Get
    suspend fun main(call: ApplicationCall) {
        val threadInfos = threads.dumpAllThreads(true, true)
        val threadInfo = buildJsonObject {
            put("count", threads.threadCount)
            put("background", threads.daemonThreadCount)

            putJsonArray("threads") {
                for (info in threadInfos) {
                    addJsonObject {
                        put("state", info.threadState.name)
                        put("name", info.threadName)
                        put("id", info.threadId)

                        if (threads.isThreadCpuTimeEnabled) {
                            put("cpu_time_ms", threads.getThreadCpuTime(info.threadId).nanoseconds.inWholeMilliseconds)
                        }

                        put("user_time_ms", threads.getThreadUserTime(info.threadId).nanoseconds.inWholeMilliseconds)
                        putJsonArray("stacktrace") {
                            for (element in info.stackTrace) {
                                addJsonObject {
                                    put("class_loader_name", element.classLoaderName)
                                    put("module_name", element.moduleName)
                                    put("module_version", element.moduleVersion)
                                    put("declaring_class", element.className)
                                    put("method_name", element.methodName)
                                    put("file_name", element.fileName)
                                    put("line_num", element.lineNumber)
                                    put("is_native_method", element.isNativeMethod)
                                }
                            }
                        }
                    }
                }
            }
        }

        val osInfo = buildJsonObject {
            put("system_load_avg", os.systemLoadAverage)
            put("processors", os.availableProcessors)
            put("version", os.version)
            put("arch", os.arch)
            put("name", os.name)
        }

        var searchBackendInfo: JsonElement? = null
        if (elastic != null) {
            val data = elastic.collect()
            searchBackendInfo = buildJsonObject {
                put("server_version", elastic.serverVersion)
                put("backend", "Elasticsearch")
                putJsonObject("cluster") {
                    put("name", elastic.clusterName)
                    put("uuid", elastic.clusterUUID)
                    put("documents", data.documents)
                    put("deleted_documents", data.deleted)
                    put("size_in_bytes", data.sizeInBytes)
                }

                for ((key, stat) in data.indexes) {
                    putJsonObject(key.replace('-', '_')) {
                        put("documents", stat.documents)
                        put("deleted_documents", stat.deleted)
                        put("size_in_bytes", stat.sizeInBytes)
                        put("health", stat.health)
                    }
                }
            }
        }

        if (meili != null) {
            searchBackendInfo = buildJsonObject {
                put("server_version", meili.serverVersion)
                put("backend", "Meilisearch")
                putJsonObject("users") {}
                putJsonObject("repos") {}
                putJsonObject("orgs") {}
            }
        }

        val chartSize: Long? = if (config.isFeatureEnabled(Feature.DOCKER_REGISTRY)) {
            null
        } else {
            storage.trailer.list("./tarballs").fold(0L) { acc, o -> acc + o.size }
        }

        val storageInfo = buildJsonObject {
            put("name", storage.trailer.name)
            if (chartSize != null) {
                put("chart_size", chartSize.formatToSize())
            }

            if (storage.trailer is FilesystemStorageTrailer) {
                val trailer = storage.trailer as FilesystemStorageTrailer
                val stats = trailer.stats()
                putJsonObject("fs_stats") {
                    put("unallocated_space_bytes", stats.unallocatedSpace)
                    put("unallocated_space", stats.unallocatedSpace.formatToSize())
                    put("usable_space_bytes", stats.usableSpace)
                    put("usable_space", stats.usableSpace.formatToSize())
                    put("total_space_bytes", stats.totalSpace)
                    put("total_space", stats.totalSpace.formatToSize())
                    put("directory", trailer.directory)
                    put("drive", stats.drive)
                    put("type", stats.type)
                }
            }
        }

        val rn = Runtime.getRuntime()
        val runtimeInfo = buildJsonObject {
            put("total_memory_bytes", rn.totalMemory())
            put("total_memory", rn.totalMemory().formatToSize())
            put("max_memory_bytes", rn.maxMemory())
            put("max_memory", rn.maxMemory().formatToSize())
            put("free_memory_bytes", rn.freeMemory())
            put("free_memory", rn.freeMemory().formatToSize())
            put("start_time_ms", System.currentTimeMillis() - runtime.startTime)
            put("start_time", (System.currentTimeMillis() - runtime.startTime).humanize())
            put("version", "${Runtime.version()}")
            put("vendor", runtime.vmVendor)
            put("name", runtime.vmName)
            put("date", System.getProperty("java.version.date"))
            put("pid", runtime.pid)
        }

        val cassandraStats = if (cassandra != null) {
            val rs = cassandra.sql("SELECT cluster_name, data_center FROM system.local;").all().first()
            val values = cassandra.cluster.metrics.requestsTimer.snapshot.values
            val latency = values.fold(0L) { acc, curr -> acc + curr } / values.size

            buildJsonObject {
                put("db_calls", cassandra.calls)
                put("version", cassandra.serverVersion)
                put("data_center", rs.getString("data_center"))
                put("cluster_name", rs.getString("cluster_name"))
                put("trashed_connections", cassandra.cluster.metrics.trashedConnections.value)
                put("request_latency_ms", latency.nanoseconds.inWholeMilliseconds)
                put("bytes_received", cassandra.cluster.metrics.bytesReceived.count)
                put("bytes_sent", cassandra.cluster.metrics.bytesSent.count)
            }
        } else {
            JsonNull
        }

        val postgres = buildJsonObject {
            asyncTransaction(ChartedScope) {
                exec(
                    // THIS IS NOT SAFE PLEASE UPDATE THIS INTO A NON SQL INJECTED VALUE
                    "SELECT datname, numbackends, tup_fetched, tup_inserted, tup_deleted FROM pg_stat_database WHERE datname = '${config.postgres.name}';"
                ) {
                    if (!it.next()) return@exec

                    val numBackends = it.getInt("numbackends")
                    val fetched = it.getLong("tup_fetched")
                    val inserted = it.getLong("tup_inserted")
                    val deleted = it.getLong("tup_deleted")

                    put("num_backends", numBackends)
                    put("fetched", fetched)
                    put("inserted", inserted)
                    put("deleted", deleted)
                }
            }
        }

        val redisStats = redis.stats
        val redisInfo = buildJsonObject {
            put("total_network_input", redisStats.totalNetworkInput)
            put("total_network_output", redisStats.totalNetworkOutput)
            put("total_commands_processed", redisStats.totalCommandsProcessed)
            put("total_connections_received", redisStats.totalConnectionsReceived)
            put("allocator", redisStats.allocator)
            put("uptime_millis", redisStats.uptime)
            put("uptime", redisStats.uptime.humanize())
            put("version", redisStats.version)
            put("mode", redisStats.mode)
            put("ping", redisStats.ping)
        }

        call.respond(
            HttpStatusCode.OK,
            buildJsonObject {
                put("success", true)
                putJsonObject("data") {
                    put("distribution", ChartedInfo.distribution.key)
                    put("commit_sha", ChartedInfo.commitHash)
                    put("build_date", ChartedInfo.buildDate)
                    put("uptime_millis", System.currentTimeMillis() - ChartedServer.bootTime)
                    put("uptime", (System.currentTimeMillis() - ChartedServer.bootTime).humanize())
                    put("product", "charted-server")
                    put("version", ChartedInfo.version)
                    put("vendor", "Noelware")

                    put("threads", threadInfo)
                    put("storage", storageInfo)
                    put("cassandra", cassandraStats)
                    put("runtime", runtimeInfo)
                    put("database", postgres)
                    put("redis", redisInfo)
                    put("search", searchBackendInfo ?: JsonNull)
                    put("os", osInfo)
                }
            }
        )
    }

    @OptIn(ExperimentalCoroutinesApi::class)
    @Get("/coroutines")
    suspend fun coroutines(call: ApplicationCall) {
        // TODO: maybe enable probes on prod without stacktraces
        if (!DebugProbes.isInstalled) return call.respond(HttpStatusCode.NotFound)

        val info = DebugProbes.dumpCoroutinesInfo()
        val data = buildJsonArray {
            for (coroutine in info) {
                addJsonObject {
                    put(
                        "state",
                        when (coroutine.state) {
                            State.CREATED -> "created"
                            State.RUNNING -> "running"
                            State.SUSPENDED -> "suspended"
                        }
                    )

                    put("context", coroutine.context.toString())
                    if (coroutine.job != null) {
                        putJsonObject("job") {
                            put("active", coroutine.job!!.isActive)
                            put("completed", coroutine.job!!.isCompleted)
                            put("cancelled", coroutine.job!!.isCancelled)
                        }
                    }

                    val stacktrace = coroutine.creationStackTrace
                    putJsonArray("stacktrace") {
                        for (element in stacktrace) {
                            addJsonObject {
                                put("class_loader_name", element.classLoaderName)
                                put("module_name", element.moduleName)
                                put("module_version", element.moduleVersion)
                                put("declaring_class", element.className)
                                put("method_name", element.methodName)
                                put("file_name", element.fileName)
                                put("line_num", element.lineNumber)
                                put("is_native_method", element.isNativeMethod)
                            }
                        }
                    }
                }
            }
        }

        call.respond(
            HttpStatusCode.OK,
            buildJsonObject {
                put("success", true)
                put("data", data)
            }
        )
    }
}
