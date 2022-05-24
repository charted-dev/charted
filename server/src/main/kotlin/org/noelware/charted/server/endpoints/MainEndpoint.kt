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

import dev.floofy.utils.koin.inject
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import org.noelware.charted.core.config.Config
import org.noelware.charted.core.config.EngineClass
import org.noelware.ktor.endpoints.AbstractEndpoint
import org.noelware.ktor.endpoints.Get

class MainEndpoint: AbstractEndpoint() {
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
        val config: Config by inject()

        call.respond(
            HttpStatusCode.OK,
            buildJsonObject {
                put("success", true)
                put(
                    "data",
                    buildJsonObject {
                        put(
                            "chart_engine",
                            when (config.engine?.engineClass) {
                                null -> "charts"
                                EngineClass.CHART -> "charts"
                                EngineClass.OCI -> "oci (private docker registry)"
                            }
                        )

                        put("search_enabled", config.search.elastic != null || config.search.meili != null)
                        put("registrations", config.registrations)
                        put("invite_only", config.inviteOnly)
                        put("telemetry", false)
                        put("analytics", config.analytics != null)
                        put("lite", false)
                    }
                )
            }
        )
    }
}
