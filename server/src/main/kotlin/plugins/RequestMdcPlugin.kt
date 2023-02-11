/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

package org.noelware.charted.server.plugins

import io.ktor.server.application.*
import io.ktor.server.application.hooks.*
import io.ktor.server.request.*
import org.slf4j.MDC

val RequestMdc = createApplicationPlugin("RequestMdc") {
    onCall { call ->
        MDC.put("request.uri", call.request.uri)
        MDC.put("request.method", call.request.httpMethod.value)
        MDC.put("http.version", call.request.httpVersion)
    }

    on(ResponseSent) {
        MDC.remove("request.uri")
        MDC.remove("request.method")
        MDC.remove("http.version")
    }
}
