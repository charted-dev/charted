/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

@file:Suppress("UNUSED")

package org.noelware.charted.server.endpoints.v1.api.admin

import guru.zoroark.tegral.openapi.dsl.RootDsl
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import org.noelware.charted.modules.metrics.MetricsSupport
import org.noelware.charted.modules.metrics.disabled.DisabledMetricsSupport
import org.noelware.charted.server.internal.metrics.ServerInfoMetricsCollector
import org.noelware.charted.server.plugins.IsAdminGuard
import org.noelware.charted.server.plugins.SessionsPlugin
import org.noelware.charted.types.responses.ApiResponse
import org.noelware.ktor.endpoints.AbstractEndpoint
import org.noelware.ktor.endpoints.Get

class AdminEndpoints(private val metrics: MetricsSupport): AbstractEndpoint("/admin") {
    init {
        install(HttpMethod.Get, "/admin", SessionsPlugin)
        install(HttpMethod.Get, "/admin/stats", SessionsPlugin) {
            this += "admin:stats"
        }

        install(IsAdminGuard)
    }

    @Get
    suspend fun main(call: ApplicationCall): Unit = call.respond(MainAdminResponse())

    @Get("/stats")
    suspend fun stats(call: ApplicationCall) {
        if (metrics is DisabledMetricsSupport) {
            return call.respond(
                HttpStatusCode.OK,
                ApiResponse.ok(
                    AdminStatsResponse(null, null, null, null, ServerInfoMetricsCollector.supply(), null),
                ),
            )
        }

        val all = metrics.collect()
        call.respond(HttpStatusCode.OK, ApiResponse.ok(all))
    }

    companion object {
        fun RootDsl.toOpenAPI() {
            "/admin" get {
            }
        }
    }
}
