/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware <team@noelware.org>
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

package org.noelware.charted.server.openapi.apis

import guru.zoroark.tegral.openapi.dsl.RootDsl
import guru.zoroark.tegral.openapi.dsl.schema
import kotlinx.datetime.Instant
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import org.noelware.charted.ChartedInfo
import org.noelware.charted.databases.postgres.models.User
import org.noelware.charted.server.endpoints.v1.api.MainUserResponse
import org.noelware.charted.types.responses.ApiResponse
import kotlin.reflect.full.createType

fun RootDsl.usersApi() {
    "/users" {
        get {
            externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/users#GET-/"
            summary = "Generic entrypoint to the Users API if no path parameter is provided, otherwise, you can look up a user's information."

            200 response {
                "application/json" content {
                    schema<ApiResponse.Ok<MainUserResponse>>()
                    example = ApiResponse.ok(MainUserResponse())
                }
            }
        }

        put {
            externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/users#PUT-/"
            summary = "Creates a new user in the database if registrations are enabled. If registrations are not enabled, an administrator has to create your user or give you an invite link"

            201 response {
                "application/json" content {
                    schema<ApiResponse.Ok<User>>()
                }
            }

            403 response {
                description = "If this instance has registrations disabled"
                "application/json" content {
                    schema<ApiResponse.Err>()
                    example = ApiResponse.err("REGISTRATIONS_DISABLED", "This instance has registrations disabled.")
                }
            }

            406 response {
                description = "If any validation errors occurred while running the REST handler."
                "application/json" content {
                    schema<ApiResponse.Err>()
                    example = ApiResponse.err("VALIDATION_EXCEPTION", "Username [noel] is already taken!")
                }
            }

            503 response {
                description = "If this instance uses any other session manager rather than the local one"
                "application/json" content {
                    schema<ApiResponse.Err>()
                    example = ApiResponse.err(
                        "REST_HANDLER_UNAVAILABLE", "Route handler is not implemented at this moment!",
                        buildJsonObject {
                            put("method", "PUT")
                            put("url", "/users")
                        }
                    )
                }
            }
        }

        patch {
            externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/users#PATCH-/"
            summary = "Mutates a user's metadata with a given session token or API key"

            security("sessionToken")
            security("apiKey")

            202 response {
                "application/json" content {
                    schema<ApiResponse.Ok<User>>()
                    example = ApiResponse.ok(
                        User(
                            "noel@noelware.org",
                            "a blepper in the wild :quantD:",
                            null,
                            Instant.parse("2022-11-08T05:03:56.550Z").toLocalDateTime(TimeZone.currentSystemDefault()),
                            Instant.parse("2022-12-16T07:07:32.473Z").toLocalDateTime(TimeZone.currentSystemDefault()),
                            "noel",
                            0,
                            "Noel",
                            1
                        )
                    )
                }
            }

            406 response {
                description = "If any validation errors occurred while running the REST handler."
                "application/json" content {
                    schema<ApiResponse.Err>()
                    example = ApiResponse.err("VALIDATION_EXCEPTION", "Username [noel] is already taken!")
                }
            }
        }

        delete {
            externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/users#DELETE-/"
            summary = "Deletes the current user from the database with the given session token. This only deletes the database entry of this user, not any other entries from other session managers!"

            security("sessionToken")

            202 response {
                "application/json" content {
                    schema<ApiResponse.Ok<Unit>>()
                    example = ApiResponse.ok()
                }
            }
        }
    }

    "/{idOrName}" {
        get {
            externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/users#GET-/:idOrName"
            description = "Retrieve a user from the database"

            200 response {
                description = "The user response if the user was found"
                "application/json" content {
                    schema<ApiResponse.Ok<User>>()
                    example = ApiResponse.ok(
                        User(
                            "cutie@floofy.dev",
                            "18 year old \uD83D\uDC3B\u200D❄ \uD83E\uDE84 in the wild - Lead Developer of charted-server",
                            null,
                            Instant.parse("2022-11-08T05:03:56.550Z").toLocalDateTime(TimeZone.currentSystemDefault()),
                            Instant.parse("2022-11-08T05:03:56.550Z").toLocalDateTime(TimeZone.currentSystemDefault()),
                            "noel",
                            0,
                            "Noel \uD83D\uDC3B\u200D❄ \uD83E\uDE84",
                            1
                        )
                    )
                }
            }

            400 response {
                description = "If the name parameter was not a snowflake or username"
                "application/json" content {
                    schema<ApiResponse.Err>()
                    example = ApiResponse.err("UNKNOWN_ENTITY", "Unable to determine if [idOrName] provided is by ID or name, provided [///wuff///]")
                }
            }

            404 response {
                description = "If a user by the name parameter was not found"
                "application/json" content {
                    schema<ApiResponse.Err>()
                    example = ApiResponse.err("UNKNOWN_USER", "User with ID or name [noel] was not found")
                }
            }
        }
    }

    "/{idOrName}/avatars/current.png" {
        get {
            externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/users#GET-/:idOrName/avatars/current.png"
            description = "Returns the user's avatar, if there is one set."

            200 response {
                description = "The actual avatar as an image file"
                "image/png" content {
                    schema(ByteArray::class.createType())
                }

                "image/jpg" content {
                    schema(ByteArray::class.createType())
                }

                "image/jpeg" content {
                    schema(ByteArray::class.createType())
                }

                "image/gif" content {
                    schema(ByteArray::class.createType())
                }
            }
        }
    }
}
