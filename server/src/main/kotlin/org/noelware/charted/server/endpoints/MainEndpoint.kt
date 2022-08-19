/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022 Noelware <team@noelware.org>
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

package org.noelware.charted.server.endpoints

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import kotlinx.serialization.SerialName
import org.noelware.charted.common.data.Config
import org.noelware.charted.common.data.Feature
import org.noelware.charted.common.data.responses.Response
import org.noelware.ktor.endpoints.AbstractEndpoint
import org.noelware.ktor.endpoints.Get

/**
 * Represents the response for the `GET /` method.
 * @param message The message, which will always be "Hello, world!"
 * @param tagline You know, for Helm charts?
 * @param docs The documentation URL.
 */
@kotlinx.serialization.Serializable
private data class MainResponse(
    val message: String,
    val tagline: String,
    val docs: String
)

/**
 * Represents the response for the `GET /features` method.
 *
 * @param registrations If registrations are enabled on the server. This is configurable from the
 *                      [`config.registrations`](https://charts.noelware.org/docs/server/self-hosting/configuration#registrations)
 *                      property.
 *
 * @param integrations Map of the integrations that are enabled for session management. Since this isn't
 *                     fully typed, results might vary.
 *
 * @param enterprise   If the distribution type of `charted-server` is the Enterprise product. This will be always
 *                     `false` in the OSS release. As it is possible to override the distribution type from the
 *                     system property we set, the response will always be `false.
 *
 * @param inviteOnly   If the server is only in "invite-only" mode, which means, only the administrators
 *                     of the server can only send out invites to join the server to create Helm charts. This
 *                     is `false` on the LDAP session management feature, since it'll use the Active Directory
 *                     rather than Redis for sessions.
 *
 * @param analytics    If Noelware Analytics is enabled on this instance, or not.
 * @param telemetry    If the server has opted to send telemetry events to Noelware or not, this is completely
 *                     optional and is disabled by default!
 *
 * @param search       Returns if the `POST /search` endpoint is enabled, or not.
 * @param engine       The engine the instance is using. This will always be "oci" or "charts"
 * @param lite         If the server is using the Lite edition of `charted-server`, this will be always
 *                     false on the JVM source code.
 */
@kotlinx.serialization.Serializable
private data class FeaturesResponse(
    val registrations: Boolean,
    val integrations: Map<String, Boolean> = mapOf(),
    val enterprise: Boolean,

    @SerialName("invite_only")
    val inviteOnly: Boolean,
    val analytics: Boolean,
    val telemetry: Boolean,
    val search: Boolean,
    val engine: String,
    val lite: Boolean
)

class MainEndpoint(private val config: Config): AbstractEndpoint() {
    @Get
    suspend fun main(call: ApplicationCall) {
        call.respond(
            HttpStatusCode.OK,
            Response.ok(
                MainResponse(
                    message = "Hello, world! \uD83D\uDC4B",
                    tagline = "You know, for Helm charts?",
                    docs = "https://charts.noelware.org/docs"
                )
            )
        )
    }

    @Get("/features")
    suspend fun features(call: ApplicationCall) {
        val integrations = config.sessions.integrations
        call.respond(
            HttpStatusCode.OK,
            Response.ok(
                FeaturesResponse(
                    config.registrations,

                    // Using the `to` infix function doesn't work, or I might have
                    // done something wrong.
                    mapOf(
                        Pair("github", integrations.github != null),
                        Pair("noelware", integrations.noelware != null),
                        Pair("google", integrations.google != null)
                    ),

                    false,
                    config.inviteOnly,
                    config.analytics != null,
                    config.telemetry,
                    config.search.elastic != null || config.search.meili != null,
                    if (config.isFeatureEnabled(Feature.DOCKER_REGISTRY)) {
                        "oci"
                    } else {
                        "charts"
                    },

                    false
                )
            )
        )
    }
}
