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

package org.noelware.charted.server.routing.v1.repositories

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
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController
import kotlin.reflect.typeOf

@Serializable
data class MainRepositoryResponse(
    val message: String = "Welcome to the Repositories API!",

    @SerialName("docs_url")
    val docsUrl: String = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/repositories"
)

class MainRepositoryRestController: RestController("/repositories") {
    override val apiVersion: APIVersion = APIVersion.V1
    override suspend fun call(call: ApplicationCall): Unit = call.respond(HttpStatusCode.OK, ApiResponse.ok(MainRepositoryResponse()))
    override fun toPathDsl(): PathItem = toPaths("/repositories") {
        get {
            description = "Generic entrypoint for the Repositories API"
            response(HttpStatusCode.OK) {
                contentType(ContentType.Application.Json) {
                    // type information get lost when using ApiResponse.ok() since
                    // ApiResponse.ok([data]) returns ApiResponse, so it gets lost.
                    schema(
                        typeOf<ApiResponse.Ok<MainRepositoryResponse>>(),
                        ApiResponse.ok(
                            MainRepositoryResponse(),
                        ),
                    )
                }
            }
        }
    }
}
