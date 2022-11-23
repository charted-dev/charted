/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022 Noelware <team@noelware.org>
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

package org.noelware.charted.server.openapi

import guru.zoroark.tegral.openapi.dsl.*
import io.ktor.http.*
import kotlinx.datetime.LocalDateTime
import org.noelware.charted.ChartedInfo
import org.noelware.charted.databases.postgres.models.User
import org.noelware.charted.server.endpoints.v1.InfoResponse
import org.noelware.charted.server.endpoints.v1.MainResponse
import org.noelware.charted.server.endpoints.v1.api.MainUserResponse
import org.noelware.charted.types.responses.ApiError
import org.noelware.charted.types.responses.ApiResponse

/**
 * Represents the definition for charted-server's OpenAPI specification.
 */
fun RootDsl.charted() {
    title = "charted-server"
    summary = "\uD83D\uDCE6 You know, for Helm Charts?"
    termsOfService = "https://charts.noelware.org/legal/tos"
    version = ChartedInfo.version
    externalDocsUrl = "https://charts.noelware.org/docs"

    contactEmail = "team@noelware.org"
    contactName = "Noelware, LLC."
    contactUrl = "https://noelware.org"

    licenseIdentifier = "Apache-2.0"
    licenseName = "Apache 2.0"
    licenseUrl = "https://www.apache.org/licenses/LICENSE-2.0.txt"

    servers()
    securitySchemes()
    mainEndpoints()
    // usersApiEndpoint()
    // repositoryApiEndpoints()
    // organizationsApiEndpoints()
    // apiKeysEndpoints()
}

fun RootDsl.servers() {
    "https://charts.noelware.org" server {}
}

fun RootDsl.securitySchemes() {
    "sessionToken" securityScheme {
        httpType
        inHeader

        bearerFormat = "jwt"
        description = "Security scheme to use a JWT (JSON Web Token) as authorization of a user."
        scheme = "Bearer"
        name = "Session Token"
    }

    "apiKey" securityScheme {
        apiKeyType
        inHeader

        description = "Security scheme to use a generated API Key to do operations with the API"
        scheme = "ApiKey"
        name = "API Key"
    }
}

fun RootDsl.mainEndpoints() {
    "/" get {
        summary = "Generic main entrypoint"
        externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api#GET-/"

        200 response {
            "application/json" content {
                schema<ApiResponse.Ok<MainResponse>>()
                example = ApiResponse.ok(
                    MainResponse(
                        message = "Hello, world! \uD83D\uDC4B",
                        tagline = "You know, for Helm charts?",
                        docs = "https://charts.noelware.org/docs"
                    )
                )
            }
        }
    }

    "/metrics" get {
        summary = "Returns the Prometheus metrics, if enabled on the server"
        externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api#GET-/metrics"

        200 response {
            "text/plain; version=0.0.4; charset=utf-8" content {
                schema<String>()
            }
        }

        404 response {
            "application/json" content {
                schema<ApiResponse.Err>()
            }
        }
    }

    "/heartbeat" get {
        summary = "Endpoint to signify that the server is healthy"
        externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api#GET-/heartbeat"

        200 response {
            "text/plain" content {
                schema<String>()
                example = "OK"
            }
        }
    }

    "/info" get {
        summary = "Returns any non-revealing information about the server"
        externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api#GET-/info"

        200 response {
            "application/json" content {
                schema<ApiResponse.Ok<InfoResponse>>()
                example = ApiResponse.Ok(
                    InfoResponse(
                        distribution = ChartedInfo.distribution.key,
                        commitHash = ChartedInfo.commitHash,
                        buildDate = ChartedInfo.buildDate,
                        product = "charted-server",
                        version = ChartedInfo.version,
                        vendor = "Noelware"
                    )
                )
            }
        }
    }
}

fun RootDsl.usersApi() {
    "/users" {
        get {
            summary = "Generic entrypoint to the Users API"
            externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/users#GET-/"

            "name" pathParameter {
                description = "The user's username or snowflake ID"
                required = false
                schema<String>()
            }

            200 response {
                "application/json" content {
                    schema<ApiResponse.Ok<MainUserResponse>>()
                    example = ApiResponse.ok(MainUserResponse())
                }
            }
        }

        put {
            summary = "Creates a new user in the database if registrations are enabled. If registrations are not enabled, an administrator has to create your user or give you an invite link"
            externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/users#PUT-/"

            201 response {
                description = "201 - Created"
                "application/json" content {
                    schema<ApiResponse.Ok<User>>()
                    example = User(
                        "cutie@floofy.dev",
                        "18 year old \uD83D\uDC3B\u200D❄ \uD83E\uDE84 in the wild - Lead Developer of charted-server",
                        null,
                        LocalDateTime.parse("2022-11-08T05:03:56.550Z"),
                        LocalDateTime.parse("2022-11-08T05:03:56.550Z"),
                        "noel",
                        0,
                        "Noel \uD83D\uDC3B\u200D❄ \uD83E\uDE84",
                        1
                    )
                }
            }

            406 response {
                description = "406 - Not Acceptable"
                "application/json" content {
                    schema<ApiResponse.Err>()
                    example = ApiResponse.err(ApiError("INVALID_USERNAME", "Username 'noel' didn't comply with username regular expression"))
                }
            }
        }
    }
}
