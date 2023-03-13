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

package org.noelware.charted.server.routing.v1.users

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import io.swagger.v3.oas.models.PathItem
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.ChartedInfo
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.openapi.toPaths
import org.noelware.charted.server.routing.RestController

@Serializable
data class MainUserResponse(
    val message: String = "Welcome to the Users API!",

    @SerialName("docs_url")
    val docsUrl: String = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/users"
)

class MainUserRestController: RestController("/users") {
    override suspend fun call(call: ApplicationCall) {
        call.respond(HttpStatusCode.OK, ApiResponse.ok(MainUserResponse()))
    }

    override fun toPathDsl(): PathItem = toPaths("/users") {
        get {
            description = "Generic entrypoint for the Users API"
            response(HttpStatusCode.OK) {
                contentType(ContentType.Application.Json) {
                    schema<ApiResponse.Ok<MainUserResponse>>()
                    example = ApiResponse.ok(MainUserResponse())
                }
            }
        }
    }
}
