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
import io.ktor.server.request.*
import io.ktor.server.response.*
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.buildJsonObject
import org.jetbrains.exposed.sql.Op
import org.jetbrains.exposed.sql.SqlExpressionBuilder.eq
import org.noelware.charted.ChartedInfo
import org.noelware.charted.ValidationException
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.databases.postgres.asyncTransaction
import org.noelware.charted.databases.postgres.entities.OrganizationEntity
import org.noelware.charted.databases.postgres.flags.MemberPermissions
import org.noelware.charted.databases.postgres.flags.OrganizationFlags
import org.noelware.charted.databases.postgres.models.Organization
import org.noelware.charted.databases.postgres.tables.OrganizationTable
import org.noelware.charted.extensions.json.toJsonArray
import org.noelware.charted.extensions.regexp.toNameRegex
import org.noelware.charted.modules.avatars.AvatarModule
import org.noelware.charted.modules.helm.charts.HelmChartModule
import org.noelware.charted.modules.storage.StorageHandler
import org.noelware.charted.server.plugins.PreconditionResult
import org.noelware.charted.server.plugins.SessionsPlugin
import org.noelware.charted.server.plugins.currentUser
import org.noelware.charted.server.plugins.currentUserEntity
import org.noelware.charted.snowflake.Snowflake
import org.noelware.charted.types.responses.ApiError
import org.noelware.charted.types.responses.ApiResponse
import org.noelware.ktor.endpoints.*

@Serializable
data class MainOrganizationsResponse(
    val message: String = "Welcome to the Repositories API!",

    @SerialName("docs_url")
    val docsUrl: String = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/organizations"
)

/**
 * Payload for creating an organization
 * @param displayName The organization's display name
 * @param private If the organization should be considered private
 * @param owner The owner's ID
 * @param name The name of the organization
 */
@Serializable
data class CreateOrganizationBody(
    @SerialName("display_name")
    val displayName: String? = null,
    val private: Boolean = false,
    val owner: Long,
    val name: String
)

/**
 * Payload for patching an organization's metadata
 * @param allowExperimentalFeatures If this organization has opted in to allow experimental features to be enabled
 * @param twitterHandle             Twitter handle to identify this organization
 * @param gravatarEmail             Gravatar email for the organization's avatar
 * @param displayName               Organization display name
 * @param private                   If the organization should be private, so only the organization members
 *                                  can assess the organization.
 * @param name                      The name of the organization
 */
@Serializable
data class PatchOrganizationBody(
    @SerialName("allow_experimental_features")
    val allowExperimentalFeatures: Boolean = false,

    @SerialName("twitter_handle")
    val twitterHandle: String? = null,

    @SerialName("gravatar_email")
    val gravatarEmail: String? = null,

    @SerialName("display_name")
    val displayName: String? = null,
    val private: Boolean = false,
    val name: String? = null
)

class OrganizationsEndpoint(
    private val config: Config,
    private val avatars: AvatarModule,
    private val storage: StorageHandler,
    private val snowflake: Snowflake,
    private val charts: HelmChartModule? = null
) : AbstractEndpoint("/organizations") {
    init {
        // +==============================+
        // Organizations Endpoints
        // +==============================+
        install(HttpMethod.Delete, "/organizations/{id}", SessionsPlugin) {
            this += "org:delete"

            condition { call ->
                val org = call.getOrganizationById() ?: return@condition PreconditionResult.Failed(ApiError.EMPTY)
                val canView = call.checkIfOrgMemberCanView(org)
                if (canView !is PreconditionResult.Success) return@condition canView

                if (call.currentUser!!.id == org.owner.id.value) {
                    PreconditionResult.Success
                } else {
                    PreconditionResult.Failed(HttpStatusCode.Forbidden, "NOT_ORGANIZATION_OWNER", "Only the organization owner can delete this organization")
                }
            }
        }

        install(HttpMethod.Patch, "/organizations/{id}", SessionsPlugin) {
            this += "org:update"

            condition { call ->
                val org = call.getOrganizationById()
                    ?: return@condition PreconditionResult.Failed(ApiError.EMPTY)

                val canView = call.checkIfOrgMemberCanView(org)
                if (canView !is PreconditionResult.Success) return@condition canView

                call.hasPermissionToDo("metadata:update", org)
            }
        }

        install(HttpMethod.Get, "/organizations/{idOrName}", SessionsPlugin) {
            allowNonAuthorizedRequests = true
            this += "org:access"

            condition { call ->
                val org = call.getOrganizationByIdOrName() ?: return@condition PreconditionResult.Failed(ApiError.EMPTY)
                call.checkIfOrgMemberCanView(org)
            }
        }

        // +==================================+
        // Organization Repositories Endpoints
        // +==================================+
        install(HttpMethod.Get, "/organizations/{idOrName}/repositories", SessionsPlugin) {
            allowNonAuthorizedRequests = true
            this += "repo:access"

            condition { call ->
                val org = call.getOrganizationByIdOrName() ?: return@condition PreconditionResult.Failed(ApiError.EMPTY)
                call.checkIfOrgMemberCanView(org)
            }
        }

        install(HttpMethod.Get, "/organizations/{idOrName}/repositories/{repoIdOrName}", SessionsPlugin) {
            allowNonAuthorizedRequests = true
            this += "repo:access"

            condition { call ->
                val org = call.getOrganizationByIdOrName() ?: return@condition PreconditionResult.Failed(ApiError.EMPTY)
                call.checkIfOrgMemberCanView(org)
            }
        }

        install(HttpMethod.Put, "/organizations/{idOrName}/repositories", SessionsPlugin) {
            this += "repo:create"

            condition { call ->
                val org = call.getOrganizationByIdOrName() ?: return@condition PreconditionResult.Failed(ApiError.EMPTY)
                val canView = call.checkIfOrgMemberCanView(org)
                if (canView !is PreconditionResult.Success) return@condition canView

                call.hasPermissionToDo("repo:create", org)
            }
        }

        // +==================================+
        // Organization Members Endpoints
        // +==================================+
        install(HttpMethod.Get, "/organizations/{idOrName}/members", SessionsPlugin) {
            allowNonAuthorizedRequests = true
            this += "org:members:list"

            condition { call ->
                val org = call.getOrganizationByIdOrName() ?: return@condition PreconditionResult.Failed(ApiError.EMPTY)
                call.checkIfOrgMemberCanView(org)
            }
        }

        install(HttpMethod.Put, "/organizations/{idOrName}/members", SessionsPlugin) {
            this += "org:members:invites"

            condition { call ->
                val org = call.getOrganizationByIdOrName() ?: return@condition PreconditionResult.Failed(ApiError.EMPTY)
                val canView = call.checkIfOrgMemberCanView(org)
                if (canView !is PreconditionResult.Success) return@condition canView

                call.hasPermissionToDo("member:invite", org)
            }
        }

        install(HttpMethod.Get, "/organizations/{idOrName}/members/invites", SessionsPlugin) {
            this += "org:members:invites"

            condition { call ->
                val org = call.getOrganizationByIdOrName() ?: return@condition PreconditionResult.Failed(ApiError.EMPTY)
                val canView = call.checkIfOrgMemberCanView(org)
                if (canView !is PreconditionResult.Success) return@condition canView

                call.hasPermissionToDo("member:invite", org)
            }
        }

        install(HttpMethod.Get, "/organizations/{idOrName}/members/{memberId}", SessionsPlugin) {
            allowNonAuthorizedRequests = true
            this += "org:members:list"

            condition { call ->
                val org = call.getOrganizationByIdOrName() ?: return@condition PreconditionResult.Failed(ApiError.EMPTY)
                call.checkIfOrgMemberCanView(org)
            }
        }

        install(HttpMethod.Delete, "/organizations/{idOrName}/members/{memberId}", SessionsPlugin) {
            this += "org:members:delete"

            condition { call ->
                val org = call.getOrganizationByIdOrName() ?: return@condition PreconditionResult.Failed(ApiError.EMPTY)
                val canView = call.checkIfOrgMemberCanView(org)
                if (canView !is PreconditionResult.Success) return@condition canView

                call.hasPermissionToDo("member:kick", org)
            }
        }

        install(HttpMethod.Patch, "/organizations/{idOrName}/members/{memberId}", SessionsPlugin) {
            this += "org:members:update"

            condition { call ->
                val org = call.getOrganizationByIdOrName() ?: return@condition PreconditionResult.Failed(ApiError.EMPTY)
                val canView = call.checkIfOrgMemberCanView(org)
                if (canView !is PreconditionResult.Success) return@condition canView

                call.hasPermissionToDo("member:update", org)
            }
        }

        install(HttpMethod.Post, "/organizations/{idOrName}/members/{memberId}/invite/{inviteId}", SessionsPlugin) {
            allowNonAuthorizedRequests = true
            this += "org:members:invites"

            condition { call ->
                val org = call.getOrganizationByIdOrName() ?: return@condition PreconditionResult.Failed(ApiError.EMPTY)
                call.checkIfOrgMemberCanView(org)
            }
        }

        // +==================================+
        // Organization Webhooks Endpoints
        // +==================================+
        install(HttpMethod.Get, "/organizations/{idOrName}/webhooks", SessionsPlugin) {
            this += "org:webhooks:list"

            condition { call ->
                val org = call.getOrganizationByIdOrName() ?: return@condition PreconditionResult.Failed(ApiError.EMPTY)
                call.checkIfOrgMemberCanView(org)
            }
        }

        install(HttpMethod.Put, "/organizations/{idOrName}/webhooks", SessionsPlugin) {
            this += "org:webhooks:create"

            condition { call ->
                val org = call.getOrganizationByIdOrName() ?: return@condition PreconditionResult.Failed(ApiError.EMPTY)
                call.checkIfOrgMemberCanView(org)
            }
        }

        install(HttpMethod.Get, "/organizations/{idOrName}/webhooks/{webhookId}", SessionsPlugin) {
            this += "org:webhooks:list"

            condition { call ->
                val org = call.getOrganizationByIdOrName() ?: return@condition PreconditionResult.Failed(ApiError.EMPTY)
                call.checkIfOrgMemberCanView(org)
            }
        }

        install(HttpMethod.Patch, "/organizations/{idOrName}/webhooks/{webhookId}", SessionsPlugin) {
            this += "org:webhooks:update"

            condition { call ->
                val org = call.getOrganizationByIdOrName()
                    ?: return@condition PreconditionResult.Failed(ApiError.EMPTY)

                val canView = call.checkIfOrgMemberCanView(org)
                if (canView !is PreconditionResult.Success) return@condition canView

                call.hasPermissionToDo("metadata:update", org)
            }
        }

        install(HttpMethod.Delete, "/organizations/{idOrName}/webhooks/{webhookId}", SessionsPlugin) {
            this += "org:webhooks:delete"

            condition { call ->
                val org = call.getOrganizationByIdOrName() ?: return@condition PreconditionResult.Failed(ApiError.EMPTY)
                call.checkIfOrgMemberCanView(org)
            }
        }

        install(HttpMethod.Get, "/organizations/{idOrName}/webhooks/{webhookId}/events", SessionsPlugin) {
            this += "org:webhooks:events:list"

            condition { call ->
                val org = call.getOrganizationByIdOrName() ?: return@condition PreconditionResult.Failed(ApiError.EMPTY)
                call.checkIfOrgMemberCanView(org)
            }
        }

        install(HttpMethod.Get, "/organizations/{idOrName}/webhooks/{webhookId}/events/{eventId}", SessionsPlugin) {
            this += "org:webhooks:events:list"

            condition { call ->
                val org = call.getOrganizationByIdOrName() ?: return@condition PreconditionResult.Failed(ApiError.EMPTY)
                call.checkIfOrgMemberCanView(org)
            }
        }

        install(HttpMethod.Delete, "/organizations/{idOrName}/webhooks/{webhookId}/events/{eventId}", SessionsPlugin) {
            this += "org:webhooks:events:delete"
        }
    }

    // +==============================+
    // Organizations Endpoints
    // +==============================+

    /**
     * Main entrypoint to the Organizations API, nothing special!
     * @statusCode 200
     */
    @Get
    suspend fun main(call: ApplicationCall): Unit = call.respond(HttpStatusCode.OK, ApiResponse.ok(MainOrganizationsResponse()))

    /**
     * Retrieves an organization's metadata by their snowflake ID or name.
     * @statusCode 200 The [Organization] entity
     * @statusCode 404 If the organization doesn't exist
     */
    @Get("/{idOrName}")
    suspend fun getById(call: ApplicationCall) {
        val organization = call.getOrganizationByIdOrName() ?: return
        call.respond(HttpStatusCode.OK, ApiResponse.ok(Organization.fromEntity(organization)))
    }

    /**
     * Creates an organization
     * @statusCode 201 The newly [Organization] entity
     * @statusCode 400 If the organization's name was not properly formatted
     * @statusCode 406 If the organization's name already exists in the database
     * @statusCode 500 If any database errors occur
     */
    @Put
    suspend fun createOrganization(call: ApplicationCall) {
        val payload: CreateOrganizationBody = call.receive()

        // Check if the organization name is already taken
        val orgName = asyncTransaction {
            OrganizationEntity.find { OrganizationTable.name eq payload.name }.firstOrNull()
        }

        if (orgName != null) {
            throw ValidationException("body.name", "Organization by name '${payload.name}' is already taken")
        }

        val flags = OrganizationFlags()
        if (payload.private) {
            flags.add("PRIVATE")
        }

        val id = snowflake.generate()
        val org = asyncTransaction {
            OrganizationEntity.new(id.value) {
                this.displayName = payload.displayName
                this.owner = call.currentUserEntity!!
                this.flags = flags.bits()
                this.name = payload.name
            }
        }

        call.respond(HttpStatusCode.OK, ApiResponse.ok(Organization.fromEntity(org)))
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

    private suspend fun ApplicationCall.getOrganization(by: String, query: Op<Boolean>): OrganizationEntity? = asyncTransaction {
        OrganizationEntity.find(query).firstOrNull()
    } ?: run {
        respond(
            HttpStatusCode.NotFound,
            ApiResponse.err(
                "UNKNOWN_ORGANIZATION", "Unable to find organization by $by",
            ),
        )

        null
    }

    private suspend fun ApplicationCall.getOrganizationById(): OrganizationEntity? {
        val id = parameters["id"]?.toLongOrNull()
            ?: return run {
                respond(
                    HttpStatusCode.BadRequest,
                    ApiResponse.err(
                        "INVALID_ORGANIZATION_ID",
                        "Provided organization ID was not a valid snowflake",
                    ),
                )

                null
            }

        return getOrganization("id path parameter [$id]", OrganizationTable.id eq id)
    }

    private suspend fun ApplicationCall.getOrganizationByIdOrName(): OrganizationEntity? {
        val idOrName = parameters["idOrName"] ?: return run {
            respond(
                HttpStatusCode.NotFound,
                ApiResponse.err(
                    "MISSING_PARAMETER", "Missing [idOrName] path parameter",
                ),
            )

            null
        }

        return when {
            idOrName.toLongOrNull() != null -> getOrganization("id path parameter [$idOrName]", OrganizationTable.id eq idOrName.toLong())
            idOrName.toNameRegex(false, 24).matches() -> getOrganization("org name path parameter [$idOrName]", OrganizationTable.name eq idOrName)
            else -> {
                respond(
                    HttpStatusCode.BadRequest,
                    ApiResponse.err(
                        "UNKNOWN_ENTITY",
                        "Unable to determine if [idOrName] provided is by ID or name, provided [$idOrName]",
                    ),
                )

                null
            }
        }
    }

    private fun ApplicationCall.checkIfOrgMemberCanView(organization: OrganizationEntity): PreconditionResult {
        // Allow all permissions to go through if the current user is the organization owner
        if (currentUser != null && organization.owner.id.value == currentUser!!.id) {
            return PreconditionResult.Success
        }

        // Check if the repository is organization and the user is a member of that organization
        val flags = OrganizationFlags(organization.flags)
        if (flags.has("PRIVATE")) {
            organization.members.firstOrNull { it.account.id.value == currentUser!!.id }
                ?: return PreconditionResult.Failed(HttpStatusCode.Forbidden, "INVALID_ORGANIZATION_ACCESS", "You do not have permission to access this organization")
        }

        // Allow it no matter what, this is only for view access and not
        // destructive actions like DELETE
        return PreconditionResult.Success
    }

    private fun ApplicationCall.hasPermissionToDo(permission: String, organization: OrganizationEntity): PreconditionResult {
        val result = checkIfOrgMemberCanView(organization)
        if (result !is PreconditionResult.Success) {
            return result
        }

        val member = organization.members.first { it.account.id.value == currentUser!!.id }
        val perms = MemberPermissions(member.permissions)

        return if (perms.has(permission)) {
            PreconditionResult.Success
        } else {
            PreconditionResult.Failed(
                "INVALID_PERMISSION",
                "You do not have the '$permission' permission",
                buildJsonObject {
                    put("permissions", perms.enabledFlags().toJsonArray())
                },
            )
        }
    }
}
