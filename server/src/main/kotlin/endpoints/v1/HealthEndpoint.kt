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

import guru.zoroark.tegral.openapi.dsl.RootDsl
import guru.zoroark.tegral.openapi.dsl.schema
import io.ktor.server.application.*
import io.ktor.server.response.*
import org.noelware.charted.ChartedInfo
import org.noelware.ktor.endpoints.AbstractEndpoint
import org.noelware.ktor.endpoints.Get

class HealthEndpoint : AbstractEndpoint("/heartbeat") {
    @Get
    suspend fun main(call: ApplicationCall): Unit = call.respond("OK")
    companion object {
        fun RootDsl.toOpenAPI() {
            "/heartbeat" get {
                summary = "Endpoint to signify that the server is healthy"
                externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api#GET-/heartbeat"

                200 response {
                    "text/plain" content {
                        schema<String>()
                        example = "OK"
                    }
                }
            }
        }
    }
}
