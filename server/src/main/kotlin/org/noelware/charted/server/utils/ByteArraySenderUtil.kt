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

package org.noelware.charted.server.utils

import io.ktor.http.*
import io.ktor.http.content.*
import io.ktor.util.cio.*
import io.ktor.utils.io.*
import io.ktor.utils.io.pool.ByteBufferPool
import io.sentry.Sentry
import io.sentry.kotlin.SentryContext
import org.noelware.charted.common.ChartedScope
import java.io.ByteArrayInputStream

fun createOutgoingContentWithBytes(
    bytes: ByteArray,
    contentLength: Long = bytes.size.toLong(),
    contentType: ContentType,
    statusCode: HttpStatusCode = HttpStatusCode.OK
): OutgoingContent.ReadChannelContent = object: OutgoingContent.ReadChannelContent() {
    override val contentType: ContentType = contentType
    override val contentLength: Long = contentLength
    override val status: HttpStatusCode = statusCode

    override fun readFrom(): ByteReadChannel = ByteArrayInputStream(bytes).toByteReadChannel(
        ByteBufferPool(4092, 8192),
        if (Sentry.isEnabled()) SentryContext() + ChartedScope.coroutineContext else ChartedScope.coroutineContext
    )
}
