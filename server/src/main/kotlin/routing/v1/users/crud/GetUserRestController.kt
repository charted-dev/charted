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
import org.noelware.charted.common.extensions.regexp.matchesNameAndIdRegex
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.models.users.User
import org.noelware.charted.modules.openapi.NameOrSnowflake
import org.noelware.charted.modules.openapi.kotlin.dsl.*
import org.noelware.charted.modules.openapi.toPaths
import org.noelware.charted.modules.postgresql.controllers.EntityNotFoundException
import org.noelware.charted.modules.postgresql.controllers.get
import org.noelware.charted.modules.postgresql.controllers.users.UserDatabaseController
import org.noelware.charted.modules.postgresql.tables.UserTable
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController
import org.noelware.charted.server.routing.openapi.ResourceDescription
import org.noelware.charted.server.routing.openapi.describeResource
import kotlin.reflect.typeOf

class GetUserRestController(private val controller: UserDatabaseController): RestController("/users/{idOrName}") {
    override val apiVersion: APIVersion = APIVersion.V1
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

            idOrName.matchesNameAndIdRegex() -> try {
                call.respond(HttpStatusCode.OK, ApiResponse.ok(controller.get(UserTable::username to idOrName)))
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

    override fun toPathDsl(): PathItem = toPaths("/users/{idOrName}") {
        get {
            description = "Retrieves a user from the database"
            pathParameter {
                description = "The snowflake or username to use"
                name = "idOrName"

                schema<NameOrSnowflake>()
            }

            ok {
                json {
                    schema<ApiResponse.Ok<User>>()
                }
            }

            badRequest {
                description = "If the provided idOrName parameter wasn't a snowflake or username"
                json {
                    schema<ApiResponse.Err>()
                }
            }

            notFound {
                description = "If a user by the idOrName parameter was not found"
                json {
                    schema<ApiResponse.Err>()
                }
            }
        }
    }

    companion object: ResourceDescription by describeResource("/users/{idOrName}", {
        description = "REST controller to retrieve a user from the server"
        get {
            description = "Retrieve a User object from the server"
            idOrName()

            ok {
                description = "Resource was found from the server"
                json {
                    schema(typeOf<ApiResponse.Ok<User>>())
                }
            }

            badRequest {
                description = "If the `idOrName` path parameter was not a [Snowflake] or a [Name]"
                json {
                    schema<ApiResponse.Err>()
                }
            }

            notFound {
                description = "If the resource was not found."
                json {
                    schema<ApiResponse.Err>()
                }
            }
        }
    })
}
