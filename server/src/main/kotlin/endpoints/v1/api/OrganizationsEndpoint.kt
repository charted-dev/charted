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

@file:Suppress("unused")

package org.noelware.charted.server.endpoints.v1.api

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.ChartedInfo
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.modules.avatars.AvatarModule
import org.noelware.charted.modules.helm.charts.HelmChartModule
import org.noelware.charted.modules.storage.StorageHandler
import org.noelware.charted.server.plugins.SessionsPlugin
import org.noelware.charted.snowflake.Snowflake
import org.noelware.charted.types.responses.ApiResponse
import org.noelware.ktor.endpoints.*

@Serializable
data class MainOrganizationsResponse(
    val message: String = "Welcome to the Repositories API!",

    @SerialName("docs_url")
    val docsUrl: String = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/organizations"
)

class OrganizationsEndpoint(
    private val config: Config,
    private val avatars: AvatarModule,
    private val storage: StorageHandler,
    private val snowflake: Snowflake,
    private val charts: HelmChartModule? = null
): AbstractEndpoint("/organizations") {
    init {
        // +==============================+
        // Organizations Endpoints
        // +==============================+
        install(HttpMethod.Delete, "/organizations/{id}", SessionsPlugin) {
            this += "org:delete"
        }

        install(HttpMethod.Patch, "/organizations/{id}", SessionsPlugin) {
            this += "org:update"
        }

        install(HttpMethod.Get, "/organizations/{owner}/{name}", SessionsPlugin) {
            allowNonAuthorizedRequests = true
            this += "org:access"
        }

        install(HttpMethod.Get, "/organizations/{id}", SessionsPlugin) {
            allowNonAuthorizedRequests = true
            this += "org:access"
        }

        // +==================================+
        // Organization Repositories Endpoints
        // +==================================+
        install(HttpMethod.Get, "/organizations/{idOrName}/repositories", SessionsPlugin) {
            allowNonAuthorizedRequests = true
            this += "repo:access"
        }

        install(HttpMethod.Get, "/organizations/{idOrName}/repositories/{repoIdOrName}", SessionsPlugin) {
            allowNonAuthorizedRequests = true
            this += "repo:access"
        }

        install(HttpMethod.Put, "/organizations/{idOrName}/repositories", SessionsPlugin) {
            allowNonAuthorizedRequests = true
            this += "repo:create"
        }

        // +==================================+
        // Organization Members Endpoints
        // +==================================+
        install(HttpMethod.Get, "/organizations/{idOrName}/members", SessionsPlugin) {
            allowNonAuthorizedRequests = true
            this += "org:members:list"
        }

        install(HttpMethod.Put, "/organizations/{idOrName}/members", SessionsPlugin) {
            this += "org:members:invites"
        }

        install(HttpMethod.Get, "/organizations/{idOrName}/members/invites", SessionsPlugin) {
            this += "org:members:invites"
        }

        install(HttpMethod.Get, "/organizations/{idOrName}/members/{memberId}", SessionsPlugin) {
            allowNonAuthorizedRequests = true
            this += "org:members:list"
        }

        install(HttpMethod.Delete, "/organizations/{idOrName}/members/{memberId}", SessionsPlugin) {
            this += "org:members:delete"
        }

        install(HttpMethod.Patch, "/organizations/{idOrName}/members/{memberId}", SessionsPlugin) {
            this += "org:members:update"
        }

        install(HttpMethod.Post, "/organizations/{idOrName}/members/{memberId}/invite/{inviteId}", SessionsPlugin) {
            this += "org:members:invites"
        }

        // +==================================+
        // Organization Webhooks Endpoints
        // +==================================+
        install(HttpMethod.Get, "/organizations/{idOrName}/webhooks", SessionsPlugin) {
            this += "org:webhooks:list"
        }

        install(HttpMethod.Put, "/organizations/{idOrName}/webhooks", SessionsPlugin) {
            this += "org:webhooks:create"
        }

        install(HttpMethod.Get, "/organizations/{idOrName}/webhooks/{webhookId}", SessionsPlugin) {
            this += "org:webhooks:list"
        }

        install(HttpMethod.Patch, "/organizations/{idOrName}/webhooks/{webhookId}", SessionsPlugin) {
            this += "org:webhooks:update"
        }

        install(HttpMethod.Delete, "/organizations/{idOrName}/webhooks/{webhookId}", SessionsPlugin) {
            this += "org:webhooks:delete"
        }

        install(HttpMethod.Get, "/organizations/{idOrName}/webhooks/{webhookId}/events", SessionsPlugin) {
            this += "org:webhooks:events:list"
        }

        install(HttpMethod.Get, "/organizations/{idOrName}/webhooks/{webhookId}/events/{eventId}", SessionsPlugin) {
            this += "org:webhooks:events:list"
        }

        install(HttpMethod.Delete, "/organizations/{idOrName}/webhooks/{webhookId}/events/{eventId}", SessionsPlugin) {
            this += "org:webhooks:events:delete"
        }
    }

    // +==============================+
    // Organizations Endpoints
    // +==============================+

    @Get
    suspend fun main(call: ApplicationCall): Unit = call.respond(HttpStatusCode.OK, ApiResponse.ok(MainOrganizationsResponse()))

    @Get("/{id}")
    suspend fun getById(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Get("/{owner}/{name}")
    suspend fun getByOwnerAndName(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Put
    suspend fun createOrganization(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Patch("/{id}")
    suspend fun updateOrganization(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Delete("/{id}")
    suspend fun deleteOrganization(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    // +==================================+
    // Organization Repositories Endpoints
    // +==================================+

    @Get("/{idOrName}/repositories")
    suspend fun listOrgRepositories(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Get("/{idOrName}/repositories/{repoIdOrName}")
    suspend fun getOrgRepositoryByName(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Put("/{idOrName}/repositories")
    suspend fun createOrgRepository(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    // +==================================+
    // Organization Members Endpoints
    // +==================================+
    @Get("/{idOrName}/members")
    suspend fun getAllOrganizationMembers(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Put("/{idOrName}/members")
    suspend fun inviteMemberToOrg(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Get("/{idOrName}/members/invites")
    suspend fun getAllPendingMemberInvites(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Get("/{idOrName}/members/{memberId}")
    suspend fun getOrgMember(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Patch("/{idOrName}/members/{memberId}")
    suspend fun updateOrgMember(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Delete("/{idOrName}/members/{memberId}")
    suspend fun kickOrgMember(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Post("/{idOrName}/members/{memberId}/invite/{inviteId}")
    suspend fun acceptPendingInvite(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    // +==================================+
    // Organization Webhooks Endpoints
    // +==================================+
    @Get("/{idOrName}/webhooks")
    suspend fun getAllWebhooks(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Get("/{idOrName}/webhooks/{webhookId}")
    suspend fun getWebhook(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Put("/{idOrName}/webhooks")
    suspend fun createWebhook(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Patch("/{idOrName}/webhooks/{webhookId}")
    suspend fun updateWebhook(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Delete("/{idOrName}/webhooks/{webhookId}")
    suspend fun deleteWebhook(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Get("/{idOrName}/webhooks/{webhookId}/events")
    suspend fun getAllWebhookEvents(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Get("/{idOrName}/webhooks/{webhookId}/events/{eventId}")
    suspend fun getWebhookEvent(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Delete("/{idOrName}/webhooks/{webhookId}/events/{eventId}")
    suspend fun deleteWebhookEvent(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }
}
