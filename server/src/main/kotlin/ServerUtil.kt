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

package org.noelware.charted.server

import io.ktor.http.*
import io.ktor.http.content.*
import io.ktor.util.cio.*
import io.ktor.utils.io.*
import io.ktor.utils.io.pool.ByteBufferPool
import io.sentry.Sentry
import io.sentry.kotlin.SentryContext
import org.noelware.charted.ChartedScope
import java.io.ByteArrayInputStream
import java.io.InputStream
import java.util.concurrent.atomic.AtomicBoolean

/** Returns a [AtomicBoolean] of if the server has started. */
val hasStarted: AtomicBoolean = AtomicBoolean(false)

/** The boot time (in nanoseconds) */
val bootTime: Long = System.nanoTime()

fun <T: InputStream> createKtorContentWithInputStream(
    `is`: T,
    contentType: ContentType,
    contentLength: Long = `is`.available().toLong(),
    status: HttpStatusCode = HttpStatusCode.OK
): OutgoingContent.ReadChannelContent {
    check(contentLength != 0L) { "Content-Length can't be 0" }
    return object: OutgoingContent.ReadChannelContent() {
        override val contentType: ContentType = contentType
        override val contentLength: Long = contentLength
        override val status: HttpStatusCode = status
        override fun readFrom(): ByteReadChannel = `is`.toByteReadChannel(
            ByteBufferPool(4092, 8192),
            if (Sentry.isEnabled()) SentryContext() + ChartedScope.coroutineContext else ChartedScope.coroutineContext
        )
    }
}

fun createKtorContentWithByteArray(
    bytes: ByteArray,
    contentType: ContentType,
    contentLength: Long = bytes.size.toLong(),
    status: HttpStatusCode = HttpStatusCode.OK
): OutgoingContent.ReadChannelContent = object: OutgoingContent.ReadChannelContent() {
    override val contentType: ContentType = contentType
    override val contentLength: Long = contentLength
    override val status: HttpStatusCode = status

    override fun readFrom(): ByteReadChannel = ByteArrayInputStream(bytes).toByteReadChannel(
        ByteBufferPool(4092, 8192),
        if (Sentry.isEnabled()) SentryContext() + ChartedScope.coroutineContext else ChartedScope.coroutineContext
    )
}
