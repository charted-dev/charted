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

package org.noelware.charted.server.plugins.sessions.preconditions

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.util.*
import org.noelware.charted.common.types.responses.ApiError
import org.noelware.charted.models.repositories.RepositoryMember
import org.noelware.charted.models.repositories.permissions
import org.noelware.charted.modules.postgresql.controllers.repositories.RepositoryDatabaseController
import org.noelware.charted.modules.postgresql.extensions.fromEntity
import org.noelware.charted.server.extensions.currentUser
import org.noelware.charted.server.plugins.sessions.PreconditionResult

suspend fun canEditMetadata(call: ApplicationCall, controller: RepositoryDatabaseController): PreconditionResult =
    doPermissionCheck(call, controller, "metadata:update")

suspend fun canDeleteMetadata(call: ApplicationCall, controller: RepositoryDatabaseController): PreconditionResult =
    doPermissionCheck(call, controller, "metadata:delete")

private suspend fun doPermissionCheck(call: ApplicationCall, controller: RepositoryDatabaseController, permission: String): PreconditionResult {
    val id = call.parameters.getOrFail<Long>("id")
    val repo = controller.getEntityOrNull(id) ?: return PreconditionResult.Failed(
        HttpStatusCode.NotFound,
        ApiError("UNKNOWN_REPOSITORY", "Repository with ID [$id] was not found"),
    )

    // this will always return false when used in organization repositories
    if (repo.owner == call.currentUser!!.id) return PreconditionResult.Success

    val member = repo.members.singleOrNull { it.account.id.value == call.currentUser!!.id }
        ?: return PreconditionResult.Failed(
            HttpStatusCode.Unauthorized,
            ApiError(
                "UNAUTHORIZED", "You are not a member of this repository",
            ),
        )

    val m = RepositoryMember.fromEntity(member)
    if (!m.permissions.has(permission)) {
        return PreconditionResult.Failed(
            ApiError(
                "MISSING_PERMISSIONS",
                "You are missing the 'metadata:update' permission",
            ),
        )
    }

    return PreconditionResult.Success
}
