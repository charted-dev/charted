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
import io.swagger.v3.oas.annotations.media.Schema
import io.swagger.v3.oas.annotations.media.SchemaProperty
import io.swagger.v3.oas.models.PathItem
import kotlinx.serialization.Serializable
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.openapi.toPaths
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController
import kotlin.reflect.typeOf

/**
 * Represents the response for the `GET /` method.
 * @param message The message, which will always be "Hello, world!"
 * @param tagline You know, for Helm charts?
 * @param docs The documentation URL.
 */
@Serializable
data class MainResponse(
    @SchemaProperty(schema = Schema(description = "Message to greet users! Will always be \"Hello, world! \uD83D\uDC4B\""))
    val message: String,

    @SchemaProperty(schema = Schema(description = "Tagline of charted-server, will always be \"You know, for Helm charts?\""))
    val tagline: String,

    @SchemaProperty(schema = Schema(description = "Documentation URI for charted-server."))
    val docs: String
)

class MainRestController: RestController("/") {
    override val apiVersion: APIVersion = APIVersion.V1
    override suspend fun call(call: ApplicationCall) {
        call.respond(
            HttpStatusCode.OK,
            ApiResponse.ok(
                MainResponse(
                    "Hello, world! \uD83D\uDC4B",
                    "You know, for Helm charts?",
                    "https://charts.noelware.org/docs",
                ),
            ),
        )
    }

    override fun toPathDsl(): PathItem = toPaths("/") {
        description = "Generic main entrypoint"
        get {
            response(HttpStatusCode.OK) {
                contentType(ContentType.Application.Json) {
                    schema(
                        typeOf<ApiResponse.Ok<MainResponse>>(),
                        ApiResponse.ok(
                            MainResponse(
                                "Hello, world! \uD83D\uDC4B",
                                "You know, for Helm charts?",
                                "https://charts.noelware.org/docs",
                            ),
                        ),
                    )
                }
            }
        }
    }
}
