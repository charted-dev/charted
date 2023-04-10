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
import io.swagger.v3.oas.annotations.media.SchemaProperty
import io.swagger.v3.oas.models.PathItem
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.features.ExperimentalFeature
import org.noelware.charted.configuration.kotlin.dsl.features.Feature
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.openapi.toPaths
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController

/**
 * Represents the response for the GET /features REST controller
 * @param dockerRegistry Whether if the [external OCI registry][org.noelware.charted.configuration.kotlin.dsl.features.ExperimentalFeature.ExternalOciRegistry]
 * or the [homemade impl.][org.noelware.charted.configuration.kotlin.dsl.features.Feature.DockerRegistry] feature is enabled
 * @param registrations  Whether if registrations are enabled on the server
 * @param auditLogs      Whether if the [Audit Logging][org.noelware.charted.configuration.kotlin.dsl.features.Feature.AuditLogging] feature is enabled or not
 * @param webhooks       Whether if the [Webhooks][org.noelware.charted.configuration.kotlin.dsl.features.Feature.Webhooks] feature is enabled
 * @param isInviteOnly   Whether if the server is on invite-only.
 * @param integrations   Hash of all the enabled session integrations available
 * @param search         Whether if the server has search capabilities with the Elasticsearch or Meilisearch backend
 */
@Serializable
data class FeaturesResponse(
    @get:SchemaProperty(
        schema = Schema(
            description = "Whether if the external OCI registry experimental feature or the home-made implementation registry feature is enabled or not.",
        ),
    )
    @JsonProperty("docker_registry")
    @SerialName("docker_registry")
    val dockerRegistry: Boolean,

    @get:SchemaProperty(schema = Schema(description = "Whether if registrations are enabled on the server"))
    val registrations: Boolean,

    @get:SchemaProperty(schema = Schema(description = "Whether if the Audit Logging feature is enabled or not."))
    @JsonProperty("audit_logs")
    @SerialName("audit_logs")
    val auditLogs: Boolean,

    @get:SchemaProperty(schema = Schema(description = "Whether if the Webhooks feature is enabled or not."))
    val webhooks: Boolean,

    @get:SchemaProperty(schema = Schema(description = "Whether if this server instance is invite-only."))
    @JsonProperty("is_invite_only")
    @SerialName("is_invite_only")
    val isInviteOnly: Boolean,

    @get:SchemaProperty(schema = Schema(description = "Mapping of all available session integrations."))
    val integrations: Map<String, Boolean>,

    @get:SchemaProperty(schema = Schema(description = "Whether if the server has search capabilities with the Elasticsearch or Meilisearch backend"))
    val search: Boolean
)

class FeaturesRestController(private val config: Config): RestController("/features") {
    override val apiVersion: APIVersion = APIVersion.V1
    override suspend fun call(call: ApplicationCall) {
        call.respond(
            ApiResponse.ok(
                FeaturesResponse(
                    config.features.contains(Feature.DockerRegistry) || config.experimentalFeatures.contains(ExperimentalFeature.ExternalOciRegistry),
                    config.registrations,
                    false,
                    false,
                    // config.features.contains(Feature.AuditLogging),
                    // config.features.contains(Feature.Webhooks),
                    config.inviteOnly,
                    mapOf(),
                    config.search != null,
                ),
            ),
        )
    }

    override fun toPathDsl(): PathItem = toPaths("/features") {
        get {
            description = "Retrieve all the server instance's features"
            response(HttpStatusCode.OK) {
                contentType(ContentType.Application.Json) {
                    schema<ApiResponse.Ok<FeaturesResponse>>()
                    example = FeaturesResponse(
                        config.features.contains(Feature.DockerRegistry) || config.experimentalFeatures.contains(ExperimentalFeature.ExternalOciRegistry),
                        config.registrations,
                        false,
                        false,
//                        config.features.contains(Feature.AuditLogging),
//                        config.features.contains(Feature.Webhooks),
                        config.inviteOnly,
                        mapOf(),
                        false,
                    )
                }
            }
        }
    }
}
