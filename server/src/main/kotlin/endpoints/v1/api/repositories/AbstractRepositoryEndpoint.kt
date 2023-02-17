package org.noelware.charted.server.endpoints.v1.api.repositories

import dev.floofy.utils.kotlin.ifNotNull
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import org.noelware.charted.databases.postgres.asyncTransaction
import org.noelware.charted.databases.postgres.entities.RepositoryEntity
import org.noelware.charted.databases.postgres.flags.RepositoryFlags
import org.noelware.charted.databases.postgres.models.Repository
import org.noelware.charted.server.plugins.PreconditionResult
import org.noelware.charted.server.plugins.currentUser
import org.noelware.charted.types.responses.ApiResponse
import org.noelware.ktor.endpoints.AbstractEndpoint

abstract class AbstractRepositoryEndpoint(path: String): AbstractEndpoint(path) {
    internal suspend fun getRepositoryEntityById(call: ApplicationCall): RepositoryEntity? {
        val id = call.parameters["id"] ?: return run {
            call.respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "MISSING_REPO_ID_OR_NAME",
                    "Request is missing the ID path parameter",
                ),
            )

            null
        }

        if (id.toLongOrNull() != null) {
            return asyncTransaction {
                RepositoryEntity.findById(id.toLong())
            }
        }

        call.respond(
            HttpStatusCode.NotAcceptable,
            ApiResponse.err(
                "INVALID_SNOWFLAKE",
                "Parameter [id] was not a valid snowflake",
            ),
        )

        return null
    }

    internal suspend fun getRepositoryById(call: ApplicationCall): Repository? = getRepositoryEntityById(call)?.ifNotNull {
        Repository.fromEntity(this)
    }

    internal fun canAccessRepository(
        call: ApplicationCall,
        repository: RepositoryEntity,
        currentUserID: Long? = call.currentUser?.id
    ): PreconditionResult {
        if (currentUserID == null) {
            return PreconditionResult.Failed(
                HttpStatusCode.Unauthorized,
                "BYPASSED_AUTHENTICATION",
                "You must be using a session token or API key to access this endpoint",
            )
        }

        // If the current user owns the repository, then just pass through.
        if (repository.owner == currentUserID) return PreconditionResult.Success

        // Now, we need to check if the repository is private.
        val flags = RepositoryFlags(repository.flags)
        if (flags.has("PRIVATE")) {
            // First, we need to check if the current user is a repository member
            // if the repository is private
            repository.members.firstOrNull { it.account.id.value == currentUserID }
                ?: return PreconditionResult.Failed(
                    HttpStatusCode.Forbidden,
                    "INVALID_REPOSITORY_ACCESS",
                    "You do not have permission to view this repository",
                )
        }

        // We should be fine, I guess?
        return PreconditionResult.Success
    }
}

/*
internal fun ApplicationCall.repoHasPermission(repository: RepositoryEntity, permission: String): PreconditionResult {
    val result = canAccessRepository(repository)
    if (result !is PreconditionResult.Success) return result

    val member = repository.members.first { it.account.id.value == currentUser!!.id }
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

 */
