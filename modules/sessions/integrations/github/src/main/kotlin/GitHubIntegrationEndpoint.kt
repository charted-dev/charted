/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

@file:Suppress("unused")

package org.noelware.charted.modules.sessions.integrations.github

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import org.noelware.charted.RandomStringGenerator
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.toApiBaseUrl
import org.noelware.charted.types.responses.ApiResponse
import org.noelware.ktor.endpoints.AbstractEndpoint
import org.noelware.ktor.endpoints.Get
import org.noelware.ktor.endpoints.Post
import java.util.concurrent.ConcurrentLinkedQueue

class GitHubIntegrationEndpoint(
    private val config: Config,
    private val githubIntegration: GitHubIntegrationSupport
) : AbstractEndpoint("/sessions/integrations/github") {
    private val statesCache: ConcurrentLinkedQueue<String> = ConcurrentLinkedQueue()

    @Get
    suspend fun main(call: ApplicationCall) {
        val stateKey = RandomStringGenerator.generate(8)
        statesCache.add(stateKey)

        val redirectUri = config.toApiBaseUrl("/sessions/integrations/github/callback")
        val url = "https://github.com/login/oauth/authorize?client_id=someclientidhere&redirect_uri=$redirectUri&state=$stateKey"

        call.respondRedirect(url)
    }

    @Post("/callback")
    suspend fun callback(call: ApplicationCall) {
        val state = call.request.queryParameters["state"]
        if (state == null || !statesCache.contains(state)) {
            return call.respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "UNKNOWN_STATE_KEY", "Missing ?state query parameter or state was invalid",
                ),
            )
        }

        // Remove the cached state key since we don't need it anymore
        statesCache.remove(state)

        val session = githubIntegration.doAuthorize(call) ?: return
        call.respond(HttpStatusCode.Created, ApiResponse.ok(session.toJsonObject(true)))
    }
}
