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
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.core.plugins

import dev.floofy.utils.slf4j.logging
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.application.hooks.*
import io.ktor.server.request.*
import io.ktor.util.*
import org.apache.commons.lang3.time.StopWatch
import org.noelware.charted.core.setOnceAndGet
import java.util.concurrent.TimeUnit

val started by setOnceAndGet<Boolean>()

val KtorLogging = createApplicationPlugin("NoelKtorLogging") {
    val stopwatchKey = AttributeKey<StopWatch>("Stopwatch")
    val log by logging("org.noelware.charted.core.plugins.KtorLoggingKt")

    environment?.monitor?.subscribe(ApplicationStarted) {
        log.info("HTTP service has started successfully! :3")
        started.value = true
    }

    environment?.monitor?.subscribe(ApplicationStopped) {
        log.warn("HTTP service has completely stopped. :(")
    }

    onCall { call ->
        call.attributes.put(stopwatchKey, StopWatch.createStarted())
    }

    on(ResponseSent) { call ->
        val method = call.request.httpMethod
        val version = call.request.httpVersion
        val endpoint = call.request.uri
        val status = call.response.status() ?: HttpStatusCode(-1, "Unknown HTTP Method")
        val stopwatch = call.attributes[stopwatchKey]
        val userAgent = call.request.userAgent()

        stopwatch.stop()
        log.info(
            "${method.value} $version $endpoint :: ${status.value} ${status.description} [$userAgent] [${stopwatch.getTime(
                TimeUnit.MILLISECONDS
            )}ms]"
        )
    }
}
