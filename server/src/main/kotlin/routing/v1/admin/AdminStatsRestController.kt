/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

package org.noelware.charted.server.routing.v1.admin

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.swagger.v3.oas.models.PathItem
import kotlinx.serialization.Serializable
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.modules.metrics.MetricsSupport
import org.noelware.charted.modules.metrics.collectors.JvmProcessInfoMetrics
import org.noelware.charted.modules.metrics.collectors.JvmThreadsMetrics
import org.noelware.charted.modules.metrics.collectors.OperatingSystemMetrics
import org.noelware.charted.modules.metrics.collectors.ServerInfoMetrics
import org.noelware.charted.modules.metrics.disabled.DisabledMetricsSupport
import org.noelware.charted.modules.postgresql.metrics.PostgresServerStats
import org.noelware.charted.server.plugins.sessions.IsAdminGuard
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.routing.RestController

@Serializable
data class AdminStatsResponse(
    val postgresql: PostgresServerStats,
    val process: JvmProcessInfoMetrics,
    val threads: JvmThreadsMetrics,
    val server: ServerInfoMetrics,
    val os: OperatingSystemMetrics
)

class AdminStatsRestController(private val metrics: MetricsSupport): RestController("/admin/stats") {
    override fun Route.init() {
        install(Sessions)
        install(IsAdminGuard)
    }

    override suspend fun call(call: ApplicationCall) {
        if (metrics is DisabledMetricsSupport) {
            return call.respond(HttpStatusCode.NotFound)
        }

        val stats = metrics.collect()
        call.respond(
            HttpStatusCode.OK,
            ApiResponse.ok(
                AdminStatsResponse(
                    stats["postgresql"] as PostgresServerStats,
                    stats["process"] as JvmProcessInfoMetrics,
                    stats["threads"] as JvmThreadsMetrics,
                    stats["server"] as ServerInfoMetrics,
                    stats["os"] as OperatingSystemMetrics,
                ),
            ),
        )
    }

    override fun toPathDsl(): PathItem = TODO("admin endpoints shouldn't be in the OpenAPI document")
}
