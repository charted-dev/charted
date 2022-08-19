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
import kotlinx.serialization.builtins.ListSerializer
import kotlinx.serialization.builtins.serializer
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.descriptors.buildClassSerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import kotlinx.serialization.encoding.encodeStructure
import kotlinx.serialization.json.JsonEncoder
import org.noelware.charted.common.DebugUtils

internal class KResponseSerializer<T>(private val dataSerializer: KSerializer<T>): KSerializer<Response<T>> {
    private val apiErrorSerializer = ListSerializer(APIError.serializer())
    override val descriptor: SerialDescriptor = buildClassSerialDescriptor("charted.APIResponse") {
        element("success", Boolean.serializer().descriptor)
        element("data", dataSerializer.descriptor, isOptional = true)
        element("errors", apiErrorSerializer.descriptor, isOptional = true)
    }

    override fun deserialize(decoder: Decoder): Response<T> {
        throw IllegalAccessException("Deserialization is not supported in KResponseSerializer.")
    }

    override fun serialize(encoder: Encoder, value: Response<T>) {
        require(encoder is JsonEncoder) { "JSON serialisation is only supported, not encoder=${encoder::class}" }

        encoder.encodeStructure(descriptor) {
            encodeBooleanElement(descriptor, 0, value is Response.Ok)

            when (value) {
                is Response.Ok -> {
                    if (value.data != null) {
                        // This has to happen since using `encodeSerializableElement` doesn't include
                        // the payload as "data": T, so we have to do this for now. If there is any other way,
                        // then submit a PR. :)
                        encoder.encodeSerializableElement(descriptor, 1, dataSerializer, value.data)
                    } else {
                        // do nothing, kotlin is complaining :(
                    }
                }

                is Response.Error -> encodeSerializableElement(descriptor, 2, apiErrorSerializer, value.errors)
            }
        }
    }
}

/**
 * Represents a generic API response. You might want to use the [Ok] and [Error] classes
 * for sending out API responses.
 *
 * @param success If the request was successful or not.
 */
@kotlinx.serialization.Serializable(with = KResponseSerializer::class)
sealed class Response<out T>(val success: Boolean) {
    /**
     * Represents a successful response, with data attached if any.
     * @param data The data to use to send out the response. The [T] generic
     *             must be marked with [Serializable][kotlinx.serialization.Serializable] or
     *             the server will error out.
     */
    data class Ok<out T>(val data: T? = null): Response<T>(true)

    /**
     * Represents an unsuccessful response, with any errors that might've occurred during
     * the invocation of the request.
     *
     * @param errors A list of API errors that might've occurred when invoking the request.
     */
    data class Error(val errors: List<APIError>): Response<Nothing>(false)

    companion object {
        /**
         * Sends out an empty response payload with only the success marker.
         */
        fun ok(): Response<Nothing> = Ok(null)

        /**
         * Sends out a response that is represented as [T].
         * @param data The data payload to send.
         */
        fun <T> ok(data: T): Response<T> = Ok(data)

        /**
         * Sends out a response that represents multiple errors that might've happened during
         * a REST request invocation.
         *
         * @param errors A list of errors to prepend to the payload itself.
         */
        fun err(errors: List<APIError>): Response<Nothing> = Error(errors)

        /**
         * Sends out a response that represents a single error that might've happened during
         * a REST request invocation.
         *
         * @param error the [APIError] object to use.
         */
        fun err(error: APIError): Response<Nothing> = err(listOf(error))

        /**
         * Sends out a response that still represents a single error, but the [code] and [message]
         * will construct a [APIError] object for you to send.
         *
         * @param code The error code that gives a human-readable message in the documentation.
         * @param message The message of what happened.
         */
        fun err(code: String, message: String): Response<Nothing> = err(APIError(code, message))

        /**
         * Sends out a response from a generic [Throwable] object. It'll transform the
         * exception into an [APIError] that the serializer can serialize.
         */
        fun <T: Throwable> err(throwable: T): Response<Nothing> = err(
            APIError(
                "INTERNAL_SERVER_ERROR",
                throwable.message ?: "(empty message)",
                if (DebugUtils.isDebugEnabled() && throwable.cause != null) {
                    APIError.APIErrorCause(
                        throwable.cause!!.message ?: "(empty message)",
                        throwable.cause!!.stackTraceToString()
                    )
                } else {
                    null
                }
            )
        )
    }
}
