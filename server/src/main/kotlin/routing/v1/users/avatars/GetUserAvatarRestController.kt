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

package org.noelware.charted.server.routing.v1.users.avatars

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.ktor.server.util.*
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.modules.avatars.AvatarModule
import org.noelware.charted.modules.openapi.kotlin.dsl.*
import org.noelware.charted.modules.postgresql.controllers.users.UserDatabaseController
import org.noelware.charted.modules.postgresql.controllers.getByIdOrNameOrNull
import org.noelware.charted.modules.postgresql.tables.UserTable
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController
import org.noelware.charted.server.routing.openapi.ResourceDescription
import org.noelware.charted.server.routing.openapi.describeResource
import org.noelware.charted.server.util.createBodyWithByteArray

class GetUserAvatarRestController(
    private val avatars: AvatarModule,
    private val controller: UserDatabaseController
): RestController("/users/{idOrName}/avatars/{hash?}") {
    override val apiVersion: APIVersion = APIVersion.V1
    override suspend fun call(call: ApplicationCall) {
        val user = controller.getByIdOrNameOrNull(call.parameters.getOrFail("idOrName"), UserTable::username)
            ?: return call.respond(HttpStatusCode.NotFound)

        val (contentType, bytes) = avatars.retrieveUserAvatar(user, call.parameters["hash"])
            ?: return call.respond(HttpStatusCode.NotFound)

        call.respond(HttpStatusCode.OK, createBodyWithByteArray(bytes, contentType))
    }

    companion object: ResourceDescription by describeResource("/users/{idOrName}/avatars/{hash}", {
        description = "REST controller for fetching a user's current avatar, with an optional hash identifier"
        get {
            description = "Retrieves and returns a user's current avatar, or with the `hash` path parameter, return it by the specific hash"
            idOrName()
            pathParameter {
                description = "Hash of the avatar to look-up for a user."
                required = false
                name = "hash"

                schema<String>()
            }

            ok {
                description = "Avatar that was fetched from the storage service"
                contentType(ContentType.Image.JPEG)
                contentType(ContentType.Image.SVG)
                contentType(ContentType.Image.GIF)
                contentType(ContentType.Image.PNG)
            }

            notFound {
                description = "If the resource was not found"
                json {
                    schema<ApiResponse.Err>()
                }
            }
        }
    })
}
