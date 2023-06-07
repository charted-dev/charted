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
import io.swagger.v3.oas.models.PathItem
import org.noelware.charted.modules.avatars.AvatarModule
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.openapi.toPaths
import org.noelware.charted.server.extensions.addAuthenticationResponses
import org.noelware.charted.server.extensions.currentUser
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController
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

    override fun toPathDsl(): PathItem = toPaths("/users/@me/avatars/{hash}") {
        get {
            description = "Returns the current authenticated user's avatar, if any."

            pathParameter {
                description = "Avatar hash, if this was not provided, then it will find the latest one."
                required = false
                name = "hash"

                schema<String>()
            }

            addAuthenticationResponses()
            response(HttpStatusCode.OK) {
                description = "The avatar itself in bytes"

                contentType(ContentType.Image.JPEG)
                contentType(ContentType.Image.SVG)
                contentType(ContentType.Image.GIF)
                contentType(ContentType.Image.PNG)
            }
        }
    }
}
