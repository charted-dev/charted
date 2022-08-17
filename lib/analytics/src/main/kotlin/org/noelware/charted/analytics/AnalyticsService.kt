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

package org.noelware.charted.analytics

import com.google.protobuf.*
import kotlinx.datetime.Clock
import kotlinx.datetime.Instant
import org.noelware.charted.analytics.protobufs.v1.*
import org.noelware.charted.common.ChartedInfo
import org.noelware.charted.stats.StatisticsCollector
import org.noelware.charted.stats.collectors.*

class AnalyticsService(
    private val server: AnalyticsServer,
    private val statistics: StatisticsCollector
): AnalyticsGrpcKt.AnalyticsCoroutineImplBase() {
    override suspend fun connectionAck(request: ConnectionAckRequest): ConnectionAckResponse = ConnectionAckResponse.newBuilder()
        .setConnected(true)
        .setInstanceUUID(fetchInstanceUUID().toString())
        .build()

    override suspend fun retrieveStats(request: ReceiveStatsRequest): ReceiveStatsResponse {
        val statData = struct {
            val memoryPools = statistics.collect<List<MemoryPoolStat>>("memory_pools")!!
            val coroutines = statistics.collect<Any>("coroutines")!!
            val threads = statistics.collect<ThreadStats>("threads")!!
            val jvm = statistics.collect<JvmStats>("jvm")!!

            put("grpc_calls", server.logInterceptor.calls)
            put(
                "memory_pools",
                memoryPools.map { stat ->
                    struct {
                        put("name", stat.name)
                        put("type", stat.type)
                        put(
                            "peak_usage",
                            struct {
                                put(
                                    "used",
                                    struct {
                                        put("human", stat.peakUsage.usedHuman)
                                        put("value", stat.peakUsage.used)
                                    }
                                )

                                put(
                                    "committed",
                                    struct {
                                        put("human", stat.peakUsage.committedHuman)
                                        put("value", stat.peakUsage.committed)
                                    }
                                )

                                put(
                                    "max",
                                    struct {
                                        put("human", stat.peakUsage.maxHuman)
                                        put("value", stat.peakUsage.max)
                                    }
                                )

                                put(
                                    "init",
                                    struct {
                                        put("human", stat.peakUsage.initHuman)
                                        put("value", stat.peakUsage.init)
                                    }
                                )
                            }
                        )
                    }
                }
            )

            put(
                "coroutines",
                (coroutines as List<CoroutineStats>).map { coroutine ->
                    struct {
                        put("state", coroutine.state)
                        put("context", coroutine.context)
                        if (coroutine.job != null) {
                            put(
                                "job",
                                struct {
                                    put("is_active", coroutine.job!!.active)
                                    put("is_cancelled", coroutine.job!!.cancelled)
                                    put("is_completed", coroutine.job!!.completed)
                                }
                            )
                        }

                        put(
                            "stacktrace",
                            coroutine.stacktrace.map { st ->
                                struct {
                                    put("class_loader_name", st.classLoaderName)
                                    put("module_name", st.moduleName)
                                    put("module_version", st.moduleVersion)
                                    put("declaring_class", st.declaringClass)
                                    put("method_name", st.methodName)
                                    put("file", st.file)
                                    put("line", st.line)
                                    put("is_native_method", st.isNativeMethod)
                                }
                            }
                        )
                    }
                }
            )

            /*
                            struct {
                    put("state", coroutines.state)
                    put("context", coroutines.context)
                    if (coroutines.job != null) {
                        put(
                            "job",
                            struct {
                                put("is_active", coroutines.job!!.active)
                                put("is_cancelled", coroutines.job!!.cancelled)
                                put("is_completed", coroutines.job!!.completed)
                            }
                        )
                    }

                    put(
                        "stacktrace",
                        coroutines.stacktrace.map { st ->
                            struct {
                                put("class_loader_name", st.classLoaderName)
                                put("module_name", st.moduleName)
                                put("module_version", st.moduleVersion)
                                put("declaring_class", st.declaringClass)
                                put("method_name", st.methodName)
                                put("file", st.file)
                                put("line", st.line)
                                put("is_native_method", st.isNativeMethod)
                            }
                        }
                    )
                }
             */

            put(
                "threads",
                struct {
                    put("count", threads.count)
                    put("background", threads.background)
                    put(
                        "threads",
                        threads.threads.map { threadInfo ->
                            struct {
                                put(
                                    "user_time",
                                    struct {
                                        put("human", threadInfo.userTimeHuman)
                                        put("value", threadInfo.userTimeMs)
                                    }
                                )

                                put(
                                    "cpu_time",
                                    struct {
                                        put("human", threadInfo.cpuTimeHuman)
                                        put("value", threadInfo.cpuTimeMs)
                                    }
                                )

                                put(
                                    "stacktrace",
                                    threadInfo.stacktrace.map { st ->
                                        struct {
                                            put("class_loader_name", st.classLoaderName)
                                            put("module_name", st.moduleName)
                                            put("module_version", st.moduleVersion)
                                            put("declaring_class", st.declaringClass)
                                            put("method_name", st.methodName)
                                            put("file", st.file)
                                            put("line", st.line)
                                            put("is_native_method", st.isNativeMethod)
                                        }
                                    }
                                )

                                put("is_background", threadInfo.background)
                                put("is_suspended", threadInfo.suspended)
                                put("state", threadInfo.state)
                                put("name", threadInfo.name)
                                put("id", threadInfo.id)
                            }
                        }
                    )
                }
            )

            put(
                "jvm",
                struct {
                    put(
                        "total_memory",
                        struct {
                            put("human", jvm.totalMemoryHuman)
                            put("value", jvm.totalMemoryBytes)
                        }
                    )

                    put(
                        "max_memory",
                        struct {
                            put("human", jvm.maxMemoryHuman)
                            put("value", jvm.maxMemoryBytes)
                        }
                    )

                    put(
                        "free_memory",
                        struct {
                            put("human", jvm.freeMemoryHuman)
                            put("value", jvm.freeMemoryBytes)
                        }
                    )

                    put(
                        "start_time",
                        struct {
                            put("human", jvm.startTimeHuman)
                            put("value", jvm.startTimeMs)
                        }
                    )

                    put("version", jvm.version)
                    put("vendor", jvm.vendor)
                    put("name", jvm.name)
                    put("date", jvm.date)
                    put("pid", jvm.pid)
                }
            )
        }

        val now = Clock.System.now()
        val response = ReceiveStatsResponse.newBuilder().apply {
            product = "charted-server"
            version = ChartedInfo.version
            commitSha = ChartedInfo.commitHash
            buildDate = Instant.parse(ChartedInfo.buildDate).toString()
            buildFlavour = ChartedInfo.distribution.toBuildFlavour()
            snapshotDate = timestamp {
                seconds = now.epochSeconds
                nanos = now.nanosecondsOfSecond
            }
            data = statData
        }

        return response.build()
    }
}
