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

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import kotlinx.serialization.json.*
import org.noelware.charted.core.ChartedInfo
import org.noelware.charted.core.StorageWrapper
import org.noelware.charted.engines.charts.ChartBackendEngine
import org.noelware.charted.search.elastic.ElasticSearchBackend
import org.noelware.charted.search.meili.MeilisearchBackend
import org.noelware.ktor.endpoints.AbstractEndpoint
import org.noelware.ktor.endpoints.Get
import java.lang.management.ManagementFactory

class DebugEndpoint(
    private val elastic: ElasticSearchBackend?,
    private val meili: MeilisearchBackend?,
    private val chartEngine: ChartBackendEngine?,
    private val storage: StorageWrapper?
): AbstractEndpoint("/debug") {
    private val runtime = Runtime.getRuntime()
    private val threads = ManagementFactory.getThreadMXBean()
    private val os = ManagementFactory.getOperatingSystemMXBean()

    @Get
    suspend fun main(call: ApplicationCall) {
        val infos = threads.getThreadInfo(threads.allThreadIds)
        val threadInfo = buildJsonArray {
            for (info in infos) {
                add(
                    buildJsonObject {
                        put("name", info.threadName)
                        put("id", info.threadId)
                        put("state", info.threadState.name)

                        if (threads.isThreadCpuTimeEnabled) {
                            put("cpu_time_ms", threads.getThreadCpuTime(info.threadId))
                        }

                        put("user_time_ms", threads.getThreadUserTime(info.threadId))
                        put(
                            "stacktrace",
                            buildJsonArray {
                                for (element in info.stackTrace) {
                                    add(
                                        buildJsonObject {
                                            put("class_loader_name", element.classLoaderName)
                                            put("module_name", element.moduleName)
                                            put("module_version", element.moduleVersion)
                                            put("declaring_class", element.className)
                                            put("method_name", element.methodName)
                                            put("file_name", element.fileName)
                                            put("line_num", element.lineNumber)
                                            put("is_native_method", element.isNativeMethod)
                                        }
                                    )
                                }
                            }
                        )
                    }
                )
            }
        }

        val osInfo = buildJsonObject {
            put("os", os.name)
            put("arch", os.arch)
            put("processors", os.availableProcessors)
            put("version", os.version)
            put("system_load_avg", os.systemLoadAverage)
        }

        var searchBackendInfo: JsonElement = JsonNull
        if (elastic != null) {
            val data = elastic.info()
            searchBackendInfo = buildJsonObject {
                put("backend", "elasticsearch")
                put("server_version", elastic.serverVersion)
                put("users", data["charted_users"]!!.jsonObject)
                put("repos", data["charted_repos"]!!.jsonObject)
                put("orgs", data["charted_orgs"]!!.jsonObject)
                put("repo_members", data["charted_repo_members"]!!.jsonObject)
                put("org_members", data["charted_org_members"]!!.jsonObject)
                put(
                    "cluster",
                    buildJsonObject {
                        put("name", elastic.clusterName)
                        put("id", elastic.clusterUUID)
                    }
                )
            }
        }

        if (meili != null) {
            searchBackendInfo = buildJsonObject {
                put("backend", "meilisearch")
                put("users_count", 0)
                put("repos_count", 0)
                put("orgs_count", 0)
                put("repo_members_count", 0)
                put("org_members_count", 0)
                put("server_version", "")
            }
        }

        var chartEngineInfo: JsonElement = JsonNull
        if (chartEngine != null) {
            chartEngineInfo = buildJsonObject {
                put("class", "legacy charts")
            }
        }

        var storageInfo: JsonElement = JsonNull
        if (storage != null) {
            storageInfo = buildJsonObject {
                put("name", storage.trailer.name)
            }
        }

        val jvmInfo = buildJsonObject {
            put("vendor", System.getProperty("java.vendor"))
            put("date", System.getProperty("java.version.date"))
            put("version", "${Runtime.version()}")
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
                put(
                    "data",
                    buildJsonObject {
                        put("product", "charted-server")
                        put("vendor", "Noelware")
                        put("version", ChartedInfo.version)
                        put("commit_sha", ChartedInfo.commitHash)
                        put("build_date", ChartedInfo.dediNode)
                        put("distribution_type", "Docker")
                        put("search", searchBackendInfo)
                        put("engine", chartEngineInfo)
                        put("os", osInfo)
                        put("threads", threadInfo)
                        put("storage", storageInfo)
                        put("jvm", jvmInfo)
                        put("runtime", runtimeInfo)
                    }
                )
            }
        )
    }
}
