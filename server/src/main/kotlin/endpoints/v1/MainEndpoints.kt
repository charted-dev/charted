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
import guru.zoroark.tegral.openapi.dsl.OpenApiVersion
import guru.zoroark.tegral.openapi.dsl.openApi
import guru.zoroark.tegral.openapi.dsl.toJson
import guru.zoroark.tegral.openapi.dsl.toYaml
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.features.ServerFeature
import org.noelware.charted.server.createKtorContentWithInputStream
import org.noelware.charted.server.openapi.charted
import org.noelware.charted.types.responses.ApiResponse
import org.noelware.ktor.endpoints.AbstractEndpoint
import org.noelware.ktor.endpoints.Get

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

@Serializable
data class FeaturesResponse(
    @SerialName("docker_registry")
    val dockerRegistry: Boolean,
    val registrations: Boolean,

    @SerialName("audit_logs")
    val auditLogs: Boolean,

    @SerialName("webhooks")
    val webhooks: Boolean,

    @SerialName("is_invite_only")
    val isInviteOnly: Boolean,
    val integrations: Map<String, Boolean>,
    val search: Boolean
)

class MainEndpoint(private val config: Config): AbstractEndpoint("/") {
    @Get
    @Traced
    suspend fun main(call: ApplicationCall) {
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

    @Get("/features")
    @Traced
    suspend fun features(call: ApplicationCall) {
        call.respond(
            HttpStatusCode.OK,
            ApiResponse.ok(
                FeaturesResponse(
                    config.features.contains(ServerFeature.DOCKER_REGISTRY),
                    config.registrations,
                    config.features.contains(ServerFeature.AUDIT_LOGS),
                    config.features.contains(ServerFeature.WEBHOOKS),
                    config.inviteOnly,
                    mapOf(),
                    config.search != null && (config.search!!.elasticsearch != null || config.search!!.meilisearch != null),
                ),
            ),
        )
    }

    @Get("/openapi")
    suspend fun openapi(call: ApplicationCall) {
        val openapi = openApi { charted() }
        val format = call.request.queryParameters["format"]
        val version = OpenApiVersion.values().find { it.version == (call.request.queryParameters["version"] ?: "3.0") } ?: OpenApiVersion.V3_0

        val document = when (format) {
            null, "json" -> {
                openapi.toJson(version)
            }

            "yaml" -> {
                openapi.toYaml(version)
            }

            else -> {
                openapi.toJson(version)
            }
        }

        call.respondText(
            document,
            if (format == "yaml") {
                ContentType.parse("text/yaml; charset=utf-8")
            } else {
                ContentType.Application.Json
            },
        )
    }

    @Get("/swagger")
    suspend fun swagger(call: ApplicationCall) {
        if (!config.swaggerUi) return call.respond(HttpStatusCode.NotFound)
        val resource = this::class.java.getResource("/swagger/swagger-ui.html")
            ?: return call.respond(HttpStatusCode.NotFound)

        val stream = withContext(Dispatchers.IO) {
            resource.openStream()
        }

        call.respond(createKtorContentWithInputStream(stream, ContentType.Text.Html))
    }
}
