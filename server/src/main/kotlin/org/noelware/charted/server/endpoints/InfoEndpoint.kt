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
 * Unless required by applicable law or agreed to in writing, software
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
import kotlinx.serialization.json.putJsonObject
import org.noelware.charted.common.ChartedInfo
import org.noelware.ktor.endpoints.AbstractEndpoint
import org.noelware.ktor.endpoints.Get

class InfoEndpoint: AbstractEndpoint("/info") {
    @Get
    suspend fun main(call: ApplicationCall) {
        call.respond(
            HttpStatusCode.OK,
            buildJsonObject {
                put("success", true)
                putJsonObject("data") {
                    put("distribution", ChartedInfo.distribution.key)
                    put("commit_sha", ChartedInfo.commitHash)
                    put("build_date", ChartedInfo.buildDate)
                    put("product", "charted-server")
                    put("version", ChartedInfo.version)
                    put("vendor", "Noelware")
                }
            }
        )
    }
}
