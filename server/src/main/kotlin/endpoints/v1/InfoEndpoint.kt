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

import co.elastic.apm.api.Traced
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.ChartedInfo
import org.noelware.charted.types.responses.ApiResponse
import org.noelware.ktor.endpoints.AbstractEndpoint
import org.noelware.ktor.endpoints.Get

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
@Serializable
data class InfoResponse(
    val distribution: String,

    @SerialName("commit_sha")
    val commitHash: String,

    @SerialName("build_date")
    val buildDate: String,
    val product: String,
    val version: String,
    val vendor: String
)

class InfoEndpoint: AbstractEndpoint("/info") {
    @Get
    @Traced
    suspend fun main(call: ApplicationCall) {
        call.respond(
            HttpStatusCode.OK,
            ApiResponse.ok(
                InfoResponse(
                    ChartedInfo.distribution.key,
                    ChartedInfo.commitHash,
                    ChartedInfo.buildDate,
                    "charted-server",
                    ChartedInfo.version,
                    "Noelware"
                )
            )
        )
    }
}
