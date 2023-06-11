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
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.modules.avatars.AvatarModule
import org.noelware.charted.modules.openapi.kotlin.dsl.*
import org.noelware.charted.server.extensions.currentUser
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController
import org.noelware.charted.server.routing.openapi.ResourceDescription
import org.noelware.charted.server.routing.openapi.describeResource
import org.noelware.charted.server.util.createBodyWithByteArray

class GetCurrentUserAvatarRestController(private val avatars: AvatarModule): RestController("/users/@me/avatars/{hash?}") {
    override val apiVersion: APIVersion = APIVersion.V1
    override fun Route.init() {
        install(Sessions)
    }

    override suspend fun call(call: ApplicationCall) {
        val (contentType, bytes) = avatars.retrieveUserAvatar(call.currentUser!!, call.parameters["hash"])
            ?: return call.respond(HttpStatusCode.NotFound)

        call.respond(HttpStatusCode.OK, createBodyWithByteArray(bytes, contentType))
    }

    companion object: ResourceDescription by describeResource("/users/@me/avatars/{hash}", {
        description = "REST controller for fetching the current authenticated user's current avatar, with an optional hash identifier"
        get {
            description = "Retrieves and returns the current authenticated user's current avatar, or with the `hash` path parameter, return it by the specific hash"
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
