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
import kotlinx.serialization.Serializable
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.openapi.toPaths
import org.noelware.charted.server.routing.RestController

/**
 * Represents the response for the `GET /` method.
 * @param message The message, which will always be "Hello, world!"
 * @param tagline You know, for Helm charts?
 * @param docs The documentation URL.
 */
@Serializable
data class MainResponse(
    val message: String,
    val tagline: String,
    val docs: String
)

class MainRestController: RestController("/") {
    override suspend fun call(call: ApplicationCall) {
        call.respond(
            HttpStatusCode.OK,
            ApiResponse.ok(
                MainResponse(
                    message = "Hello, world! \uD83D\uDC4B",
                    tagline = "You know, for Helm charts?",
                    docs = "https://charts.noelware.org/docs",
                ),
            ),
        )
    }

    override fun toPathDsl(): PathItem = toPaths("/") {
        description = "Generic main entrypoint"
        get {
            response(HttpStatusCode.OK) {
                contentType(ContentType.Application.Json) {
                    example = MainResponse(
                        message = "Hello, world! \uD83D\uDC4B",
                        tagline = "You know, for Helm charts?",
                        docs = "https://charts.noelware.org/docs",
                    )

                    schema<ApiResponse.Ok<MainResponse>>()
                }
            }
        }
    }
}
