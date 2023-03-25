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
import io.ktor.http.content.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.swagger.v3.oas.models.PathItem
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.models.flags.ApiKeyScope.User
import org.noelware.charted.modules.avatars.AvatarModule
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.openapi.toPaths
import org.noelware.charted.server.extensions.currentUser
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController

class UploadUserAvatarRestController(
    private val avatars: AvatarModule
): RestController("/users/@me/avatars", HttpMethod.Post) {
    override val apiVersion: APIVersion = APIVersion.V1
    override fun Route.init() {
        install(Sessions) {
            this += User.Avatar.Update
        }
    }

    override suspend fun call(call: ApplicationCall) {
        val multipart = call.receiveMultipart()
        val part = multipart.readPart() ?: return call.respond(
            HttpStatusCode.BadRequest,
            ApiResponse.err(
                "MISSING_FILE_PART",
                "Unable to determine file part to use as the image",
            ),
        )

        if (part !is PartData.FileItem) {
            return call.respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "NOT_FILE_PART",
                    "Part was not a file.",
                ),
            )
        }

        avatars.updateUserAvatar(call.currentUser!!, part)
        call.respond(HttpStatusCode.Accepted, ApiResponse.ok())
    }

    override fun toPathDsl(): PathItem = toPaths("/users/@me/avatars") {
        post {
            description = "Updates the current authenticated user's avatar"

            requestBody {
                description = "multipart/form-data of the image. If multiple parts were appended, then it will only use the first one"
                contentType(ContentType.MultiPart.FormData)
            }

            response(HttpStatusCode.Accepted) {
                description = "The avatar was successfully updated"
                contentType(ContentType.Application.Json) {
                    schema<ApiResponse.Ok<Unit>>()
                }
            }

            response(HttpStatusCode.BadRequest) {
                description = "If we couldn't find the file part to use, or if the selected part was not a file"
                contentType(ContentType.Application.Json) {
                    schema<ApiResponse.Err>()
                }
            }
        }
    }
}
