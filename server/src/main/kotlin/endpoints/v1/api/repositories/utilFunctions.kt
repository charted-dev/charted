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

package org.noelware.charted.server.endpoints.v1.api.repositories

import dev.floofy.utils.kotlin.ifNotNull
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import kotlinx.serialization.json.buildJsonObject
import org.noelware.charted.databases.postgres.asyncTransaction
import org.noelware.charted.databases.postgres.entities.RepositoryEntity
import org.noelware.charted.databases.postgres.flags.MemberPermissions
import org.noelware.charted.databases.postgres.flags.RepositoryFlags
import org.noelware.charted.databases.postgres.models.Repository
import org.noelware.charted.extensions.json.toJsonArray
import org.noelware.charted.server.plugins.PreconditionResult
import org.noelware.charted.server.plugins.currentUser
import org.noelware.charted.types.responses.ApiResponse

// +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
//             Fetching utilities
// +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+

internal suspend fun ApplicationCall.getRepositoryEntityById(): RepositoryEntity? {
    val idOrName = parameters["id"]
        ?: return run {
            respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "MISSING_REPO_ID_OR_NAME",
                    "Request is missing the ID path parameter",
                ),
            )

            null
        }

    if (idOrName.toLongOrNull() == null) {
        respond(
            HttpStatusCode.NotAcceptable,
            ApiResponse.err(
                "INVALID_SNOWFLAKE",
                "Parameter [id] was not a valid snowflake",
            ),
        )

        return null
    }

    return asyncTransaction {
        RepositoryEntity.findById(idOrName.toLong())
    } ?: run {
        respond(
            HttpStatusCode.NotFound,
            ApiResponse.err(
                "UNKNOWN_REPOSITORY",
                "Repository with ID [$idOrName] was not found.",
            ),
        )
        null
    }
}

internal suspend fun ApplicationCall.getRepositoryById(): Repository? =
    getRepositoryEntityById()?.ifNotNull {
        Repository.fromEntity(this)
    }

// +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+
//  Repository Member Permission Preconditions
// +~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+

internal fun ApplicationCall.canAccessRepository(repository: RepositoryEntity): PreconditionResult {
    if (currentUser == null) {
        return PreconditionResult.Failed(
            HttpStatusCode.Unauthorized,
            "MISSING_SESSION",
            "You must be using a session token or API key to access this endpoint",
        )
    }

    // If the current user owns the repository, then just pass through.
    if (currentUser != null && repository.owner == currentUser!!.id) return PreconditionResult.Success

    // Now, we need to check if the repository is private.
    val flags = RepositoryFlags(repository.flags)
    if (flags.has("PRIVATE")) {
        // First, we need to check if the current user is a repository member
        // if the repository is private
        repository.members.firstOrNull { it.account.id.value == currentUser!!.id }
            ?: return PreconditionResult.Failed(
                HttpStatusCode.Forbidden,
                "INVALID_REPOSITORY_ACCESS",
                "You do not have permission to view this repository",
            )
    }

    // We should be fine, I guess?
    return PreconditionResult.Success
}

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
