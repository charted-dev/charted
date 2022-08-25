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

package org.noelware.charted.common.data.responses

import kotlinx.serialization.KSerializer
import kotlinx.serialization.builtins.serializer
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.descriptors.buildClassSerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import kotlinx.serialization.encoding.encodeStructure

/**
 * Represents an error that occurred when invoking a REST handler.
 *
 * @param code The error code that gives a human-readable message in the documentation.
 * @param message The message of what happened.
 * @param cause The exception cause, that might've occurred.
 */
@kotlinx.serialization.Serializable(with = APIError.Companion::class)
data class APIError(
    val code: String,
    val message: String,
    val cause: APIErrorCause? = null
) {
    /**
     * Represents the exception cause, if any. This is only suited for development,
     * so this is never populated in production.
     *
     * @param message The message of what happened.
     * @param stacktrace A Java stacktrace of what happened.
     */
    @kotlinx.serialization.Serializable
    data class APIErrorCause(
        val message: String,
        val stacktrace: String
    )

    // this only exists so `cause` isn't present in the response even though it exists as `null`!
    //
    // kotlinx.serialization doesn't have a `@SkipIfNull` annotation, so this is the best we'll do.
    companion object: KSerializer<APIError> {
        override val descriptor: SerialDescriptor = buildClassSerialDescriptor("charted.APIError") {
            element("code", String.serializer().descriptor)
            element("message", String.serializer().descriptor)
            element("cause", APIErrorCause.serializer().descriptor, isOptional = true)
        }

        override fun deserialize(decoder: Decoder): APIError {
            throw IllegalAccessException("Deserialization is not supported in APIError.")
        }

        override fun serialize(encoder: Encoder, value: APIError) = encoder.encodeStructure(descriptor) {
            encodeStringElement(descriptor, 0, value.code)
            encodeStringElement(descriptor, 1, value.message)
            if (value.cause != null) {
                encodeSerializableElement(descriptor, 2, APIErrorCause.serializer(), value.cause)
            }
        }
    }
}
