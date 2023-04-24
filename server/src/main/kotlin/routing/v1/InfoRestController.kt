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

import com.fasterxml.jackson.annotation.JsonProperty
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import io.swagger.v3.oas.annotations.media.Schema
import io.swagger.v3.oas.models.PathItem
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.ChartedInfo
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.openapi.toPaths
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController

/**
 * Represents the response for the `GET /info` REST handler.
 *
 * @param distribution The distribution the server is running off from.
 * @param commitHash The commit hash from the Git repository.
 * @param buildDate The build date, in ISO-8601 format.
 * @param product The product name, will always be `charted-server`
 * @param version The version of the server.
 * @param vendor The vendor that maintains this project, will always be Noelware.
 */
@Schema(description = "Represents the response for the `GET /info` REST handler.")
@Serializable
data class InfoResponse(
    @get:Schema(description = "The distribution the server is running off from")
    val distribution: ChartedInfo.Distribution,

    @get:Schema(description = "The commit hash from the Git repository.")
    @JsonProperty("commit_sha")
    @SerialName("commit_sha")
    val commitHash: String,

    @get:Schema(description = "Build date in RFC3339 format")
    @JsonProperty("build_date")
    @SerialName("build_date")
    val buildDate: String,

    @get:Schema(description = "Product name. Will always be \"charted-server\"")
    val product: String,

    @get:Schema(description = "Valid SemVer 2 of the current version of this instance")
    val version: String,

    @get:Schema(description = "Vendor of charted-server, will always be \"Noelware\"")
    val vendor: String
)

class InfoRestController: RestController("/info") {
    override val apiVersion: APIVersion = APIVersion.V1
    override suspend fun call(call: ApplicationCall) {
        call.respond(
            HttpStatusCode.OK,
            ApiResponse.ok(
                InfoResponse(
                    ChartedInfo.distribution,
                    ChartedInfo.commitHash,
                    ChartedInfo.buildDate,
                    "charted-server",
                    ChartedInfo.version,
                    "Noelware",
                ),
            ),
        )
    }

    override fun toPathDsl(): PathItem = toPaths("/info") {
        description = "Returns basic information about the server"
        get {
            response(HttpStatusCode.OK) {
                contentType(ContentType.Application.Json) {
                    schema(
                        InfoResponse(
                            ChartedInfo.distribution,
                            ChartedInfo.commitHash,
                            ChartedInfo.buildDate,
                            "charted-server",
                            ChartedInfo.version,
                            "Noelware",
                        ),
                    )
                }
            }
        }
    }
}
