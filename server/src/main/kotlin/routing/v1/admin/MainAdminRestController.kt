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

import com.fasterxml.jackson.annotation.JsonProperty
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.swagger.v3.oas.annotations.media.Schema
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.ChartedInfo
import org.noelware.charted.server.plugins.sessions.IsAdminGuard
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController

@Schema(description = "Generic entrypoint response for the Admin API")
@Serializable
data class MainAdminResponse(
    @get:Schema(description = "A cute welcoming message.")
    val message: String = "Welcome to the Admin API!",

    @get:Schema(description = "URI that points to the Admin API documentation. This is a deprecated property since Admin APIs are not documented.", deprecated = true)
    @Deprecated("Admin APIs are not documented. Scheduled to be removed in v0.5-nightly")
    @JsonProperty("docs_url")
    @SerialName("docs_url")
    val docsUrl: String = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/admin"
)

class MainAdminRestController: RestController("/admin") {
    override val apiVersion: APIVersion = APIVersion.V1
    override fun Route.init() {
        install(Sessions)
        install(IsAdminGuard)
    }

    override suspend fun call(call: ApplicationCall) {
        call.respond(MainAdminResponse())
    }
}
