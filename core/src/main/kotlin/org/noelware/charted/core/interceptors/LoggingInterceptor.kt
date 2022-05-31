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

package org.noelware.charted.core.interceptors

import dev.floofy.utils.slf4j.logging
import okhttp3.Interceptor
import okhttp3.Response
import org.apache.commons.lang3.time.StopWatch
import java.util.concurrent.TimeUnit

object LoggingInterceptor: Interceptor {
    private val log by logging<LoggingInterceptor>()

    override fun intercept(chain: Interceptor.Chain): Response {
        val request = chain.request()
        val watch = StopWatch.createStarted()

        log.info("-> ${request.method.uppercase()} ${request.url}")
        val res = chain.proceed(request)
        watch.stop()

        log.info(
            "<- [${res.code} ${res.message.ifEmpty { "OK" }} / ${res.protocol.toString().replace("h2", "http/2")}] ${request.method.uppercase()} ${request.url} [${watch.getTime(
                TimeUnit.MILLISECONDS
            )}ms]"
        )

        return res
    }
}
