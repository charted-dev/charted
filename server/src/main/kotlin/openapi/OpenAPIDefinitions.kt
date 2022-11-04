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
import org.noelware.charted.ChartedInfo

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
