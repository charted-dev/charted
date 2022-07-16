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

package org.noelware.charted.server.endpoints

import dev.floofy.utils.kotlin.sizeToStr
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.debug.DebugProbes
import kotlinx.coroutines.debug.State
import kotlinx.serialization.json.*
import org.noelware.charted.common.ChartedInfo
import org.noelware.charted.common.data.Config
import org.noelware.charted.common.data.Feature
import org.noelware.charted.core.StorageWrapper
import org.noelware.charted.search.elasticsearch.ElasticsearchClient
import org.noelware.charted.search.elasticsearch.index.Index
import org.noelware.charted.search.meilisearch.MeilisearchClient
import org.noelware.ktor.endpoints.AbstractEndpoint
import org.noelware.ktor.endpoints.Get
import java.lang.management.ManagementFactory

class DebugEndpoint(
    private val elastic: ElasticsearchClient? = null,
    private val meili: MeilisearchClient? = null,
    private val storage: StorageWrapper,
    private val config: Config
): AbstractEndpoint("/debug") {
    private val runtime = Runtime.getRuntime()
    private val threads = ManagementFactory.getThreadMXBean()
    private val os = ManagementFactory.getOperatingSystemMXBean()

//    init {
//        install(IsAdminGuard)
//        install(Sessions)
//    }

    @Get
    suspend fun main(call: ApplicationCall) {
        val threadInfos = threads.getThreadInfo(threads.allThreadIds)
        val threadInfo = buildJsonArray {
            for (info in threadInfos) {
                addJsonObject {
                    put("state", info.threadState.name)
                    put("name", info.threadName)
                    put("id", info.threadId)

                    if (threads.isThreadCpuTimeEnabled) {
                        put("cpu_time_ms", threads.getThreadCpuTime(info.threadId))
                    }

                    put("user_time_ms", threads.getThreadUserTime(info.threadId))
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

        val osInfo = buildJsonObject {
            put("system_load_avg", os.systemLoadAverage)
            put("processors", os.availableProcessors)
            put("version", os.version)
            put("arch", os.arch)
            put("name", os.name)
        }

        var searchBackendInfo: JsonElement? = null
        if (elastic != null) {
            val data = elastic.info()
            searchBackendInfo = buildJsonObject {
                put("server_version", elastic.serverVersion)
                put("backend", "Elasticsearch")
                put("repos", data[Index.REPOSITORIES.name]!!.jsonObject)
                put("users", data[Index.USERS.name]!!.jsonObject)
                put("orgs", data[Index.ORGANIZATIONS.name]!!.jsonObject)
                putJsonObject("cluster") {
                    put("name", elastic.clusterName)
                    put("uuid", elastic.clusterUUID)
                }
            }
        }

        if (meili != null) {
            searchBackendInfo = buildJsonObject {
                put("server_version", "unknown")
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
            put("chart_size", chartSize?.sizeToStr())
        }

        val jvmInfo = buildJsonObject {
            put("version", "${Runtime.version()}")
            put("vendor", System.getProperty("java.vendor"))
            put("date", System.getProperty("java.version.date"))
        }

        val runtimeInfo = buildJsonObject {
            put("free_memory", runtime.freeMemory())
            put("total_memory", runtime.totalMemory())
            put("max_memory", runtime.maxMemory())
        }

        call.respond(
            HttpStatusCode.OK,
            buildJsonObject {
                put("success", true)
                putJsonObject("data") {
                    put("distribution", ChartedInfo.distribution.key)
                    put("commit_sha", ChartedInfo.commitHash)
                    put("build_date", ChartedInfo.buildDate)
                    put("storage", storageInfo)
                    put("threads", threadInfo)
                    put("product", "charted-server")
                    put("version", ChartedInfo.version)
                    put("runtime", runtimeInfo)
                    put("search", searchBackendInfo ?: JsonNull)
                    put("vendor", "Noelware")
                    put("jvm", jvmInfo)
                    put("os", osInfo)
                }
            }
        )
    }

    @OptIn(ExperimentalCoroutinesApi::class)
    @Get("/coroutines")
    suspend fun coroutines(call: ApplicationCall) {
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
