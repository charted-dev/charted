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
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.models.flags.ApiKeyScope.User
import org.noelware.charted.modules.avatars.AvatarModule
import org.noelware.charted.modules.openapi.kotlin.dsl.*
import org.noelware.charted.server.extensions.currentUser
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController
import org.noelware.charted.server.routing.openapi.ResourceDescription
import org.noelware.charted.server.routing.openapi.describeResource
import kotlin.reflect.typeOf

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

    companion object: ResourceDescription by describeResource("/users/@me/avatars", {
        description = "Upload a new avatar for the current authenticated user"
        post {
            description = "Upload a new avatar, this can be in a `multipart/form-data` content-type, or in a `text/plain` type with the image being base64 encoded"
            requestBody {
                description = "A multipart form-data of the image itself; if multiple parts were appended to this request, then all subsequent parts except the first one will be used. If this is a 'text/plain' request, then the server will only accept 'data:image/png;base64,<...>' as the body"

                text()
                multipart()
            }

            accepted {
                description = "Avatar was successfully updated, this will return an empty response"
                json {
                    schema(typeOf<ApiResponse.Ok<Unit>>())
                }
            }

            badRequest {
                description = "If the request was a multipart form-data, this will indicate that the selected part was not a File type"
                json {
                    schema<ApiResponse.Err>()
                }
            }

            unauthorized {
                description = "If the session token couldn't be authorized successfully"
                json {
                    schema<ApiResponse.Err>()
                }
            }

            forbidden {
                description = "Whether if the `Authorization` header is not present or the body was not a proper session token"
                json {
                    schema<ApiResponse.Err>()
                }
            }

            notAcceptable {
                description = "This can indicate two types of problems:\n" +
                    "* If it failed at the authentication level, it is indicated that the `Authorization` header was not in a valid format the server can accept,\n" +
                    "* This can also indicate that the request body was not formatted in the way it can be accepted; it has to be in the form of 'data:image/{format};base64,...'"

                json {
                    schema<ApiResponse.Err>()
                }
            }
        }
    })
}
