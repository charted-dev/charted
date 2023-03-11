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
import io.ktor.server.application.*
import io.ktor.server.plugins.*
import io.ktor.server.request.*

/**
 * Returns the true, real IP if the application is running behind the proxy. If not, it'll
 * return the origin host from [request.origin][io.ktor.server.request.ApplicationRequest.origin]
 */
public val ApplicationCall.realIP: String
    get() {
        val headers = request.headers
        return if (headers.contains("True-Client-IP")) {
            headers["True-Client-IP"]!!
        } else if (headers.contains("X-Real-IP")) {
            headers["X-Real-IP"]!!
        } else if (headers.contains(HttpHeaders.XForwardedFor)) {
            var index = headers[HttpHeaders.XForwardedFor]!!.indexOf(", ")
            if (index == -1) {
                index = headers[HttpHeaders.XForwardedFor]!!.length
            }

            if (index == headers[HttpHeaders.XForwardedFor]!!.length) {
                headers[HttpHeaders.XForwardedFor]!!
            } else {
                headers[HttpHeaders.XForwardedFor]!!.slice(0..index)
            }
        } else {
            request.origin.remoteHost
        }
    }
