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

package org.noelware.charted.server.routing.v1.apikeys

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.swagger.v3.oas.models.PathItem
import kotlinx.datetime.Clock
import kotlinx.datetime.LocalDateTime
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import org.noelware.charted.common.TimeSpan
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.models.ApiKeys
import org.noelware.charted.models.flags.ApiKeyScope
import org.noelware.charted.models.users.User
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.openapi.toPaths
import org.noelware.charted.modules.postgresql.controllers.apikeys.ApiKeysDatabaseController
import org.noelware.charted.modules.postgresql.controllers.apikeys.CreateApiKeyPayload
import org.noelware.charted.modules.postgresql.ktor.UserEntityAttributeKey
import org.noelware.charted.server.extensions.addAuthenticationResponses
import org.noelware.charted.server.extensions.currentUserEntity
import org.noelware.charted.server.extensions.putAndRemove
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController
import kotlin.time.Duration.Companion.days
import kotlin.time.DurationUnit
import kotlin.time.toDuration

class CreateApiKeyRestController(private val controller: ApiKeysDatabaseController): RestController("/apikeys", HttpMethod.Put) {
    override val apiVersion: APIVersion = APIVersion.V1
    override fun Route.init() {
        install(Sessions) {
            this += ApiKeyScope.ApiKeys.Create
        }
    }

    override suspend fun call(call: ApplicationCall): Unit = call.attributes.putAndRemove(UserEntityAttributeKey, call.currentUserEntity!!) {
        val apikey = controller.create(call, call.receive())
        call.respond(HttpStatusCode.Created, ApiResponse.ok(apikey))
    }

    override fun toPathDsl(): PathItem = toPaths("/apikeys") {
        put {
            description = "Creates an API key under the current authenticated user"

            requestBody {
                contentType(ContentType.Application.Json) {
                    schema(
                        CreateApiKeyPayload(
                            "API key to automate some stuff!",
                            TimeSpan.ofString("3 weeks"),
                            listOf("apikeys:create", "user:access"),
                            "some-api-key",
                        ),
                    )
                }
            }

            addAuthenticationResponses()
            response(HttpStatusCode.Created) {
                contentType(ContentType.Application.Json) {
                    schema(
                        ApiResponse.ok(
                            ApiKeys(
                                "API key to automate some stuff!",
                                Clock.System.now().plus(24.days.inWholeMilliseconds.toDuration(DurationUnit.MILLISECONDS)).toLocalDateTime(TimeZone.currentSystemDefault()),
                                536870913,
                                null,
                                User(
                                    true,
                                    null,
                                    null,
                                    null,
                                    LocalDateTime.parse("2023-04-08T02:37:53.741502369"),
                                    LocalDateTime.parse("2023-04-08T02:37:53.741502369"),
                                    "noel",
                                    true,
                                    "Noel",
                                    1,
                                ),
                                "some-api-key",
                                1234,
                            ),
                        ),
                    )
                }
            }
        }
    }
}
