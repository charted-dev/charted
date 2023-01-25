@file:JvmName("UserEndpointUtilFunctionsKt")

package org.noelware.charted.server.endpoints.v1.api.users

import dev.floofy.utils.exposed.asyncTransaction
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import org.noelware.charted.ChartedScope
import org.noelware.charted.databases.postgres.entities.UserEntity
import org.noelware.charted.databases.postgres.models.User
import org.noelware.charted.databases.postgres.tables.UserTable
import org.noelware.charted.extensions.regexp.toNameRegex
import org.noelware.charted.types.responses.ApiResponse

internal suspend fun ApplicationCall.getUserByIdOrName(): User? {
    val idOrName = parameters["idOrName"]
        ?: return run {
            respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "MISSING_USER_ID_OR_NAME",
                    "Request is missing the `idOrName` path parameter",
                ),
            )

            null
        }

    return when {
        idOrName.toLongOrNull() != null -> asyncTransaction(ChartedScope) {
            UserEntity.find { UserTable.id eq idOrName.toLong() }.firstOrNull()?.let { entity -> User.fromEntity(entity) }
        } ?: run {
            respond(
                HttpStatusCode.NotFound,
                ApiResponse.err(
                    "UNKNOWN_USER",
                    "User with ID [$idOrName] was not found.",
                ),
            )

            null
        }

        idOrName.toNameRegex(false).matches() -> asyncTransaction(ChartedScope) {
            UserEntity.find { UserTable.username eq idOrName }.firstOrNull()?.let { entity -> User.fromEntity(entity) }
        } ?: run {
            respond(
                HttpStatusCode.NotFound,
                ApiResponse.err(
                    "UNKNOWN_USER",
                    "User with username [$idOrName] was not found.",
                ),
            )

            null
        }

        else -> {
            respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "INVALID_USAGE",
                    "Provided `idOrName` path parameter by request was not a valid snowflake or user name.",
                ),
            )

            null
        }
    }
}
