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

package org.noelware.charted.server.routing.v1.organizations

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import kotlinx.serialization.Serializable
import org.noelware.charted.ChartedInfo
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.modules.openapi.kotlin.dsl.json
import org.noelware.charted.modules.openapi.kotlin.dsl.ok
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController
import org.noelware.charted.server.routing.openapi.ResourceDescription
import org.noelware.charted.server.routing.openapi.describeResource

@Serializable
data class MainOrganizationResponse(
    val message: String = "Welcome to the Organizations API!",
    val docs: String = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/organizations"
)

class MainOrganizationsRestController: RestController("/organizations") {
    override val apiVersion: APIVersion = APIVersion.V1
    override suspend fun call(call: ApplicationCall): Unit = call.respond(HttpStatusCode.OK, ApiResponse.ok(MainOrganizationResponse()))
    companion object: ResourceDescription by describeResource("/organizations", {
        get {
            description = "Generic entrypoint for the Repositories API"
            ok {
                json {
                    schema(MainOrganizationResponse())
                }
            }
        }
    })
}
