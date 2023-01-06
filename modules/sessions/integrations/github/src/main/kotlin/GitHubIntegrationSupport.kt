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

package org.noelware.charted.modules.sessions.integrations.github

import dev.floofy.utils.slf4j.logging
import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.request.*
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.jsonPrimitive
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.toApiBaseUrl
import org.noelware.charted.modules.sessions.Session
import org.noelware.charted.modules.sessions.SessionManager
import org.noelware.charted.modules.sessions.integrations.IntegrationSupport
import org.noelware.charted.types.responses.ApiResponse

class GitHubIntegrationSupport(
    private val config: Config,
    private val httpClient: HttpClient,
    private val sessionManager: SessionManager
) : IntegrationSupport {
    private val log by logging<GitHubIntegrationSupport>()

    override suspend fun doAuthorize(call: ApplicationCall): Session? {
        val code = call.request.queryParameters["code"] ?: return run {
            call.respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "MISSING_CODE_PARAMETER", "Missing ?code query parameter...",
                ),
            )

            null
        }

        // Now, we need to get an access token to validate the user
        val redirectUri = config.toApiBaseUrl("/sessions/integrations/github/callback")
        val accessTokenRes = httpClient.post("https://github.com/login/oauth/access_token") {
            parameter("client_id", "someclientid")
            parameter("client_secret", "someclientsecret")
            parameter("code", code)
            parameter("redirect_uri", redirectUri)
        }

        val atrBody: JsonObject = accessTokenRes.body()
        val accessToken = atrBody["access_token"]!!.jsonPrimitive.content
        val tokenType = atrBody["token_type"]!!.jsonPrimitive.content

        // Now, we need to get the user metadata
        val realTokenType = "${tokenType.replaceFirstChar { it.uppercaseChar() }}${tokenType.substring(1)}"
        val userRes = httpClient.get("https://api.github.com/user") {
            header("Authorization", "$realTokenType $accessToken")
        }

        return null
    }
}
