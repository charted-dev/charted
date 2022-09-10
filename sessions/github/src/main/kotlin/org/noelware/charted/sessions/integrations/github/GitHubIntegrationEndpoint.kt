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

package org.noelware.charted.sessions.integrations.github

import dev.floofy.utils.exposed.asyncTransaction
import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.request.*
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import kotlinx.serialization.json.*
import org.noelware.charted.common.ChartedScope
import org.noelware.charted.common.RandomGenerator
import org.noelware.charted.configuration.dsl.Config
import org.noelware.charted.database.entities.UserConnectionEntity
import org.noelware.charted.database.tables.UserConnectionsTable
import org.noelware.charted.sessions.integrations.github.responses.GitHubOAuth2Response
import org.noelware.ktor.endpoints.AbstractEndpoint
import org.noelware.ktor.endpoints.Get

private val states = mutableListOf<String>()

class GitHubIntegrationEndpoint(
    private val github: GitHubIntegration,
    private val httpClient: HttpClient,
    private val config: Config
): AbstractEndpoint("/integrations/github") {
    @Get
    suspend fun main(call: ApplicationCall) {
        call.respond(
            HttpStatusCode.OK,
            buildJsonObject {
                put("success", true)
                putJsonObject("data") {
                    put("message", "Welcome to the GitHub Integrations API!")
                    put("docs_uri", "https://charts.noelware.org/docs/server/integrations/github")
                }
            }
        )
    }

    @Get("/redirect")
    suspend fun redirect(call: ApplicationCall) {
        val state = RandomGenerator.generate(8)
        states.add(state)

        val redirectUrl = if (config.baseUrl != null) "${config.baseUrl}/integrations/github/callback" else "http://${config.server.host}:${config.server.port}/integrations/github/callback"
        val clientID = config.sessions.integrations.github!!.clientID // this is validated in the koin module, so this is safe
        val url = "https://github.com/login/oauth/authorize?client_id=$clientID&redirect_url=$redirectUrl&scopes=user&state=$state"

        call.respondRedirect(url)
    }

    @Get("/callback")
    suspend fun callback(call: ApplicationCall) {
        val state = call.request.queryParameters["state"]
            ?: return call.respond(
                HttpStatusCode.BadRequest,
                buildJsonObject {
                    put("success", false)
                    putJsonArray("errors") {
                        addJsonObject {
                            put("message", "Missing `?state` query parameter. You must authenticate via /integrations/github/redirect!")
                            put("code", "MISSING_QUERY_PARAMETER")
                        }
                    }
                }
            )

        if (!states.contains(state)) {
            return call.respond(
                HttpStatusCode.BadRequest,
                buildJsonObject {
                    put("success", false)
                    putJsonArray("errors") {
                        addJsonObject {
                            put("message", "Unable to find state hash. Did you authenticate correctly?")
                            put("code", "MISSING_STATE_HASH")
                        }
                    }
                }
            )
        }

        states.remove(state)
        val code = call.request.queryParameters["code"]
            ?: return call.respond(
                HttpStatusCode.BadRequest,
                buildJsonObject {
                    put("success", false)
                    putJsonArray("errors") {
                        addJsonObject {
                            put("message", "Missing `?code` query parameter. You must authenticate via /integrations/github/redirect!")
                            put("code", "MISSING_QUERY_PARAMETER")
                        }
                    }
                }
            )

        val clientID = config.sessions.integrations.github!!.clientID // this is validated in the koin module, so this is safe
        val res = httpClient.post("https://github.com/login/oauth/access_token") {
            header("Content-Type", "application/json")
            header("Accept", "application/json")
            setBody(
                buildJsonObject {
                    put("client_id", clientID)
                    put("client_secret", config.sessions.integrations.github!!.clientSecret)
                    put("code", code)
                }
            )
        }

        val body = res.body<GitHubOAuth2Response>()
        val user = httpClient.get("https://api.github.com/user") {
            header("Authorization", "token ${body.accessToken}")
            header("Accept", "application/vnd.github+json")
        }

        val userPayload = user.body<JsonObject>()
        if (user.status.value !in 200 until 300 && user.status.value != 304) {
            return call.respond(
                HttpStatusCode.InternalServerError,
                buildJsonObject {
                    put("success", false)
                    putJsonArray("errors") {
                        addJsonObject {
                            put("message", "Received ${user.status.value} ${user.status.description} when requesting to GitHub. If this is a common bug, please report it to Noelware: https://github.com/charted-dev/charted/issues/new")
                            put("code", "INTERNAL_SERVER_ERROR")
                        }
                    }
                }
            )
        }

        val id = userPayload["id"]!!.jsonPrimitive.content
        val connection = asyncTransaction(ChartedScope) {
            UserConnectionEntity.find {
                UserConnectionsTable.githubAccountID eq id
            }.firstOrNull()
        } ?: return call.respond(
            HttpStatusCode.NotFound,
            buildJsonObject {
                put("success", false)
                putJsonArray("errors") {
                    addJsonObject {
                        put("message", "GitHub account ID [$id] doesn't have a user connected with it.")
                    }
                }
            }
        )

        // Create the session
        val session = github.createSession(connection.id.value)
        call.respond(
            HttpStatusCode.Created,
            buildJsonObject {
                put("success", true)
                put("data", session.toJsonObject(true))
            }
        )
    }
}
