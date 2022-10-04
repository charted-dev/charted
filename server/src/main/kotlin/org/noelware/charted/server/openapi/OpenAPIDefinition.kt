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
import org.noelware.charted.common.ChartedInfo
import org.noelware.charted.common.data.responses.Response
import org.noelware.charted.database.models.ApiKeys
import org.noelware.charted.server.endpoints.FeaturesResponse
import org.noelware.charted.server.endpoints.InfoResponse
import org.noelware.charted.server.endpoints.MainResponse
import org.noelware.charted.server.endpoints.api.*

/**
 * Represents the definition for charted-server's OpenAPI specification.
 */
fun RootDsl.chartedServer() {
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
    adminEndpoints()
    apiKeysEndpoints()
    organizationEndpoints()
    repositoryEndpoints()
    userEndpoints()
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
                schema<Response.Ok<MainResponse>>()
                example = Response.ok(
                    MainResponse(
                        message = "Hello, world! \uD83D\uDC4B",
                        tagline = "You know, for Helm charts?",
                        docs = "https://charts.noelware.org/docs"
                    )
                )
            }
        }
    }

    "/info" get {
        summary = "Returns any non-revealing information about the server to a consumer source."
        externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api#GET-/info"

        200 response {
            "application/json" content {
                schema<Response.Ok<InfoResponse>>()
                example = Response.ok(
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

    "/features" get {
        summary = "Returns all the features the server has enabled"
        externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api#GET-/features"

        200 response {
            "application/json" content {
                schema<Response.Ok<FeaturesResponse>>()
                example = Response.ok(
                    FeaturesResponse(
                        registrations = true,
                        integrations = mapOf("noelware" to true, "github" to true),
                        enterprise = false,
                        inviteOnly = false,
                        analytics = true,
                        telemetry = true,
                        search = true,
                        engine = "charts",
                        lite = false
                    )
                )
            }
        }
    }

    "/health" get {
        summary = "Returns a simple payload to the recipient to check server health"
        externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api#GET-/health"

        200 response {
            plainText { schema("OK") }
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
                schema<Response.Error>()
            }
        }
    }
}

fun RootDsl.adminEndpoints() {
    "/admin" get {
        summary = "Simple payload to indicate this is the Admin v1 API"
        externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/admin"

        200 response {
            "application/json" content {
                schema<Response.Ok<AdminResponse>>()
                example = Response.ok(
                    AdminResponse(
                        message = "Welcome to the Admin API!",
                        docs = "https://charts.noelware.org/docs/server/api/admin"
                    )
                )
            }
        }
    }

    "/admin/stats" get {
        summary = "Returns full debug statistics for administrators of this instance."
        externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/admin#GET-/stats"

        security("ApiKey")
        security("SessionToken")

        200 response {
            "application/json" content {
                schema<Response.Ok<AdminStatsResponse>>()
            }
        }

        401 response {
            "application/json" content {
                schema<Response.Error>()
            }
        }

        403 response {
            "application/json" content {
                schema<Response.Error>()
            }
        }
    }
}

fun RootDsl.apiKeysEndpoints() {
    "/apikeys" {
        get {
            summary = "Returns a generic entrypoint for the API Keys API."
            externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/apikeys"

            "name" pathParameter {
                description = "The name of the API key to look up for"
                required = false
                schema<String>()
            }

            200 response {
                "application/json" content {
                    schema<Response.Ok<ApiKeysResponse>>()
                    example = Response.ok(
                        ApiKeysResponse(
                            message = "Welcome to the API Keys API!",
                            docs = "https://charts.noelware.org/docs/api/api-keys"
                        )
                    )
                }
            }
        }

        put {
            summary = "Creates a API key on this user's account. Requires a session token to identify the user"
            externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/apikeys#PUT-/"

            security("SessionToken")

            body {
                description = "The metadata for creating the API key"
                "application/json" content {
                    schema<CreateApiKeyBody>()
                }
            }

            201 response {
                "application/json" content {
                    schema<Response.Ok<ApiKeys>>()
                }
            }

            401 response {
                "application/json" content {
                    schema<Response.Error>()
                }
            }

            403 response {
                "application/json" content {
                    schema<Response.Error>()
                }
            }
        }

        delete {
            summary = "Deletes an API key on the server that was tied to this user."
            externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/apikeys#DELETE-/"

            security("ApiKey")

            201 response {
                "application/json" content {
                    schema<Response.Ok<Nothing>>()
                }
            }

            401 response {
                "application/json" content {
                    schema<Response.Error>()
                }
            }

            403 response {
                "application/json" content {
                    schema<Response.Error>()
                }
            }
        }
    }

    "/apikeys/all" get {
        summary = "Returns all the API keys that this user has generated. This excludes the token itself."
        externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/apikeys#GET-/all"

        security("ApiKey")
        security("SessionToken")

        200 response {
            "application/json" content {
                schema<Response.Ok<List<ApiKeys>>>()
            }
        }

        401 response {
            "application/json" content {
                schema<Response.Error>()
            }
        }

        403 response {
            "application/json" content {
                schema<Response.Error>()
            }
        }
    }
}

fun RootDsl.organizationEndpoints() {
    "/organizations" {
        get {
            summary = "Returns a generic entrypoint for the Organizations API"
            externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/organizations"

            "name" pathParameter {
                description = "The name or snowflake ID of the organization"
                required = false
                schema<String>()
            }

            200 response {
                "application/json" content {
                    schema<Response.Ok<ApiKeysResponse>>()
                    example = Response.ok(
                        OrganizationsResponse(
                            message = "Welcome to the Organizations API!",
                            docs = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/organizations"
                        )
                    )
                }
            }
        }

        put {
            summary = "Creates an organization"
            externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/organizations#PUT-/"

            security("SessionToken")
            security("ApiKey")

            415 response {
                "application/json" content {
                    schema<Response.Error>()
                }
            }
        }

        patch {
            summary = "Updates an organization's metadata"
            externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/organizations#PUT-/"

            security("SessionToken")
            security("ApiKey")

            "name" pathParameter {
                description = "The name or snowflake ID of the organization"
                required = false
                schema<String>()
            }

            415 response {
                "application/json" content {
                    schema<Response.Error>()
                }
            }
        }

        delete {
            summary = "Creates an organization"
            externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/organizations#PUT-/"

            security("SessionToken")
            security("ApiKey")

            "name" pathParameter {
                description = "The name or snowflake ID of the organization"
                required = false
                schema<String>()
            }

            415 response {
                "application/json" content {
                    schema<Response.Error>()
                }
            }
        }
    }
}

fun RootDsl.repositoryEndpoints() {
}

fun RootDsl.userEndpoints() {
}
