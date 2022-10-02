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

import org.noelware.charted.server.endpoints.*

title = "charted-server"
summary = "\uD83D\uDCE6 You know, for Helm Charts?"
termsOfService = "https://charts.noelware.org/legal/tos"
version = "0.2-nightly"
externalDocsUrl = "https://charts.noelware.org/docs"

// contact
contactName = "Noelware Team"
contactUrl = "https://noelware.org"
contactEmail = "team@noelware.org"

// licensing
licenseName = "Apache 2.0"
licenseIdentifier = "Apache-2.0"

// production server
"https://charts.noelware.org" server {}

// test server
"http://localhost:3651" server {}

// Session Token Security Scheme
"session" securityScheme {
    httpType
    inHeader

    scheme = "Bearer"
    name = "Session Token"
    description = "This security scheme uses a JSON Web Token to authenticate a user. Meant for the web UI, mainly."
    bearerFormat = "Bearer <token>"
}

// Api Key Security Scheme
"apikey" securityScheme {
    apiKeyType
    inHeader

    scheme = "ApiKey"
    name = "Api Key"
    description = "This security scheme uses the Api Key mechanism, used for API consumption"
    bearerFormat = "ApiKey <token>"
}

// main endpoints
"/" get {
    summary = "Generic main entrypoint"
    externalDocsUrl = "https://charts.noelware.org/docs/server/0.2-nightly/api#GET-/"

    200 response {
        "application/json" content {
            schema<MainResponse>()
        }
    }
}

"/info" get {
    summary = "Returns any non-revealing information about the server to a consumer source."
    externalDocsUrl = "https://charts.noelware.org/docs/server/0.2-nightly/api#GET-/info"

    200 response {
        "application/json" content {
            schema<InfoResponse>()
        }
    }
}

"/features" get {
    summary = "Returns all the features the server has enabled"
    externalDocsUrl = "https://charts.noelware.org/docs/server/0.2-nightly/api#GET-/features"

    200 response {
        "application/json" content {
            schema<FeaturesResponse>()
        }
    }
}

"/health" get {
    summary = "Returns a simple payload to the end user to check server health"
    externalDocsUrl = "https://charts.noelware.org/docs/server/0.2-nightly/api#GET-/health"

    200 response {
        plainText { schema("OK") }
    }
}

"/metrics" get {
    summary = "Returns the Prometheus metrics, if enabled on the server"
    externalDocsUrl = "https://charts.noelware.org/docs/server/0.2-nightly/api#GET-/metrics"

    200 response {
        "text/plain; version=0.0.4; charset=utf-8" content {
            schema<String>()
        }
    }
}
