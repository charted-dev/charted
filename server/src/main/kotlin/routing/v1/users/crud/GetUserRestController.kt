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

package org.noelware.charted.server.routing.v1.users.crud

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import io.ktor.server.util.*
import io.swagger.v3.oas.models.PathItem
import org.noelware.charted.common.extensions.regexp.matchesNameRegex
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.modules.postgresql.controllers.EntityNotFoundException
import org.noelware.charted.modules.postgresql.controllers.get
import org.noelware.charted.modules.postgresql.controllers.getByProp
import org.noelware.charted.modules.postgresql.controllers.users.UserController
import org.noelware.charted.modules.postgresql.tables.UserTable
import org.noelware.charted.server.routing.RestController

class GetUserRestController(private val controller: UserController): RestController("/users/{idOrName}") {
    override suspend fun call(call: ApplicationCall) {
        val idOrName = call.parameters.getOrFail("idOrName")
        return when {
            idOrName.toLongOrNull() != null -> try {
                call.respond(HttpStatusCode.OK, ApiResponse.ok(controller.get(idOrName.toLong())))
            } catch (e: EntityNotFoundException) {
                call.respond(
                    HttpStatusCode.NotFound,
                    ApiResponse.err(
                        "UNKNOWN_USER", "User with ID [$idOrName] was not found",
                    ),
                )
            }

            idOrName.matchesNameRegex() -> try {
                call.respond(HttpStatusCode.OK, ApiResponse.ok(controller.getByProp(UserTable::username to idOrName)))
            } catch (e: EntityNotFoundException) {
                call.respond(
                    HttpStatusCode.NotFound,
                    ApiResponse.err(
                        "UNKNOWN_USER", "User with ID [$idOrName] was not found",
                    ),
                )
            }

            else -> call.respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "INVALID_USAGE",
                    "Provided idOrName parameter by request was not a valid snowflake or username.",
                ),
            )
        }
    }

    override fun toPathDsl(): PathItem {
        TODO("Not yet implemented")
    }
}
