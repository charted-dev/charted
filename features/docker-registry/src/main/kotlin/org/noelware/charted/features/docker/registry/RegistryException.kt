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

package org.noelware.charted.features.docker.registry

import io.ktor.http.*
import kotlinx.serialization.json.JsonObject

enum class RegistryErrorCode(val code: String, val description: String, val status: HttpStatusCode) {
    BLOB_UNKNOWN("BLOB_UNKNOWN", "blob unknown to registry", HttpStatusCode.NotFound),
    BLOB_UPLOAD_INVALID("BLOB_UPLOAD_INVALID", "blob upload invalid", HttpStatusCode.BadRequest),
    BLOB_UPLOAD_UNKNOWN("BLOB_UPLOAD_UNKNOWN", "blob upload unknown to registry", HttpStatusCode.NotFound),
    DIGEST_INVALID("DIGEST_INVALID", "provided digest did not match uploaded content", HttpStatusCode.BadRequest),
    MANIFEST_BLOB_UNKNOWN("MANIFEST_BLOB_UNKNOWN", "manifest references a manifest or blob unknown to registry", HttpStatusCode.NotFound),
    MANIFEST_INVALID("MANIFEST_INVALID", "manifest invalid", HttpStatusCode.BadRequest),
    MANIFEST_UNKNOWN("MANIFEST_UNKNOWN", "manifest unknown to registry", HttpStatusCode.NotFound),
    NAME_INVALID("NAME_INVALID", "invalid repository name", HttpStatusCode.BadRequest),
    SIZE_INVALID("SIZE_INVALID", "provided length did not match content length", HttpStatusCode.BadRequest),
    UNAUTHORIZED("UNAUTHORIZED", "authentication required", HttpStatusCode.Unauthorized),
    UNSUPPORTED("UNSUPPORTED", "the operation is unsupported", HttpStatusCode.NotImplemented);
}

/**
 * Represents an exception based off a [RegistryErrorCode].
 * @param errorCode The error code that is used to represent this exception.
 * @param detail Any extra detail to use when sending out the message.
 */
class RegistryException(
    private val errorCode: RegistryErrorCode,
    val detail: JsonObject? = null
): RuntimeException(errorCode.description) {
    val code: String
        get() = errorCode.code

    val description: String
        get() = errorCode.description

    val status: HttpStatusCode
        get() = errorCode.status
}
