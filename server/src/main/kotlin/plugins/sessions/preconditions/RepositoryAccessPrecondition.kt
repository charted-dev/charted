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
import org.noelware.charted.common.types.responses.ApiError
import org.noelware.charted.modules.postgresql.entities.RepositoryEntity
import org.noelware.charted.server.extensions.currentUser
import org.noelware.charted.server.plugins.sessions.PreconditionResult

/**
 * Precondition to determine if an authenticated user can access a repository based off
 * their permissions.
 */
suspend fun canAccessRepository(repository: RepositoryEntity, call: ApplicationCall): PreconditionResult {
    if (call.currentUser == null) {
        return PreconditionResult.Failed(
            HttpStatusCode.Unauthorized,
            ApiError(
                "MISSING_AUTHENTICATION", "This route requires authentication, please sign-in or provide authentication details.",
            ),
        )
    }

    // Owners can bypass any repository they own (even if it is public)
    if (call.currentUser != null && repository.owner == call.currentUser!!.id) return PreconditionResult.Success

    // Check if we have access (or not)
    if (repository.private) {
        repository.members.firstOrNull { it.account.id.value == call.currentUser!!.id }
            ?: return PreconditionResult.Failed(
                HttpStatusCode.Forbidden,
                ApiError(
                    "INVALID_REPOSITORY_ACCESS",
                    "You do not have permission to view this repository",
                ),
            )
    }

    return PreconditionResult.Success
}
