/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware <team@noelware.org>
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

package org.noelware.charted.server.endpoints.v1

import co.elastic.apm.api.Traced
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import org.noelware.charted.modules.metrics.MetricsSupport
import org.noelware.charted.modules.metrics.disabled.DisabledMetricsSupport
import org.noelware.charted.modules.metrics.prometheus.PrometheusMetricsSupport
import org.noelware.ktor.endpoints.AbstractEndpoint
import org.noelware.ktor.endpoints.Get

class MetricsEndpoint(private val metrics: MetricsSupport): AbstractEndpoint("/metrics") {
    @Get
    @Traced
    suspend fun main(call: ApplicationCall): Unit = if (metrics is DisabledMetricsSupport) {
        call.respond(HttpStatusCode.NotFound)
    } else {
        call.respondTextWriter { (metrics as PrometheusMetricsSupport).writeIn(this) }
    }
}
