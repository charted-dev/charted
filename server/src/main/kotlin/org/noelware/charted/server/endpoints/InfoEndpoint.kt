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
import org.noelware.charted.engines.charts.ChartBackendEngine
import org.noelware.charted.search.elastic.ElasticsearchBackend
import org.noelware.charted.search.meili.MeilisearchBackend
import org.noelware.ktor.endpoints.AbstractEndpoint
import org.noelware.ktor.endpoints.Get
import org.noelware.remi.core.StorageTrailer
import java.lang.management.ManagementFactory

class InfoEndpoint(
    private val elastic: ElasticsearchBackend?,
    private val meili: MeilisearchBackend?,
    private val chartEngine: ChartBackendEngine?,
    private val storage: StorageTrailer<*>?
): AbstractEndpoint("/info") {
    private val runtime = Runtime.getRuntime()
    private val os = ManagementFactory.getOperatingSystemMXBean()

    @Get
    suspend fun main(call: ApplicationCall) {
        val osInfo = buildJsonObject {
            put("os", os.name)
            put("arch", os.arch)
            put("processors", os.availableProcessors)
            put("version", os.version)
            put("system_load_avg", os.systemLoadAverage)
        }

        var searchBackendInfo: JsonElement = JsonNull
        if (elastic != null) {
            searchBackendInfo = buildJsonObject {
                put("backend", "elasticsearch")
                put("server_version", "")
                put("users_count", 0)
                put("repos_count", 0)
                put("orgs_count", 0)
                put("repo_members_count", 0)
                put("org_members_count", 0)
                put(
                    "cluster",
                    buildJsonObject {
                        put("name", "noel-es-cluster")
                        put("id", "jsjdksdasdashbsaisda")
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
                put("name", storage.name)
            }
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
                        put("storage", storageInfo)
                    }
                )
            }
        )
    }
}
