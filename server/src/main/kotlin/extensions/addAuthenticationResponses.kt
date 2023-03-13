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

package org.noelware.charted.server.extensions

import io.ktor.http.*
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.modules.openapi.kotlin.dsl.OperationDsl
import org.noelware.charted.modules.openapi.kotlin.dsl.schema

fun OperationDsl.addAuthenticationResponses() {
    response(HttpStatusCode.Unauthorized) {
        description = "If the authentication handler couldn't authorize successfully"
        contentType(ContentType.Application.Json) {
            schema<ApiResponse.Err>()
        }
    }

    response(HttpStatusCode.Forbidden) {
        description = "Whether if the `Authorization` header is not present or a REST controller requires the authentication type to be from a Session Token"
        contentType(ContentType.Application.Json) {
            schema<ApiResponse.Err>()
        }
    }

    response(HttpStatusCode.NotFound) {
        description = "If a session couldn't be found based off the authentication details given, or if a user wasn't found (can happen if a user was deleted)"
        contentType(ContentType.Application.Json) {
            schema<ApiResponse.Err>()
        }
    }

    response(HttpStatusCode.NotAcceptable) {
        description = "Whether if the `Authorization` header is not in an acceptable format"
        contentType(ContentType.Application.Json) {
            schema<ApiResponse.Err>()
        }
    }
}
