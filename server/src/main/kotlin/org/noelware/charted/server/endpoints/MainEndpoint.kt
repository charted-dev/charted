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
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import org.noelware.charted.common.data.Config
import org.noelware.charted.common.data.Feature
import org.noelware.ktor.endpoints.AbstractEndpoint
import org.noelware.ktor.endpoints.Get

class MainEndpoint(private val config: Config): AbstractEndpoint() {
    @Get
    suspend fun main(call: ApplicationCall) {
        call.respond(
            HttpStatusCode.OK,
            buildJsonObject {
                put("success", true)
                put(
                    "data",
                    buildJsonObject {
                        put("message", "Hello, world! \uD83D\uDC4B")
                        put("tagline", "You know, for Helm charts?")
                        put("docs", "https://charts.noelware.org/docs")
                    }
                )
            }
        )
    }

    @Get("/features")
    suspend fun features(call: ApplicationCall) {
        call.respond(
            HttpStatusCode.OK,
            buildJsonObject {
                put("success", true)
                put(
                    "data",
                    buildJsonObject {
                        put("engine", if (config.isFeatureEnabled(Feature.DOCKER_REGISTRY)) "oci (private docker registry)" else "charts")
                        put("search", config.search.enabled)
                        put("registrations", config.registrations)
                        put("invite_only", config.inviteOnly)
                        put("telemetry", config.telemetry)
                        put("analytics", config.analytics != null)
                        put("enterprise", false)
                        put("lite", false)
                    }
                )
            }
        )
    }
}
