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

@file:JvmName("KtorCreateBodyUtilKt")

package org.noelware.charted.server.util

import io.ktor.http.*
import io.ktor.http.content.*
import io.ktor.util.cio.*
import io.ktor.utils.io.*
import io.ktor.utils.io.jvm.javaio.*
import io.ktor.utils.io.jvm.javaio.toByteReadChannel
import io.ktor.utils.io.pool.*
import io.ktor.utils.io.pool.ByteBufferPool
import io.sentry.Sentry
import io.sentry.kotlin.SentryContext
import org.noelware.charted.ChartedScope
import java.io.ByteArrayInputStream
import java.io.InputStream

/**
 * Creates a [read channel][OutgoingContent.ReadChannelContent] from a given [InputStream].
 * @param is The [InputStream] to consume, if [allowEmpty] is false, then this will fail if this was
 *           already consumed or at 0 length.
 * @param contentType [ContentType] of the payload.
 * @param status [HttpStatusCode] when sending back the response
 * @param allowEmpty If the [input stream][is] should be allowed empty or not.
 */
fun createBodyFromInputStream(
    `is`: InputStream,
    contentType: ContentType,
    status: HttpStatusCode = HttpStatusCode.OK,
    allowEmpty: Boolean = status == HttpStatusCode.NoContent
): OutgoingContent.ReadChannelContent {
    check(!allowEmpty && `is`.available().toLong() != 0L) { "Content-Length cannot be zero" }
    return object: OutgoingContent.ReadChannelContent() {
        override val contentLength: Long = `is`.available().toLong()
        override val contentType: ContentType = contentType
        override val status: HttpStatusCode = status

        override fun readFrom(): ByteReadChannel = `is`.toByteReadChannel(
            ByteBufferPool(4092, 8192),
            if (Sentry.isEnabled()) SentryContext() + ChartedScope.coroutineContext else ChartedScope.coroutineContext,
        )
    }
}

/**
 * Creates a [read channel][OutgoingContent.ReadChannelContent] from a given [ByteArray]. This will wrap
 * the given [ByteArray] into a [ByteArrayInputStream] and calls [createBodyFromInputStream].
 *
 * @param data The [ByteArray] to consume, if [allowEmpty] is false, then this will fail if this was
 *           already consumed or at 0 length.
 * @param contentType [ContentType] of the payload.
 * @param status [HttpStatusCode] when sending back the response
 * @param allowEmpty If the [data][ByteArray] should be allowed empty or not.
 */
fun createBodyWithByteArray(
    data: ByteArray,
    contentType: ContentType,
    status: HttpStatusCode = HttpStatusCode.OK,
    allowEmpty: Boolean = status == HttpStatusCode.NoContent
): OutgoingContent.ReadChannelContent = createBodyFromInputStream(ByteArrayInputStream(data), contentType, status, allowEmpty)
