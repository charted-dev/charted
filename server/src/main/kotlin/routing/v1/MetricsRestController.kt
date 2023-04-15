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

package org.noelware.charted.server.routing.v1

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import io.swagger.v3.oas.models.PathItem
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.modules.metrics.MetricsSupport
import org.noelware.charted.modules.metrics.disabled.DisabledMetricsSupport
import org.noelware.charted.modules.metrics.prometheus.PrometheusMetricsSupport
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController

class MetricsRestController(
    private val metrics: MetricsSupport,
    private val config: Config
): RestController(config.metrics.path) {
    override val apiVersion: APIVersion = APIVersion.V1
    override suspend fun call(call: ApplicationCall) {
        if (metrics is DisabledMetricsSupport) {
            return call.respond(HttpStatusCode.NotFound)
        }

        val m = metrics as PrometheusMetricsSupport
        return call.respondTextWriter(ContentType.parse("text/plain; version=0.0.4; charset=utf-8")) {
            m.writeIn(this)
        }
    }

    override fun toPathDsl(): PathItem = TODO("Dynamic route prefix")
}
