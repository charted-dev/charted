package org.noelware.charted.server.endpoints.v1.api.organizations

import dev.floofy.utils.kotlin.ifNotNull
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import kotlinx.serialization.json.buildJsonObject
import org.noelware.charted.databases.postgres.asyncTransaction
import org.noelware.charted.databases.postgres.entities.OrganizationEntity
import org.noelware.charted.databases.postgres.flags.MemberPermissions
import org.noelware.charted.databases.postgres.flags.OrganizationFlags
import org.noelware.charted.databases.postgres.models.Organization
import org.noelware.charted.extensions.json.toJsonArray
import org.noelware.charted.server.plugins.PreconditionResult
import org.noelware.charted.server.plugins.currentUser
import org.noelware.charted.types.responses.ApiResponse
import org.noelware.ktor.endpoints.AbstractEndpoint

open class AbstractOrganizationEndpoints(path: String): AbstractEndpoint(path) {
    private suspend fun getOrganizationEntityById(call: ApplicationCall): OrganizationEntity? {
        val id = call.parameters["id"] ?: return run {
            call.respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "MISSING_ORG_ID",
                    "Request is missing the ID path parameter",
                ),
            )

            null
        }

        if (id.toLongOrNull() == null) {
            call.respond(
                HttpStatusCode.NotAcceptable,
                ApiResponse.err(
                    "INVALID_SNOWFLAKE",
                    "Parameter [id] was not a valid snowflake",
                ),
            )

            return null
        }

        return asyncTransaction {
            OrganizationEntity.findById(id.toLong())
        } ?: run {
            call.respond(
                HttpStatusCode.NotFound,
                ApiResponse.err(
                    "UNKNOWN_ORGANIZATION",
                    "Organization with ID [$id] was not found.",
                ),
            )
            null
        }
    }

    internal suspend fun getOrganizationById(call: ApplicationCall): Organization? = getOrganizationEntityById(call)?.ifNotNull {
        Organization.fromEntity(this)
    }

    internal suspend fun canAccessOrganization(call: ApplicationCall, org: OrganizationEntity): PreconditionResult {
        if (call.currentUser == null) {
            return PreconditionResult.Failed(
                HttpStatusCode.Unauthorized,
                "MISSING_SESSION",
                "You must be using a session token or API key to access this endpoint",
            )
        }

        if (org.owner.id.value == call.currentUser!!.id) return PreconditionResult.Success

        val flags = OrganizationFlags(org.flags)
        if (flags.has("PRIVATE")) {
            org.members.firstOrNull { it.account.id.value == call.currentUser!!.id }
                ?: return PreconditionResult.Failed(
                    HttpStatusCode.Forbidden,
                    "INVALID_ORGANIZATION_ACCESS",
                    "You do not have permission to view this organization",
                )
        }

        return PreconditionResult.Success
    }

    internal suspend fun orgHasPermission(
        call: ApplicationCall,
        org: OrganizationEntity,
        permission: String
    ): PreconditionResult {
        val result = canAccessOrganization(call, org)
        if (result !is PreconditionResult.Success) return result

        val member = org.members.first { it.account.id.value == call.currentUser!!.id }
        val permissions = MemberPermissions(member.permissions)
        return if (permissions.has(permission)) {
            PreconditionResult.Success
        } else {
            PreconditionResult.Failed(
                HttpStatusCode.Forbidden,
                "INVALID_PERMISSIONS",
                "You do not have the [$permission] permission.",
                buildJsonObject {
                    put("permissions", permissions.enabledFlags().toJsonArray())
                },
            )
        }
    }
}
