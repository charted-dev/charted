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

import dev.floofy.utils.koin.inject
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import io.ktor.server.util.*
import org.noelware.charted.common.types.responses.ApiError
import org.noelware.charted.modules.postgresql.controllers.repositories.RepositoryDatabaseController
import org.noelware.charted.server.extensions.currentUser
import org.noelware.charted.server.plugins.sessions.PreconditionResult

suspend fun canAccessRepository(call: ApplicationCall, failOnNoAuth: Boolean = true): PreconditionResult {
    val controller: RepositoryDatabaseController by inject()
    if (call.currentUser == null) {
        if (failOnNoAuth) return PreconditionResult.Failed(ApiError("UNAUTHORIZED", "Unable to view this resource"))
    }

    val id = call.parameters.getOrFail<Long>("id")
    val repo = controller.getEntityOrNull(id) ?: return PreconditionResult.Failed(
        HttpStatusCode.NotFound,
        ApiError("UNKNOWN_REPOSITORY", "Repository with ID [$id] was not found"),
    )

    if (repo.private) {
        if (call.currentUser == null) {
            return PreconditionResult.Failed(
                HttpStatusCode.Unauthorized,
                ApiError(
                    "INVALID_ACCESS",
                    "Repository ${repo.name} is private and you don't have access to it",
                ),
            )
        }

        if (!repo.members.any { it.account.id.value == call.currentUser!!.id }) {
            return PreconditionResult.Failed(
                HttpStatusCode.Unauthorized,
                ApiError(
                    "INVALID_ACCESS",
                    "Repository ${repo.name} is private and you don't have access to it",
                ),
            )
        }
    }

    return PreconditionResult.Success
}
